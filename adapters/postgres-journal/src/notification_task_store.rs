//! PostgreSQL implementation of [`NotificationTaskStore`] (`im_notification_tasks`).

use chrono::{DateTime, Utc};
use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_platform_contracts::ContractError;
use sdkwork_im_contract_notification::{
    NotificationTaskListCursor, NotificationTaskRecord, NotificationTaskStore,
};
use sdkwork_utils_rust::sha256_hash;

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_jsonb_payload, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

const LOAD_TASK_SQL: &str = r#"
select tenant_id, organization_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json::text,
    requested_at, dispatched_at, failure_reason, updated_at, attempt_count, available_at
from im_notification_tasks
where tenant_id = $1 and organization_id = $2 and notification_id = $3
"#;

/// SDKWork list-standard page size cap (default 20, max 200 per
/// `DATABASE_SPEC.md` section 20.5). Mirrors `message_store.rs`.
const NOTIFICATION_TASK_PAGE_SIZE_MAX: usize = 200;

/// Clamp a requested notification-task page size to the SDKWork list bounds.
fn normalize_notification_task_page_size(page_size: usize) -> usize {
    page_size.clamp(1, NOTIFICATION_TASK_PAGE_SIZE_MAX)
}

const LIST_TASKS_FOR_RECIPIENT_SQL: &str = r#"
select tenant_id, organization_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json::text,
    requested_at, dispatched_at, failure_reason, updated_at, attempt_count, available_at
from im_notification_tasks
where tenant_id = $1 and organization_id = $2 and recipient_kind = $3 and recipient_id = $4
  and ($5::timestamptz is null or (updated_at, notification_id) < ($5, $6))
order by updated_at desc, notification_id desc
limit $7
"#;

const UPSERT_TASK_SQL: &str = r#"
insert into im_notification_tasks (
    tenant_id, organization_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json,
    payload_hash, requested_at, dispatched_at, failure_reason, created_at, updated_at,
    attempt_count, available_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13::jsonb, $14, $15, $16, $17, $18, $19,
    $20, $21
)
on conflict (tenant_id, organization_id, notification_id) do update set
    source_event_id = excluded.source_event_id,
    source_event_type = excluded.source_event_type,
    category = excluded.category,
    channel = excluded.channel,
    recipient_kind = excluded.recipient_kind,
    recipient_id = excluded.recipient_id,
    notification_status = excluded.notification_status,
    title = excluded.title,
    body = excluded.body,
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    requested_at = excluded.requested_at,
    dispatched_at = excluded.dispatched_at,
    failure_reason = excluded.failure_reason,
    updated_at = excluded.updated_at
"#;

const CLAIM_TASKS_SQL: &str = r#"/* sdkwork:cross-organization-operation=notification-worker-claim */

with candidates as materialized (
    select tenant_id, organization_id, notification_id
    from im_notification_tasks
    where notification_status = 'requested'
      and available_at <= $1
    order by available_at, notification_id
    for update skip locked
    limit $2
), claimed as (
    update im_notification_tasks as task
    set available_at = $3, updated_at = $3
    from candidates as candidate
    where task.tenant_id = candidate.tenant_id
      and task.organization_id = candidate.organization_id
      and task.notification_id = candidate.notification_id
      and task.notification_status = 'requested'
    returning task.tenant_id, task.organization_id, task.notification_id
)
select task.tenant_id, task.organization_id, task.notification_id,
    task.source_event_id, task.source_event_type, task.category, task.channel,
    task.recipient_kind, task.recipient_id, task.notification_status, task.title, task.body,
    task.payload_json::text, task.requested_at, task.dispatched_at, task.failure_reason,
    task.updated_at, task.attempt_count, task.available_at
from candidates as candidate
join claimed using (tenant_id, organization_id, notification_id)
join im_notification_tasks as task
  on task.tenant_id = candidate.tenant_id
 and task.organization_id = candidate.organization_id
 and task.notification_id = candidate.notification_id
order by task.available_at, task.notification_id
"#;

const COMPLETE_TASK_SQL: &str = r#"
update im_notification_tasks
set notification_status = 'dispatched',
    dispatched_at = $4,
    failure_reason = null,
    updated_at = $4
where tenant_id = $1 and organization_id = $2 and notification_id = $3
  and notification_status = 'requested'
"#;

const FAIL_TASK_SQL: &str = r#"
update im_notification_tasks
set attempt_count = attempt_count + 1,
    notification_status = case
        when attempt_count + 1 >= $4 then 'failed'
        else 'requested'
    end,
    available_at = case
        when attempt_count + 1 >= $4 then available_at
        else $5 + make_interval(secs => LEAST(300, POWER(2, LEAST(attempt_count, 8)::int))::int)
    end,
    failure_reason = $6,
    updated_at = $5
where tenant_id = $1 and organization_id = $2 and notification_id = $3
  and notification_status = 'requested'
"#;

const NOTIFICATION_MAX_ATTEMPTS_ENV: &str = "SDKWORK_IM_NOTIFICATION_MAX_ATTEMPTS";
const NOTIFICATION_MAX_ATTEMPTS_DEFAULT: i32 = 10;
const NOTIFICATION_CLAIM_LEASE_SECS_ENV: &str = "SDKWORK_IM_NOTIFICATION_CLAIM_LEASE_SECS";
const NOTIFICATION_CLAIM_LEASE_SECS_DEFAULT: i64 = 60;

fn resolve_notification_max_attempts() -> i32 {
    std::env::var(NOTIFICATION_MAX_ATTEMPTS_ENV)
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NOTIFICATION_MAX_ATTEMPTS_DEFAULT)
}

fn resolve_notification_claim_lease_secs() -> i64 {
    std::env::var(NOTIFICATION_CLAIM_LEASE_SECS_ENV)
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NOTIFICATION_CLAIM_LEASE_SECS_DEFAULT)
}

#[derive(Clone)]
pub struct PostgresNotificationTaskStore {
    pool: PostgresJournalPool,
}

impl PostgresNotificationTaskStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl NotificationTaskStore for PostgresNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let notification_id = notification_id.to_owned();
        run_postgres_io(move || {
            load_task_blocking(
                &pool,
                tenant_id.as_str(),
                organization_id.as_str(),
                notification_id.as_str(),
            )
        })
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || save_task_blocking(&pool, record))
    }

    fn list_tasks_for_recipient_page(
        &self,
        tenant_id: &str,
        organization_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
        cursor: Option<&NotificationTaskListCursor>,
        page_size: usize,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let recipient_kind = recipient_kind.to_owned();
        let recipient_id = recipient_id.to_owned();
        let cursor = cursor.cloned();
        let page_size = normalize_notification_task_page_size(page_size);
        run_postgres_io(move || {
            list_tasks_for_recipient_blocking(
                &pool,
                tenant_id.as_str(),
                organization_id.as_str(),
                recipient_kind.as_str(),
                recipient_id.as_str(),
                cursor.as_ref(),
                page_size,
            )
        })
    }

    fn claim_tasks(
        &self,
        limit: usize,
        now: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let pool = self.pool.clone();
        let now = now.to_owned();
        run_postgres_io(move || claim_tasks_blocking(&pool, limit, now.as_str()))
    }

    fn complete_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
        dispatched_at: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let notification_id = notification_id.to_owned();
        let dispatched_at = dispatched_at.to_owned();
        run_postgres_io(move || {
            complete_task_blocking(
                &pool,
                tenant_id.as_str(),
                organization_id.as_str(),
                notification_id.as_str(),
                dispatched_at.as_str(),
            )
        })
    }

    fn fail_task(
        &self,
        tenant_id: &str,
        organization_id: &str,
        notification_id: &str,
        failure_reason: &str,
        now: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let notification_id = notification_id.to_owned();
        let failure_reason = failure_reason.to_owned();
        let now = now.to_owned();
        run_postgres_io(move || {
            fail_task_blocking(
                &pool,
                tenant_id.as_str(),
                organization_id.as_str(),
                notification_id.as_str(),
                failure_reason.as_str(),
                now.as_str(),
            )
        })
    }
}

fn load_task_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    organization_id: &str,
    notification_id: &str,
) -> Result<Option<NotificationTaskRecord>, ContractError> {
    let mut client = postgres_pool_client(pool, "notification task load")?;
    let rows = client
        .query(
            LOAD_TASK_SQL,
            &[&tenant_id, &organization_id, &notification_id],
        )
        .map_err(|error| postgres_unavailable("notification task load", error))?;
    rows.first().map(task_record_from_row).transpose()
}

fn list_tasks_for_recipient_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    organization_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
    cursor: Option<&NotificationTaskListCursor>,
    page_size: usize,
) -> Result<Vec<NotificationTaskRecord>, ContractError> {
    let mut client = postgres_pool_client(pool, "notification task list")?;
    let cursor_updated_at = cursor
        .map(|value| postgres_timestamptz(value.updated_at.as_str(), "cursor.updated_at"))
        .transpose()?;
    let cursor_notification_id = cursor.map(|value| value.notification_id.clone());
    let limit = i64::try_from(page_size.saturating_add(1))
        .map_err(|_| ContractError::Invalid("notification page size exceeds i64 range".into()))?;
    let rows = client
        .query(
            LIST_TASKS_FOR_RECIPIENT_SQL,
            &[
                &tenant_id,
                &organization_id,
                &recipient_kind,
                &recipient_id,
                &cursor_updated_at,
                &cursor_notification_id,
                &limit,
            ],
        )
        .map_err(|error| postgres_unavailable("notification task list", error))?;
    rows.iter().map(task_record_from_row).collect()
}

fn claim_tasks_blocking(
    pool: &PostgresJournalPool,
    limit: usize,
    now: &str,
) -> Result<Vec<NotificationTaskRecord>, ContractError> {
    let limit_i64 = i64::try_from(limit)
        .map_err(|_| ContractError::Invalid("notification claim limit exceeds i64 range".into()))?;
    let now_dt = postgres_timestamptz(now, "claim now")?;
    let lease_expires_at =
        now_dt + chrono::Duration::seconds(resolve_notification_claim_lease_secs());
    let mut client = postgres_pool_client(pool, "notification task claim")?;
    let rows = client
        .query(CLAIM_TASKS_SQL, &[&now_dt, &limit_i64, &lease_expires_at])
        .map_err(|error| postgres_unavailable("notification task claim", error))?;
    rows.iter().map(task_record_from_row).collect()
}

fn complete_task_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    organization_id: &str,
    notification_id: &str,
    dispatched_at: &str,
) -> Result<(), ContractError> {
    let dispatched_at_dt = postgres_timestamptz(dispatched_at, "dispatched_at")?;
    let mut client = postgres_pool_client(pool, "notification task complete")?;
    client
        .execute(
            COMPLETE_TASK_SQL,
            &[
                &tenant_id,
                &organization_id,
                &notification_id,
                &dispatched_at_dt,
            ],
        )
        .map_err(|error| postgres_unavailable("notification task complete", error))?;
    Ok(())
}

fn fail_task_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    organization_id: &str,
    notification_id: &str,
    failure_reason: &str,
    now: &str,
) -> Result<(), ContractError> {
    let now_dt = postgres_timestamptz(now, "failure now")?;
    let max_attempts = resolve_notification_max_attempts();
    let mut client = postgres_pool_client(pool, "notification task fail")?;
    client
        .execute(
            FAIL_TASK_SQL,
            &[
                &tenant_id,
                &organization_id,
                &notification_id,
                &max_attempts,
                &now_dt,
                &failure_reason,
            ],
        )
        .map_err(|error| postgres_unavailable("notification task fail", error))?;
    Ok(())
}

fn save_task_blocking(
    pool: &PostgresJournalPool,
    record: NotificationTaskRecord,
) -> Result<(), ContractError> {
    let mut client = postgres_pool_client(pool, "notification task save")?;
    let mut transaction = client
        .transaction()
        .map_err(|error| postgres_unavailable("notification task save transaction", error))?;
    let lock_key = format!(
        "{}:{}:{}",
        record.tenant_id, record.organization_id, record.notification_id
    );
    transaction
        .query_one(
            "select pg_advisory_xact_lock(hashtextextended($1, 0))",
            &[&lock_key],
        )
        .map_err(|error| postgres_unavailable("notification task save lock", error))?;
    let merged: NotificationTaskRecord = if let Some(existing) = load_task_in_transaction(
        &mut transaction,
        record.tenant_id.as_str(),
        record.organization_id.as_str(),
        record.notification_id.as_str(),
    )? {
        existing.merge_monotonic(record)
    } else {
        record
    };
    upsert_task_in_transaction(&mut transaction, &merged)?;
    transaction
        .commit()
        .map_err(|error| postgres_unavailable("notification task save commit", error))
}

fn load_task_in_transaction(
    transaction: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    organization_id: &str,
    notification_id: &str,
) -> Result<Option<NotificationTaskRecord>, ContractError> {
    let rows = transaction
        .query(
            LOAD_TASK_SQL,
            &[&tenant_id, &organization_id, &notification_id],
        )
        .map_err(|error| postgres_unavailable("notification task load", error))?;
    rows.first().map(task_record_from_row).transpose()
}

fn upsert_task_in_transaction(
    transaction: &mut postgres::Transaction<'_>,
    record: &NotificationTaskRecord,
) -> Result<(), ContractError> {
    let task = &record.task;
    let payload_text = task.payload.clone().unwrap_or_else(|| "{}".into());
    let payload_json = postgres_jsonb_payload(payload_text.as_str())?;
    let payload_hash = sha256_hash(payload_json.to_string().as_bytes());
    let requested_at = postgres_timestamptz(task.requested_at.as_str(), "requested_at")?;
    let dispatched_at = optional_timestamptz(task.dispatched_at.as_deref())?;
    let updated_at = postgres_timestamptz(record.updated_at.as_str(), "updated_at")?;
    let created_at = postgres_timestamptz(now_rfc3339().as_str(), "created_at")?;
    // `attempt_count` is a bounded u32 contract field; i64 conversion is infallible.
    let attempt_count = i64::from(record.attempt_count);
    let available_at = postgres_timestamptz(record.available_at.as_str(), "available_at")?;

    transaction
        .execute(
            UPSERT_TASK_SQL,
            &[
                &task.tenant_id,
                &record.organization_id,
                &task.notification_id,
                &task.source_event_id,
                &task.source_event_type,
                &task.category,
                &task.channel,
                &task.recipient_kind,
                &task.recipient_id,
                &task.status.as_str(),
                &task.title,
                &task.body,
                &payload_json,
                &payload_hash,
                &requested_at,
                &dispatched_at,
                &task.failure_reason,
                &created_at,
                &updated_at,
                &attempt_count,
                &available_at,
            ],
        )
        .map_err(|error| postgres_unavailable("notification task save", error))?;
    Ok(())
}

fn task_record_from_row(row: &postgres::Row) -> Result<NotificationTaskRecord, ContractError> {
    let updated_at = format_timestamptz(row.get::<_, DateTime<Utc>>(16))?;
    let task = NotificationTask {
        tenant_id: row.get(0),
        notification_id: row.get(2),
        source_event_id: row.get(3),
        source_event_type: row.get(4),
        category: row.get(5),
        channel: row.get(6),
        recipient_kind: row.get(7),
        recipient_id: row.get(8),
        status: parse_notification_status(row.get::<_, String>(9).as_str())?,
        title: row.get(10),
        body: row.get(11),
        payload: Some(row.get::<_, String>(12)),
        requested_at: format_timestamptz(row.get::<_, DateTime<Utc>>(13))?,
        dispatched_at: row
            .get::<_, Option<DateTime<Utc>>>(14)
            .map(format_timestamptz)
            .transpose()?,
        failure_reason: row.get(15),
    };
    let attempt_count = u32::try_from(row.get::<_, i64>(17)).map_err(|_| {
        ContractError::Conflict("notification attempt_count exceeds u32 range".into())
    })?;
    Ok(NotificationTaskRecord {
        tenant_id: task.tenant_id.clone(),
        organization_id: row.get(1),
        notification_id: task.notification_id.clone(),
        task,
        updated_at,
        attempt_count,
        available_at: format_timestamptz(row.get::<_, DateTime<Utc>>(18))?,
    })
}

fn parse_notification_status(value: &str) -> Result<NotificationStatus, ContractError> {
    match value {
        "requested" => Ok(NotificationStatus::Requested),
        "dispatched" => Ok(NotificationStatus::Dispatched),
        "failed" => Ok(NotificationStatus::Failed),
        other => Err(ContractError::Conflict(format!(
            "unknown notification status: {other}"
        ))),
    }
}

fn optional_timestamptz(value: Option<&str>) -> Result<Option<DateTime<Utc>>, ContractError> {
    value
        .map(|instant| postgres_timestamptz(instant, "timestamp"))
        .transpose()
}

fn format_timestamptz(value: DateTime<Utc>) -> Result<String, ContractError> {
    Ok(value.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
}

/// Idempotent task insert for the message-post fanout path: existing task
/// rows (same notification id or same source-event unique tuple) are skipped
/// so replaying a message post never duplicates notification requests. Runs
/// inside the caller's transaction so notification requests commit atomically
/// with the message journal/outbox writes.
pub(crate) fn enqueue_notification_tasks_in_transaction(
    transaction: &mut postgres::Transaction<'_>,
    records: &[NotificationTaskRecord],
) -> Result<(), ContractError> {
    const ENQUEUE_TASK_SQL: &str = r#"
insert into im_notification_tasks (
    tenant_id, organization_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json,
    payload_hash, requested_at, dispatched_at, failure_reason, created_at, updated_at,
    attempt_count, available_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13::jsonb, $14, $15, $16, $17, $18, $19,
    $20, $21
)
on conflict do nothing
"#;
    for record in records {
        let task = &record.task;
        let payload_text = task.payload.clone().unwrap_or_else(|| "{}".into());
        let payload_json = postgres_jsonb_payload(payload_text.as_str())?;
        let payload_hash = sha256_hash(payload_json.to_string().as_bytes());
        let requested_at = postgres_timestamptz(task.requested_at.as_str(), "requested_at")?;
        let dispatched_at = optional_timestamptz(task.dispatched_at.as_deref())?;
        let updated_at = postgres_timestamptz(record.updated_at.as_str(), "updated_at")?;
        let created_at = postgres_timestamptz(now_rfc3339().as_str(), "created_at")?;
        // `attempt_count` is a bounded u32 contract field; i64 conversion is infallible.
        let attempt_count = i64::from(record.attempt_count);
        let available_at = postgres_timestamptz(record.available_at.as_str(), "available_at")?;
        transaction
            .execute(
                ENQUEUE_TASK_SQL,
                &[
                    &task.tenant_id,
                    &record.organization_id,
                    &task.notification_id,
                    &task.source_event_id,
                    &task.source_event_type,
                    &task.category,
                    &task.channel,
                    &task.recipient_kind,
                    &task.recipient_id,
                    &task.status.as_str(),
                    &task.title,
                    &task.body,
                    &payload_json,
                    &payload_hash,
                    &requested_at,
                    &dispatched_at,
                    &task.failure_reason,
                    &created_at,
                    &updated_at,
                    &attempt_count,
                    &available_at,
                ],
            )
            .map_err(|error| postgres_unavailable("notification task enqueue", error))?;
    }
    Ok(())
}
