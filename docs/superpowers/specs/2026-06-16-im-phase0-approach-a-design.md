# IM Phase 0 — Approach A Design

**Date:** 2026-06-16  
**Status:** Approved for implementation  
**Scope:** Truth layer + tenant/org isolation + ID/sequence consistency (PostgreSQL source of truth; Redis optional)

## Decision

Phase 0 uses **single-table PostgreSQL truth** with migrations `001` → `010` → `011` → `012` → `014`.  
Migration `013` (partitioning / dual-write) is **deferred to Phase 2** and must not run in local or greenfield deploys.

## Architecture

| Layer | Phase 0 choice |
|-------|----------------|
| Message truth | `im_conversation_messages` (BIGINT Snowflake `message_id`, BIGINT `message_seq`) |
| Seq allocation | `im_conversation_seq_counters` (Postgres atomic upsert) |
| Journal / outbox | `im_commit_journal`, `im_outbox_events` (same transaction as message write) |
| Tenant isolation | `tenant_id` + `organization_id TEXT NOT NULL DEFAULT 'default'` on all `im_*` tables |
| Redis | Optional hot cache only; not required for seq or truth |
| Partitioning | Deferred (`013` in `migrations/deferred/`) |

## Migration path

```
001_im_core_schema.sql          — bootstrap (legacy-compatible CREATE IF NOT EXISTS)
010_im_tenant_organization_isolation.sql — DROP/CREATE core tables + org_id + seq counters
011_im_projections_rtc_streams.sql       — RTC, audit, notifications, projections, streams
012_im_social_org_interactions.sql      — social / org / messaging interaction tables
014_im_search_cjk.sql                   — search trigger only (non-destructive)
```

`pnpm db:postgres:migrate` applies all `migrations/*.sql` in lexical order; `deferred/` is excluded.

## ID contract

- **Database:** Snowflake `BIGINT` for `message_id`, `message_seq`, and other high-cardinality counters.
- **HTTP / OpenAPI:** `type: string`, `format: int64`, digit `pattern`, `x-sdkwork-int64-string: true`.
- **Runtime:** `sdkwork-im-runtime-id` + `PostgresMessageStore::allocate_message_seq`; memory `high_watermark` is fallback only when no store is wired.

## API surfaces

Authority OpenAPI files (not generated SDK output):

- `sdks/sdkwork-im-sdk/openapi/sdkwork-im-im.openapi.yaml`
- `sdks/sdkwork-im-app-sdk/openapi/sdkwork-im-app-api.openapi.yaml`
- `sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml`

Derived `*.sdkgen.yaml` is regenerated via `node sdks/materialize-im-v3-openapi-boundaries.mjs`.

## Registry alignment

`specs/database-table-registry.json` migration fields point to the owning migration file (`010`, `011`, or `012`).  
`im_conversation_seq_counters` is registered under migration `010`.

## Verification

```bash
pnpm run test:database-naming-standard
pnpm run test:workflow-commercial-gates
node sdks/materialize-im-v3-openapi-boundaries.mjs
cargo test -p session-gateway --test database_schema_contract_test
cargo fmt --all --check
```

## Out of scope (Phase 2+)

- `013` expand-contract partitioning and dual-write
- Kafka / Elasticsearch
- Full removal of in-memory runtime fallback paths
