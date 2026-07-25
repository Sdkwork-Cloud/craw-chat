# sdkwork-im-database-host

- Domain: communication
- Capability: database-lifecycle
- Package type: rust-crate
- Status: standardizing

Registers the IM root `database/` lifecycle module through `sdkwork-database`. The host accepts the
process PostgreSQL authority only; SQLite belongs exclusively to client-local native adapters.

## Public API

- `bootstrap_im_database(pool)` rejects non-PostgreSQL pools, loads the manifest, runs declared
  lifecycle orchestration, and returns `ImDatabaseHost`.
- `bootstrap_im_database_from_env()` resolves the canonical IM configuration and delegates to the
  same guarded lifecycle.
- `ImDatabaseHost::pool()` returns the shared server pool.
- `ImDatabaseHost::module()` returns the database module SPI handle used by lifecycle tooling.

The host does not run ad hoc schema repair SQL. Schema evolution belongs to the checked-in baseline
and PostgreSQL migrations, and manifest `autoMigrate=false` remains effective.

## Configuration

Configuration is resolved from `SDKWORK_IM_DATABASE_*`; `SDKWORK_IM_APP_ROOT` may select an explicit
application root. Database credentials must use the approved secret/config channel.

## Verification

```powershell
cargo test -p sdkwork-im-database-host
pnpm db:validate
```

Module integration authority: [specs/component.spec.json](specs/component.spec.json).
