# sdkwork-im-database-pool

Domain: communication
Capability: im
Package type: rust-crate
Status: active

Canonical IM sqlx pool bootstrap through `sdkwork-database`. New async services should use `create_im_database_pool_from_env()` instead of ad-hoc pool wiring.

## Public API

- `create_im_database_pool_from_env()` — builds the IM `DatabasePool` from `SDKWORK_IM_DATABASE_*` env vars.

## Configuration

Reads `SDKWORK_IM_DATABASE_*` through `sdkwork-database-config` with service name `IM`.

## Verification

- `cargo test -p sdkwork-im-database-pool`
