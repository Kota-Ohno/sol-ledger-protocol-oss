/* Generated from schemas/*.schema.json by pnpm generate. Do not edit. */

export interface ProvenanceEdge {
  edgeId: string;
  relationship: "generated_by" | "used" | "derived_from" | "attributed_to" | "acted_on_behalf_of" | "invalidated_by";
  fromRef: string;
  toRef: string;
  recordedAt: string;
  attributes?: {
    [k: string]: unknown;
  };
}
