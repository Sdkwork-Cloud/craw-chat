# ADR-20260719: IM Process Database Pool Migration

- Status: temporary exception
- Owner: sdkwork-im persistence maintainers
- Removal milestone: before the next IM production release
- Canonical standard: `../../../sdkwork-specs/DATABASE_SPEC_PROCESS_SHARED_POOL.md`

Every IM HTTP and RPC process enables the canonical process-shared SQLx pool before database bootstrap. Embedded IAM and other modules using `sdkwork-database-sqlx` reuse that handle when identity matches.

Legacy synchronous IM adapters still require one process-singleton r2d2 PostgreSQL pool.
`SDKWORK_DATABASE_MAX_CONNECTIONS` is the total process budget, not a per-driver value. Profiles
for standalone gateway composition also embed the Drive API assembly. Drive repositories currently
require the framework-owned, identity-checked `sqlx::AnyPool` compatibility pool declared by
`sdkwork-drive/docs/architecture/decisions/ADR-20260719-drive-pool-driver-migration.md`.

Standalone gateway profiles therefore set `SDKWORK_DATABASE_TEMPORARY_ANY_POOL_EXCEPTION=true` and
declare two temporary drivers through `SDKWORK_DATABASE_TEMPORARY_DRIVER_POOL_COUNT=2`. With the
default process maximum of 10, the framework reserves 4 connections for canonical SQLx, 3 for
Drive AnyPool, and 3 for IM r2d2. Other IM processes that do not embed Drive continue to declare
only the r2d2 exception and reserve one temporary driver.

This is not single-pool compliance. The AnyPool exception is removed after Drive PostgreSQL
repositories consume the installed typed process pool. The r2d2 exception is removed after
synchronous IM adapters migrate to SQLx or move behind a separate process boundary. Each removal
must delete its profile switch, reservation, ADR metadata, and process-contract exception.
