//! PostgreSQL implementation of [`StreamStateStore`].
//!
//! Persists stream session metadata to `im_stream_sessions` and frame rows to
//! `im_stream_frames` (see `database/ddl/baseline/postgres/0001_im_baseline.sql`).

use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_platform_contracts::{ContractError, StreamStateRecord, StreamStateStore};
use sdkwork_utils_rust::sha256_hash;
use serde::{Deserialize, Serialize};

use crate::{
    now_rfc3339, postgres_jsonb_payload, postgres_pool_client, postgres_timestamptz,
    postgres_unavailable, run_postgres_io, PostgresJournalPool,
};

const DEFAULT_ORGANIZATION_ID: &str = "0";

const LOAD_SESSION_SQL: &str = r#"
select tenant_id, stream_id, owner_principal_kind, owner_principal_id, stream_type,
    scope_kind, scope_id, durability_class, ordering_scope, schema_ref, stream_state,
    last_frame_seq, last_checkpoint_seq, result_message_id, complete_frame_seq,
    abort_frame_seq, abort_reason, opened_at, closed_at, expires_at,
    payload_json::text, updated_at
from im_stream_sessions
where tenant_id = $1 and organization_id = $2 and stream_id = $3
"#;

const LOAD_FRAMES_SQL: &str = r#"
select frame_seq, producer_principal_kind, producer_principal_id, schema_ref,
    payload_json::text, occurred_at
from im_stream_frames
where tenant_id = $1 and organization_id = $2 and stream_id = $3
order by frame_seq asc
"#;

const UPSERT_SESSION_SQL: &str = r#"
insert into im_stream_sessions (
    tenant_id, organization_id, stream_id, owner_principal_kind, owner_principal_id,
    stream_type, scope_kind, scope_id, durability_class, ordering_scope, schema_ref,
    stream_state, last_frame_seq, last_checkpoint_seq, result_message_id,
    complete_frame_seq, abort_frame_seq, abort_reason, opened_at, closed_at, expires_at,
    payload_json, payload_hash, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
    $19, $20, $21, $22::jsonb, $23, $24, $25
)
on conflict (tenant_id, organization_id, stream_id) do update set
    owner_principal_kind = excluded.owner_principal_kind,
    owner_principal_id = excluded.owner_principal_id,
    stream_type = excluded.stream_type,
    scope_kind = excluded.scope_kind,
    scope_id = excluded.scope_id,
    durability_class = excluded.durability_class,
    ordering_scope = excluded.ordering_scope,
    schema_ref = excluded.schema_ref,
    stream_state = excluded.stream_state,
    last_frame_seq = excluded.last_frame_seq,
    last_checkpoint_seq = excluded.last_checkpoint_seq,
    result_message_id = excluded.result_message_id,
    complete_frame_seq = excluded.complete_frame_seq,
    abort_frame_seq = excluded.abort_frame_seq,
    abort_reason = excluded.abort_reason,
    opened_at = excluded.opened_at,
    closed_at = excluded.closed_at,
    expires_at = excluded.expires_at,
    payload_json = excluded.payload_json,
    payload_hash = excluded.payload_hash,
    updated_at = excluded.updated_at
"#;

const UPSERT_FRAME_SQL: &str = r#"
insert into im_stream_frames (
    tenant_id, organization_id, stream_id, frame_seq, producer_principal_kind,
    producer_principal_id, schema_ref, payload_json, payload_hash, occurred_at, created_at
) values ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10, $11)
on conflict (tenant_id, organization_id, stream_id, frame_seq) do nothing
"#;

const DELETE_FRAMES_SQL: &str = r#"
delete from im_stream_frames
where tenant_id = $1 and organization_id = $2 and stream_id = $3
"#;

const DELETE_SESSION_SQL: &str = r#"
delete from im_stream_sessions
where tenant_id = $1 and organization_id = $2 and stream_id = $3
"#;

const SESSION_EXISTS_SQL: &str = r#"
select exists (
    select 1 from im_stream_sessions
    where tenant_id = $1 and organization_id = $2 and stream_id = $3
)
"#;

#[derive(Clone)]
pub struct PostgresStreamStateStore {
    pool: PostgresJournalPool,
}

impl PostgresStreamStateStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamSessionPayloadExtras {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    result_message_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamFramePayload {
    stream_type: String,
    scope_kind: String,
    scope_id: String,
    frame_type: String,
    encoding: String,
    payload: String,
    sender: im_domain_core::message::Sender,
    attributes: im_domain_core::message::MessageAttributes,
}

impl StreamStateStore for PostgresStreamStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let stream_id = stream_id.to_owned();
        run_postgres_io(move || load_state(&pool, tenant_id.as_str(), stream_id.as_str()))
    }

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let existing = load_state(&pool, record.tenant_id.as_str(), record.stream_id.as_str())?;
            let merged = existing
                .map(|previous| previous.merge_monotonic(record.clone()))
                .unwrap_or(record);
            save_state(&pool, merged)
        })
    }

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let stream_id = stream_id.to_owned();
        run_postgres_io(move || clear_state(&pool, tenant_id.as_str(), stream_id.as_str()))
    }
}

fn load_state(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    stream_id: &str,
) -> Result<Option<StreamStateRecord>, ContractError> {
    let mut client = postgres_pool_client(pool, "stream state load")?;
    let session_row = client
        .query_opt(
            LOAD_SESSION_SQL,
            &[&tenant_id, &DEFAULT_ORGANIZATION_ID, &stream_id],
        )
        .map_err(|error| postgres_unavailable("stream state load session", error))?;
    let Some(session_row) = session_row else {
        return Ok(None);
    };

    let frame_rows = client
        .query(
            LOAD_FRAMES_SQL,
            &[&tenant_id, &DEFAULT_ORGANIZATION_ID, &stream_id],
        )
        .map_err(|error| postgres_unavailable("stream state load frames", error))?;

    let session = session_from_row(&session_row)?;
    let frames = frame_rows
        .iter()
        .map(|row| frame_from_row(row, tenant_id, stream_id))
        .collect::<Result<Vec<_>, _>>()?;
    let updated_at: String = session_row.get(21);

    Ok(Some(StreamStateRecord {
        tenant_id: tenant_id.into(),
        stream_id: stream_id.into(),
        session,
        frames,
        updated_at,
    }))
}

fn save_state(pool: &PostgresJournalPool, record: StreamStateRecord) -> Result<(), ContractError> {
    let mut client = postgres_pool_client(pool, "stream state save")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("stream state save begin", error))?;

    let session = &record.session;
    let (result_message_id, payload_extras) = result_message_id_for_db(session);
    let payload_json = postgres_jsonb_payload(
        &serde_json::to_string(&payload_extras).map_err(|error| {
            ContractError::Conflict(format!("stream session payload encode failed: {error}"))
        })?,
    )?;
    let payload_hash = sha256_hash(payload_json.to_string().as_bytes());
    let opened_at = postgres_timestamptz(session.opened_at.as_str(), "opened_at")?;
    let closed_at = optional_timestamptz(session.closed_at.as_deref())?;
    let expires_at = optional_timestamptz(session.expires_at.as_deref())?;
    let updated_at = postgres_timestamptz(record.updated_at.as_str(), "updated_at")?;
    let created_at = postgres_timestamptz(now_rfc3339().as_str(), "created_at")?;
    let last_frame_seq = i64::try_from(session.last_frame_seq).map_err(|_| {
        ContractError::Conflict("stream session last_frame_seq exceeds i64 range".into())
    })?;
    let last_checkpoint_seq = optional_u64_as_i64(session.last_checkpoint_seq)?;
    let complete_frame_seq = optional_u64_as_i64(session.complete_frame_seq)?;
    let abort_frame_seq = optional_u64_as_i64(session.abort_frame_seq)?;

    txn.execute(
        UPSERT_SESSION_SQL,
        &[
            &record.tenant_id,
            &DEFAULT_ORGANIZATION_ID,
            &record.stream_id,
            &session.owner_principal_kind,
            &session.owner_principal_id,
            &session.stream_type,
            &session.scope_kind,
            &session.scope_id,
            &session.durability_class.as_wire_value(),
            &session.ordering_scope,
            &session.schema_ref,
            &session.state.as_wire_value(),
            &last_frame_seq,
            &last_checkpoint_seq,
            &result_message_id,
            &complete_frame_seq,
            &abort_frame_seq,
            &session.abort_reason,
            &opened_at,
            &closed_at,
            &expires_at,
            &payload_json,
            &payload_hash,
            &created_at,
            &updated_at,
        ],
    )
    .map_err(|error| postgres_unavailable("stream state save session", error))?;

    for frame in &record.frames {
        let frame_payload = StreamFramePayload {
            stream_type: frame.stream_type.clone(),
            scope_kind: frame.scope_kind.clone(),
            scope_id: frame.scope_id.clone(),
            frame_type: frame.frame_type.clone(),
            encoding: frame.encoding.clone(),
            payload: frame.payload.clone(),
            sender: frame.sender.clone(),
            attributes: frame.attributes.clone(),
        };
        let frame_json = postgres_jsonb_payload(
            &serde_json::to_string(&frame_payload).map_err(|error| {
                ContractError::Conflict(format!("stream frame payload encode failed: {error}"))
            })?,
        )?;
        let frame_hash = sha256_hash(frame_json.to_string().as_bytes());
        let frame_seq = i64::try_from(frame.frame_seq).map_err(|_| {
            ContractError::Conflict("stream frame frame_seq exceeds i64 range".into())
        })?;
        let occurred_at = postgres_timestamptz(frame.occurred_at.as_str(), "occurred_at")?;
        txn.execute(
            UPSERT_FRAME_SQL,
            &[
                &record.tenant_id,
                &DEFAULT_ORGANIZATION_ID,
                &record.stream_id,
                &frame_seq,
                &frame.sender.kind,
                &frame.sender.id,
                &frame.schema_ref,
                &frame_json,
                &frame_hash,
                &occurred_at,
                &created_at,
            ],
        )
        .map_err(|error| postgres_unavailable("stream state save frame", error))?;
    }

    txn.commit()
        .map_err(|error| postgres_unavailable("stream state save commit", error))?;
    Ok(())
}

fn clear_state(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    stream_id: &str,
) -> Result<bool, ContractError> {
    let mut client = postgres_pool_client(pool, "stream state clear")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("stream state clear begin", error))?;
    let existed: bool = txn
        .query_one(
            SESSION_EXISTS_SQL,
            &[&tenant_id, &DEFAULT_ORGANIZATION_ID, &stream_id],
        )
        .map_err(|error| postgres_unavailable("stream state clear exists", error))?
        .get(0);
    txn.execute(
        DELETE_FRAMES_SQL,
        &[&tenant_id, &DEFAULT_ORGANIZATION_ID, &stream_id],
    )
    .map_err(|error| postgres_unavailable("stream state clear frames", error))?;
    txn.execute(
        DELETE_SESSION_SQL,
        &[&tenant_id, &DEFAULT_ORGANIZATION_ID, &stream_id],
    )
    .map_err(|error| postgres_unavailable("stream state clear session", error))?;
    txn.commit()
        .map_err(|error| postgres_unavailable("stream state clear commit", error))?;
    Ok(existed)
}

fn session_from_row(row: &postgres::Row) -> Result<StreamSession, ContractError> {
    let payload_text: String = row.get(20);
    let extras: StreamSessionPayloadExtras = serde_json::from_str(payload_text.as_str())
        .map_err(|error| {
            ContractError::Conflict(format!("stream session payload decode failed: {error}"))
        })?;
    let result_message_id = row
        .get::<_, Option<i64>>(13)
        .map(|value| value.to_string())
        .or(extras.result_message_id);

    Ok(StreamSession {
        tenant_id: row.get(0),
        stream_id: row.get(1),
        owner_principal_kind: row.get(2),
        owner_principal_id: row.get(3),
        stream_type: row.get(4),
        scope_kind: row.get(5),
        scope_id: row.get(6),
        durability_class: parse_durability_class(row.get::<_, String>(7).as_str())?,
        ordering_scope: row.get(8),
        schema_ref: row.get(9),
        state: parse_stream_session_state(row.get::<_, String>(10).as_str())?,
        last_frame_seq: row.get::<_, i64>(11) as u64,
        last_checkpoint_seq: optional_i64_as_u64(row.get(12))?,
        result_message_id,
        complete_frame_seq: optional_i64_as_u64(row.get(14))?,
        abort_frame_seq: optional_i64_as_u64(row.get(15))?,
        abort_reason: row.get(16),
        opened_at: format_timestamptz(row.get(17))?,
        closed_at: optional_format_timestamptz(row.get(18))?,
        expires_at: optional_format_timestamptz(row.get(19))?,
    })
}

fn frame_from_row(
    row: &postgres::Row,
    tenant_id: &str,
    stream_id: &str,
) -> Result<StreamFrame, ContractError> {
    let payload_text: String = row.get(4);
    let payload: StreamFramePayload = serde_json::from_str(payload_text.as_str()).map_err(|error| {
        ContractError::Conflict(format!("stream frame payload decode failed: {error}"))
    })?;
    let occurred_at = format_timestamptz(row.get(5))?;
    Ok(StreamFrame {
        tenant_id: tenant_id.to_owned(),
        stream_id: stream_id.to_owned(),
        stream_type: payload.stream_type,
        scope_kind: payload.scope_kind,
        scope_id: payload.scope_id,
        frame_seq: row.get::<_, i64>(0) as u64,
        frame_type: payload.frame_type,
        schema_ref: row.get(3),
        encoding: payload.encoding,
        payload: payload.payload,
        sender: payload.sender,
        attributes: payload.attributes,
        occurred_at,
    })
}

fn result_message_id_for_db(session: &StreamSession) -> (Option<i64>, StreamSessionPayloadExtras) {
    let Some(result_message_id) = session.result_message_id.as_deref() else {
        return (None, StreamSessionPayloadExtras { result_message_id: None });
    };
    if let Ok(parsed) = result_message_id.parse::<i64>() {
        return (
            Some(parsed),
            StreamSessionPayloadExtras {
                result_message_id: None,
            },
        );
    }
    (
        None,
        StreamSessionPayloadExtras {
            result_message_id: Some(result_message_id.to_owned()),
        },
    )
}

fn parse_durability_class(value: &str) -> Result<StreamDurabilityClass, ContractError> {
    match value {
        "transient" => Ok(StreamDurabilityClass::Transient),
        "durable_session" => Ok(StreamDurabilityClass::DurableSession),
        "event_log" => Ok(StreamDurabilityClass::EventLog),
        other => Err(ContractError::Conflict(format!(
            "unknown stream durability class: {other}"
        ))),
    }
}

fn parse_stream_session_state(value: &str) -> Result<StreamSessionState, ContractError> {
    match value {
        "created" => Ok(StreamSessionState::Created),
        "opened" => Ok(StreamSessionState::Opened),
        "active" => Ok(StreamSessionState::Active),
        "checkpointed" => Ok(StreamSessionState::Checkpointed),
        "completed" => Ok(StreamSessionState::Completed),
        "aborted" => Ok(StreamSessionState::Aborted),
        "expired" => Ok(StreamSessionState::Expired),
        other => Err(ContractError::Conflict(format!(
            "unknown stream session state: {other}"
        ))),
    }
}

fn optional_timestamptz(value: Option<&str>) -> Result<Option<chrono::DateTime<chrono::Utc>>, ContractError> {
    value
        .map(|instant| postgres_timestamptz(instant, "timestamp"))
        .transpose()
}

fn optional_u64_as_i64(value: Option<u64>) -> Result<Option<i64>, ContractError> {
    value
        .map(|seq| {
            i64::try_from(seq).map_err(|_| {
                ContractError::Conflict("stream sequence exceeds i64 range".into())
            })
        })
        .transpose()
}

fn optional_i64_as_u64(value: Option<i64>) -> Result<Option<u64>, ContractError> {
    value
        .map(|seq| {
            u64::try_from(seq).map_err(|_| {
                ContractError::Conflict("stream sequence is negative".into())
            })
        })
        .transpose()
}

fn format_timestamptz(value: chrono::DateTime<chrono::Utc>) -> Result<String, ContractError> {
    Ok(value.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
}

fn optional_format_timestamptz(
    value: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Option<String>, ContractError> {
    value.map(format_timestamptz).transpose()
}
