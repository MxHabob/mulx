# Deployment foundation

MULX is designed for customer-premises deployment with dedicated runtime, database, storage, and backups.

## Expected model

- One isolated environment per customer
- Dedicated PostgreSQL instance
- Local persistence and backups
- Local maintenance windows
- External licensing service integration with explicit boundary and offline resilience

## Runtime

Use Docker Compose for local development and production-like testing. A Caddy reverse proxy can terminate TLS for local deployments when required.
