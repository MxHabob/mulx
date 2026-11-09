# Frontend architecture

The frontend uses SvelteKit with Svelte 5 runes and strict TypeScript. The app is intentionally minimal but consistent with the ERP platform direction.

## Current implementation

- SvelteKit app under `apps/web`
- Tailwind CSS integration
- Route layout and app shell baseline
- Type-checking via `svelte-check`

## Design principles

- Shared SDK usage rather than duplicated API logic
- Feature-oriented architecture rather than a large single shared folder
- Accessible component patterns
- Strong separation between UI, data access, and domain logic

## Desktop shell

The Tauri desktop project is configured as a native wrapper around the web frontend and the shared API layer. It does not duplicate business logic.
