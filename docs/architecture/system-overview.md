# System overview

MULX is a customer-premises ERP foundation designed for local-first operation and independent deployment per customer. The solution is intentionally structured as a modular monolith with a Rust backend, SvelteKit web client, and a Tauri desktop shell for local desktop environments.

## Implemented status

- Implemented: workspace root, backend crate boundaries, base API, SvelteKit app, Tauri desktop init, docs, local Docker foundation
- Partially implemented: database schema, auth, licensing adapter, shared SDK contracts
- Planned: ERP domain modules, migrations, RBAC, file storage, release pipeline

## Trust boundaries

1. Local users and administrators
2. Local network / perimeter controls
3. MULX application runtime
4. PostgreSQL database
5. External integrations and service adapters
6. Existing licensing service
7. Remote maintenance channels

The application is designed to avoid requiring continuous external connectivity for ordinary ERP operations.
