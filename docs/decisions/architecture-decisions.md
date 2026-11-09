# Architecture decisions

## ADR-001: Modular monolith

We use a modular monolith instead of separate network services because the project is customer-premises, local-first, and operationally simpler to run. This keeps the architecture maintainable while still separating domain, application, and infrastructure concerns.

## ADR-002: Rust backend + SvelteKit frontend

This pairing provides strong compile-time guarantees, a fast runtime, and a modern frontend stack with TypeScript and runes-based component authoring.

## ADR-003: Local-first deployment

MULX is designed for local operation without continuous internet connectivity for ordinary ERP tasks. External licensing is isolated as a service adapter rather than an embedded runtime dependency.

## ADR-004: Bun + Turbo for workspace orchestration

The project uses Bun and Turbo for local developer workflows because they are compatible with the SvelteKit app and provide a predictable monorepo runner.

## ADR-005: Tauri desktop shell

The desktop client is a wrapper around the web frontend and shared API contract rather than a second ERP implementation.
