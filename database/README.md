# Database

## Purpose

Canonical database lifecycle assets for `sdkwork-im`: contract schema, DDL baseline, migrations,
seeds, drift policy, and bootstrap metadata governed by `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `im`
- serviceCode: `IM`
- tablePrefix: `im_`

## Owner

Sdkwork IM maintainers.

## Allowed Content

- `database.manifest.json`, `contract/`, `ddl/`, `migrations/`, and `seeds/` lifecycle assets.
- Contract-first schema definitions and versioned migration pairs.
- Database validation fixtures and module-local README guidance.

## Forbidden Content

- Runtime service binaries, HTTP handlers, or repository business logic.
- Generated SDK output or secrets committed to Git.
- Ad-hoc SQL executed outside the `sdkwork-database-cli` lifecycle.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DATABASE_SPEC.md`
- `../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run from the repository root:

```bash
pnpm db:validate
pnpm test:database-framework-standard
pnpm test:database-naming-standard
pnpm test:contract:database
```

## Commands

```bash
pnpm run db:validate
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

## Migration status

Legacy SQL was consolidated into `ddl/baseline/postgres/0001_im_legacy_baseline.sql` for bootstrap review.
Author contract-first tables in `contract/schema.yaml`, then split baseline into versioned `migrations/` pairs.

Provenance markers inside the baseline file reference the original migration filenames from the retired
pre-framework migration tree (removed in favor of `database/` lifecycle assets).

Runtime tests and bootstrap MUST use `database/ddl/baseline/postgres/0001_im_legacy_baseline.sql`.

Runtime services MUST create pools through `sdkwork-database-sqlx` and register `DefaultDatabaseModule` at bootstrap.

## Runtime integration

- Bootstrap crate: `crates/sdkwork-im-database-host`
- Pool facade: `crates/sdkwork-im-database-pool`
- Entrypoints: `bootstrap_im_database_from_env()` / `bootstrap_im_database(pool)`
- Wired from: `services/sdkwork-im-standalone-gateway`
- IAM tenant application provisioning: `crates/sdkwork-im-iam-application-bootstrap` (standalone gateway startup)
- Dev migrate: `pnpm db:postgres:migrate` delegates to `sdkwork-database-cli bootstrap`
- Contract materialization: `pnpm run db:materialize:contract`
- IAM schema is owned by `sdkwork-appbase/database/` and bootstrapped by embedded IAM routes
