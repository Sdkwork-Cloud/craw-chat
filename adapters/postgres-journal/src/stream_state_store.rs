//! Transactional PostgreSQL authority for stream sessions and frames.

use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
use im_platform_contracts::{
    ContractError, StreamAppendOutcome, StreamCreateOutcome, StreamScope, StreamSessionRecord,
    StreamStateStore, StreamTransitionOutcome,
};
use sdkwork_utils_rust::sha256_hash;
use serde::{Deserialize, Serialize};

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_jsonb_payload, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

const LOAD_SESSION_SQL: &str = r#"
select tenant_id, organization_id, stream_id, owner_principal_kind, owner_principal_id,
    stream_type, scope_kind, scope_id, durability_class, ordering_scope, schema_ref,
    stream_state, last_frame_seq, last_checkpoint_seq, result_message_id,
    complete_frame_seq, abort_frame_seq, abort_reason, opened_at, closed_at, expires_at,
    payload_json::text, version, updated_at
from im_stream_sessions
where tenant_id = $1 and organization_id = $2 and stream_id = $3
"#;

const LOAD_SESSION_FOR_UPDATE_SQL: &str = r#"
select tenant_id, organization_id, stream_id, owner_principal_kind, owner_principal_id,
    stream_type, scope_kind, scope_id, durability_class, ordering_scope, schema_ref,
    stream_state, last_frame_seq, last_checkpoint_seq, result_message_id,
    complete_frame_seq, abort_frame_seq, abort_reason, opened_at, closed_at, expires_at,
    payload_json::text, version, updated_at
from im_stream_sessions
where tenant_id = $1 and organization_id = $2 and stream_id = $3
for update
"#;

const LOAD_FRAME_SQL: &str = r#"
select frame_seq, producer_principal_kind, producer_principal_id, schema_ref,
    payload_json::text, occurred_at
from im_stream_frames
where tenant_id = $1 and organization_id = $2 and stream_id = $3 and frame_seq = $4
"#;

const LIST_FRAMES_SQL: &str = r#"
select frame_seq, producer_principal_kind, producer_principal_id, schema_ref,
    payload_json::text, occurred_at
from im_stream_frames
where tenant_id = $1 and organization_id = $2 and stream_id = $3 and frame_seq > $4
order by frame_seq asc
limit $5
"#;

const INSERT_SESSION_SQL: &str = r#"
insert into im_stream_sessions (
    tenant_id, organization_id, stream_id, owner_principal_kind, owner_principal_id,
    stream_type, scope_kind, scope_id, durability_class, ordering_scope, schema_ref,
    stream_state, last_frame_seq, last_checkpoint_seq, result_message_id,
    complete_frame_seq, abort_frame_seq, abort_reason, opened_at, closed_at, expires_at,
    payload_json, payload_hash, version, created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
    $19, $20, $21, $22::jsonb, $23, $24, $25, $26
)
on conflict (tenant_id, organization_id, stream_id) do nothing
"#;

const UPDATE_SESSION_SQL: &str = r#"
update im_stream_sessions set
    stream_state = $4, last_frame_seq = $5, last_checkpoint_seq = $6,
    result_message_id = $7, complete_frame_seq = $8, abort_frame_seq = $9,
    abort_reason = $10, closed_at = $11, expires_at = $12,
    payload_json = $13::jsonb, payload_hash = $14, version = $15, updated_at = $16
where tenant_id = $1 and organization_id = $2 and stream_id = $3 and version = $17
"#;

const INSERT_FRAME_SQL: &str = r#"
insert into im_stream_frames (
    tenant_id, organization_id, stream_id, frame_seq, producer_principal_kind,
    producer_principal_id, schema_ref, payload_json, payload_hash, occurred_at, created_at
) values ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10, $11)
"#;

const ACTIVE_COUNT_SQL: &str = r#"
select count(*) from im_stream_sessions
where tenant_id = $1 and organization_id = $2
  and stream_state not in ('completed', 'aborted', 'expired')
"#;

const DELETE_FRAMES_SQL: &str = r#"
delete from im_stream_frames where tenant_id = $1 and organization_id = $2 and stream_id = $3
"#;
const DELETE_SESSION_SQL: &str = r#"
delete from im_stream_sessions where tenant_id = $1 and organization_id = $2 and stream_id = $3
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
    fn check_ready(&self) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "stream readiness")?;
            client
                .simple_query("select 1")
                .map_err(|error| postgres_unavailable("stream readiness", error))?;
            Ok(())
        })
    }

    fn load_session(
        &self,
        scope: &StreamScope,
    ) -> Result<Option<StreamSessionRecord>, ContractError> {
        let pool = self.pool.clone();
        let scope = scope.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "stream session load")?;
            client
                .query_opt(
                    LOAD_SESSION_SQL,
                    &[&scope.tenant_id, &scope.organization_id, &scope.stream_id],
                )
                .map_err(|error| postgres_unavailable("stream session load", error))?
                .map(session_record_from_row)
                .transpose()
        })
    }

    fn create_session(
        &self,
        record: StreamSessionRecord,
        max_active_streams: u64,
    ) -> Result<StreamCreateOutcome, ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || create_session(&pool, record, max_active_streams))
    }

    fn append_frame(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
        frame: StreamFrame,
    ) -> Result<StreamAppendOutcome, ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || append_frame(&pool, expected_version, next_session, frame))
    }

    fn transition_session(
        &self,
        expected_version: u64,
        next_session: StreamSessionRecord,
    ) -> Result<StreamTransitionOutcome, ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || transition_session(&pool, expected_version, next_session))
    }

    fn list_frames_after(
        &self,
        scope: &StreamScope,
        after_frame_seq: u64,
        page_size: usize,
    ) -> Result<Vec<StreamFrame>, ContractError> {
        let pool = self.pool.clone();
        let scope = scope.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "stream frame page")?;
            let after = u64_as_i64(after_frame_seq, "after_frame_seq")?;
            let limit = i64::try_from(page_size)
                .map_err(|_| ContractError::Invalid("stream page_size exceeds i64".into()))?;
            client
                .query(
                    LIST_FRAMES_SQL,
                    &[
                        &scope.tenant_id,
                        &scope.organization_id,
                        &scope.stream_id,
                        &after,
                        &limit,
                    ],
                )
                .map_err(|error| postgres_unavailable("stream frame page", error))?
                .iter()
                .map(|row| frame_from_row(row, &scope))
                .collect()
        })
    }

    fn clear_stream(&self, scope: &StreamScope) -> Result<bool, ContractError> {
        let pool = self.pool.clone();
        let scope = scope.clone();
        run_postgres_io(move || clear_stream(&pool, &scope))
    }
}

fn create_session(
    pool: &PostgresJournalPool,
    record: StreamSessionRecord,
    max_active_streams: u64,
) -> Result<StreamCreateOutcome, ContractError> {
    let mut client = postgres_pool_client(pool, "stream session create")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("stream session create begin", error))?;
    txn.execute(
        "select pg_advisory_xact_lock(hashtextextended($1 || ':' || $2, 0))",
        &[&record.scope.tenant_id, &record.scope.organization_id],
    )
    .map_err(|error| postgres_unavailable("stream capacity lock", error))?;
    if let Some(row) = txn
        .query_opt(
            LOAD_SESSION_FOR_UPDATE_SQL,
            &[
                &record.scope.tenant_id,
                &record.scope.organization_id,
                &record.scope.stream_id,
            ],
        )
        .map_err(|error| postgres_unavailable("stream session existing load", error))?
    {
        return Ok(StreamCreateOutcome::Existing(session_record_from_row(row)?));
    }
    let active: i64 = txn
        .query_one(
            ACTIVE_COUNT_SQL,
            &[&record.scope.tenant_id, &record.scope.organization_id],
        )
        .map_err(|error| postgres_unavailable("stream active count", error))?
        .get(0);
    if u64::try_from(active).unwrap_or(u64::MAX) >= max_active_streams {
        return Ok(StreamCreateOutcome::CapacityExceeded);
    }
    let affected = insert_session(&mut txn, &record)?;
    if affected != 1 {
        return Err(ContractError::Conflict(
            "stream session create lost an unexpected uniqueness race".into(),
        ));
    }
    txn.commit()
        .map_err(|error| postgres_unavailable("stream session create commit", error))?;
    Ok(StreamCreateOutcome::Applied(record))
}

fn append_frame(
    pool: &PostgresJournalPool,
    expected_version: u64,
    next_session: StreamSessionRecord,
    frame: StreamFrame,
) -> Result<StreamAppendOutcome, ContractError> {
    let mut client = postgres_pool_client(pool, "stream frame append")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("stream frame append begin", error))?;
    let scope = &next_session.scope;
    let Some(row) = txn
        .query_opt(
            LOAD_SESSION_FOR_UPDATE_SQL,
            &[&scope.tenant_id, &scope.organization_id, &scope.stream_id],
        )
        .map_err(|error| postgres_unavailable("stream frame session lock", error))?
    else {
        return Err(ContractError::Invalid(
            "stream session does not exist".into(),
        ));
    };
    let current = session_record_from_row(row)?;
    let frame_seq = u64_as_i64(frame.frame_seq, "frame_seq")?;
    if let Some(row) = txn
        .query_opt(
            LOAD_FRAME_SQL,
            &[
                &scope.tenant_id,
                &scope.organization_id,
                &scope.stream_id,
                &frame_seq,
            ],
        )
        .map_err(|error| postgres_unavailable("stream existing frame load", error))?
    {
        return Ok(StreamAppendOutcome::Existing {
            session: current,
            frame: frame_from_row(&row, scope)?,
        });
    }
    if current.version != expected_version {
        return Ok(StreamAppendOutcome::VersionConflict);
    }
    insert_frame(&mut txn, scope, &frame)?;
    if update_session(&mut txn, expected_version, &next_session)? != 1 {
        return Ok(StreamAppendOutcome::VersionConflict);
    }
    txn.commit()
        .map_err(|error| postgres_unavailable("stream frame append commit", error))?;
    Ok(StreamAppendOutcome::Applied {
        session: next_session,
        frame,
    })
}

fn transition_session(
    pool: &PostgresJournalPool,
    expected_version: u64,
    next_session: StreamSessionRecord,
) -> Result<StreamTransitionOutcome, ContractError> {
    let mut client = postgres_pool_client(pool, "stream session transition")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("stream session transition begin", error))?;
    if update_session(&mut txn, expected_version, &next_session)? != 1 {
        return Ok(StreamTransitionOutcome::VersionConflict);
    }
    txn.commit()
        .map_err(|error| postgres_unavailable("stream session transition commit", error))?;
    Ok(StreamTransitionOutcome::Applied(next_session))
}

fn insert_session(
    txn: &mut postgres::Transaction<'_>,
    record: &StreamSessionRecord,
) -> Result<u64, ContractError> {
    let session = &record.session;
    let (result_message_id, payload_json, payload_hash) = session_payload(session)?;
    let opened_at = postgres_timestamptz(session.opened_at.as_str(), "opened_at")?;
    let closed_at = optional_timestamptz(session.closed_at.as_deref())?;
    let expires_at = optional_timestamptz(session.expires_at.as_deref())?;
    let created_at = postgres_timestamptz(now_rfc3339().as_str(), "created_at")?;
    let updated_at = postgres_timestamptz(record.updated_at.as_str(), "updated_at")?;
    let version = u64_as_i64(record.version, "version")?;
    let last_frame_seq = u64_as_i64(session.last_frame_seq, "last_frame_seq")?;
    let last_checkpoint_seq = optional_u64_as_i64(session.last_checkpoint_seq)?;
    let complete_frame_seq = optional_u64_as_i64(session.complete_frame_seq)?;
    let abort_frame_seq = optional_u64_as_i64(session.abort_frame_seq)?;
    txn.execute(
        INSERT_SESSION_SQL,
        &[
            &record.scope.tenant_id,
            &record.scope.organization_id,
            &record.scope.stream_id,
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
            &version,
            &created_at,
            &updated_at,
        ],
    )
    .map_err(|error| postgres_unavailable("stream session insert", error))
}

fn update_session(
    txn: &mut postgres::Transaction<'_>,
    expected_version: u64,
    record: &StreamSessionRecord,
) -> Result<u64, ContractError> {
    let session = &record.session;
    let (result_message_id, payload_json, payload_hash) = session_payload(session)?;
    let last_frame_seq = u64_as_i64(session.last_frame_seq, "last_frame_seq")?;
    let last_checkpoint_seq = optional_u64_as_i64(session.last_checkpoint_seq)?;
    let complete_frame_seq = optional_u64_as_i64(session.complete_frame_seq)?;
    let abort_frame_seq = optional_u64_as_i64(session.abort_frame_seq)?;
    let closed_at = optional_timestamptz(session.closed_at.as_deref())?;
    let expires_at = optional_timestamptz(session.expires_at.as_deref())?;
    let version = u64_as_i64(record.version, "version")?;
    let expected_version = u64_as_i64(expected_version, "expected_version")?;
    let updated_at = postgres_timestamptz(record.updated_at.as_str(), "updated_at")?;
    txn.execute(
        UPDATE_SESSION_SQL,
        &[
            &record.scope.tenant_id,
            &record.scope.organization_id,
            &record.scope.stream_id,
            &session.state.as_wire_value(),
            &last_frame_seq,
            &last_checkpoint_seq,
            &result_message_id,
            &complete_frame_seq,
            &abort_frame_seq,
            &session.abort_reason,
            &closed_at,
            &expires_at,
            &payload_json,
            &payload_hash,
            &version,
            &updated_at,
            &expected_version,
        ],
    )
    .map_err(|error| postgres_unavailable("stream session update", error))
}

fn insert_frame(
    txn: &mut postgres::Transaction<'_>,
    scope: &StreamScope,
    frame: &StreamFrame,
) -> Result<(), ContractError> {
    let payload = StreamFramePayload {
        stream_type: frame.stream_type.clone(),
        scope_kind: frame.scope_kind.clone(),
        scope_id: frame.scope_id.clone(),
        frame_type: frame.frame_type.clone(),
        encoding: frame.encoding.clone(),
        payload: frame.payload.clone(),
        sender: frame.sender.clone(),
        attributes: frame.attributes.clone(),
    };
    let payload_json =
        postgres_jsonb_payload(&serde_json::to_string(&payload).map_err(|error| {
            ContractError::Conflict(format!("stream frame encode failed: {error}"))
        })?)?;
    let payload_hash = sha256_hash(payload_json.to_string().as_bytes());
    let frame_seq = u64_as_i64(frame.frame_seq, "frame_seq")?;
    let occurred_at = postgres_timestamptz(frame.occurred_at.as_str(), "occurred_at")?;
    let created_at = postgres_timestamptz(now_rfc3339().as_str(), "created_at")?;
    txn.execute(
        INSERT_FRAME_SQL,
        &[
            &scope.tenant_id,
            &scope.organization_id,
            &scope.stream_id,
            &frame_seq,
            &frame.sender.kind,
            &frame.sender.id,
            &frame.schema_ref,
            &payload_json,
            &payload_hash,
            &occurred_at,
            &created_at,
        ],
    )
    .map_err(|error| postgres_unavailable("stream frame insert", error))?;
    Ok(())
}

fn session_record_from_row(row: postgres::Row) -> Result<StreamSessionRecord, ContractError> {
    let payload_text: String = row.get(21);
    let extras: StreamSessionPayloadExtras =
        serde_json::from_str(payload_text.as_str()).map_err(|error| {
            ContractError::Conflict(format!("stream session decode failed: {error}"))
        })?;
    let result_message_id = row
        .get::<_, Option<i64>>(14)
        .map(|value| value.to_string())
        .or(extras.result_message_id);
    let scope = StreamScope::new(
        row.get::<_, String>(0),
        row.get::<_, String>(1),
        row.get::<_, String>(2),
    );
    let session = StreamSession {
        tenant_id: scope.tenant_id.clone(),
        stream_id: scope.stream_id.clone(),
        owner_principal_kind: row.get(3),
        owner_principal_id: row.get(4),
        stream_type: row.get(5),
        scope_kind: row.get(6),
        scope_id: row.get(7),
        durability_class: parse_durability_class(row.get::<_, String>(8).as_str())?,
        ordering_scope: row.get(9),
        schema_ref: row.get(10),
        state: parse_stream_session_state(row.get::<_, String>(11).as_str())?,
        last_frame_seq: i64_as_u64(row.get(12), "last_frame_seq")?,
        last_checkpoint_seq: optional_i64_as_u64(row.get(13))?,
        result_message_id,
        complete_frame_seq: optional_i64_as_u64(row.get(15))?,
        abort_frame_seq: optional_i64_as_u64(row.get(16))?,
        abort_reason: row.get(17),
        opened_at: format_timestamptz(row.get(18)),
        closed_at: optional_format_timestamptz(row.get(19)),
        expires_at: optional_format_timestamptz(row.get(20)),
    };
    Ok(StreamSessionRecord {
        scope,
        session,
        version: i64_as_u64(row.get(22), "version")?,
        updated_at: format_timestamptz(row.get(23)),
    })
}

fn frame_from_row(row: &postgres::Row, scope: &StreamScope) -> Result<StreamFrame, ContractError> {
    let payload_text: String = row.get(4);
    let payload: StreamFramePayload = serde_json::from_str(payload_text.as_str())
        .map_err(|error| ContractError::Conflict(format!("stream frame decode failed: {error}")))?;
    Ok(StreamFrame {
        tenant_id: scope.tenant_id.clone(),
        stream_id: scope.stream_id.clone(),
        stream_type: payload.stream_type,
        scope_kind: payload.scope_kind,
        scope_id: payload.scope_id,
        frame_seq: i64_as_u64(row.get(0), "frame_seq")?,
        frame_type: payload.frame_type,
        schema_ref: row.get(3),
        encoding: payload.encoding,
        payload: payload.payload,
        sender: payload.sender,
        attributes: payload.attributes,
        occurred_at: format_timestamptz(row.get(5)),
    })
}

fn clear_stream(pool: &PostgresJournalPool, scope: &StreamScope) -> Result<bool, ContractError> {
    let mut client = postgres_pool_client(pool, "stream clear")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("stream clear begin", error))?;
    txn.execute(
        DELETE_FRAMES_SQL,
        &[&scope.tenant_id, &scope.organization_id, &scope.stream_id],
    )
    .map_err(|error| postgres_unavailable("stream frames clear", error))?;
    let deleted = txn
        .execute(
            DELETE_SESSION_SQL,
            &[&scope.tenant_id, &scope.organization_id, &scope.stream_id],
        )
        .map_err(|error| postgres_unavailable("stream session clear", error))?;
    txn.commit()
        .map_err(|error| postgres_unavailable("stream clear commit", error))?;
    Ok(deleted == 1)
}

fn session_payload(
    session: &StreamSession,
) -> Result<(Option<i64>, serde_json::Value, String), ContractError> {
    let (result_message_id, extras) = if let Some(value) = session.result_message_id.as_deref() {
        if let Ok(parsed) = value.parse::<i64>() {
            (
                Some(parsed),
                StreamSessionPayloadExtras {
                    result_message_id: None,
                },
            )
        } else {
            (
                None,
                StreamSessionPayloadExtras {
                    result_message_id: Some(value.to_owned()),
                },
            )
        }
    } else {
        (
            None,
            StreamSessionPayloadExtras {
                result_message_id: None,
            },
        )
    };
    let payload_json =
        postgres_jsonb_payload(&serde_json::to_string(&extras).map_err(|error| {
            ContractError::Conflict(format!("stream session encode failed: {error}"))
        })?)?;
    let payload_hash = sha256_hash(payload_json.to_string().as_bytes());
    Ok((result_message_id, payload_json, payload_hash))
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

fn u64_as_i64(value: u64, field: &str) -> Result<i64, ContractError> {
    i64::try_from(value).map_err(|_| ContractError::Invalid(format!("{field} exceeds i64")))
}
fn i64_as_u64(value: i64, field: &str) -> Result<u64, ContractError> {
    u64::try_from(value).map_err(|_| ContractError::Conflict(format!("{field} is negative")))
}
fn optional_u64_as_i64(value: Option<u64>) -> Result<Option<i64>, ContractError> {
    value
        .map(|value| u64_as_i64(value, "stream sequence"))
        .transpose()
}
fn optional_i64_as_u64(value: Option<i64>) -> Result<Option<u64>, ContractError> {
    value
        .map(|value| i64_as_u64(value, "stream sequence"))
        .transpose()
}
fn optional_timestamptz(
    value: Option<&str>,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, ContractError> {
    value
        .map(|value| postgres_timestamptz(value, "timestamp"))
        .transpose()
}
fn format_timestamptz(value: chrono::DateTime<chrono::Utc>) -> String {
    value.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
fn optional_format_timestamptz(value: Option<chrono::DateTime<chrono::Utc>>) -> Option<String> {
    value.map(format_timestamptz)
}
