# Security baseline

The security baseline keeps focus on customer-premises deployment, secure runtime operations, and least-privilege boundaries.

## Mandatory rules

- Never commit secrets or credentials.
- Keep authorization enforcement server-side.
- Do not rely on frontend-only permission controls.
- Use secure hashing algorithms for passwords.
- Keep all external service integrations isolated behind adapters.
- Validate environment configuration at startup.

## Current implementation status

- Config templates and local env separation are in place.
- Security boundary documentation is present.
- Full RBAC, session management, and auth middleware are not yet implemented.
