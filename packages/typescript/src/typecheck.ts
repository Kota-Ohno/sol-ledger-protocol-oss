import type {
  ArtifactRef,
  EventEnvelope,
  ProvenanceEdge,
  SecurityPolicy,
} from "./index.js";

const security = {
  sensitivity: "private",
  contentMode: "hash_only",
  retentionClass: "audit",
} satisfies SecurityPolicy;

const event = {
  schemaVersion: "0.1.0",
  eventId: "evt_01JABCDE0001",
  eventType: "artifact.observed",
  occurredAt: "2026-07-11T12:00:00Z",
  recordedAt: "2026-07-11T12:00:01Z",
  actor: { kind: "agent", id: "agent_sol" },
  payload: { status: "observed" },
  security,
  integrity: { payloadSha256: "a".repeat(64), previousEventSha256: null },
} satisfies EventEnvelope<{ status: string }>;

const artifact = {
  artifactId: `artifact:sha256:${"a".repeat(64)}`,
  mediaType: "text/plain",
  byteLength: 1,
  storage: "local_blob",
  locator: "blobs/aa",
  redaction: "none",
} satisfies ArtifactRef;

const edge = {
  edgeId: "edge_01JABCDE0001",
  relationship: "derived_from",
  fromRef: artifact.artifactId,
  toRef: event.eventId,
  recordedAt: event.recordedAt,
} satisfies ProvenanceEdge;

void [event, artifact, edge];
