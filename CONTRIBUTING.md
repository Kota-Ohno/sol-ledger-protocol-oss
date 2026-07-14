# Contributing

Keep the protocol product-neutral. Raw prompts, model responses, tool arguments,
and tool results remain opt-in protocol payloads; product-specific promotion and
privacy policy do not belong in the shared schema.

Before opening a pull request:

```bash
npm ci --ignore-scripts
npm test
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run audit:secrets
```

Schema changes require matching valid and invalid fixtures plus regenerated and
verified TypeScript and Rust projections. Use synthetic records only.
