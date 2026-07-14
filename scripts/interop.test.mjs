import test from "node:test";
import assert from "node:assert/strict";
import {
  eventEnvelopeToOtelSpan,
  eventEnvelopeToProv,
  otelSpanToEventFields,
  provenanceEdgeToOtelLink,
  provenanceEdgeToProv,
  provRelationToProvenanceEdge,
} from "../.tmp/interop/index.js";

const event = {
  schemaVersion: "0.1.0",
  eventId: "evt_01JABCDE1234",
  eventType: "artifact.observed",
  occurredAt: "2026-07-11T12:00:00.123456789+02:30",
  recordedAt: "2026-07-11T12:00:01Z",
  runId: "run_tenant_secret",
  traceId: "4bf92f3577b34da6a3ce929d0e0e4736",
  spanId: "00f067aa0ba902b7",
  actor: { kind: "agent", id: "person@example.com", software: "tenant-secret" },
  subjectRefs: ["customer:secret"],
  payload: { sensitiveField: "do-not-export", nested: { prompt: "secret" } },
  security: { sensitivity: "private", contentMode: "hash_only", retentionClass: "user_managed" },
  integrity: { payloadSha256: "a".repeat(64), previousEventSha256: null },
};

test("OTel mapping preserves nanoseconds and omits sensitive identifiers", () => {
  const result = eventEnvelopeToOtelSpan(event);
  assert.deepEqual(result.errors, []);
  assert.equal(result.value.startTimeUnixNano, "1783762200123456789");
  assert.equal(result.value.endTimeUnixNano, result.value.startTimeUnixNano);
  const serialized = JSON.stringify(result);
  for (const secret of ["do-not-export", "person@example.com", "tenant-secret", "customer:secret"]) assert.equal(serialized.includes(secret), false);
  assert.ok(result.losses.some((loss) => loss.path === "endTimeUnixNano"));
  const reverse = otelSpanToEventFields(result.value);
  assert.equal(reverse.value.occurredAt, "2026-07-11T09:30:00.123456789Z");
});

test("secret_never_export fails closed", () => {
  const result = eventEnvelopeToOtelSpan({ ...event, security: { ...event.security, sensitivity: "secret_never_export" } });
  assert.equal(result.value, undefined);
  assert.match(result.errors[0], /cannot be exported/);
});

test("timestamps reject invalid calendar days and trailing data", () => {
  for (const occurredAt of ["2026-02-30T00:00:00Z", "2026-01-01T00:00:00Zjunk", "2026-01-01 00:00:00Z", "2026-01-01T00:00:00.1234567890Z"]) {
    assert.equal(eventEnvelopeToOtelSpan({ ...event, occurredAt }).value, undefined);
  }
});

test("OTel mapping rejects invalid and zero context IDs", () => {
  assert.equal(eventEnvelopeToOtelSpan({ ...event, traceId: "0".repeat(32) }).value, undefined);
  assert.equal(eventEnvelopeToOtelSpan({ ...event, spanId: "ABCDEF0123456789" }).value, undefined);
});

test("PROV maps event nodes without payload and declares subjects without inferred relations", () => {
  const result = eventEnvelopeToProv(event);
  assert.deepEqual(result.errors, []);
  assert.equal(result.value.activities.length, 1);
  assert.equal(result.value.agents.length, 1);
  assert.equal(result.value.entities.length, 0);
  assert.equal(result.value.relations.length, 0);
  assert.equal(JSON.stringify(result).includes("do-not-export"), false);
  for (const secret of ["person@example.com", "tenant-secret", "customer:secret"]) assert.equal(JSON.stringify(result).includes(secret), false);
});

test("all provenance relationships retain direction and round trip", () => {
  const mappings = {
    generated_by: "wasGeneratedBy", used: "used", derived_from: "wasDerivedFrom",
    attributed_to: "wasAttributedTo", acted_on_behalf_of: "actedOnBehalfOf", invalidated_by: "wasInvalidatedBy",
  };
  for (const [relationship, type] of Object.entries(mappings)) {
    const edge = { edgeId: "edge_01JABCDE1234", relationship, fromRef: "from", toRef: "to", recordedAt: "2026-07-11T12:00:00Z" };
    const mapped = provenanceEdgeToProv(edge);
    assert.equal(mapped.value.type, type);
    assert.equal(mapped.value.source, "from");
    assert.equal(mapped.value.target, "to");
    assert.deepEqual(provRelationToProvenanceEdge(mapped.value).value, edge);
  }
});

test("edge custom attributes stay private in PROV and OTel", () => {
  const edge = { edgeId: "edge_secret_identifier", relationship: "used", fromRef: "tenant-secret-from", toRef: "tenant-secret-to", recordedAt: "2026-07-11T12:00:00Z", attributes: { token: "edge-secret" } };
  const prov = provenanceEdgeToProv(edge);
  const otel = provenanceEdgeToOtelLink(edge, { traceId: event.traceId, spanId: event.spanId });
  assert.equal(JSON.stringify(prov).includes("edge-secret"), false);
  assert.equal(JSON.stringify(otel).includes("edge-secret"), false);
  assert.equal(JSON.stringify(otel).includes("tenant-secret"), false);
  assert.ok(prov.losses.some((loss) => loss.path === "attributes"));
  assert.ok(otel.losses.some((loss) => loss.path === "attributes"));
});

test("OTel timestamp range rejects pre-epoch and uint64 overflow", () => {
  assert.equal(eventEnvelopeToOtelSpan({ ...event, occurredAt: "1969-12-31T23:59:59.999999999Z" }).value, undefined);
  const span = eventEnvelopeToOtelSpan(event).value;
  assert.equal(otelSpanToEventFields({ ...span, startTimeUnixNano: "18446744073709551616" }).value, undefined);
  assert.equal(otelSpanToEventFields({ ...span, endTimeUnixNano: "18446744073709551616" }).value, undefined);
  assert.equal(otelSpanToEventFields({ ...span, endTimeUnixNano: (BigInt(span.startTimeUnixNano) - 1n).toString() }).value, undefined);
  assert.equal(otelSpanToEventFields({ ...span, attributes: { ...span.attributes, "solledger.recorded_at": "2026-02-30T00:00:00Z" } }).value, undefined);
});

test("unknown attribute names never leak through loss reports", () => {
  const span = eventEnvelopeToOtelSpan(event).value;
  const result = otelSpanToEventFields({ ...span, attributes: { ...span.attributes, "api-key-sk_secret": "x" } });
  assert.equal(JSON.stringify(result).includes("sk_secret"), false);
});

test("prototype relationship names fail closed", () => {
  const edge = { edgeId: "edge_01JABCDE1234", relationship: "toString", fromRef: "from", toRef: "to", recordedAt: "2026-07-11T12:00:00Z" };
  assert.equal(provenanceEdgeToProv(edge).value, undefined);
});
