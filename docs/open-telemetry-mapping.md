# OpenTelemetry mapping

This profile maps Sol Ledger records to dependency-free TypeScript data transfer
objects. The adapter is synchronous and pure: it does not import an
OpenTelemetry SDK, perform I/O, or transmit data.

## Event envelope to span

| Sol Ledger field | OTel representation | Round trip |
| --- | --- | --- |
| `traceId`, `spanId` | native span context | Yes, after W3C non-zero lowercase-hex validation |
| `eventType` | span name and `solledger.event_type` | Yes |
| `occurredAt` | start time as decimal Unix nanoseconds | Yes, including 1–9 fractional digits and offsets, within OTel uint64 range |
| duration | end time equals start time | No duration exists; reported as a loss |
| schema/event/security/integrity metadata | fixed `solledger.*` allowlist | Supported fields only |
| `actor.id`, `actor.software`, `runId`, `subjectRefs` | omitted | Privacy default; raw identifiers require a future explicit policy |
| `payload` | omitted | Never expanded into attributes |
| resource attributes | empty | No service/resource identity is inferred |

`secret_never_export` fails closed and produces no span. Other sensitivities
still omit raw identifiers. Unknown OTel attributes and all resource attributes
are ignored on import and are never promoted to payload. Reverse conversion
therefore returns only a projection, not a schema-valid envelope. Loss reports
contain paths and fixed descriptions, never omitted values.

Trace IDs must be 32 lowercase hexadecimal characters, span IDs 16, and neither
may be all zero. Timestamps must be complete RFC 3339 values with a real calendar
date, explicit `Z` or numeric offset, and at most nine fractional digits.

## Provenance edge to link

An edge becomes an OTel link only when the caller supplies a valid target span
context. Only relationship and observation timestamp use the fixed allowlist.
Raw edge ID and endpoints are omitted and reported because an edge has no
sensitivity policy authorizing their telemetry export. OTel has no native provenance relation,
so this semantic projection is always reported as a loss. Arbitrary edge
attributes are omitted by default.
