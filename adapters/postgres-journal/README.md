# im-adapters-postgres-journal

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

Postgres journal adapter for domain event persistence. Consumes `sdkwork-database-config` for unified database configuration per `DATABASE_SPEC.md`.

## Public API

- Postgres-backed journal storage for `im-domain-events`.
- Retention purge batch execution (`purge_expired_retention_batch`).
- Legal-hold reconcile (`PostgresRetentionScopeStore` / `clear_conversation_retention_until`).
- Background retention purge scheduler with PostgreSQL advisory lock.
- Prometheus retention purge metrics (`im_retention_purge_*`).

## Configuration

Database connection uses `SDKWORK_IM_DATABASE_*` environment variables through `sdkwork-database-config`.

Retention purge scheduler (enabled by default when `SDKWORK_IM_DATABASE_URL` is set):

| Variable | Default | Purpose |
| --- | --- | --- |
| `SDKWORK_IM_RETENTION_PURGE_SCHEDULER_ENABLED` | `true` | Enable background purge ticks |
| `SDKWORK_IM_RETENTION_PURGE_INTERVAL_SECONDS` | `3600` | Tick interval (60–86400) |
| `SDKWORK_IM_RETENTION_PURGE_BATCH_SIZE` | `500` | Rows deleted per store per batch |
| `SDKWORK_IM_RETENTION_PURGE_MAX_BATCHES_PER_TICK` | `100` | Max batches per tick |

Manual purge is also available via `POST /backend/v3/api/ops/retention/purge` (requires `ops.write`).

## Verification

- `cargo test -p im-adapters-postgres-journal retention`
- `pnpm run check:retention-enforcement`
