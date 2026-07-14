# W3C PROV mapping

The TypeScript adapter emits a bounded PROV-oriented semantic DTO containing entities,
activities, agents, associations, and relations. It performs no I/O or network
transmission. It is not a PROV-JSON wire serializer; callers translate the
documented semantic roles to their chosen W3C PROV representation. Qualified
identifiers are deterministic base64url encodings.

## Nodes

An `EventEnvelope` is an Activity. Its actor is an Agent connected with an
association. Subject references and raw actor/run/software identifiers are
omitted by the privacy default, and no `used`, `generated`, or other relation is guessed. Fixed protocol metadata is retained
under `sol:*`; payload is never copied. Consequently envelope metadata can be
projected back only while these extensions survive, and payload cannot be
reconstructed.

## Relation direction

`fromRef` is the first/subject endpoint and `toRef` the second/object endpoint:

| Sol Ledger | W3C PROV | from → to kinds |
| --- | --- | --- |
| `generated_by` | `wasGeneratedBy` | Entity → Activity |
| `used` | `used` | Activity → Entity |
| `derived_from` | `wasDerivedFrom` | generated Entity → source Entity |
| `attributed_to` | `wasAttributedTo` | Entity → Agent |
| `acted_on_behalf_of` | `actedOnBehalfOf` | delegate Agent → responsible Agent |
| `invalidated_by` | `wasInvalidatedBy` | Entity → Activity |

Edge ID, endpoints, relationship, and `recordedAt` round trip through the
supported profile. `recordedAt` remains the ledger observation time rather than
being relabeled as an activity lifecycle time. Arbitrary edge attributes are
omitted and reported. Translation to standard PROV retains graph meaning, but
discarding `sol:*` extensions prevents reconstruction of ledger metadata.

This profile does not change the protocol schemas. Invalid timestamps, edge IDs,
references, relation names, or span contexts fail closed without partial output.
