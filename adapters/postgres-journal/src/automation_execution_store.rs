//! PostgreSQL implementation of [`AutomationExecutionStore`] (`im_automation_executions`).

use chrono::{DateTime, Utc};
use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};
use im_platform_contracts::ContractError;
use sdkwork_im_contract_agent::{AutomationExecutionRecord, AutomationExecutionStore};
use sdkwork_utils_rust::sha256_hash;

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_jsonb_payload, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

const LOAD_EXECUTION_SQL: &str = r#"
select tenant_id, principal_kind, principal_id, execution_id, trigger_type, target_kind,
    target_ref, input_payload_json::text, output_payload_json::text, execution_state,
    retry_count, requested_at, completed_at, failure_reason, updated_at
from im_automation_executions
where tenant_id = $1 and principal_kind = $2 and principal_id = $3 and execution_id = $4
"#;

const UPSERT_EXECUTION_SQL: &str = r#"
insert into im_automation_executions (
    tenant_id, principal_kind, principal_id, execution_id, trigger_type, target_kind,
    target_ref, request_hash, input_payload_json, input_payload_hash, output_payload_json,
    output_payload_hash, execution_state, retry_count, requested_at, completed_at,
    failure_reason, payload_json, payload_hash, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb, $10, $11::jsonb, $12, $13, $14, $15, $16,
    $17, $18::jsonb, $19, $20, $21
)
on conflict (tenant_id, principal_kind, principal_id, execution_id) do update set
    trigger_type = excluded.trigger_type,
    target_kind = excluded.target_kind,
    target_ref = excluded.target_ref,
    request_hash = excluded.request_hash,
    input_payload_json = excluded.input_payload_json,
    input_payload_hash = excluded.input_payload_hash,
    output_payload_json = excluded.output_payload_json,
    output_payload_hash = excluded.output_payload_hash,
    execution_state = excluded.execution_state,
    retry_count = excluded.retry_count,
    requested_at = excluded.requested_at,
    completed_at = excluded.completed_at,
    failure_reason = excluded.failure_reason,
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    updated_at = excluded.updated_at
"#;

#[derive(Clone)]
pub struct PostgresAutomationExecutionStore {
    pool: PostgresJournalPool,
}

impl PostgresAutomationExecutionStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl AutomationExecutionStore for PostgresAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let principal_id = principal_id.to_owned();
        let execution_id = execution_id.to_owned();
        run_postgres_io(move || {
            load_execution_blocking(
                &pool,
                tenant_id.as_str(),
                principal_kind.as_str(),
                principal_id.as_str(),
                execution_id.as_str(),
            )
        })
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || save_execution_blocking(&pool, record))
    }
}

fn load_execution_blocking(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> Result<Option<AutomationExecutionRecord>, ContractError> {
    let mut client = postgres_pool_client(pool, "automation execution load")?;
    let rows = client
        .query(
            LOAD_EXECUTION_SQL,
            &[&tenant_id, &principal_kind, &principal_id, &execution_id],
        )
        .map_err(|error| postgres_unavailable("automation execution load", error))?;
    rows.first().map(execution_record_from_row).transpose()
}

fn save_execution_blocking(
    pool: &PostgresJournalPool,
    record: AutomationExecutionRecord,
) -> Result<(), ContractError> {
    let mut client = postgres_pool_client(pool, "automation execution save")?;
    let mut transaction = client
        .transaction()
        .map_err(|error| postgres_unavailable("automation execution save transaction", error))?;
    let merged: AutomationExecutionRecord = if let Some(existing) = load_execution_in_transaction(
        &mut transaction,
        record.tenant_id.as_str(),
        record.execution.principal_kind.as_str(),
        record.principal_id.as_str(),
        record.execution_id.as_str(),
    )? {
        existing.merge_monotonic(record)
    } else {
        record
    };
    upsert_execution_in_transaction(&mut transaction, &merged)?;
    transaction
        .commit()
        .map_err(|error| postgres_unavailable("automation execution save commit", error))
}

fn load_execution_in_transaction(
    transaction: &mut postgres::Transaction<'_>,
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> Result<Option<AutomationExecutionRecord>, ContractError> {
    let rows = transaction
        .query(
            LOAD_EXECUTION_SQL,
            &[&tenant_id, &principal_kind, &principal_id, &execution_id],
        )
        .map_err(|error| postgres_unavailable("automation execution load", error))?;
    rows.first().map(execution_record_from_row).transpose()
}

fn upsert_execution_in_transaction(
    transaction: &mut postgres::Transaction<'_>,
    record: &AutomationExecutionRecord,
) -> Result<(), ContractError> {
    let execution = &record.execution;
    let input_payload_text = execution
        .input_payload
        .clone()
        .unwrap_or_else(|| "{}".into());
    let input_payload_json = postgres_jsonb_payload(input_payload_text.as_str())?;
    let input_payload_hash = sha256_hash(input_payload_json.to_string().as_bytes());
    let output_payload_json = execution
        .output_payload
        .as_deref()
        .map(postgres_jsonb_payload)
        .transpose()?;
    let output_payload_hash = output_payload_json
        .as_ref()
        .map(|value| sha256_hash(value.to_string().as_bytes()));
    let payload_json =
        postgres_jsonb_payload(&serde_json::to_string(execution).map_err(|error| {
            ContractError::Conflict(format!(
                "automation execution payload encode failed: {error}"
            ))
        })?)?;
    let payload_hash = sha256_hash(payload_json.to_string().as_bytes());
    let request_hash = sha256_hash(input_payload_text.as_bytes());
    let requested_at = postgres_timestamptz(execution.requested_at.as_str(), "requested_at")?;
    let completed_at = optional_timestamptz(execution.completed_at.as_deref())?;
    let updated_at = postgres_timestamptz(record.updated_at.as_str(), "updated_at")?;
    let created_at = postgres_timestamptz(now_rfc3339().as_str(), "created_at")?;
    let retry_count = i32::try_from(execution.retry_count).map_err(|_| {
        ContractError::Conflict("automation execution retry_count exceeds i32 range".into())
    })?;

    transaction
        .execute(
            UPSERT_EXECUTION_SQL,
            &[
                &execution.tenant_id,
                &execution.principal_kind,
                &execution.principal_id,
                &execution.execution_id,
                &execution.trigger_type,
                &execution.target_kind,
                &execution.target_ref,
                &request_hash,
                &input_payload_json,
                &input_payload_hash,
                &output_payload_json,
                &output_payload_hash,
                &execution.state.as_str(),
                &retry_count,
                &requested_at,
                &completed_at,
                &execution.failure_reason,
                &payload_json,
                &payload_hash,
                &created_at,
                &updated_at,
            ],
        )
        .map_err(|error| postgres_unavailable("automation execution save", error))?;
    Ok(())
}

fn execution_record_from_row(
    row: &postgres::Row,
) -> Result<AutomationExecutionRecord, ContractError> {
    let updated_at = format_timestamptz(row.get::<_, DateTime<Utc>>(14))?;
    let execution = AutomationExecution {
        tenant_id: row.get(0),
        principal_kind: row.get(1),
        principal_id: row.get(2),
        execution_id: row.get(3),
        trigger_type: row.get(4),
        target_kind: row.get(5),
        target_ref: row.get(6),
        input_payload: row.get::<_, Option<String>>(7),
        output_payload: row.get::<_, Option<String>>(8),
        state: parse_execution_state(row.get::<_, String>(9).as_str())?,
        retry_count: u32::try_from(row.get::<_, i32>(10)).map_err(|_| {
            ContractError::Conflict("automation execution retry_count is negative".into())
        })?,
        requested_at: format_timestamptz(row.get::<_, DateTime<Utc>>(11))?,
        completed_at: row
            .get::<_, Option<DateTime<Utc>>>(12)
            .map(format_timestamptz)
            .transpose()?,
        failure_reason: row.get(13),
    };
    Ok(AutomationExecutionRecord {
        tenant_id: execution.tenant_id.clone(),
        principal_id: execution.principal_id.clone(),
        execution_id: execution.execution_id.clone(),
        execution,
        updated_at,
    })
}

fn parse_execution_state(value: &str) -> Result<AutomationExecutionState, ContractError> {
    match value {
        "requested" => Ok(AutomationExecutionState::Requested),
        "running" => Ok(AutomationExecutionState::Running),
        "succeeded" => Ok(AutomationExecutionState::Succeeded),
        "failed" => Ok(AutomationExecutionState::Failed),
        other => Err(ContractError::Conflict(format!(
            "unknown automation execution state: {other}"
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
