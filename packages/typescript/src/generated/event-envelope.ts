/* Generated from schemas/*.schema.json by npm run generate. Do not edit. */

export interface EventEnvelope {
  schemaVersion: "0.1.0";
  eventId: string;
  eventType: string;
  occurredAt: string;
  recordedAt: string;
  runId?: string;
  traceId?: string;
  spanId?: string;
  actor: {
    kind: "human" | "agent" | "service" | "system";
    id: string;
    software?: string;
  };
  subjectRefs?: string[];
  payload: {
    [k: string]: unknown;
  };
  security: SecurityPolicy;
  integrity: {
    payloadSha256: string;
    previousEventSha256?: string | null;
  };
}
export interface SecurityPolicy {
  sensitivity: "public" | "internal" | "private" | "secret_never_export";
  contentMode: "metadata_only" | "hash_only" | "redacted" | "full_opt_in";
  retentionClass: "ephemeral" | "user_managed" | "audit" | "legal_hold";
  expiresAt?: string;
  redactionProfile?: string;
}
