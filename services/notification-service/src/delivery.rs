//! Notification delivery pipeline: claims `requested` tasks from the
//! `NotificationTaskStore`, delivers each task into the recipient's realtime
//! event window (the `im_realtime_device_events` mechanism the session
//! gateway serves to connected clients), then marks the task `dispatched` or
//! retries/dead-letters it on failure.

use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use im_domain_core::notification::NotificationTask;
use im_postgres_realtime_contracts::{
    LOAD_REALTIME_CHECKPOINT_SQL, UPSERT_REALTIME_CHECKPOINT_SQL,
    UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
};
use im_time::utc_now_rfc3339_millis;
use r2d2::PooledConnection;
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_notification::{GlobalNotificationTaskClaimRequest, NotificationTaskRecord, NotificationTaskStore};

use crate::error::NotificationError;

const NOTIFICATION_REALTIME_EVENT_TYPE: &str = "notification.created";
const NOTIFICATION_REALTIME_DELIVERY_CLASS: &str = "standard";
const NOTIFICATION_DELIVERY_BATCH_LIMIT: usize = 64;
const NOTIFICATION_DELIVERY_POLL_MS_ENV: &str = "SDKWORK_IM_NOTIFICATION_DELIVERY_POLL_MS";
const NOTIFICATION_DELIVERY_POLL_MS_DEFAULT: u64 = 1_000;
const NOTIFICATION_DELIVERY_BATCH_LIMIT_ENV: &str = "SDKWORK_IM_NOTIFICATION_DELIVERY_BATCH_LIMIT";

/// Resolves the recipient's registered device ids (rows in
/// `im_realtime_subscriptions` are created for every device that synced
/// realtime subscriptions through the session gateway).
const LIST_RECIPIENT_DEVICES_SQL: &str = r#"
select distinct device_id
from im_realtime_subscriptions
where tenant_id = $1 and organization_id = $2
  and principal_kind = $3 and principal_id = $4
order by device_id asc
"#;

const ADVISORY_LOCK_SQL: &str = "select pg_advisory_xact_lock(hashtextextended($1, 0))";

/// Realtime delivery sink for one claimed notification task.
///
/// Kept as a trait so the delivery pipeline can be exercised with a fake
/// sink in tests and so hosts without a realtime plane can opt into a
/// documented no-op (development in-memory runtimes only).
pub trait NotificationRealtimeDelivery: Send + Sync {
    /// Writes the durable `notification.created` event into every registered
    /// device window of the task recipient. Returns the number of device
    /// windows the event was written for.
    fn deliver(&self, record: &NotificationTaskRecord) -> Result<usize, ContractError>;
}

/// Development-only sink for in-memory runtimes that have no PostgreSQL
/// realtime plane. Delivery is a documented no-op (the task is still marked
/// dispatched so the in-memory pipeline remains observable); production
/// deployments must configure PostgreSQL.
pub struct NoopNotificationRealtimeDelivery;

impl NotificationRealtimeDelivery for NoopNotificationRealtimeDelivery {
    fn deliver(&self, record: &NotificationTaskRecord) -> Result<usize, ContractError> {
        tracing::warn!(
            notification_id = %record.task.notification_id,
            recipient_id = %record.task.recipient_id,
            "notification delivery skipped: no PostgreSQL realtime plane configured (dev-only in-memory runtime)"
        );
        Ok(0)
    }
}

/// PostgreSQL-backed delivery sink: writes one durable event per registered
/// device of the recipient into `im_realtime_device_events` and advances the
/// per-device checkpoint monotonically. Cross-process writes are safe because
/// each device scope is serialized by an advisory transaction lock and the
/// checkpoint upsert takes `greatest` of the seq columns. This is the same
/// event window the session gateway serves to connected clients
/// (`client_route_events`), so a notification written here is delivered on
/// the recipient's next event-window read (immediately for connected clients
/// that rebuild their window, otherwise on reconnect).
pub struct PostgresNotificationRealtimeDelivery {
    pool: im_adapters_postgres_journal::PostgresJournalPool,
}

impl PostgresNotificationRealtimeDelivery {
    pub fn from_pool(pool: im_adapters_postgres_journal::PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl NotificationRealtimeDelivery for PostgresNotificationRealtimeDelivery {
    fn deliver(&self, record: &NotificationTaskRecord) -> Result<usize, ContractError> {
        let pool = self.pool.clone();
        let record = record.clone();
        run_pg_io(move || deliver_to_recipient_devices(&pool, &record))
    }
}

fn deliver_to_recipient_devices(
    pool: &im_adapters_postgres_journal::PostgresJournalPool,
    record: &NotificationTaskRecord,
) -> Result<usize, ContractError> {
    let task = &record.task;
    let mut client = pg_pool_client(pool, "notification delivery connect")?;
    let devices = client
        .query(
            LIST_RECIPIENT_DEVICES_SQL,
            &[
                &task.tenant_id,
                &record.organization_id,
                &task.recipient_kind,
                &task.recipient_id,
            ],
        )
        .map_err(|error| pg_unavailable("list notification recipient devices", &error))?;
    let mut delivered = 0usize;
    for row in devices {
        let device_id: String = row.get(0);
        deliver_to_device_window(&mut client, record, device_id.as_str())?;
        delivered = delivered.saturating_add(1);
    }
    Ok(delivered)
}

fn deliver_to_device_window(
    client: &mut PooledConnection<im_adapters_postgres_journal::PostgresJournalConnectionManager>,
    record: &NotificationTaskRecord,
    device_id: &str,
) -> Result<(), ContractError> {
    let task = &record.task;
    let client_route_scope_key = realtime_client_route_scope_key(record, device_id);
    let mut transaction = client
        .transaction()
        .map_err(|error| pg_unavailable("notification delivery begin", &error))?;
    transaction
        .query_one(ADVISORY_LOCK_SQL, &[&client_route_scope_key])
        .map_err(|error| pg_unavailable("notification delivery lock", &error))?;
    let next_seq = next_realtime_seq_in_transaction(&mut transaction, record, device_id)?;
    let occurred_at = utc_now_rfc3339_millis();
    let payload_json = notification_event_payload_json(task);
    let payload_value = postgres_jsonb_value(payload_json.as_str())?;
    let payload_hash = format!(
        "sha256:{}",
        sdkwork_utils_rust::sha256_hash(payload_json.as_bytes())
    );
    let retention_until = im_domain_core::retention::retention_until_from_class(
        NOTIFICATION_REALTIME_DELIVERY_CLASS,
        occurred_at.as_str(),
    )
    .and_then(|value| parse_utc(value.as_str()).ok());
    let statement_timestamp = Utc::now();
    transaction
        .execute(
            UPSERT_REALTIME_CLIENT_ROUTE_EVENT_SQL,
            &[
                &task.tenant_id,
                &record.organization_id,
                &client_route_scope_key,
                &(next_seq as i64),
                &task.recipient_kind,
                &task.recipient_id,
                &device_id,
                &"user".to_string(),
                &task.recipient_id,
                &NOTIFICATION_REALTIME_EVENT_TYPE,
                &NOTIFICATION_REALTIME_DELIVERY_CLASS,
                &payload_value,
                &payload_hash,
                &parse_utc(occurred_at.as_str())?,
                &statement_timestamp,
                &retention_until,
            ],
        )
        .map_err(|error| pg_unavailable("upsert notification delivery event", &error))?;
    transaction
        .execute(
            UPSERT_REALTIME_CHECKPOINT_SQL,
            &[
                &task.tenant_id,
                &record.organization_id,
                &client_route_scope_key,
                &task.recipient_kind,
                &task.recipient_id,
                &device_id,
                &(next_seq as i64),
                &0i64,
                &0i64,
                &0i64,
                &0i64,
                &Option::<DateTime<Utc>>::None,
                &statement_timestamp,
                &statement_timestamp,
            ],
        )
        .map_err(|error| pg_unavailable("upsert notification delivery checkpoint", &error))?;
    transaction
        .commit()
        .map_err(|error| pg_unavailable("notification delivery commit", &error))?;
    Ok(())
}

fn next_realtime_seq_in_transaction(
    transaction: &mut postgres::Transaction<'_>,
    record: &NotificationTaskRecord,
    device_id: &str,
) -> Result<u64, ContractError> {
    let client_route_scope_key = realtime_client_route_scope_key(record, device_id);
    let row = transaction
        .query_opt(
            LOAD_REALTIME_CHECKPOINT_SQL,
            &[
                &record.task.tenant_id,
                &record.organization_id,
                &client_route_scope_key,
            ],
        )
        .map_err(|error| pg_unavailable("load notification delivery checkpoint", &error))?;
    let latest: i64 = row.map(|row| row.get::<_, i64>(5)).unwrap_or(0);
    Ok(latest.max(0) as u64 + 1)
}

/// `im_realtime_device_events` / `im_realtime_checkpoints` scope key with the
/// canonical organization normalization, matching the session gateway.
fn realtime_client_route_scope_key(record: &NotificationTaskRecord, device_id: &str) -> String {
    let organization_id =
        im_platform_contracts::normalize_realtime_organization_id(record.organization_id.as_str());
    realtime_scope_key_parts(&[
        record.task.tenant_id.as_str(),
        organization_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
        device_id,
    ])
}

fn realtime_scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

fn notification_event_payload_json(task: &NotificationTask) -> String {
    serde_json::json!({
        "notificationId": task.notification_id,
        "sourceEventId": task.source_event_id,
        "sourceEventType": task.source_event_type,
        "category": task.category,
        "channel": task.channel,
        "recipientId": task.recipient_id,
        "recipientKind": task.recipient_kind,
        "title": task.title,
        "body": task.body,
        "requestedAt": task.requested_at,
        "payload": task.payload.as_deref().and_then(|value| {
            serde_json::from_str::<serde_json::Value>(value).ok()
        }),
    })
    .to_string()
}

/// Summary of one delivery cycle for observability and tests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeliveryCycleSummary {
    pub claimed: usize,
    pub dispatched: usize,
    pub failed: usize,
}

/// Runs one delivery cycle: claim up to `limit` tasks, dispatch each, and
/// record dispatched/failed outcomes. Failures are retried by the store
/// (exponential backoff, dead-letter after the attempt cap).
pub fn run_delivery_cycle(
    task_store: &dyn NotificationTaskStore,
    realtime: &dyn NotificationRealtimeDelivery,
    limit: usize,
) -> Result<DeliveryCycleSummary, NotificationError> {
    let now = utc_now_rfc3339_millis();
    let request = GlobalNotificationTaskClaimRequest::try_new(limit, now.clone())
        .map_err(NotificationError::notification_store)?;
    let claimed = task_store
        .claim_tasks(request.limit, request.now.as_str())
        .map_err(NotificationError::notification_store)?;
    tracing::info!(
        target: "sdkwork.im.security",
        event = "im.notification_task_claim.operation_completed",
        actor_kind = "notification-delivery-worker",
        actor_id = "notification-delivery",
        trace_id = %request.now,
        outcome = "succeeded",
        claimed = claimed.len(),
        "cross-organization notification worker claim completed"
    );
    let mut summary = DeliveryCycleSummary {
        claimed: claimed.len(),
        ..DeliveryCycleSummary::default()
    };
    for record in claimed {
        let dispatched_at = utc_now_rfc3339_millis();
        match realtime.deliver(&record) {
            Ok(_) => {
                task_store
                    .complete_task(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.notification_id.as_str(),
                        dispatched_at.as_str(),
                    )
                    .map_err(NotificationError::notification_store)?;
                summary.dispatched = summary.dispatched.saturating_add(1);
            }
            Err(error) => {
                task_store
                    .fail_task(
                        record.tenant_id.as_str(),
                        record.organization_id.as_str(),
                        record.notification_id.as_str(),
                        &format!("notification delivery failed: {error:?}"),
                        utc_now_rfc3339_millis().as_str(),
                    )
                    .map_err(NotificationError::notification_store)?;
                summary.failed = summary.failed.saturating_add(1);
            }
        }
    }
    Ok(summary)
}

pub fn resolve_delivery_batch_limit() -> usize {
    std::env::var(NOTIFICATION_DELIVERY_BATCH_LIMIT_ENV)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NOTIFICATION_DELIVERY_BATCH_LIMIT)
}

pub fn resolve_delivery_poll_interval() -> Duration {
    let millis = std::env::var(NOTIFICATION_DELIVERY_POLL_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NOTIFICATION_DELIVERY_POLL_MS_DEFAULT);
    Duration::from_millis(millis)
}

/// Spawns the delivery worker loop on the current tokio runtime.
///
/// The worker ticks on an interval, claims available tasks with
/// `FOR UPDATE SKIP LOCKED` semantics, and dispatches them into the
/// recipients' realtime event windows. The task handle can be aborted during
/// graceful shutdown; aborted mid-cycle tasks are re-claimed after their
/// lease expires.
pub fn spawn_delivery_worker(
    task_store: Arc<dyn NotificationTaskStore>,
    realtime: Arc<dyn NotificationRealtimeDelivery>,
) -> tokio::task::JoinHandle<()> {
    let poll_interval = resolve_delivery_poll_interval();
    let batch_limit = resolve_delivery_batch_limit();
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(poll_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            match run_delivery_cycle(task_store.as_ref(), realtime.as_ref(), batch_limit) {
                Ok(summary) => {
                    if summary.claimed > 0 {
                        tracing::info!(
                            claimed = summary.claimed,
                            dispatched = summary.dispatched,
                            failed = summary.failed,
                            "notification delivery cycle completed"
                        );
                    }
                }
                Err(error) => {
                    tracing::warn!(
                        error = ?error,
                        "notification delivery cycle failed; tasks remain claimable after their lease expires"
                    );
                }
            }
        }
    })
}

fn run_pg_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
    if let Ok(handle) = tokio::runtime::Handle::try_current()
        && handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread
    {
        return tokio::task::block_in_place(operation);
    }
    std::thread::scope(|scope| {
        scope.spawn(operation).join().map_err(|_| {
            ContractError::Unavailable("notification delivery IO worker panicked".into())
        })?
    })
}

fn pg_pool_client(
    pool: &im_adapters_postgres_journal::PostgresJournalPool,
    action: &'static str,
) -> Result<
    PooledConnection<im_adapters_postgres_journal::PostgresJournalConnectionManager>,
    ContractError,
> {
    pool.get().map_err(|error| pg_unavailable(action, &error))
}

fn pg_unavailable(action: &str, error: &dyn std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!("{action} failed: {error}"))
}

fn postgres_jsonb_value(
    payload: &str,
) -> Result<postgres::types::Json<serde_json::Value>, ContractError> {
    serde_json::from_str::<serde_json::Value>(payload)
        .map(postgres::types::Json)
        .map_err(|error| {
            ContractError::Invalid(format!(
                "notification realtime payload is not valid JSON: {error}"
            ))
        })
}

fn parse_utc(value: &str) -> Result<DateTime<Utc>, ContractError> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_| {
            ContractError::Invalid(format!(
                "notification realtime timestamp is invalid: {value}"
            ))
        })
}
