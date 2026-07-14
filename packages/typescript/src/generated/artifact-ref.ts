/* Generated from schemas/*.schema.json by pnpm generate. Do not edit. */

export interface ArtifactRef {
  artifactId: string;
  mediaType: string;
  byteLength: number;
  storage: "none" | "local_blob" | "external";
  locator?: string;
  redaction: "none" | "partial" | "full";
  originalName?: string;
}
