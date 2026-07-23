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

1. **Baseline** — `database/ddl/baseline/postgres/0001_im_baseline.sql` is the immutable 57-table PostgreSQL bootstrap base for normalized Conversation, Message, Member, ReadCursor, realtime, Social, stream, call-signaling, journal, outbox, and search state. It is not the complete contract by itself.
2. **SQLite compatibility baseline** — `database/ddl/baseline/sqlite/0001_im_baseline.sql` exists only for lifecycle validation and desktop gateway co-location checks. It is not engine parity. **IM services do not persist to SQLite**; `SDKWORK_IM_DATABASE_ENGINE=sqlite` uses in-memory ephemeral IM state in dev/test. Desktop `chat.sqlite` hosts gateway webstore and sibling module databases, not the IM event log.
3. **Migrations** — `database/migrations/postgres/` completes the active contract, including the three IM-to-Agents assignment/binding/dispatch tables introduced by `0005`, and evolves installed databases in version order. The effective PostgreSQL authority is always baseline plus migrations; SQLite migration files validate only the explicitly limited compatibility surface.

Contract `2.0.0` activates the three IM-to-Agents tables from paired PostgreSQL
migration `0005`; paired migration `0006` adds validated scope/sign guards.
Their tenant, organization, end-user subject, message, and
Snowflake fields were BIGINT from creation; the adapter rejects non-decimal,
zero where forbidden, and values above signed int64 before persistence. The
table registry retains migration provenance instead of rewriting the historical
baseline.
Migration `0008` preserves positive signed-int64 IAM user actors while reserving
`assigned_by = 0` for trusted system and other non-user assignment events.
Migration `0009` aligns the IM-to-Agents assignment stale-write guard with the IM
journal's zero-based aggregate sequence while keeping assignment generations positive.
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
pnpm run db:contract:check
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

`db:materialize:contract` deterministically composes the PostgreSQL baseline and every ordered
`database/migrations/postgres/*.up.sql` file, then aligns the canonical schema and table registry.
`db:contract:check` performs the same 60-table equality check without writing. Neither command reads
the SQLite compatibility baseline or treats it as a production IM persistence profile.
