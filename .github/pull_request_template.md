## Summary

Describe the contract change or implementation outcome.

## Verification

- [ ] `npm test`
- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `npm run audit:secrets` for release-related changes

## Protocol and release boundary

- [ ] Product-specific policy was not moved into the protocol
- [ ] Schema changes include valid/invalid fixtures and regenerated projections
- [ ] No real prompts, tool data, credentials, paths, or private traces were added
- [ ] This PR does not make anything public or publish a package or crate
