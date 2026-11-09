# Local development

This repository is configured for a local development workflow using Bun for frontend tasks and Cargo for Rust.

## Commands

```bash
bun install
bun run check:web
cargo check --workspace --manifest-path Cargo.toml
cargo run --manifest-path backend/api/Cargo.toml
```

## Services

- `apps/web`: SvelteKit frontend
- `backend/api`: HTTP API
- `backend/worker`: job worker
- Postgres via Docker Compose

## Notes

This project assumes a local-execution model and does not depend on external services for routine development tasks.
