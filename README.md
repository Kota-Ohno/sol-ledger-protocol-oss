# Sol Ledger Protocol

Portable contracts for tamper-evident AI execution traces and evidence provenance.

The protocol deliberately separates an observed execution from verified evidence.
Observations may become evidence candidates, but only a product-specific promotion
gate can turn them into verified evidence.

## Quick verification

The protocol has no hosted service or paid runtime dependency. Verify both
language implementations locally:

```bash
corepack enable
pnpm install --frozen-lockfile --ignore-scripts
pnpm test
cargo test --workspace
```

## Development

```bash
pnpm install
pnpm test
cargo test --workspace
cargo run -p sol-ledger-cli -- verify-chain trace.jsonl --expected-head-sha256 <trusted-sha256>
```

See [the protocol](docs/protocol.md), [threat model](docs/threat-model.md), and
[roadmap](docs/ROADMAP.md). Tested private consumers are listed in
[consumer compatibility](docs/consumer-compatibility.md), including the reusable
three-product operator acceptance entry point.

Interop profiles are documented for [OpenTelemetry](docs/open-telemetry-mapping.md)
and [W3C PROV](docs/w3c-prov-mapping.md). Their pure TypeScript adapters create
local semantic projections only; they contain no exporter or network transport.

## Schema-derived types

The four files in `schemas/` are the only source of truth for the protocol
contract. TypeScript and Rust projections are committed so consumers do not
need generators at install time.

```sh
pnpm generate
pnpm check:generated
```

Generation uses exact, lockfile-pinned `json-schema-to-typescript` 15.0.4 and
`typify` 0.7.0 versions, plus the repository-pinned Rust/rustfmt toolchain. The
generator resolves the event envelope's security
policy reference from the local checkout and never fetches schemas over the
network. Conditional, format, pattern, and bound constraints that cannot be
represented soundly in language types remain enforced by the existing AJV and
Rust runtime validators. `pnpm test` regenerates both languages and fails on a
checked-in generated-file diff.

## Security and license

Run `pnpm audit:secrets` with Gitleaks installed before preparing a release.
See [SECURITY.md](SECURITY.md) for private vulnerability reporting. Sol Ledger
Protocol is available under the MIT License; see [LICENSE](LICENSE).

Contributions are welcome under the protocol and verification requirements in
[CONTRIBUTING.md](CONTRIBUTING.md).
