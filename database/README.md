# IM Database

## Purpose

This directory is the canonical database lifecycle root for `sdkwork-im`. It contains the
PostgreSQL schema contract, bootstrap baseline, ordered migrations, seeds, drift policy, and
module manifest governed by `DATABASE_SPEC.md` and `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `im`
- serviceCode: `IM`
- databaseRole: `authoritative-server`
- engine: `postgres`
- tablePrefix: `im_`
- contractVersion: `2.1.0`

## Owner

`im-platform` owns this lifecycle root. Individual IM repository crates consume the resolved
contract; no client, gateway, or sibling SDKWork application may introduce an independent IM
schema authority here.

## Allowed Content

- PostgreSQL schema registry, baseline DDL, ordered migrations, seed manifests, drift policy, and
  verification fixtures for IM-owned tables.
- Machine-readable ownership, index, retention, and migration provenance for the registered
  `im_` tables.
- Safe operational examples that do not contain credentials, production rows, or database dumps.

## Forbidden Content

- SQLite, browser, desktop, cache, or other client-local database schemas.
- Sibling application tables, cross-database foreign keys, generated SDK output, or application
  service code.
- Credentials, production data, backups, ad hoc recovery scripts, or untracked schema changes.

## Ownership Boundary

The IM platform owns only the tables registered in `contract/table-registry.json`. IAM, Agents,
Drive, Knowledgebase, RTC media, and other sibling products own their databases independently.
Cross-domain identifiers in IM are bounded opaque references and do not create cross-database
foreign keys.

This root must remain PostgreSQL-only. A PC client-local store, when present, owns an independent
manifest, schema, migrations, retention policy, and synchronization contract. Client-local assets
must not mirror this server schema or appear under this directory.

## Lifecycle

The module uses `baseline-plus-migrations`:

1. `ddl/baseline/postgres/0001_im_baseline.sql` is the current 60-table bootstrap snapshot.
2. `migrations/postgres/` upgrades installed databases in version order and adds three
   IM-to-Agents assignment, binding, and dispatch tables.
3. Baseline plus migrations is the complete 63-table IM contract.
4. `lifecycle.autoMigrate` defaults to `false`; deployment runs an explicit, elected migration
   step before application traffic.

Contract `2.0.0` introduced the IM-owned Agents assignment, binding, and dispatch records through
migration `0005`; later migrations harden their signed-int64 subject scope and concurrency rules.
Contract `2.1.0` introduces typed Conversation policy, business binding, handoff, archive metadata,
and commit fingerprint authority through migration `0012`.

Existing pre-launch databases that cannot satisfy the typed archive or handoff invariants must be
reset or restored from a verified PostgreSQL backup. Journal replay, JSON snapshot reconstruction,
and fabricated archive metadata are not migration strategies.

## Rules

- Normalized tables are current-state authority.
- `im_commit_journal` is immutable audit and integration evidence, not a current-state recovery
  source.
- Business state, journal evidence, and required outbox rows commit in one PostgreSQL transaction.
- Migrations are ordered, checksum-governed, and PostgreSQL-specific.
- Seeds are separate from structural migrations.
- Secrets and production row data must not be committed to this directory.
- Ad hoc SQL outside the database lifecycle is forbidden.

## Verification

Run from the repository root:

```bash
pnpm db:validate
pnpm db:materialize:contract
pnpm db:contract:check
pnpm db:plan
pnpm db:init
pnpm db:migrate
pnpm db:seed
pnpm db:status
pnpm db:drift:check
pnpm test:database-framework-standard
pnpm test:database-naming-standard
```

`db:materialize:contract` deterministically composes the PostgreSQL baseline and ordered up
migrations, then aligns the schema and table registry. `db:contract:check` verifies the same
63-table contract without writing files.

PostgreSQL repository and migration changes also require their focused live tests when a test
database is available:

```bash
cargo test -p im-adapters-postgres-journal --test agent_integration_migration_live_test -- --ignored --nocapture
```

## Related Specs

- Repository workspace authority: `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`; from this
  directory the resolved path is `../../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- `../../sdkwork-specs/DATABASE_SPEC.md`
- `../../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`
- `../../sdkwork-specs/MIGRATION_SPEC.md`
- `../../sdkwork-specs/TEST_SPEC.md`
