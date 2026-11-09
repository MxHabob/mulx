# Database architecture

MULX targets PostgreSQL with SQLx migration versioning and explicit transaction boundaries.

## Rules implemented at the foundation level

- Keep money as integer-backed monetary values in the domain model
- Use database NUMERIC for financial values in future migrations
- Keep migration versioning and auditability in scope
- Separate startup health checks from runtime availability

## Planned schema direction

A future phase will add organizations, branches, users, roles, permissions, accounting contexts, and audit tables before ERP-specific tables are introduced.

## Notes

The project currently has infrastructure scaffolding but no production accounting chart implementation. That remains explicitly deferred.
