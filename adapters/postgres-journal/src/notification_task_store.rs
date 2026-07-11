//! PostgreSQL implementation of [`NotificationTaskStore`] (`im_notification_tasks`).

use chrono::{DateTime, Utc};
use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_platform_contracts::ContractError;
use sdkwork_im_contract_notification::{NotificationTaskRecord, NotificationTaskStore};
use sdkwork_utils_rust::sha256_hash;
use tracing::warn;

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_jsonb_payload, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

const LOAD_TASK_SQL: &str = r#"
select tenant_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json::text,
    requested_at, dispatched_at, failure_reason, updated_at
from im_notification_tasks
where tenant_id = $1 and notification_id = $2
"#;

const LIST_TASKS_FOR_RECIPIENT_SQL: &str = r#"
select tenant_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json::text,
    requested_at, dispatched_at, failure_reason, updated_at
from im_notification_tasks
where tenant_id = $1 and recipient_kind = $2 and recipient_id = $3
  and ($4::timestamptz is null or (updated_at, notification_id) < ($4, $5))
order by updated_at desc, notification_id desc
limit $6
"#;

const NOTIFICATION_TASK_RESTORE_BATCH_SIZE: i64 = 200;

const NOTIFICATION_TASK_RESTORE_MAX_TOTAL_RECORDS: usize = 10_000;

const UPSERT_TASK_SQL: &str = r#"
insert into im_notification_tasks (
    tenant_id, notification_id, source_event_id, source_event_type, category, channel,
    recipient_kind, recipient_id, notification_status, title, body, payload_json,
    payload_hash, requested_at, dispatched_at, failure_reason, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::jsonb, $13, $14, $15, $16, $17, $18
)
on conflict (tenant_id, notification_id) do update set
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
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let notification_id = notification_id.to_owned();
        run_postgres_io(move || {
            load_task_blocking(&pool, tenant_id.as_str(), notification_id.as_str())
        })
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || save_task_blocking(&pool, record))
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let recipient_kind = recipient_kind.to_owned();
        let recipient_id = recipient_id.to_owned();
        run_postgres_io(move || {
            list_tasks_for_recipient_blocking(
                &pool,
                tenant_id.as_str(),
                recipient_kind.as_str(),
                recipient_id.as_str(),
            )
        })
    }
}

fn load_task_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    notification_id: &str,
) -> Result<Option<NotificationTaskRecord>, ContractError> {
    let mut client = postgres_pool_client(pool, "notification task load")?;
    let rows = client
        .query(LOAD_TASK_SQL, &[&tenant_id, &notification_id])
        .map_err(|error| postgres_unavailable("notification task load", error))?;
    rows.first().map(task_record_from_row).transpose()
}

fn list_tasks_for_recipient_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> Result<Vec<NotificationTaskRecord>, ContractError> {
    let mut client = postgres_pool_client(pool, "notification task list")?;
    let mut last_updated_at: Option<DateTime<Utc>> = None;
    let mut last_notification_id: Option<String> = None;
    let mut records = Vec::new();
    loop {
        let rows = client
            .query(
                LIST_TASKS_FOR_RECIPIENT_SQL,
                &[
                    &tenant_id,
                    &recipient_kind,
                    &recipient_id,
                    &last_updated_at,
                    &last_notification_id,
                    &NOTIFICATION_TASK_RESTORE_BATCH_SIZE,
                ],
            )
            .map_err(|error| postgres_unavailable("notification task list", error))?;
        let batch_len = rows.len();
        if batch_len == 0 {
            break;
        }
        let last_row = rows.last().expect("non-empty batch has a last row");
        let next_updated_at: DateTime<Utc> = last_row.get(15);
        let next_notification_id: String = last_row.get(1);
        for row in rows {
            records.push(task_record_from_row(&row)?);
        }
        if records.len() >= NOTIFICATION_TASK_RESTORE_MAX_TOTAL_RECORDS {
            warn!(
                target: "sdkwork.im",
                event = "im.notification_task.list_max_total_records_reached",
                tenant_id = %tenant_id,
                recipient_kind = %recipient_kind,
                recipient_id = %recipient_id,
                records = records.len(),
                limit = NOTIFICATION_TASK_RESTORE_MAX_TOTAL_RECORDS,
                "notification task list hit max_total_records safety cap; truncating restore"
            );
            break;
        }
        if batch_len < NOTIFICATION_TASK_RESTORE_BATCH_SIZE as usize {
            break;
        }
        last_updated_at = Some(next_updated_at);
        last_notification_id = Some(next_notification_id);
    }
    Ok(records)
}

fn save_task_blocking(
    pool: &PostgresJournalPool,
    record: NotificationTaskRecord,
) -> Result<(), ContractError> {
    let mut client = postgres_pool_client(pool, "notification task save")?;
    let mut transaction = client
        .transaction()
        .map_err(|error| postgres_unavailable("notification task save transaction", error))?;
    let merged: NotificationTaskRecord = if let Some(existing) = load_task_in_transaction(
        &mut transaction,
        record.tenant_id.as_str(),
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
    notification_id: &str,
) -> Result<Option<NotificationTaskRecord>, ContractError> {
    let rows = transaction
        .query(LOAD_TASK_SQL, &[&tenant_id, &notification_id])
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
    let updated_at = format_timestamptz(row.get::<_, DateTime<Utc>>(15))?;
    let task = NotificationTask {
        tenant_id: row.get(0),
        notification_id: row.get(1),
        source_event_id: row.get(2),
        source_event_type: row.get(3),
        category: row.get(4),
        channel: row.get(5),
        recipient_kind: row.get(6),
        recipient_id: row.get(7),
        status: parse_notification_status(row.get::<_, String>(8).as_str())?,
        title: row.get(9),
        body: row.get(10),
        payload: Some(row.get::<_, String>(11)),
        requested_at: format_timestamptz(row.get::<_, DateTime<Utc>>(12))?,
        dispatched_at: row
            .get::<_, Option<DateTime<Utc>>>(13)
            .map(format_timestamptz)
            .transpose()?,
        failure_reason: row.get(14),
    };
    Ok(NotificationTaskRecord {
        tenant_id: task.tenant_id.clone(),
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
