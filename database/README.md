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

## Initialization state

This module is in **initialization state** for greenfield deployments:

1. **Baseline** — `database/ddl/baseline/{engine}/0001_im_baseline.sql` contains the full DDL snapshot.
2. **Migrations** — `database/migrations/{engine}/` is reserved for post-GA incremental schema changes only. It is intentionally empty at initialization.
3. **Drift** — run `pnpm db:drift:check` before release.

## Commands

```bash
pnpm run db:validate
pnpm run db:materialize:contract
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```
