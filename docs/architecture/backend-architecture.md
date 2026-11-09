# Backend architecture

The backend uses a modular monolith rather than a distributed microservice topology. This keeps operational complexity lower while still separating domain, application, and infrastructure responsibilities.

## Layering

- Domain: business invariants and value objects
- Application: service orchestration and use cases
- Infrastructure: SQL, persistence, storage, and external adapters
- API: Axum HTTP endpoints and versioned routes
- Worker: background jobs and maintenance tasks

## Current implementation

- Axum API server with `/api/v1/health` and `/api/v1/ready`
- Rust workspace with dedicated crates
- Docker-ready API and worker entrypoints
- startup-ready configuration structure

## Constraints

- Business logic remains outside HTTP handlers.
- SQLx models are not exposed as public domain entities.
- Module-to-module database access is not allowed by design.
