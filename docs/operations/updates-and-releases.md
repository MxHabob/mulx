# Updates and releases

MULX should support local update and release workflows without a centralized control plane.

## Principles

- Release artifacts should be reproducible and checksum-verified.
- Database migrations must be validated before rollout.
- Pre-update backup and health verification are mandatory.
- Development, test, and production builds must be separated.

## Current status

The repository contains release-oriented documentation and local deployment scaffolding, but automated signed release packaging and end-to-end update validation remain future work.
