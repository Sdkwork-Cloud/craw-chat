//! Parameterized SQL owned by the PostgreSQL commit-journal repository.

/// Insert a single event and return its committed position.
pub(crate) const APPEND_EVENT_SQL: &str = r#"
insert into im_commit_journal (
    partition_key,
    commit_offset,
    event_id,
    tenant_id,
    organization_id,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    event_type,
    payload_json,
    payload_hash,
    idempotency_key,
    occurred_at,
    created_at,
    retention_until
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::jsonb, $11, $12, $13, $14, $15)
on conflict (event_id) do nothing
returning partition_key, commit_offset
"#;

pub(crate) const LOAD_EVENT_BY_ID_SQL: &str = r#"
select
    partition_key,
    commit_offset,
    tenant_id,
    organization_id,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    event_type,
    payload_hash,
    idempotency_key,
    occurred_at,
    retention_until
from im_commit_journal
where event_id = $1 and tenant_id = $2 and organization_id = $3
"#;

pub(crate) const LOAD_EVENT_BY_POSITION_SQL: &str = r#"
select event_id, partition_key, commit_offset
from im_commit_journal
where partition_key = $1 and commit_offset = $2
  and tenant_id = $3 and organization_id = $4
"#;

pub(crate) const LOCK_JOURNAL_PARTITION_SQL: &str =
    "select pg_advisory_xact_lock(hashtextextended($1, 0::bigint))";

pub(crate) const LOAD_MAX_AGGREGATE_SEQ_SQL: &str = r#"
select coalesce(max(commit_offset), 0)::bigint
from im_commit_journal
where partition_key = $1 and tenant_id = $2 and organization_id = $3
"#;

/// Journal replay high-water mark: committed row count and the head
/// `commit_offset` of the append-only ledger (`ops replay_status`).
pub(crate) const REPLAY_STATE_SQL: &str = r#"/* sdkwork:cross-organization-operation=journal-replay-state */

select count(*)::bigint as total,
       max(commit_offset) as head_offset,
       max(occurred_at) as latest_occurred_at
from im_commit_journal
"#;

pub(crate) const LOAD_RECORDED_AGGREGATE_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and organization_id = $3
  and aggregate_id = $4
order by commit_offset asc
limit $5
"#;

pub(crate) const LOAD_RECORDED_AGGREGATE_AFTER_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and organization_id = $3
  and aggregate_id = $4
  and commit_offset > $5
order by commit_offset asc
limit $6
"#;

pub(crate) const LOAD_RECORDED_AGGREGATE_EVENT_TYPES_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and organization_id = $3
  and aggregate_type = $4
  and aggregate_id = $5
  and event_type = any($6)
order by commit_offset asc
limit $7
"#;

pub(crate) const LOAD_RECORDED_AGGREGATE_EVENT_TYPES_AFTER_SQL: &str = r#"
select
    event_id,
    tenant_id,
    organization_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at,
    payload_json::text,
    idempotency_key,
    partition_key,
    commit_offset
from im_commit_journal
where partition_key like $1 || '%'
  and tenant_id = $2
  and organization_id = $3
  and aggregate_type = $4
  and aggregate_id = $5
  and event_type = any($6)
  and commit_offset > $7
order by commit_offset asc
limit $8
"#;
