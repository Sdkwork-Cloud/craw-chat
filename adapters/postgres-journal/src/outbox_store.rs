//! PostgreSQL implementation of [`OutboxStore`] trait.
//!
//! Implements distributed outbox pattern with FOR UPDATE SKIP LOCKED.

use im_platform_contracts::{
    ContractError, OutboxEventClaim, OutboxEventRecord, OutboxPublishStatus, OutboxStore,
};

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_jsonb_payload, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

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

const CLAIM_PENDING_SQL: &str = r#"
with candidates as materialized (
    select tenant_id, organization_id, outbox_id, aggregate_type, aggregate_id,
        event_id, event_type, payload_json, payload_hash, publish_status,
        attempt_count, available_at, published_at, created_at, updated_at
    from im_outbox_events
    where tenant_id = $1 and organization_id = $2
        and aggregate_type = $3
        and publish_status = 'pending' and available_at <= $5
    order by available_at, outbox_id
    for update skip locked
    limit $6
), claimed as (
    update im_outbox_events as event
    set available_at = $4, updated_at = $5
    from candidates as candidate
    where event.tenant_id = candidate.tenant_id
        and event.organization_id = candidate.organization_id
        and event.outbox_id = candidate.outbox_id
        and event.publish_status = 'pending'
        and event.available_at = candidate.available_at
    returning event.tenant_id, event.organization_id, event.outbox_id,
        event.available_at as lease_expires_at
)
select candidate.tenant_id, candidate.organization_id, candidate.outbox_id,
    candidate.aggregate_type, candidate.aggregate_id, candidate.event_id,
    candidate.event_type, candidate.payload_json::text, candidate.payload_hash,
    candidate.publish_status, candidate.attempt_count, candidate.available_at,
    candidate.published_at, candidate.created_at, candidate.updated_at,
    claimed.lease_expires_at
from candidates as candidate
join claimed using (tenant_id, organization_id, outbox_id)
order by candidate.available_at, candidate.outbox_id
"#;

const MARK_PUBLISHED_SQL: &str = r#"
update im_outbox_events
set publish_status = 'published', published_at = $5, updated_at = $5
where tenant_id = $1 and organization_id = $2 and outbox_id = $3
    and publish_status = 'pending' and available_at = $4
"#;

const MARK_FAILED_SQL: &str = r#"
UPDATE im_outbox_events
SET
    attempt_count = attempt_count + 1,
    publish_status = CASE
        WHEN attempt_count + 1 >= $6 THEN 'failed'
        ELSE 'pending'
    END,
    available_at = CASE
        WHEN attempt_count + 1 >= $6 THEN available_at
        ELSE $5 + make_interval(secs => LEAST(300, POWER(2, LEAST(attempt_count, 8)::int))::int)
    END,
    updated_at = $5
WHERE tenant_id = $1 AND organization_id = $2 AND outbox_id = $3
    AND publish_status = 'pending' AND available_at = $4
"#;

const OUTBOX_MAX_ATTEMPTS_ENV: &str = "SDKWORK_IM_OUTBOX_MAX_ATTEMPTS";
const OUTBOX_MAX_ATTEMPTS_DEFAULT: i32 = 10;

fn resolve_outbox_max_attempts() -> i32 {
    std::env::var(OUTBOX_MAX_ATTEMPTS_ENV)
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(OUTBOX_MAX_ATTEMPTS_DEFAULT)
}

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

const LIST_PENDING_SCOPES_SQL: &str = r#"
select tenant_id, organization_id
from im_outbox_events
where publish_status = 'pending' and available_at <= $1
    and aggregate_type = $2
group by tenant_id, organization_id
order by min(available_at), tenant_id, organization_id
limit $3
"#;

const RETRY_FAILED_SQL: &str = r#"
update im_outbox_events
set publish_status = 'pending', attempt_count = 0, available_at = $4,
    published_at = null, updated_at = $4
where tenant_id = $1 and organization_id = $2 and outbox_id = $3
    and publish_status = 'failed'
"#;

fn row_to_record(row: &postgres::Row) -> OutboxEventRecord {
    let status_str: String = row.get(9);
    let available_at: chrono::DateTime<chrono::Utc> = row.get(11);
    let published_at: Option<chrono::DateTime<chrono::Utc>> = row.get(12);
    let created_at: chrono::DateTime<chrono::Utc> = row.get(13);
    let updated_at: chrono::DateTime<chrono::Utc> = row.get(14);
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
        available_at: available_at.to_rfc3339(),
        published_at: published_at.map(|dt| dt.to_rfc3339()),
        created_at: created_at.to_rfc3339(),
        updated_at: updated_at.to_rfc3339(),
    }
}

impl OutboxStore for PostgresOutboxStore {
    fn enqueue(&self, event: OutboxEventRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "enqueue")?;
            let attempt_count_i32 = event.attempt_count as i32;
            let payload_json = postgres_jsonb_payload(event.payload_json.as_str())?;
            let available_at = postgres_timestamptz(event.available_at.as_str(), "available_at")?;
            let created_at = postgres_timestamptz(event.created_at.as_str(), "created_at")?;
            let updated_at = postgres_timestamptz(event.updated_at.as_str(), "updated_at")?;
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
                &available_at,
                &created_at,
                &updated_at,
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

    fn claim_pending(
        &self,
        tenant_id: &str,
        organization_id: &str,
        aggregate_type: &str,
        batch_size: usize,
        lease_duration: std::time::Duration,
    ) -> Result<Vec<OutboxEventClaim>, ContractError> {
        if batch_size == 0 {
            return Ok(Vec::new());
        }
        if lease_duration.is_zero() {
            return Err(ContractError::Invalid(
                "outbox claim lease_duration must be greater than zero".into(),
            ));
        }
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let aggregate_type = aggregate_type.to_owned();
        let now = postgres_timestamptz(&now_rfc3339(), "now")?;
        let lease_delta = chrono::Duration::from_std(lease_duration).map_err(|_| {
            ContractError::Invalid("outbox claim lease_duration is out of range".into())
        })?;
        let lease_expires_at = now + lease_delta;
        let limit = i64::try_from(batch_size)
            .map_err(|_| ContractError::Invalid("outbox claim batch_size is too large".into()))?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "claim_pending")?;
            let rows = client
                .query(
                    CLAIM_PENDING_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &aggregate_type,
                        &lease_expires_at,
                        &now,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("claim_pending", error))?;
            Ok(rows.iter().map(row_to_claim).collect())
        })
    }

    fn mark_published(&self, claim: &OutboxEventClaim) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = claim.event.tenant_id.clone();
        let organization_id = claim.event.organization_id.clone();
        let outbox_id = claim.event.outbox_id.clone();
        let lease_expires_at =
            postgres_timestamptz(claim.lease_expires_at.as_str(), "lease_expires_at")?;
        let now = postgres_timestamptz(&now_rfc3339(), "now")?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "mark_published")?;
            let affected_rows = client
                .execute(
                    MARK_PUBLISHED_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &outbox_id,
                        &lease_expires_at,
                        &now,
                    ],
                )
                .map_err(|error| postgres_unavailable("mark_published", error))?;
            require_claim_transition("mark_published", affected_rows)
        })
    }

    fn mark_failed(&self, claim: &OutboxEventClaim, _reason: &str) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = claim.event.tenant_id.clone();
        let organization_id = claim.event.organization_id.clone();
        let outbox_id = claim.event.outbox_id.clone();
        let lease_expires_at =
            postgres_timestamptz(claim.lease_expires_at.as_str(), "lease_expires_at")?;
        let now = postgres_timestamptz(&now_rfc3339(), "now")?;
        let max_attempts = resolve_outbox_max_attempts();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "mark_failed")?;
            let affected_rows = client
                .execute(
                    MARK_FAILED_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &outbox_id,
                        &lease_expires_at,
                        &now,
                        &max_attempts,
                    ],
                )
                .map_err(|error| postgres_unavailable("mark_failed", error))?;
            require_claim_transition("mark_failed", affected_rows)
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
        let now = postgres_timestamptz(&now_rfc3339(), "now")?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "retry_failed")?;
            client
                .execute(
                    RETRY_FAILED_SQL,
                    &[&tenant_id, &organization_id, &outbox_id, &now],
                )
                .map_err(|error| postgres_unavailable("retry_failed", error))?;
            Ok(())
        })
    }

    fn list_pending_scopes(
        &self,
        aggregate_type: &str,
        limit: usize,
    ) -> Result<Vec<(String, String)>, ContractError> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let pool = self.pool.clone();
        let aggregate_type = aggregate_type.to_owned();
        let now = postgres_timestamptz(&now_rfc3339(), "now")?;
        let limit = i64::try_from(limit)
            .map_err(|_| ContractError::Invalid("outbox scope limit is too large".into()))?;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "list_pending_scopes")?;
            let rows = client
                .query(LIST_PENDING_SCOPES_SQL, &[&now, &aggregate_type, &limit])
                .map_err(|error| postgres_unavailable("list_pending_scopes", error))?;
            Ok(rows
                .iter()
                .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
                .collect())
        })
    }
}

fn row_to_claim(row: &postgres::Row) -> OutboxEventClaim {
    let event = row_to_record(row);
    let lease_expires_at: chrono::DateTime<chrono::Utc> = row.get(15);
    OutboxEventClaim {
        event,
        lease_expires_at: lease_expires_at.to_rfc3339(),
    }
}

fn require_claim_transition(operation: &str, affected_rows: u64) -> Result<(), ContractError> {
    if affected_rows == 1 {
        Ok(())
    } else {
        Err(ContractError::Conflict(format!(
            "{operation} rejected because the outbox claim expired or changed"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized_sql(sql: &str) -> String {
        sql.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase()
    }

    #[test]
    fn pending_claim_is_atomic_domain_scoped_and_leased() {
        let sql = normalized_sql(CLAIM_PENDING_SQL);

        assert!(sql.contains("aggregate_type = $3"));
        assert!(sql.contains("for update skip locked"));
        assert!(sql.contains("update im_outbox_events"));
        assert!(sql.contains("returning"));
        assert!(sql.contains("available_at = $4"));
    }

    #[test]
    fn publish_transition_is_fenced_by_pending_status_and_lease() {
        let sql = normalized_sql(MARK_PUBLISHED_SQL);

        assert!(sql.contains("publish_status = 'pending'"));
        assert!(sql.contains("available_at = $4"));
    }

    #[test]
    fn failure_transition_is_fenced_by_pending_status_and_lease() {
        let sql = normalized_sql(MARK_FAILED_SQL);

        assert!(sql.contains("publish_status = 'pending'"));
        assert!(sql.contains("available_at = $4"));
    }

    #[test]
    fn pending_scope_discovery_is_domain_scoped() {
        let sql = normalized_sql(LIST_PENDING_SCOPES_SQL);

        assert!(sql.contains("aggregate_type = $2"));
    }

    #[test]
    fn retry_failed_makes_the_event_immediately_claimable() {
        let sql = normalized_sql(RETRY_FAILED_SQL);

        assert!(sql.contains("available_at = $4"));
        assert!(sql.contains("published_at = null"));
        assert!(sql.contains("attempt_count = 0"));
        assert!(sql.contains("publish_status = 'failed'"));
    }
}
