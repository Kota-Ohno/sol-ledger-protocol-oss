# Consumer compatibility

The private protocol `v0.1.0` revision
`cc955517a37a544583354b78fb2bba20239a764f` is exercised by both consumers.

| Consumer | Verified revision | Protocol use |
| --- | --- | --- |
| Agent Black Box | `1ed2f7bfc5330167a583ce42401f5158f458f03d` | Metadata-only four-event lifecycle, serialized recovery, independently derived trusted head, and Rust chain verification |
| Evidence Forge `v6.3.1` | `ccef4d91ddd0d13d527565a44fce8279e55292c6` | Artifact/Event/Provenance compatibility at this exact protocol pin, corrected producer identity, installed-package fail-closed coverage, and signed three-product release evidence |

The consumers keep product policy outside the protocol: Evidence Forge owns
promotion, while Agent Black Box owns its metadata/hash-only privacy boundary.
Compatibility does not imply a network service or raw-content transport.
The current packed acceptance used this repository at
`5afd8ffd39288d2008ae0f738ba4a69c15e8ca47`; the consumer wire pin remains the
tagged `v0.1.0` contract above. Later generated types, OTel/PROV adapters, and
documentation do not change the four wire schemas or either consumer's policy.

## Reusable operator acceptance

The private [`Kota-Ohno/ecosystem-acceptance-kit`](https://github.com/Kota-Ohno/ecosystem-acceptance-kit)
`v0.2.0` release at commit
`3f08fc9e703e98ccdfc905d5f0bd58022e20a3ab` pins both this current implementation
revision and the separate stable `v0.1.0` wire-contract revision. It runs the
protocol's TypeScript, Rust, formatting, lint, fixture, and interoperability
checks before the packed three-product acceptance.

Its non-executing preflight only classifies exact changed paths; it does not prove
semantic compatibility. Contract or schema changes require review and a changed
lock always requires a complete acceptance. Sol Ledger remains product-neutral
and has no runtime dependency on the kit.
