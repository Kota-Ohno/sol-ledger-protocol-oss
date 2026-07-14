# Roadmap

- [x] M0 — Protocol foundation: four schemas, fixtures, dual validators, threat model, and tamper-detection CLI exist.
- [x] M1 — Protocol hardening: payload and event hashes are verified; chain tests cover modification, deletion, and reorder.
- [x] M2 — Type packages: JSON Schema 2020-12 drives committed TypeScript and Rust domain types, with local/CI drift detection and compatibility tests.
- [x] M3 — Interop: OpenTelemetry and W3C PROV mapping documents plus bounded adapters exist.
- [x] M4 — Agent Black Box bootstrap: `Kota-Ohno/agent-black-box` pins private protocol `v0.1.0`, captures command runs, and verifies its chain with the Rust CLI.
- [x] M5 — Evidence Forge bootstrap: `Kota-Ohno/evidence-forge` pins private protocol `v0.1.0`, promotes sourced candidates, and validates Artifact/Event/Provenance records plus Rust/JCS chain compatibility.
- [x] M6 — Consumer acceptance: Agent Black Box and Evidence Forge publish pinned private compatibility baselines while keeping privacy and promotion policy outside the protocol.
- [x] M7 — Current consumer reacceptance: clean Agent Black Box and Evidence Forge `v6.3.1` revisions complete installed three-product acceptance against current protocol main while both product policies remain fail closed.
- [x] M8 — Reusable acceptance operations: the private Ecosystem Acceptance Kit separately pins current protocol implementation and stable wire contract revisions, runs all native checks, and retains externally headed receipts without moving consumer policy into the protocol.
- [x] M9 — Complete non-destructive public hygiene with an MIT license file,
  package metadata, redacted history/tree secret scanning, private security
  reporting guidance, and environment/session-file ignores.
- [x] M10 — Resolve personal-email history through a reviewed fresh snapshot
  with a GitHub noreply author while retaining the private development history
  in its original repository.
- [ ] M11 — Rerun ecosystem acceptance against the clean-history repositories
  and obtain explicit approval before any visibility or package publication
  change.
