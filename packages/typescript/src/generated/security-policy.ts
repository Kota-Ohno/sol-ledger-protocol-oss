/* Generated from schemas/*.schema.json by pnpm generate. Do not edit. */

export interface SecurityPolicy {
  sensitivity: "public" | "internal" | "private" | "secret_never_export";
  contentMode: "metadata_only" | "hash_only" | "redacted" | "full_opt_in";
  retentionClass: "ephemeral" | "user_managed" | "audit" | "legal_hold";
  expiresAt?: string;
  redactionProfile?: string;
}
