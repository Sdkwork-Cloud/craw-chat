# im-adapters-postgres-journal

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

Postgres journal adapter for domain event persistence. Consumes `sdkwork-database-config` for unified database configuration per `DATABASE_SPEC.md`.

## Public API

- Postgres-backed journal storage for `im-domain-events`.
- Bounded journal audit queries scoped by tenant, organization, and aggregate.
- Retention purge batch execution (`purge_expired_retention_batch`) through a validated
  `RetentionPurgeRequest` carrying authorized actor and trace evidence.
- Legal-hold reconcile (`PostgresRetentionScopeStore` / `clear_conversation_retention_until`).
- Background retention purge scheduler with PostgreSQL advisory lock.
- Prometheus retention purge metrics (`im_retention_purge_*`).

## Configuration

Database connection uses the process-level `SDKWORK_DATABASE_*` profile through `sdkwork-database-config`.

Retention purge scheduler (enabled by default when `SDKWORK_DATABASE_URL` is set):

| Variable | Default | Purpose |
| --- | --- | --- |
| `SDKWORK_IM_RETENTION_PURGE_SCHEDULER_ENABLED` | `true` | Enable background purge ticks |
| `SDKWORK_IM_RETENTION_PURGE_INTERVAL_SECONDS` | `3600` | Tick interval (60–86400) |
| `SDKWORK_IM_RETENTION_PURGE_BATCH_SIZE` | `500` | Rows deleted per store per batch |
| `SDKWORK_IM_RETENTION_PURGE_MAX_BATCHES_PER_TICK` | `100` | Max batches per tick |

Manual purge is also available via `POST /backend/v3/api/ops/retention/purge` (requires `ops.write`).

## Isolation And Module Boundaries

The production adapter does not implement global `recorded_page` replay. Ordinary runtime reads are
bounded by `tenant_id`, `organization_id`, and aggregate identity; current state comes from normalized
tables, not journal replay. Cross-organization journal recovery requires a separately specified
operations component with explicit authorization and audit evidence.

`src/lib.rs` is assembly only. Journal SQL lives in `journal_queries.rs`, journal transaction and row
mapping logic in `journal_repository.rs`, and shared pool/config/I/O support in
`postgres_support.rs`.

## Verification

- `cargo test -p im-adapters-postgres-journal retention`
- `pnpm --dir ../.. test:tenant-isolation-standard`
- `pnpm run check:retention-enforcement`
