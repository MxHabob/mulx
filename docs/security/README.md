# Security foundation

- Use bcrypt or argon2 for password hashing, not custom cryptography.
- Scope authorization to organization and branch boundaries.
- Keep authentication middleware separated from business logic.
- Require explicit audit logging for privilege changes.
- Keep external licensing calls non-blocking for ordinary local workflows.

This repository intentionally creates only the foundational security boundaries and not a complete ERP security implementation.
