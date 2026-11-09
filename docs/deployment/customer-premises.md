# Customer-premises deployment

MULX is designed for dedicated single-customer installation on a local server environment. The deployment model intentionally keeps runtime, database, storage, and backup configuration local to the customer environment.

## Current implementation status

- Docker Compose foundation for PostgreSQL and API runtime
- Local configuration template in `.env.example`
- Local reverse proxy foundation using Caddy
- Persistent volume pattern for PostgreSQL data

## Constraints

- No shared control plane or centralized licensing administration is implemented.
- External licensing checks are isolated behind an adapter and must not block ordinary local ERP workflows.
