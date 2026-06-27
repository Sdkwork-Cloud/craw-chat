# im-adapters-social-postgres

Domain: communication
Capability: social
Package type: rust-crate
Status: standardizing

Postgres persistence adapter for social graph data. Consumes `sdkwork-database-config` for unified pool configuration per `DATABASE_SPEC.md`.

## Public API

- Postgres repositories for social-service persistence (friend requests, friendships, blocks, direct chats, user profiles, user settings).

## Configuration

Database connection uses `SDKWORK_IM_DATABASE_*` environment variables through `sdkwork-database-config`.

## Verification

- `cargo test -p im-adapters-social-postgres`
