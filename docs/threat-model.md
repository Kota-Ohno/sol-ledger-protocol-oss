# Threat model

## Protected properties

- Raw private content is not recorded without explicit opt-in.
- `secret_never_export` content cannot use `full_opt_in` or `redacted` capture.
- Event deletion, insertion, and modification are detectable by a chain verifier.
- Model output is never implicitly treated as verified evidence.

## Primary threats

- secrets embedded in prompts, tool arguments, outputs, paths, and environment variables
- prompt injection attempting to promote unsupported claims
- mutable URLs changing after citation
- trace tampering or selective deletion
- replay re-executing destructive side effects

## Required mitigations

- redact before persistence and again before export
- default to metadata-only or hash-only recording
- snapshot external evidence by content hash
- require a separate promotion gate for verified evidence
- make replay dry-run by default
