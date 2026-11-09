# Backup and recovery

The foundation includes backup concepts, local persistence, and a documented path toward encrypted, retained backups.

## Current implementation

- PostgreSQL persistent volume via Docker Compose
- Local configuration backup strategy defined in docs
- Requirement to keep backup destinations configurable

## Planned production work

- PostgreSQL logical or physical backup automation
- WAL retention strategy when required
- File storage backup verification
- Restore smoke tests
- Key management and backup encryption

This remains a partial foundation rather than a fully tested disaster-recovery system.
