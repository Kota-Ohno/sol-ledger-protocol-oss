import type { EventEnvelope, ProvenanceEdge } from "../index.js";
import {
  type AdapterResult,
  type LossItem,
  fromUnixNano,
  isSpanId,
  isTraceId,
  parseTimestamp,
} from "./common.js";

export type OtelAttributeValue = string | number | boolean | string[] | number[] | boolean[];

export interface OtelResource {
  attributes: Record<string, OtelAttributeValue>;
}

export interface OtelSpan {
  traceId: string;
  spanId: string;
  name: string;
  startTimeUnixNano: string;
  endTimeUnixNano: string;
  attributes: Record<string, OtelAttributeValue>;
  resource: OtelResource;
}

export interface OtelSpanLink {
  traceId: string;
  spanId: string;
  attributes: Record<string, OtelAttributeValue>;
}

const SAFE_ATTRIBUTE_KEYS = new Set([
  "solledger.schema_version",
  "solledger.event_id",
  "solledger.event_type",
  "solledger.recorded_at",
  "solledger.run_id",
  "solledger.actor.kind",
  "solledger.actor.id",
  "solledger.actor.software",
  "solledger.subject_refs",
  "solledger.security.sensitivity",
  "solledger.security.content_mode",
  "solledger.security.retention_class",
  "solledger.integrity.payload_sha256",
  "solledger.integrity.previous_event_sha256",
]);
const MAX_ATTRIBUTE_COUNT = 128;
const MAX_ATTRIBUTE_STRING_LENGTH = 4096;

export function eventEnvelopeToOtelSpan(event: EventEnvelope): AdapterResult<OtelSpan> {
  const errors: string[] = [];
  const losses: LossItem[] = [
    { path: "payload", reason: "privacy", detail: "payload is never expanded into telemetry attributes" },
    { path: "endTimeUnixNano", reason: "not_mapped", detail: "EventEnvelope has no duration; end time is synthesized equal to start time" },
  ];
  if (event.security.sensitivity === "secret_never_export") {
    errors.push("secret_never_export events cannot be exported to telemetry");
    return { losses, errors };
  }
  if (!event.traceId || !isTraceId(event.traceId)) errors.push("traceId must be a non-zero 32-character lowercase hex W3C trace ID");
  if (!event.spanId || !isSpanId(event.spanId)) errors.push("spanId must be a non-zero 16-character lowercase hex W3C span ID");
  const occurredAt = parseTimestamp(event.occurredAt, "occurredAt", errors);
  parseTimestamp(event.recordedAt, "recordedAt", errors);
  if (errors.length || !occurredAt || !event.traceId || !event.spanId) return { losses, errors };

  const attributes: Record<string, OtelAttributeValue> = {
    "solledger.schema_version": event.schemaVersion,
    "solledger.event_id": event.eventId,
    "solledger.event_type": event.eventType,
    "solledger.recorded_at": event.recordedAt,
    "solledger.actor.kind": event.actor.kind,
    "solledger.security.sensitivity": event.security.sensitivity,
    "solledger.security.content_mode": event.security.contentMode,
    "solledger.security.retention_class": event.security.retentionClass,
    "solledger.integrity.payload_sha256": event.integrity.payloadSha256,
  };
  if (event.runId) losses.push({ path: "runId", reason: "privacy", detail: "raw run identifiers require an explicit future export policy" });
  losses.push({ path: "actor.id", reason: "privacy", detail: "raw actor identifiers require an explicit future export policy" });
  if (event.actor.software) losses.push({ path: "actor.software", reason: "privacy", detail: "software identifiers require an explicit future export policy" });
  if (event.subjectRefs?.length) losses.push({ path: "subjectRefs", reason: "privacy", detail: "raw subject references require an explicit future export policy" });
  if (event.integrity.previousEventSha256 !== undefined) {
    attributes["solledger.integrity.previous_event_sha256"] = event.integrity.previousEventSha256 ?? "";
  }
  return {
    value: {
      traceId: event.traceId,
      spanId: event.spanId,
      name: event.eventType,
      startTimeUnixNano: occurredAt.unixNano,
      endTimeUnixNano: occurredAt.unixNano,
      attributes,
      resource: { attributes: {} },
    },
    losses,
    errors,
  };
}

export function otelSpanToEventFields(span: OtelSpan): AdapterResult<Partial<EventEnvelope>> {
  const errors: string[] = [];
  const losses: LossItem[] = [];
  if (!isTraceId(span.traceId)) errors.push("traceId must be a non-zero 32-character lowercase hex W3C trace ID");
  if (!isSpanId(span.spanId)) errors.push("spanId must be a non-zero 16-character lowercase hex W3C span ID");
  const occurredAt = fromUnixNano(span.startTimeUnixNano, "startTimeUnixNano", errors);
  const endAt = fromUnixNano(span.endTimeUnixNano, "endTimeUnixNano", errors);
  if (occurredAt && endAt && BigInt(span.endTimeUnixNano) < BigInt(span.startTimeUnixNano)) errors.push("endTimeUnixNano must not precede startTimeUnixNano");
  const attributeKeys = Object.keys(span.attributes);
  if (attributeKeys.length > MAX_ATTRIBUTE_COUNT) errors.push(`attributes must contain at most ${MAX_ATTRIBUTE_COUNT} entries`);
  if (attributeKeys.some((key) => !SAFE_ATTRIBUTE_KEYS.has(key))) losses.push({ path: "attributes", reason: "privacy", detail: "one or more unknown telemetry attributes were not imported" });
  if (Object.keys(span.resource.attributes).length) losses.push({ path: "resource.attributes", reason: "privacy", detail: "resource attributes are not imported by default" });
  losses.push(
    { path: "payload", reason: "required_for_round_trip", detail: "payload cannot be reconstructed because it is deliberately not exported" },
    { path: "security", reason: "required_for_round_trip", detail: "only allowlisted security labels can be reconstructed" },
    { path: "integrity", reason: "required_for_round_trip", detail: "only allowlisted integrity hashes can be reconstructed" },
  );
  if (errors.length || !occurredAt) return { losses, errors };
  const a = span.attributes;
  const value: Partial<EventEnvelope> = { traceId: span.traceId, spanId: span.spanId, occurredAt };
  const eventId = a["solledger.event_id"];
  if (typeof eventId === "string" && /^evt_[A-Za-z0-9_-]{8,}$/.test(eventId)) value.eventId = eventId;
  const eventType = a["solledger.event_type"];
  if (typeof eventType === "string" && /^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$/.test(eventType)) value.eventType = eventType;
  const recordedAt = a["solledger.recorded_at"];
  if (typeof recordedAt === "string" && recordedAt.length <= MAX_ATTRIBUTE_STRING_LENGTH && parseTimestamp(recordedAt, "attributes.solledger.recorded_at", errors)) value.recordedAt = recordedAt;
  const runId = a["solledger.run_id"];
  if (typeof runId === "string" && /^run_[A-Za-z0-9_-]{8,}$/.test(runId)) value.runId = runId;
  if (errors.length) return { losses, errors };
  return { value, losses, errors };
}

export function provenanceEdgeToOtelLink(
  edge: ProvenanceEdge,
  context: { traceId: string; spanId: string },
): AdapterResult<OtelSpanLink> {
  const errors: string[] = [];
  const losses: LossItem[] = [{ path: "relationship", reason: "not_mapped", detail: "OTel links have no native PROV relationship semantics; the value is retained as an attribute" }];
  if (!isTraceId(context.traceId)) errors.push("traceId must be a non-zero 32-character lowercase hex W3C trace ID");
  if (!isSpanId(context.spanId)) errors.push("spanId must be a non-zero 16-character lowercase hex W3C span ID");
  if (!/^edge_[A-Za-z0-9_-]{8,}$/.test(edge.edgeId)) errors.push("edgeId is invalid");
  if (!(["generated_by", "used", "derived_from", "attributed_to", "acted_on_behalf_of", "invalidated_by"] as string[]).includes(edge.relationship)) errors.push("relationship is unsupported");
  parseTimestamp(edge.recordedAt, "recordedAt", errors);
  if (edge.attributes && Object.keys(edge.attributes).length) losses.push({ path: "attributes", reason: "privacy", detail: "edge attributes are not exported by default" });
  if (errors.length) return { losses, errors };
  return {
    value: {
      ...context,
      attributes: {
        "solledger.relationship": edge.relationship,
        "solledger.recorded_at": edge.recordedAt,
      },
    },
    losses: losses.concat([
      { path: "edgeId", reason: "privacy", detail: "raw edge identifiers are not exported to telemetry by default" },
      { path: "fromRef", reason: "privacy", detail: "raw provenance endpoints are not exported to telemetry by default" },
      { path: "toRef", reason: "privacy", detail: "raw provenance endpoints are not exported to telemetry by default" },
    ]),
    errors,
  };
}
