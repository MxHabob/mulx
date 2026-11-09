# Operations and observability

The foundation includes:

- health checks for HTTP and dependencies
- readiness endpoints
- structured logs via tracing
- background worker heartbeat
- local backup and restore conventions

Operational guidance for the next phase:

- Capture request IDs for each API request.
- Persist audit events for security-sensitive actions.
- Add migration and deployment checks before production rollout.
