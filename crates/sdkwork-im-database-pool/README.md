# sdkwork-im-database-pool

- Domain: communication
- Capability: database-pool
- Package type: rust-crate
- Status: standardizing

Provides the canonical IM process database pool bootstrap. Every server process installs one
PostgreSQL SQLx lifecycle pool and one bounded compatibility r2d2 pool for the same normalized
database identity. Missing configuration, SQLite, and identity mismatch fail closed.

## Public API

- `bootstrap_im_process_database_pools_from_env()` installs the process pool bundle before module
  bootstrap.
- `create_im_database_pool_from_env()` resolves or reuses the canonical PostgreSQL pool.
- `ensure_im_process_postgres_r2d2_pool()` returns the installed compatibility driver handle.
- `ImProcessDatabasePools` retains the lifecycle host and shared handles for the process lifetime.

Independent pools and server SQLite fallbacks are forbidden. The PC client-local cache has a
separate native adapter, manifest, schema, and lifecycle.

## Configuration

Configuration is resolved from `SDKWORK_IM_DATABASE_*`; temporary compatibility driver capacity is
reserved through `SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT` before pool installation.

## Verification

```powershell
cargo test -p sdkwork-im-database-pool
pnpm db:pool:validate
```

Module integration authority: [specs/component.spec.json](specs/component.spec.json).
