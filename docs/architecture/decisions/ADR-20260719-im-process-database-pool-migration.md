# ADR-20260719: IM Process Database Pool Migration

- Status: temporary exception
- Owner: sdkwork-im persistence maintainers
- Removal milestone: before the next IM production release
- Canonical standard: `../../../sdkwork-specs/DATABASE_SPEC_PROCESS_SHARED_POOL.md`

Every IM HTTP and RPC process enables the canonical process-shared SQLx pool before database bootstrap. Embedded IAM and other modules using `sdkwork-database-sqlx` reuse that handle when identity matches.

Legacy synchronous IM adapters still require one process-singleton r2d2 PostgreSQL pool.
`SDKWORK_IM_DATABASE_MAX_CONNECTIONS` is the total process budget, not a per-driver value. Profiles
declare one temporary driver through `SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT=1`; the database
framework reserves the r2d2 share before creating SQLx, and the two maxima sum exactly to the
configured budget. Values below two fail startup while both drivers remain.

This is not single-pool compliance. The exception is removed after synchronous adapters migrate to
SQLx or move behind a separate process boundary; the r2d2 pool, reservation profile, ADR metadata,
and contract exceptions must then be deleted.
