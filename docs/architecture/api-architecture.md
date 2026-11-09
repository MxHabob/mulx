# API architecture

The API is versioned under `/api/v1` and uses a consistent envelope response pattern for health and readiness.

## Implemented contract

- `GET /api/v1/health`
- `GET /api/v1/ready`

Both endpoints return a JSON envelope with an `ok` flag and a typed payload.

## Future direction

Next steps include:

- pagination conventions
- filtering/sorting metadata
- request IDs and correlation metadata
- auth middleware boundaries
- OpenAPI generation and TypeScript client generation
- validation middleware and structured error mapping

This project currently establishes the foundational route shape and contract boundaries, not the final ERP API surface.
