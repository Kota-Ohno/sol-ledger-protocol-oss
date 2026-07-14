# Protocol

Sol Ledger records observations without claiming that they are true. Products may
promote observations to evidence candidates and verified evidence through their own
explicit validation gates.

The protocol consists of four versioned JSON Schema 2020-12 documents:

- event envelope
- artifact reference
- provenance edge
- security policy

Events are append-only. Implementations should hash canonical JSON and link each
event to the previous event hash. Verification requires a trusted final event hash
stored outside the ledger; an internal hash chain alone cannot detect tail truncation.
Artifacts use content-derived SHA-256 identities.

The provenance vocabulary is a compact mapping of W3C PROV relationships. Internal
storage does not need RDF; exporters may map records to PROV Entity, Activity, and Agent.
