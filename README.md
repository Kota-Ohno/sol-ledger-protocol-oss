# Sol Ledger Protocol

Sol Ledger Protocol gives tool builders one product-neutral contract for
tamper-evident AI execution traces and evidence provenance, with generated
TypeScript and Rust types. Use it when multiple tools need to exchange records
without sharing a hosted service or inventing incompatible event formats.

The protocol deliberately separates an observed execution from verified
evidence. An observation may become an evidence candidate, but only a
product-specific promotion gate can turn it into verified evidence.

> **Installation status:** this repository is currently private and its Node
> package has `private: true`; it is not published to npm. Clone it from an
> account with access. pnpm is the supported JavaScript package manager.

## Shortest path

The protocol has no hosted service or paid runtime dependency. Repository
verification requires Node.js 22.13 or newer, pnpm 11.0.8, Rust, and Cargo;
the generated/runtime Node package itself retains its Node.js 20 floor. If
`corepack` is unavailable but the pinned pnpm version is already installed,
skip the `corepack enable` line.

```bash
git clone https://github.com/Kota-Ohno/sol-ledger-protocol-oss.git
cd sol-ledger-protocol-oss
corepack enable
pnpm install --frozen-lockfile --ignore-scripts
pnpm test
cargo test --workspace
```

## Everyday workflows

```bash
# Validate the shipped fixtures.
pnpm validate:fixtures

# Regenerate schema-derived types and prove the checkout is current.
pnpm generate
pnpm check:generated

# Verify a JSONL chain against a head retained through another channel.
cargo run -p sol-ledger-cli -- verify-chain trace.jsonl --expected-head-sha256 <trusted-sha256>
```

## Role in the ecosystem

Sol Ledger is the shared wire contract and verification layer.
[Agent Black Box](https://github.com/Kota-Ohno/agent-black-box-oss) emits
privacy-bounded observations, while
[Evidence Forge](https://github.com/Kota-Ohno/evidence-forge-oss) owns the
source-backed promotion decision. The
[Ecosystem Acceptance Kit](https://github.com/Kota-Ohno/ecosystem-acceptance-kit-oss)
verifies pinned revisions together. Tested private consumers are listed in
[consumer compatibility](docs/consumer-compatibility.md).

## Safety limits

- Hash-chain validation detects inconsistency relative to a trusted head; it
  does not prove authorship, truthfulness, or trusted time.
- Generated language types cannot encode every JSON Schema constraint. Runtime
  validators remain authoritative for formats, bounds, and conditional rules.
- Raw prompts, model responses, tool arguments, and tool results are not a
  default requirement of the protocol; products must make any retention explicit.
- Read the [threat model](docs/threat-model.md) before treating a valid record as
  a security or evidence claim.

See [the protocol](docs/protocol.md), [threat model](docs/threat-model.md), and
[roadmap](docs/ROADMAP.md).

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
