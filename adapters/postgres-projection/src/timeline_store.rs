use im_domain_core::retention::retention_until_from_class;
use im_platform_contracts::ContractError;
use r2d2_postgres::postgres::types::Json;
use sdkwork_im_contract_message::{TimelineProjectionBatch, TimelineProjectionRecord};
use sdkwork_utils_rust::sha256_hash;
use serde::Deserialize;

use crate::{
    default_projection_organization_id, now_rfc3339, postgres_pool_client, postgres_unavailable,
    run_postgres_io, PostgresProjectionPool,
};

const UPSERT_TIMELINE_ENTRY_SQL: &str = r#"
insert into im_projection_timeline_entries (
    tenant_id,
    organization_id,
    conversation_id,
    message_seq,
    message_id,
    summary,
    payload_json,
    payload_hash,
    created_at,
    updated_at,
    retention_until
) values ($1, $2, $3, $4, $5, $6, $7::jsonb, $8, $9, $9, $10)
on conflict (tenant_id, organization_id, conversation_id, message_seq) do update set
    message_id = excluded.message_id,
    summary = excluded.summary,
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    updated_at = excluded.updated_at,
    retention_until = excluded.retention_until
"#;

const LOAD_TIMELINE_SQL: &str = r#"
select message_seq, payload_json::text
from im_projection_timeline_entries
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and (retention_until is null or retention_until > now())
order by message_seq asc
"#;

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
        tenant_id: &str,
        timeline_scope: &str,
        message_seq: u64,
        payload: &str,
    ) -> Result<(), ContractError> {
        upsert_timeline_rows(
            &self.pool,
            &[(
                tenant_id.to_owned(),
                timeline_scope.to_owned(),
                message_seq,
                payload.to_owned(),
            )],
        )
    }

    fn load_timeline(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
    ) -> Result<Vec<(u64, String)>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let timeline_scope = timeline_scope.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "timeline load")?;
            let rows = client
                .query(
                    LOAD_TIMELINE_SQL,
                    &[
                        &tenant_id,
                        &default_projection_organization_id(),
                        &timeline_scope,
                    ],
                )
                .map_err(|error| postgres_unavailable("timeline load select", error))?;
            Ok(rows
                .into_iter()
                .map(|row| {
                    let message_seq: i64 = row.get(0);
                    let payload: String = row.get(1);
                    (message_seq.max(0) as u64, payload)
                })
                .collect())
        })
    }

    fn upsert_timeline_entries(
        &self,
        tenant_id: &str,
        timeline_scope: &str,
        records: &[TimelineProjectionRecord],
    ) -> Result<(), ContractError> {
        let rows = records
            .iter()
            .map(|record| {
                (
                    tenant_id.to_owned(),
                    timeline_scope.to_owned(),
                    record.message_seq,
                    record.payload.clone(),
                )
            })
            .collect::<Vec<_>>();
        upsert_timeline_rows(&self.pool, &rows)
    }

    fn upsert_timeline_batches(
        &self,
        batches: &[TimelineProjectionBatch],
    ) -> Result<(), ContractError> {
        let mut rows = Vec::new();
        for batch in batches {
            for record in &batch.records {
                rows.push((
                    batch.tenant_id.clone(),
                    batch.timeline_scope.clone(),
                    record.message_seq,
                    record.payload.clone(),
                ));
            }
        }
        upsert_timeline_rows(&self.pool, &rows)
    }
}

fn upsert_timeline_rows(
    pool: &PostgresProjectionPool,
    rows: &[(String, String, u64, String)],
) -> Result<(), ContractError> {
    if rows.is_empty() {
        return Ok(());
    }
    let pool = pool.clone();
    let rows = rows.to_vec();
    run_postgres_io(move || {
        let mut client = postgres_pool_client(&pool, "timeline upsert")?;
        let mut transaction = client
            .transaction()
            .map_err(|error| postgres_unavailable("timeline upsert begin", error))?;
        let created_at = now_rfc3339();
        for (tenant_id, conversation_id, message_seq, payload) in rows {
            let parsed = parse_timeline_payload(payload.as_str());
            let retention_until = resolve_timeline_retention_until(&parsed);
            let message_id = parsed.message_id;
            let summary = parsed.summary;
            let payload_hash = sha256_hash(payload.as_bytes());
            let message_seq_i64 = i64::try_from(message_seq).unwrap_or(i64::MAX);
            transaction
                .execute(
                    UPSERT_TIMELINE_ENTRY_SQL,
                    &[
                        &tenant_id,
                        &default_projection_organization_id(),
                        &conversation_id,
                        &message_seq_i64,
                        &message_id,
                        &summary,
                        &Json(payload),
                        &payload_hash,
                        &created_at,
                        &retention_until,
                    ],
                )
                .map_err(|error| postgres_unavailable("timeline upsert execute", error))?;
        }
        transaction
            .commit()
            .map_err(|error| postgres_unavailable("timeline upsert commit", error))?;
        Ok(())
    })
}

#[derive(Default)]
struct ParsedTimelinePayload {
    message_id: i64,
    summary: Option<String>,
    occurred_at: Option<String>,
    retention_until: Option<String>,
    retention_class: Option<String>,
}

fn parse_timeline_payload(payload: &str) -> ParsedTimelinePayload {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TimelinePayloadFields {
        message_id: String,
        summary: Option<String>,
        occurred_at: Option<String>,
        retention_until: Option<String>,
        retention_class: Option<String>,
    }

    let Ok(fields) = serde_json::from_str::<TimelinePayloadFields>(payload) else {
        return ParsedTimelinePayload::default();
    };
    ParsedTimelinePayload {
        message_id: fields.message_id.parse().unwrap_or(0),
        summary: fields.summary,
        occurred_at: fields.occurred_at,
        retention_until: fields.retention_until,
        retention_class: fields.retention_class,
    }
}

fn resolve_timeline_retention_until(parsed: &ParsedTimelinePayload) -> Option<String> {
    if let Some(until) = parsed
        .retention_until
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Some(until.to_owned());
    }
    parsed.occurred_at.as_deref().and_then(|occurred_at| {
        let retention_class = parsed
            .retention_class
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("standard");
        retention_until_from_class(retention_class, occurred_at)
    })
}

#[cfg(test)]
mod retention_tests {
    use super::*;

    #[test]
    fn test_resolve_timeline_retention_until_prefers_payload_value() {
        let parsed = ParsedTimelinePayload {
            message_id: 1,
            summary: None,
            occurred_at: Some("2026-01-01T00:00:00.000Z".into()),
            retention_until: Some("2027-01-01T00:00:00.000Z".into()),
            retention_class: Some("ephemeral".into()),
        };
        assert_eq!(
            resolve_timeline_retention_until(&parsed).as_deref(),
            Some("2027-01-01T00:00:00.000Z")
        );
    }

    #[test]
    fn test_resolve_timeline_retention_until_uses_retention_class() {
        let parsed = ParsedTimelinePayload {
            message_id: 1,
            summary: None,
            occurred_at: Some("2026-01-01T00:00:00.000Z".into()),
            retention_until: None,
            retention_class: Some("ephemeral".into()),
        };
        assert_eq!(
            resolve_timeline_retention_until(&parsed).as_deref(),
            Some("2026-01-08T00:00:00.000Z")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_timeline_payload_extracts_message_id_and_summary() {
        let payload = r#"{"messageId":"42","messageSeq":1,"summary":"hello"}"#;
        let parsed = parse_timeline_payload(payload);
        assert_eq!(parsed.message_id, 42);
        assert_eq!(parsed.summary.as_deref(), Some("hello"));
    }
}
