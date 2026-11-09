# Architecture overview

MULX follows a modular monolith design. Domain logic stays in the domain crate, application orchestration in the app crate, and database/external integrations in the infrastructure crate.

## Boundaries

- Domain: entities, value objects, errors, and invariant checks
- Application: use cases, transactions, authorization checks, service orchestration
- Infrastructure: SQLx repositories, storage adapters, jobs, telemetry, and external APIs
- API: HTTP, validation, versioning, and OpenAPI

## Module roadmap

The initial foundation establishes clear boundaries for finance, sales, purchasing, inventory, POS, CRM, supply chain, manufacturing, HR, assets, projects, documents, reporting, integrations, and automation. Each module should expose explicit interfaces and avoid direct database access from other modules.
