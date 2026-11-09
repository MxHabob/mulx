# MULX

MULX is a customer-premises ERP foundation built as a modular monolith with a Rust backend and SvelteKit frontend.

## Architecture

- Backend: Rust workspace with domain/application/infrastructure layers and Axum API
- Frontend: SvelteKit + TypeScript + Tailwind
- Database: PostgreSQL with SQLx migrations
- Deployment: local Docker + Compose + Caddy + persistent volumes
- Domain modules: core, finance, sales, purchasing, inventory, POS, CRM, supply chain, manufacturing, HR, assets, projects, documents, reporting, integrations, automation

## Workspace

- `apps/web`: SvelteKit web client
- `backend/api`: HTTP API server
- `backend/worker`: background jobs
- `backend/crates/domain`: business/domain contracts
- `backend/crates/app`: use cases and application services
- `backend/crates/infrastructure`: database, storage, and service adapters

## Run locally

```bash
bun run dev:web
cargo run --manifest-path backend/api/Cargo.toml
cargo run --manifest-path backend/worker/Cargo.toml
```

## Check

```bash
bun run check:web
cargo check --workspace --manifest-path Cargo.toml
```

## Deployment notes

- Keep runtime local to customer premises.
- Do not depend on continuous internet access for essential ERP operations.
- Use external licensing service as an isolated integration boundary.
