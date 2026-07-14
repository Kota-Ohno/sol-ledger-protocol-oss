import type { EventEnvelope, ProvenanceEdge, ProvenanceRelationship } from "../index.js";
import { type AdapterResult, type LossItem, isSpanId, isTraceId, parseTimestamp } from "./common.js";

export type ProvRelationKind =
  | "wasGeneratedBy"
  | "used"
  | "wasDerivedFrom"
  | "wasAttributedTo"
  | "actedOnBehalfOf"
  | "wasInvalidatedBy";

export interface ProvRelation {
  id: string;
  type: ProvRelationKind;
  source: string;
  target: string;
  time: string;
  attributes: Record<string, string | number | boolean>;
}

export interface ProvNode {
  id: string;
  attributes: Record<string, string | string[]>;
}

export interface ProvDocument {
  entities: ProvNode[];
  activities: ProvNode[];
  agents: ProvNode[];
  associations: Array<{ activity: string; agent: string }>;
  relations: ProvRelation[];
}

function encodeId(namespace: string, value: string): string {
  const bytes = new TextEncoder().encode(value);
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return `sol:${namespace}_${btoa(binary).replaceAll("+", "-").replaceAll("/", "_").replace(/=+$/, "")}`;
}

export function eventEnvelopeToProv(event: EventEnvelope): AdapterResult<ProvDocument> {
  const errors: string[] = [];
  const losses: LossItem[] = [
    { path: "payload", reason: "privacy", detail: "payload is never expanded into PROV attributes" },
    { path: "subjectRefs", reason: "not_mapped", detail: "subjects are declared as entities without guessing a relationship" },
  ];
  if (event.security.sensitivity === "secret_never_export") {
    errors.push("secret_never_export events cannot be exported to PROV");
    return { losses, errors };
  }
  const occurredAt = parseTimestamp(event.occurredAt, "occurredAt", errors);
  parseTimestamp(event.recordedAt, "recordedAt", errors);
  if (!/^evt_[A-Za-z0-9_-]{8,}$/.test(event.eventId)) errors.push("eventId is invalid");
  if (event.traceId && !isTraceId(event.traceId)) errors.push("traceId must be a non-zero W3C trace ID");
  if (event.spanId && !isSpanId(event.spanId)) errors.push("spanId must be a non-zero W3C span ID");
  if (!occurredAt || errors.length) return { losses, errors };
  const activityId = encodeId("event", event.eventId);
  const agentId = encodeId("agent", event.eventId);
  const activityAttributes: Record<string, string | string[]> = {
    "sol:schemaVersion": event.schemaVersion,
    "sol:eventId": event.eventId,
    "sol:eventType": event.eventType,
    "sol:occurredAt": event.occurredAt,
    "sol:recordedAt": event.recordedAt,
    "sol:actorKind": event.actor.kind,
    "sol:sensitivity": event.security.sensitivity,
    "sol:contentMode": event.security.contentMode,
    "sol:retentionClass": event.security.retentionClass,
    "sol:payloadSha256": event.integrity.payloadSha256,
  };
  if (event.runId) losses.push({ path: "runId", reason: "privacy", detail: "raw run identifiers are omitted by default" });
  if (event.traceId) activityAttributes["sol:traceId"] = event.traceId;
  if (event.spanId) activityAttributes["sol:spanId"] = event.spanId;
  losses.push({ path: "actor.id", reason: "privacy", detail: "raw actor identifiers are omitted by default" });
  if (event.actor.software) losses.push({ path: "actor.software", reason: "privacy", detail: "software identifiers are omitted by default" });
  if (event.subjectRefs?.length) losses.push({ path: "subjectRefs", reason: "privacy", detail: "raw subject identifiers are omitted by default" });
  if (event.integrity.previousEventSha256 !== undefined) activityAttributes["sol:previousEventSha256"] = event.integrity.previousEventSha256 ?? "";
  return {
    value: {
      entities: [],
      activities: [{ id: activityId, attributes: activityAttributes }],
      agents: [{ id: agentId, attributes: { "sol:actorKind": event.actor.kind } }],
      associations: [{ activity: activityId, agent: agentId }],
      relations: [],
    },
    losses,
    errors,
  };
}

const TO_PROV: Record<ProvenanceRelationship, ProvRelationKind> = {
  generated_by: "wasGeneratedBy",
  used: "used",
  derived_from: "wasDerivedFrom",
  attributed_to: "wasAttributedTo",
  acted_on_behalf_of: "actedOnBehalfOf",
  invalidated_by: "wasInvalidatedBy",
};
const FROM_PROV = Object.fromEntries(Object.entries(TO_PROV).map(([key, value]) => [value, key])) as Record<ProvRelationKind, ProvenanceRelationship>;

function validRef(value: string): boolean {
  return value.length > 0 && !/[\u0000-\u001f\u007f]/.test(value);
}

export function provenanceEdgeToProv(edge: ProvenanceEdge): AdapterResult<ProvRelation> {
  const errors: string[] = [];
  const losses: LossItem[] = [];
  if (!/^edge_[A-Za-z0-9_-]{8,}$/.test(edge.edgeId)) errors.push("edgeId is invalid");
  if (!Object.hasOwn(TO_PROV, edge.relationship)) errors.push("relationship is unsupported");
  if (!validRef(edge.fromRef)) errors.push("fromRef is empty or contains control characters");
  if (!validRef(edge.toRef)) errors.push("toRef is empty or contains control characters");
  parseTimestamp(edge.recordedAt, "recordedAt", errors);
  if (edge.attributes && Object.keys(edge.attributes).length) {
    losses.push({ path: "attributes", reason: "privacy", detail: "edge attributes are not exported unless a future explicit policy allows them" });
  }
  if (errors.length) return { losses, errors };
  return {
    value: {
      id: edge.edgeId,
      type: TO_PROV[edge.relationship],
      source: edge.fromRef,
      target: edge.toRef,
      time: edge.recordedAt,
      attributes: {},
    },
    losses,
    errors,
  };
}

export function provRelationToProvenanceEdge(relation: ProvRelation): AdapterResult<ProvenanceEdge> {
  const errors: string[] = [];
  const losses: LossItem[] = [];
  if (!/^edge_[A-Za-z0-9_-]{8,}$/.test(relation.id)) errors.push("id cannot be represented as a Sol Ledger edgeId");
  if (!Object.hasOwn(FROM_PROV, relation.type)) errors.push("PROV relation type is unsupported");
  if (!validRef(relation.source)) errors.push("source is empty or contains control characters");
  if (!validRef(relation.target)) errors.push("target is empty or contains control characters");
  parseTimestamp(relation.time, "time", errors);
  if (!relation.attributes || Array.isArray(relation.attributes) || typeof relation.attributes !== "object") errors.push("attributes must be an object");
  else if (Object.keys(relation.attributes).length) losses.push({ path: "attributes", reason: "privacy", detail: "unknown PROV attributes were not imported" });
  if (errors.length) return { losses, errors };
  return {
    value: {
      edgeId: relation.id,
      relationship: FROM_PROV[relation.type],
      fromRef: relation.source,
      toRef: relation.target,
      recordedAt: relation.time,
    },
    losses,
    errors,
  };
}
