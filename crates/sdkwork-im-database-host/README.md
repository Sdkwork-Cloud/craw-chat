# sdkwork-im-database-host

Domain: communication
Capability: im
Package type: rust-crate
Status: active

Registers the IM `database/` lifecycle module through `sdkwork-database` and exposes the shared sqlx pool for service bootstrap.

## Public API

- `bootstrap_im_database(pool)` — loads the IM database manifest, runs lifecycle orchestration, and returns `ImDatabaseHost`.
- `ImDatabaseHost::pool()` — shared `DatabasePool` for IM services.
- `ImDatabaseHost::module()` — `DefaultDatabaseModule` SPI handle for drift/status tooling.

## Configuration

Uses `SDKWORK_IM_DATABASE_*` and unified `SDKWORK_CLAW_DATABASE_*` fallbacks through `sdkwork-database-config`.

## Verification

- `cargo test -p sdkwork-im-database-host`
