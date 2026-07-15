# sol-ledger-protocol

Product-neutral TypeScript contracts and loss-reporting adapters for exchanging
tamper-evident AI execution traces and evidence provenance.

```bash
pnpm add sol-ledger-protocol
```

```ts
import type { EventEnvelope, SecurityPolicy } from "sol-ledger-protocol";
import { eventEnvelopeToOtelSpan } from "sol-ledger-protocol";

const event = {
  schemaVersion: "0.1.0",
  eventId: "evt_example01",
  eventType: "artifact.observed",
  occurredAt: "2026-07-11T12:00:00Z",
  recordedAt: "2026-07-11T12:00:01Z",
  actor: { kind: "agent", id: "example" },
  payload: {},
  security: {
    sensitivity: "private",
    contentMode: "hash_only",
    retentionClass: "audit",
  } satisfies SecurityPolicy,
  integrity: { payloadSha256: "a".repeat(64) },
} satisfies EventEnvelope;

const result = eventEnvelopeToOtelSpan(event);
console.log(result.value, result.losses);
```

The adapter reports lossy mappings; it is not a validator or trust verifier.
The package performs no network access and provides no hosted service. Use the
repository JSON Schemas or `sol-ledger-schema` for runtime validation and the
`sol-ledger` CLI for trusted-head chain verification. See the
[protocol repository](https://github.com/Kota-Ohno/sol-ledger-protocol-oss)
for schemas, fixtures, Rust tooling, mappings, and security limits.

MIT licensed.
