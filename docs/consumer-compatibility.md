# Consumer compatibility

The clean-history protocol contract revision
`6139085503dec278e86cf0d9673d84ba34eb1e92` is exercised by both public
candidates. Its four wire schemas are byte-identical to the previously accepted
private `v0.1.0` contract.

| Consumer | Verified revision | Protocol use |
| --- | --- | --- |
| Agent Black Box | `e89afaa2ad6e61e1f4720d3c92fb6f22b0b8a77d` | Metadata-only four-event lifecycle, serialized recovery, independently derived trusted head, and Rust chain verification |
| Evidence Forge `v6.3.1` | `6747fdd1fea1618ef96302e1442c39e474d8a7e1` | Artifact/Event/Provenance compatibility at this exact protocol pin, corrected producer identity, installed-package fail-closed coverage, and signed three-product release evidence |

The consumers keep product policy outside the protocol: Evidence Forge owns
promotion, while Agent Black Box owns its metadata/hash-only privacy boundary.
Compatibility does not imply a network service or raw-content transport.
The clean-history candidates retain the same generated types and OTel/PROV
adapters without changing the four wire schemas or either consumer's policy.

## Reusable operator acceptance

The [`Kota-Ohno/ecosystem-acceptance-kit-oss`](https://github.com/Kota-Ohno/ecosystem-acceptance-kit-oss)
public candidate pins both the current implementation revision and the separate
clean-history wire-contract revision. It runs the
protocol's TypeScript, Rust, formatting, lint, fixture, and interoperability
checks before the packed three-product acceptance.

Its non-executing preflight only classifies exact changed paths; it does not prove
semantic compatibility. Contract or schema changes require review and a changed
lock always requires a complete acceptance. Sol Ledger remains product-neutral
and has no runtime dependency on the kit.
