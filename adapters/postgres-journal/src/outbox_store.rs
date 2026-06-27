//! PostgreSQL implementation of [`OutboxStore`] trait.
//!
//! Implements distributed outbox pattern with FOR UPDATE SKIP LOCKED.

use im_platform_contracts::{ContractError, OutboxEventRecord, OutboxPublishStatus, OutboxStore};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::NoTls;

use crate::{
    now_rfc3339, postgres_jsonb_payload, postgres_pool_client, postgres_unavailable, run_postgres_io,
};

pub type PostgresJournalPool = Pool<PostgresConnectionManager<NoTls>>;

/// PostgreSQL implementation of [`OutboxStore`].
#[derive(Clone)]
pub struct PostgresOutboxStore {
    pool: PostgresJournalPool,
}

impl PostgresOutboxStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

// SQL constants

const ENQUEUE_SQL: &str = r#"
insert into im_outbox_events (
    tenant_id, organization_id, outbox_id, aggregate_type, aggregate_id,
    event_id, event_type, payload_json, payload_hash, publish_status,
    attempt_count, available_at, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10, $11, $12, $13, $14)
"#;

const DRAIN_PENDING_SQL: &str = r#"
select tenant_id, organization_id, outbox_id, aggregate_type, aggregate_id,
    event_id, event_type, payload_json::text, payload_hash, publish_status,
    attempt_count, available_at, published_at, created_at, updated_at
from im_outbox_events
where tenant_id = $1 and organization_id = $2
    and publish_status = 'pending' and available_at <= $3
order by available_at, outbox_id
for update skip locked
limit $4
"#;

const MARK_PUBLISHED_SQL: &str = r#"
update im_outbox_events
set publish_status = 'published', published_at = $4, updated_at = $4
where tenant_id = $1 and organization_id = $2 and outbox_id = $3
"#;

const MARK_FAILED_SQL: &str = r#"
update im_outbox_events
set publish_status = 'failed', attempt_count = attempt_count + 1, updated_at = $4
where tenant_id = $1 and organization_id = $2 and outbox_id = $3
"#;

const READ_BY_EVENT_ID_SQL: &str = r#"
select tenant_id, organization_id, outbox_id, aggregate_type, aggregate_id,
    event_id, event_type, payload_json::text, payload_hash, publish_status,
    attempt_count, available_at, published_at, created_at, updated_at
from im_outbox_events
where tenant_id = $1 and organization_id = $2 and event_id = $3
"#;

const COUNT_PENDING_SQL: &str = r#"
select count(*) from im_outbox_events
where tenant_id = $1 and organization_id = $2 and publish_status = 'pending'
"#;

fn row_to_record(row: &postgres::Row) -> OutboxEventRecord {
    let status_str: String = row.get(9);
    OutboxEventRecord {
        tenant_id: row.get(0),
        organization_id: row.get(1),
        outbox_id: row.get(2),
        aggregate_type: row.get(3),
        aggregate_id: row.get(4),
        event_id: row.get(5),
        event_type: row.get(6),
        payload_json: row.get(7),
        payload_hash: row.get(8),
        publish_status: OutboxPublishStatus::from_str(&status_str)
            .unwrap_or(OutboxPublishStatus::Pending),
        attempt_count: row.get::<_, i32>(10) as u32,
        available_at: row.get(11),
        published_at: row.get(12),
        created_at: row.get(13),
        updated_at: row.get(14),
    }
}

impl OutboxStore for PostgresOutboxStore {
    fn enqueue(&self, event: OutboxEventRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "enqueue")?;
            let attempt_count_i32 = event.attempt_count as i32;
            let payload_json = postgres_jsonb_payload(event.payload_json.as_str())?;
            let params: &[&(dyn postgres::types::ToSql + Sync)] = &[
                &event.tenant_id,
                &event.organization_id,
                &event.outbox_id,
                &event.aggregate_type,
                &event.aggregate_id,
                &event.event_id,
                &event.event_type,
                &payload_json,
                &event.payload_hash,
                &event.publish_status.as_str(),
                &attempt_count_i32,
                &event.available_at,
                &event.created_at,
                &event.updated_at,
            ];
            let result = client.execute(ENQUEUE_SQL, params);
            match result {
                Ok(_) => Ok(()),
                Err(error) => {
                    if error.code() == Some(&postgres::error::SqlState::UNIQUE_VIOLATION) {
                        Err(ContractError::Conflict("event already enqueued".into()))
                    } else {
                        Err(postgres_unavailable("enqueue", error))
                    }
                }
            }
        })
    }

    fn drain_pending(
        &self,
        tenant_id: &str,
        organization_id: &str,
        batch_size: usize,
    ) -> Result<Vec<OutboxEventRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let now = now_rfc3339();
        let limit = batch_size as i32;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "drain_pending")?;
            let rows = client
                .query(
                    DRAIN_PENDING_SQL,
                    &[&tenant_id, &organization_id, &now, &limit],
                )
                .map_err(|error| postgres_unavailable("drain_pending", error))?;
            Ok(rows.iter().map(row_to_record).collect())
        })
    }

    fn mark_published(
        &self,
        tenant_id: &str,
        organization_id: &str,
        outbox_id: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let outbox_id = outbox_id.to_owned();
        let now = now_rfc3339();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "mark_published")?;
            client
                .execute(
                    MARK_PUBLISHED_SQL,
                    &[&tenant_id, &organization_id, &outbox_id, &now],
                )
                .map_err(|error| postgres_unavailable("mark_published", error))?;
            Ok(())
        })
    }

    fn mark_failed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        outbox_id: &str,
        _reason: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let outbox_id = outbox_id.to_owned();
        let now = now_rfc3339();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "mark_failed")?;
            client
                .execute(
                    MARK_FAILED_SQL,
                    &[&tenant_id, &organization_id, &outbox_id, &now],
                )
                .map_err(|error| postgres_unavailable("mark_failed", error))?;
            Ok(())
        })
    }

    fn read_by_event_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        event_id: &str,
    ) -> Result<Option<OutboxEventRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let event_id = event_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_by_event_id")?;
            let row = client
                .query_opt(
                    READ_BY_EVENT_ID_SQL,
                    &[&tenant_id, &organization_id, &event_id],
                )
                .map_err(|error| postgres_unavailable("read_by_event_id", error))?;
            Ok(row.map(|r| row_to_record(&r)))
        })
    }

    fn count_pending(&self, tenant_id: &str, organization_id: &str) -> Result<u64, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "count_pending")?;
            let row = client
                .query_one(COUNT_PENDING_SQL, &[&tenant_id, &organization_id])
                .map_err(|error| postgres_unavailable("count_pending", error))?;
            let count: i64 = row.get(0);
            Ok(count as u64)
        })
    }

    fn retry_failed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        outbox_id: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let outbox_id = outbox_id.to_owned();
        let now = now_rfc3339();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "retry_failed")?;
            client
                .execute(
                    "update im_outbox_events set publish_status = 'pending', updated_at = $4 where tenant_id = $1 and organization_id = $2 and outbox_id = $3 and publish_status = 'failed'",
                    &[&tenant_id, &organization_id, &outbox_id, &now],
                )
                .map_err(|error| postgres_unavailable("retry_failed", error))?;
            Ok(())
        })
    }
}
