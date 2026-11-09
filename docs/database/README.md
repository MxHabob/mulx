# Database foundation

MULX uses PostgreSQL with SQLx and explicit migration versioning.

## Rules

- Use NUMERIC/DECIMAL for financial values; never floating point for money.
- Keep migration files versioned and reviewable.
- Validate startup connectivity and database health.
- Enforce statements within explicit transaction boundaries.
- Preserve auditability for posted financial records and avoid direct mutation of posted entries.

## Next phase

The next database implementation should add migration-driven schema for organizations, branches, users, accounting contexts, and audit tables before extending ERP domain tables.
