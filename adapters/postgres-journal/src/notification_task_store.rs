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
    requested_at, dispatched_at, failure_reason, updated_at
from im_notification_tasks
where tenant_id = $1 and organization_id = $2 and notification_id = $3
"#;

const LIST_TASKS_FOR_RECIPIENT_SQL: &str = r#"
select tenant_id, organization_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json::text,
    requested_at, dispatched_at, failure_reason, updated_at
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
    payload_hash, requested_at, dispatched_at, failure_reason, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13::jsonb, $14, $15, $16, $17, $18, $19
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
    Ok(NotificationTaskRecord {
        tenant_id: task.tenant_id.clone(),
        organization_id: row.get(1),
        notification_id: task.notification_id.clone(),
        task,
        updated_at,
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
