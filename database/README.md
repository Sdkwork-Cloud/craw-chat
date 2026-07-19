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
cargo test -p im-adapters-postgres-journal --test agent_integration_migration_live_test -- --ignored --nocapture
```

## Initialization state

This module uses an immutable baseline plus versioned migrations:

1. **Baseline** — `database/ddl/baseline/postgres/0001_im_baseline.sql` is the **runtime authority** for IM core (journal, projection, social materializer, search).
2. **SQLite compatibility baseline** — `database/ddl/baseline/sqlite/0001_im_baseline.sql` exists only for lifecycle validation and desktop gateway co-location checks. It is not engine parity. **IM services do not persist to SQLite**; `SDKWORK_IM_DATABASE_ENGINE=sqlite` uses in-memory ephemeral IM state in dev/test. Desktop `chat.sqlite` hosts gateway webstore and sibling module databases, not the IM event log.
3. **Migrations** — `database/migrations/{engine}/` contains the conversation-id rewrite, managed group Knowledgebase binding, stream authority, and the active IM-to-Agents integration migration. PostgreSQL is the runtime authority; SQLite migration files validate the explicitly limited compatibility surface.

Contract `2.0.0` activates the three IM-to-Agents tables from paired PostgreSQL
migration `0005`; paired migration `0006` adds validated scope/sign guards.
Their tenant, organization, end-user subject, message, and
Snowflake fields were BIGINT from creation; the adapter rejects non-decimal,
zero where forbidden, and values above signed int64 before persistence. The
table registry retains migration provenance instead of rewriting the historical
baseline.
Migration `0007` normalizes legacy projection metadata and timeline rows that
stored serialized JSON as JSONB strings; current adapters persist JSONB values directly.
Migration `0008` preserves positive signed-int64 IAM user actors while reserving
`assigned_by = 0` for trusted system and other non-user assignment events.
4. **Drift** — run `pnpm db:drift:check` before release.

The PostgreSQL baseline is tenant-and-organization isolated: primary keys, unique
constraints, and hot-path indexes include `tenant_id, organization_id` where
business data is scoped. Realtime device event windows also keep a deferrable
foreign key to realtime checkpoints so trim/ack state and event windows cannot
silently drift apart.

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

`db:materialize:contract` materializes the PostgreSQL runtime authority only. SQLite compatibility assets are maintained under `database/ddl/baseline/sqlite/` for checker coverage and desktop co-location validation; they are not a production IM persistence profile and must not be presented as PostgreSQL-equivalent support.
