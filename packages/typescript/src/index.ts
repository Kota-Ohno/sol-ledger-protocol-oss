import type { ArtifactRef as GeneratedArtifactRef } from "./generated/artifact-ref.js";
import type { EventEnvelope as GeneratedEventEnvelope } from "./generated/event-envelope.js";
import type { ProvenanceEdge as GeneratedProvenanceEdge } from "./generated/provenance-edge.js";
import type { SecurityPolicy as GeneratedSecurityPolicy } from "./generated/security-policy.js";

type GeneratedActor = GeneratedEventEnvelope["actor"];
type GeneratedEventIntegrity = GeneratedEventEnvelope["integrity"];

export type ActorKind = GeneratedEventEnvelope["actor"]["kind"];

export type Sensitivity = GeneratedSecurityPolicy["sensitivity"];

export type ContentMode = GeneratedSecurityPolicy["contentMode"];

export type RetentionClass = GeneratedSecurityPolicy["retentionClass"];

export interface SecurityPolicy extends GeneratedSecurityPolicy {}

export interface Actor extends GeneratedActor {}

export interface EventIntegrity extends GeneratedEventIntegrity {}

export interface EventEnvelope<
  Payload extends Record<string, unknown> = Record<string, unknown>,
> extends Omit<GeneratedEventEnvelope, "payload"> {
  payload: Payload;
}

export type ArtifactStorage = GeneratedArtifactRef["storage"];
export type ArtifactRedaction = GeneratedArtifactRef["redaction"];

export interface ArtifactRef extends GeneratedArtifactRef {}

export type ProvenanceRelationship = GeneratedProvenanceEdge["relationship"];

export interface ProvenanceEdge extends GeneratedProvenanceEdge {}

export type { AdapterResult, LossItem, LossReason } from "./interop/common.js";
export type { OtelAttributeValue, OtelResource, OtelSpan, OtelSpanLink } from "./interop/otel.js";
export { eventEnvelopeToOtelSpan, otelSpanToEventFields, provenanceEdgeToOtelLink } from "./interop/otel.js";
export type { ProvDocument, ProvNode, ProvRelation, ProvRelationKind } from "./interop/prov.js";
export { eventEnvelopeToProv, provenanceEdgeToProv, provRelationToProvenanceEdge } from "./interop/prov.js";
