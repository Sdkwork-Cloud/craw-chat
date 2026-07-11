//! Atomic journal + message truth + optional outbox enqueue in one Postgres transaction.

use chrono::{DateTime, Utc};
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitPosition, ContractError, OutboxEventRecord, StoredMessageRecord,
};
use r2d2_postgres::postgres::Transaction;

use crate::{
    PostgresJournalPool, compose_partition_key, journal_aggregate_seq, journal_position_conflict,
    journal_retention_until, postgres_bigint_input, postgres_bigint_output, postgres_jsonb_payload,
    postgres_pool_client, postgres_row_get, postgres_timestamptz, postgres_unavailable_db,
    resolve_journal_event_id_replay, run_postgres_io,
};

const INSERT_MESSAGE_SQL: &str = r#"
insert into im_conversation_messages (
    tenant_id, organization_id, conversation_id, message_id, message_seq,
    sender_principal_kind, sender_principal_id, sender_device_id, client_msg_id,
    message_type, payload_json, payload_hash, created_at, updated_at, retention_until
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::jsonb, $12, $13, $14, $15)
"#;

const ENQUEUE_OUTBOX_SQL: &str = r#"
insert into im_outbox_events (
    tenant_id, organization_id, outbox_id, aggregate_type, aggregate_id,
    event_id, event_type, payload_json, payload_hash, publish_status,
    attempt_count, available_at, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10, $11, $12, $13, $14)
"#;

const LOAD_REPLAY_MESSAGE_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    conversation_id,
    message_id,
    message_seq,
    sender_principal_kind,
    sender_principal_id,
    sender_device_id,
    client_msg_id,
    message_type,
    payload_json,
    payload_hash,
    created_at
from im_conversation_messages
where tenant_id = $1
  and organization_id = $2
  and conversation_id = $3
  and message_id = $4
"#;

const LOAD_REPLAY_OUTBOX_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    outbox_id,
    aggregate_type,
    aggregate_id,
    event_id,
    event_type,
    payload_json,
    payload_hash,
    created_at
from im_outbox_events
where tenant_id = $1
  and organization_id = $2
  and outbox_id = $3
"#;

const MESSAGE_POST_REPLAY_CONFLICT_MESSAGE: &str =
    "message post replay conflicts with existing durable state";

enum JournalAppendOutcome {
    Inserted(String, i64),
    EventIdAbsorbed(String, i64),
}

#[derive(Debug, PartialEq, Eq)]
struct MessageCreationFingerprint {
    tenant_id: String,
    organization_id: String,
    conversation_id: String,
    message_id: i64,
    message_seq: i64,
    sender_principal_kind: String,
    sender_principal_id: String,
    sender_device_id: Option<String>,
    client_msg_id: Option<String>,
    message_type: String,
    payload_json: serde_json::Value,
    payload_hash: String,
    created_at_micros: i64,
}

impl MessageCreationFingerprint {
    fn from_record(message: &StoredMessageRecord) -> Result<Self, ContractError> {
        let message_seq = postgres_bigint_input(message.message_seq, "message sequence")
            .map_err(|_| message_post_replay_conflict())?;
        let payload_json = postgres_jsonb_payload(message.payload_json.as_str())
            .map_err(|_| message_post_replay_conflict())?;
        let created_at = postgres_timestamptz(message.created_at.as_str(), "created_at")
            .map_err(|_| message_post_replay_conflict())?;
        Ok(Self {
            tenant_id: message.tenant_id.clone(),
            organization_id: message.organization_id.clone(),
            conversation_id: message.conversation_id.clone(),
            message_id: message.message_id,
            message_seq,
            sender_principal_kind: message.sender_principal_kind.clone(),
            sender_principal_id: message.sender_principal_id.clone(),
            sender_device_id: message.sender_device_id.clone(),
            client_msg_id: message.client_msg_id.clone(),
            message_type: message.message_type.clone(),
            payload_json,
            payload_hash: message.payload_hash.clone(),
            created_at_micros: created_at.timestamp_micros(),
        })
    }

    fn from_row(row: &postgres::Row) -> Result<Self, ContractError> {
        Ok(Self {
            tenant_id: replay_message_row_get(row, 0, "tenant_id")?,
            organization_id: replay_message_row_get(row, 1, "organization_id")?,
            conversation_id: replay_message_row_get(row, 2, "conversation_id")?,
            message_id: replay_message_row_get(row, 3, "message_id")?,
            message_seq: replay_message_row_get(row, 4, "message_seq")?,
            sender_principal_kind: replay_message_row_get(row, 5, "sender_principal_kind")?,
            sender_principal_id: replay_message_row_get(row, 6, "sender_principal_id")?,
            sender_device_id: replay_message_row_get(row, 7, "sender_device_id")?,
            client_msg_id: replay_message_row_get(row, 8, "client_msg_id")?,
            message_type: replay_message_row_get(row, 9, "message_type")?,
            payload_json: replay_message_row_get(row, 10, "payload_json")?,
            payload_hash: replay_message_row_get(row, 11, "payload_hash")?,
            created_at_micros: replay_message_row_get::<DateTime<Utc>>(row, 12, "created_at")?
                .timestamp_micros(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OutboxCreationFingerprint {
    tenant_id: String,
    organization_id: String,
    outbox_id: String,
    aggregate_type: String,
    aggregate_id: String,
    event_id: String,
    event_type: String,
    payload_json: serde_json::Value,
    payload_hash: String,
    created_at_micros: i64,
}

impl OutboxCreationFingerprint {
    fn from_record(event: &OutboxEventRecord) -> Result<Self, ContractError> {
        let payload_json = postgres_jsonb_payload(event.payload_json.as_str())
            .map_err(|_| message_post_replay_conflict())?;
        let created_at = postgres_timestamptz(event.created_at.as_str(), "created_at")
            .map_err(|_| message_post_replay_conflict())?;
        Ok(Self {
            tenant_id: event.tenant_id.clone(),
            organization_id: event.organization_id.clone(),
            outbox_id: event.outbox_id.clone(),
            aggregate_type: event.aggregate_type.clone(),
            aggregate_id: event.aggregate_id.clone(),
            event_id: event.event_id.clone(),
            event_type: event.event_type.clone(),
            payload_json,
            payload_hash: event.payload_hash.clone(),
            created_at_micros: created_at.timestamp_micros(),
        })
    }

    fn from_row(row: &postgres::Row) -> Result<Self, ContractError> {
        Ok(Self {
            tenant_id: replay_outbox_row_get(row, 0, "tenant_id")?,
            organization_id: replay_outbox_row_get(row, 1, "organization_id")?,
            outbox_id: replay_outbox_row_get(row, 2, "outbox_id")?,
            aggregate_type: replay_outbox_row_get(row, 3, "aggregate_type")?,
            aggregate_id: replay_outbox_row_get(row, 4, "aggregate_id")?,
            event_id: replay_outbox_row_get(row, 5, "event_id")?,
            event_type: replay_outbox_row_get(row, 6, "event_type")?,
            payload_json: replay_outbox_row_get(row, 7, "payload_json")?,
            payload_hash: replay_outbox_row_get(row, 8, "payload_hash")?,
            created_at_micros: replay_outbox_row_get::<DateTime<Utc>>(row, 9, "created_at")?
                .timestamp_micros(),
        })
    }
}

/// Postgres-backed atomic message post writer (journal + message + outbox).
#[derive(Clone)]
pub struct PostgresDurableMessagePostWriter {
    pool: PostgresJournalPool,
    partition_prefix: std::sync::Arc<str>,
}

impl PostgresDurableMessagePostWriter {
    pub fn new(pool: PostgresJournalPool, partition_prefix: std::sync::Arc<str>) -> Self {
        Self {
            pool,
            partition_prefix,
        }
    }

    pub fn from_journal(journal: &crate::PostgresCommitJournal) -> Self {
        Self::new(journal.pool().clone(), journal.partition_prefix().clone())
    }

    pub fn persist_message_post(
        &self,
        envelope: CommitEnvelope,
        message: StoredMessageRecord,
        outbox: Option<OutboxEventRecord>,
    ) -> Result<CommitPosition, ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || {
            persist_message_post_txn(&pool, prefix.as_ref(), &envelope, &message, outbox.as_ref())
        })
    }
}

fn persist_message_post_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelope: &CommitEnvelope,
    message: &StoredMessageRecord,
    outbox: Option<&OutboxEventRecord>,
) -> Result<CommitPosition, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_message_post")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("persist_message_post begin", error))?;

    match append_journal_in_transaction(&mut txn, prefix, envelope)? {
        JournalAppendOutcome::EventIdAbsorbed(partition, offset) => {
            ensure_message_post_replay_matches(&mut txn, message, outbox)?;
            let offset = postgres_bigint_output(offset, "commit_offset")?;
            txn.commit()
                .map_err(|error| postgres_unavailable_db("persist_message_post commit", error))?;
            Ok(CommitPosition::new(partition, offset))
        }
        JournalAppendOutcome::Inserted(partition, offset) => {
            insert_message_in_transaction(&mut txn, message)?;
            if let Some(outbox) = outbox {
                enqueue_outbox_in_transaction(&mut txn, outbox)?;
            }
            let offset = postgres_bigint_output(offset, "commit_offset")?;
            txn.commit()
                .map_err(|error| postgres_unavailable_db("persist_message_post commit", error))?;
            Ok(CommitPosition::new(partition, offset))
        }
    }
}

fn ensure_message_post_replay_matches(
    txn: &mut Transaction<'_>,
    message: &StoredMessageRecord,
    outbox: Option<&OutboxEventRecord>,
) -> Result<(), ContractError> {
    let attempted_message = MessageCreationFingerprint::from_record(message)?;
    let message_row = txn
        .query_opt(
            LOAD_REPLAY_MESSAGE_SQL,
            &[
                &message.tenant_id,
                &message.organization_id,
                &message.conversation_id,
                &message.message_id,
            ],
        )
        .map_err(|error| postgres_unavailable_db("message post replay message lookup", error))?
        .ok_or_else(message_post_replay_conflict)?;
    let existing_message = MessageCreationFingerprint::from_row(&message_row)?;
    if existing_message != attempted_message {
        return Err(message_post_replay_conflict());
    }

    if let Some(outbox) = outbox {
        let attempted_outbox = OutboxCreationFingerprint::from_record(outbox)?;
        let outbox_row = txn
            .query_opt(
                LOAD_REPLAY_OUTBOX_SQL,
                &[
                    &outbox.tenant_id,
                    &outbox.organization_id,
                    &outbox.outbox_id,
                ],
            )
            .map_err(|error| postgres_unavailable_db("message post replay outbox lookup", error))?
            .ok_or_else(message_post_replay_conflict)?;
        let existing_outbox = OutboxCreationFingerprint::from_row(&outbox_row)?;
        if existing_outbox != attempted_outbox {
            return Err(message_post_replay_conflict());
        }
    }

    Ok(())
}

fn replay_message_row_get<T>(
    row: &postgres::Row,
    column: usize,
    field: &'static str,
) -> Result<T, ContractError>
where
    T: for<'a> postgres::types::FromSql<'a>,
{
    postgres_row_get(row, column, "message post replay message", field)
}

fn replay_outbox_row_get<T>(
    row: &postgres::Row,
    column: usize,
    field: &'static str,
) -> Result<T, ContractError>
where
    T: for<'a> postgres::types::FromSql<'a>,
{
    postgres_row_get(row, column, "message post replay outbox", field)
}

fn message_post_replay_conflict() -> ContractError {
    ContractError::Conflict(MESSAGE_POST_REPLAY_CONFLICT_MESSAGE.into())
}

fn append_journal_in_transaction(
    txn: &mut Transaction<'_>,
    prefix: &str,
    envelope: &CommitEnvelope,
) -> Result<JournalAppendOutcome, ContractError> {
    use crate::{APPEND_EVENT_SQL, LOAD_EVENT_BY_POSITION_SQL, is_unique_violation};
    use sdkwork_utils_rust::sha256_hash;

    let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
    let payload_json = postgres_jsonb_payload(envelope.payload.as_str())?;
    let payload_hash = sha256_hash(envelope.payload.as_bytes());
    let created_at = Utc::now();
    let aggregate_seq = journal_aggregate_seq(envelope.ordering_seq)?;
    let commit_offset = aggregate_seq;
    let organization_id = envelope.normalized_organization_id();
    let occurred_at = postgres_timestamptz(envelope.occurred_at.as_str(), "occurred_at")?;
    let retention_until = journal_retention_until(envelope)
        .as_deref()
        .map(|value| postgres_timestamptz(value, "retention_until"))
        .transpose()?;

    let outcome = {
        let mut savepoint = txn
            .savepoint("im_message_post_journal_append")
            .map_err(|error| postgres_unavailable_db("message post journal savepoint", error))?;
        let result = savepoint.query_opt(
            APPEND_EVENT_SQL,
            &[
                &partition_key,
                &commit_offset,
                &envelope.event_id,
                &envelope.tenant_id,
                &organization_id,
                &envelope.aggregate_type.as_wire_value(),
                &envelope.aggregate_id,
                &aggregate_seq,
                &envelope.event_type,
                &payload_json,
                &payload_hash,
                &envelope.idempotency_key,
                &occurred_at,
                &created_at,
                &retention_until,
            ],
        );
        match result {
            Ok(row) => {
                savepoint.commit().map_err(|error| {
                    postgres_unavailable_db("message post journal savepoint commit", error)
                })?;
                match row {
                    Some(row) => {
                        let partition: String =
                            postgres_row_get(&row, 0, "message post append", "partition_key")?;
                        let offset: i64 =
                            postgres_row_get(&row, 1, "message post append", "commit_offset")?;
                        JournalAppendOutcome::Inserted(partition, offset)
                    }
                    None => {
                        let (partition, offset) = resolve_journal_event_id_replay(
                            txn,
                            prefix,
                            envelope,
                            "message post journal replay lookup",
                        )?;
                        JournalAppendOutcome::EventIdAbsorbed(partition, offset)
                    }
                }
            }
            Err(error) if is_unique_violation(&error) => {
                savepoint.rollback().map_err(|error| {
                    postgres_unavailable_db("message post journal rollback", error)
                })?;
                let row = txn
                    .query_one(
                        LOAD_EVENT_BY_POSITION_SQL,
                        &[&partition_key, &commit_offset],
                    )
                    .map_err(|error| {
                        postgres_unavailable_db("message post journal position lookup", error)
                    })?;
                let existing_event_id: String =
                    postgres_row_get(&row, 0, "message post position lookup", "event_id")?;
                if existing_event_id == envelope.event_id {
                    let (partition, offset) = resolve_journal_event_id_replay(
                        txn,
                        prefix,
                        envelope,
                        "message post journal defensive replay lookup",
                    )?;
                    JournalAppendOutcome::EventIdAbsorbed(partition, offset)
                } else {
                    return Err(journal_position_conflict());
                }
            }
            Err(error) => {
                return Err(postgres_unavailable_db(
                    "message post journal insert",
                    error,
                ));
            }
        }
    };

    Ok(outcome)
}

fn insert_message_in_transaction(
    txn: &mut Transaction<'_>,
    message: &StoredMessageRecord,
) -> Result<(), ContractError> {
    use crate::is_unique_violation;

    let message_seq_i64 = postgres_bigint_input(message.message_seq, "message sequence")?;
    let payload_json = postgres_jsonb_payload(message.payload_json.as_str())?;
    let created_at = postgres_timestamptz(message.created_at.as_str(), "created_at")?;
    let updated_at = postgres_timestamptz(message.updated_at.as_str(), "updated_at")?;
    let retention_until = message
        .retention_until
        .as_deref()
        .map(|value| postgres_timestamptz(value, "retention_until"))
        .transpose()?;
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[
        &message.tenant_id,
        &message.organization_id,
        &message.conversation_id,
        &message.message_id,
        &message_seq_i64,
        &message.sender_principal_kind,
        &message.sender_principal_id,
        &message.sender_device_id,
        &message.client_msg_id,
        &message.message_type,
        &payload_json,
        &message.payload_hash,
        &created_at,
        &updated_at,
        &retention_until,
    ];
    match txn.execute(INSERT_MESSAGE_SQL, params) {
        Ok(_) => Ok(()),
        Err(error) if is_unique_violation(&error) => Err(ContractError::Conflict(
            "message already exists for client_msg_id".into(),
        )),
        Err(error) => Err(postgres_unavailable_db("message post insert", error)),
    }
}

fn enqueue_outbox_in_transaction(
    txn: &mut Transaction<'_>,
    event: &OutboxEventRecord,
) -> Result<(), ContractError> {
    use crate::is_unique_violation;

    let payload_json = postgres_jsonb_payload(event.payload_json.as_str())?;
    let attempt_count_i32 = i32::try_from(event.attempt_count).map_err(|_| {
        ContractError::Invalid(
            "message post outbox attempt count exceeds the PostgreSQL INTEGER range".into(),
        )
    })?;
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
    match txn.execute(ENQUEUE_OUTBOX_SQL, params) {
        Ok(_) => Ok(()),
        Err(error) if is_unique_violation(&error) => {
            Err(ContractError::Conflict("event already enqueued".into()))
        }
        Err(error) => Err(postgres_unavailable_db(
            "message post outbox enqueue",
            error,
        )),
    }
}
