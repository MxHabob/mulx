# Development workflow

## Tooling

- Rust stable toolchain
- Bun for frontend package management
- Docker Compose for local runtime

## Commands

```bash
bun run check:web
cargo check --workspace --manifest-path Cargo.toml
cargo test --workspace --manifest-path Cargo.toml
```

## Standards

- Keep business logic outside HTTP handlers.
- Prefer explicit dependency boundaries.
- Use strict TypeScript and Rust checks before merging changes.
- Build APIs with versioned routes such as `/api/v1`.
