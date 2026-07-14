# Agent guidance

- Keep the protocol product-neutral; product-specific fields remain extensions.
- Raw prompts, model responses, tool arguments, and tool results are opt-in.
- Schema changes require valid and invalid fixtures plus TypeScript and Rust checks.
- Do not weaken validation to make fixtures pass.
- Run `npm test`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` before completion.
