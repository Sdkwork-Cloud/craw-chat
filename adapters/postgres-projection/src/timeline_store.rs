use im_platform_contracts::ContractError;
use r2d2_postgres::postgres::types::Json;
use sdkwork_im_contract_message::{
    TimelineProjectionBatch, TimelineProjectionRecord, TimelineProjectionScope,
    TimelineProjectionWindow,
};
use serde::Deserialize;

use crate::{
    PostgresProjectionPool, postgres_jsonb_payload_text, postgres_pool_client,
    postgres_unavailable, run_postgres_io,
};

const VERIFY_CANONICAL_MESSAGE_SQL: &str = r#"
select exists (
    select 1
    from im_conversation_messages
    where tenant_id = $1
      and organization_id = $2
      and conversation_id = $3
      and message_seq = $4
      and message_id = $5
      and deleted_at is null
      and (retention_until is null or retention_until > now())
)
"#;

const LOAD_TIMELINE_SQL: &str = r#"
select message_seq, payload_json
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and deleted_at is null
  and (retention_until is null or retention_until > now())
order by message_seq asc
limit $4
"#;

const LOAD_TIMELINE_DEFAULT_LIMIT: i64 = 200;

const LOAD_TIMELINE_WINDOW_SQL: &str = r#"
select message_seq, payload_json
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and message_seq > $4
  and deleted_at is null
  and (retention_until is null or retention_until > now())
order by message_seq asc
limit $5
"#;

/// Compatibility implementation of the old timeline-store port.
///
/// PostgreSQL no longer persists a second timeline. Reads come from the
/// canonical `im_conversation_messages` table and write callbacks only verify
/// that the canonical message transaction has committed.
#[derive(Clone)]
pub struct PostgresTimelineProjectionStore {
    pool: PostgresProjectionPool,
}

impl PostgresTimelineProjectionStore {
    pub fn from_pool(pool: PostgresProjectionPool) -> Self {
        Self { pool }
    }
}

impl sdkwork_im_contract_message::TimelineProjectionStore for PostgresTimelineProjectionStore {
    fn upsert_timeline_entry(
        &self,
        scope: &TimelineProjectionScope,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        verify_canonical_messages(
            &self.pool,
            &[(scope.clone(), message_seq, payload.to_owned())],
        )
    }

    fn load_timeline(
        &self,
        scope: &TimelineProjectionScope,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        let pool = self.pool.clone();
        let scope = scope.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "canonical timeline load")?;
            let rows = client
                .query(
                    LOAD_TIMELINE_SQL,
                    &[
                        &scope.tenant_id(),
                        &scope.organization_id(),
                        &scope.timeline_scope(),
                        &LOAD_TIMELINE_DEFAULT_LIMIT,
                    ],
                )
                .map_err(|error| postgres_unavailable("canonical timeline load", error))?;
            timeline_rows(rows)
        })
    }

    fn load_timeline_window(
        &self,
        scope: &TimelineProjectionScope,
        after_seq: u64,
        limit: usize,
    ) -> Result<TimelineProjectionWindow, ContractError> {
        let pool = self.pool.clone();
        let scope = scope.clone();
        let fetch_limit = i64::try_from(limit.saturating_add(1)).unwrap_or(i64::MAX);
        let after_seq_i64 = i64::try_from(after_seq).unwrap_or(i64::MAX);
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "canonical timeline window")?;
            let rows = client
                .query(
                    LOAD_TIMELINE_WINDOW_SQL,
                    &[
                        &scope.tenant_id(),
                        &scope.organization_id(),
                        &scope.timeline_scope(),
                        &after_seq_i64,
                        &fetch_limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("canonical timeline window", error))?;
            let mut items = timeline_rows(rows)?;
            let has_more = items.len() > limit;
            items.truncate(limit);
            Ok(TimelineProjectionWindow { items, has_more })
        })
    }

    fn upsert_timeline_entries(
        &self,
        scope: &TimelineProjectionScope,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        let rows = records
            .iter()
            .map(|record| (scope.clone(), record.message_seq, record.payload.clone()))
            .collect::<Vec<_>>();
        verify_canonical_messages(&self.pool, &rows)
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        let rows = batches
            .iter()
            .flat_map(|batch| {
                batch.records.iter().map(|record| {
                    (
                        batch.scope.clone(),
                        record.message_seq,
                        record.payload.clone(),
                    )
                })
            })
            .collect::<Vec<_>>();
        verify_canonical_messages(&self.pool, &rows)
    }
}

fn timeline_rows(rows: Vec<postgres::Row>) -> Result<Vec<(u64, String)>, ContractError> {
    rows.into_iter()
        .map(|row| {
            let message_seq: i64 = row.get(0);
            let Json(payload) = row.get::<_, Json<serde_json::Value>>(1);
            Ok((
                message_seq.max(0) as u64,
                postgres_jsonb_payload_text(payload, "canonical message payload")?,
            ))
        })
        .collect()
}

fn verify_canonical_messages(
    pool: &PostgresProjectionPool,
    rows: &[(TimelineProjectionScope, u64, String)],
) -> Result<(), ContractError> {
    if rows.is_empty() {
        return Ok(());
    }
    let pool = pool.clone();
    let rows = rows.to_vec();
    run_postgres_io(move || {
        let mut client = postgres_pool_client(&pool, "canonical timeline verification")?;
        for (scope, message_seq, payload) in rows {
            let message_id = parse_timeline_message_id(payload.as_str())?;
            let message_seq = i64::try_from(message_seq).map_err(|_| {
                ContractError::Invalid("message sequence exceeds signed int64".into())
            })?;
            let row = client
                .query_one(
                    VERIFY_CANONICAL_MESSAGE_SQL,
                    &[
                        &scope.tenant_id(),
                        &scope.organization_id(),
                        &scope.timeline_scope(),
                        &message_seq,
                        &message_id,
                    ],
                )
                .map_err(|error| {
                    postgres_unavailable("canonical timeline verification", error)
                })?;
            if !row.get::<_, bool>(0) {
                return Err(ContractError::Unavailable(
                    "canonical IM message is unavailable for timeline read".into(),
                ));
            }
        }
        Ok(())
    })
}

fn parse_timeline_message_id(payload: &str) -> Result<i64, ContractError> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TimelineIdentity {
        message_id: String,
    }

    let identity = serde_json::from_str::<TimelineIdentity>(payload)
        .map_err(|_| ContractError::Invalid("timeline payload has no valid messageId".into()))?;
    identity
        .message_id
        .parse::<i64>()
        .ok()
        .filter(|message_id| *message_id > 0)
        .ok_or_else(|| ContractError::Invalid("timeline messageId must be positive int64".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_payload_requires_a_positive_message_id() {
        assert_eq!(
            parse_timeline_message_id(r#"{"messageId":"42","messageSeq":1}"#)
                .expect("message id"),
            42
        );
        assert!(parse_timeline_message_id(r#"{"messageId":"0"}"#).is_err());
        assert!(parse_timeline_message_id("{}").is_err());
    }

    #[test]
    fn timeline_sql_reads_the_canonical_message_table() {
        assert!(LOAD_TIMELINE_SQL.contains("from im_conversation_messages"));
        assert!(LOAD_TIMELINE_WINDOW_SQL.contains("from im_conversation_messages"));
        assert!(!LOAD_TIMELINE_SQL.contains("im_projection_"));
    }
}
