//! Atomic journal + message truth + optional outbox enqueue in one Postgres transaction.

use chrono::{DateTime, Utc};
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{
    AGENT_MENTION_DISPATCH_EVENT_TYPE, CommitPosition, ContractError, OutboxEventRecord,
    OutboxPublishStatus, StoredMessageRecord,
};
use r2d2_postgres::postgres::Transaction;
use std::collections::HashSet;

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
on conflict do nothing
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

const LOAD_CONVERSATION_OUTBOX_BY_IDENTITY_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    outbox_id,
    aggregate_type,
    aggregate_id,
    event_id,
    event_type,
    payload_json,
    payload_hash
from im_outbox_events
where tenant_id = $1
  and organization_id = $2
  and (outbox_id = $3 or event_id = $4)
order by outbox_id
for update
"#;

const LOAD_REPLAY_OUTBOX_COUNT_SQL: &str = r#"
select count(*)
from im_outbox_events
where tenant_id = $1
  and organization_id = $2
  and aggregate_id = $3
  and event_type = $4
  and payload_json ->> 'messageId' = $5
"#;

const MESSAGE_POST_REPLAY_CONFLICT_MESSAGE: &str =
    "message post replay conflicts with existing durable state";
const CONVERSATION_EVENT_REPLAY_CONFLICT_MESSAGE: &str =
    "conversation event replay conflicts with existing durable outbox state";
const CONVERSATION_SCOPE_TYPE: &str = "conversation";
const CONVERSATION_OUTBOX_AGGREGATE_TYPE: &str = "conversation";

enum JournalAppendOutcome {
    Inserted(String, i64),
    EventIdAbsorbed(String, i64),
}

impl JournalAppendOutcome {
    fn into_commit_position(self) -> Result<CommitPosition, ContractError> {
        let (partition, offset) = match self {
            Self::Inserted(partition, offset) | Self::EventIdAbsorbed(partition, offset) => {
                (partition, offset)
            }
        };
        Ok(CommitPosition::new(
            partition,
            postgres_bigint_output(offset, "commit_offset")?,
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutboxEnqueueOutcome {
    Inserted,
    IdentityConflict,
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

#[derive(Debug, PartialEq, Eq)]
struct ConversationOutboxFingerprint {
    tenant_id: String,
    organization_id: String,
    outbox_id: String,
    aggregate_type: String,
    aggregate_id: String,
    event_id: String,
    event_type: String,
    payload_json: serde_json::Value,
    payload_hash: String,
}

impl ConversationOutboxFingerprint {
    fn from_record(event: &OutboxEventRecord) -> Result<Self, ContractError> {
        Ok(Self {
            tenant_id: event.tenant_id.clone(),
            organization_id: event.organization_id.clone(),
            outbox_id: event.outbox_id.clone(),
            aggregate_type: event.aggregate_type.clone(),
            aggregate_id: event.aggregate_id.clone(),
            event_id: event.event_id.clone(),
            event_type: event.event_type.clone(),
            payload_json: postgres_jsonb_payload(event.payload_json.as_str())
                .map_err(|_| conversation_event_replay_conflict())?,
            payload_hash: event.payload_hash.clone(),
        })
    }

    fn from_row(row: &postgres::Row) -> Result<Self, ContractError> {
        Ok(Self {
            tenant_id: conversation_outbox_row_get(row, 0, "tenant_id")?,
            organization_id: conversation_outbox_row_get(row, 1, "organization_id")?,
            outbox_id: conversation_outbox_row_get(row, 2, "outbox_id")?,
            aggregate_type: conversation_outbox_row_get(row, 3, "aggregate_type")?,
            aggregate_id: conversation_outbox_row_get(row, 4, "aggregate_id")?,
            event_id: conversation_outbox_row_get(row, 5, "event_id")?,
            event_type: conversation_outbox_row_get(row, 6, "event_type")?,
            payload_json: conversation_outbox_row_get(row, 7, "payload_json")?,
            payload_hash: conversation_outbox_row_get(row, 8, "payload_hash")?,
        })
    }
}

/// Postgres-backed atomic conversation event writer (journal + outbox).
///
/// Exact replays return the original journal position. If the journal row
/// exists but its deterministic outbox row is absent, the writer repairs the
/// outbox row in the same transaction. Existing outbox rows must retain the
/// same immutable identity and producer payload hash.
#[derive(Clone)]
pub struct PostgresDurableConversationEventWriter {
    pool: PostgresJournalPool,
    partition_prefix: std::sync::Arc<str>,
}

impl PostgresDurableConversationEventWriter {
    pub fn new(pool: PostgresJournalPool, partition_prefix: std::sync::Arc<str>) -> Self {
        Self {
            pool,
            partition_prefix,
        }
    }

    pub fn from_journal(journal: &crate::PostgresCommitJournal) -> Self {
        Self::new(journal.pool().clone(), journal.partition_prefix().clone())
    }

    pub fn persist_conversation_event(
        &self,
        envelope: CommitEnvelope,
        outbox: OutboxEventRecord,
    ) -> Result<CommitPosition, ContractError> {
        validate_conversation_event(&envelope, &outbox)?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || {
            persist_conversation_event_txn(&pool, prefix.as_ref(), &envelope, &outbox)
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
        let positions =
            self.persist_message_post_batch(vec![envelope], message, outbox.into_iter().collect())?;
        match positions.as_slice() {
            [position] => Ok(position.clone()),
            _ => Err(ContractError::Invalid(
                "durable message post writer returned an invalid journal position count".into(),
            )),
        }
    }

    pub fn persist_message_post_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        if envelopes.is_empty() {
            return Err(ContractError::Invalid(
                "durable message post requires at least one journal envelope".into(),
            ));
        }
        validate_message_post_batch(envelopes.as_slice(), &message, outboxes.as_slice())?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || {
            persist_message_post_txn(
                &pool,
                prefix.as_ref(),
                envelopes.as_slice(),
                &message,
                outboxes.as_slice(),
            )
        })
    }
}

fn validate_message_post_batch(
    envelopes: &[CommitEnvelope],
    message: &StoredMessageRecord,
    outboxes: &[OutboxEventRecord],
) -> Result<(), ContractError> {
    let organization_id =
        im_domain_events::normalize_commit_organization_id(message.organization_id.as_str());
    let mut journal_positions = HashSet::new();
    let mut journal_event_ids = HashSet::new();
    for (index, envelope) in envelopes.iter().enumerate() {
        if envelope.tenant_id != message.tenant_id
            || envelope.normalized_organization_id() != organization_id
            || envelope.aggregate_id != message.conversation_id
            || envelope.scope_id != message.conversation_id
            || envelope.event_id.trim().is_empty()
            || !journal_event_ids.insert(envelope.event_id.as_str())
            || !journal_positions.insert((envelope.ordering_key.clone(), envelope.ordering_seq))
            || (index == 0 && envelope.event_type != "message.posted")
        {
            return Err(ContractError::Invalid(
                "durable message post journal batch identity is invalid".into(),
            ));
        }
    }
    let mut outbox_ids = HashSet::new();
    let mut outbox_event_ids = HashSet::new();
    for outbox in outboxes {
        if outbox.tenant_id != message.tenant_id
            || im_domain_events::normalize_commit_organization_id(outbox.organization_id.as_str())
                != organization_id
            || outbox.aggregate_id != message.conversation_id
            || outbox.aggregate_type.trim().is_empty()
            || outbox.event_type.trim().is_empty()
            || outbox.outbox_id.trim().is_empty()
            || outbox.event_id.trim().is_empty()
            || !outbox_ids.insert(outbox.outbox_id.as_str())
            || !outbox_event_ids.insert(outbox.event_id.as_str())
        {
            return Err(ContractError::Invalid(
                "durable message post outbox batch identity is invalid".into(),
            ));
        }
    }
    Ok(())
}

fn persist_message_post_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelopes: &[CommitEnvelope],
    message: &StoredMessageRecord,
    outboxes: &[OutboxEventRecord],
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_message_post")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("persist_message_post begin", error))?;

    let outcomes = envelopes
        .iter()
        .map(|envelope| append_journal_in_transaction(&mut txn, prefix, envelope))
        .collect::<Result<Vec<_>, _>>()?;
    let inserted_count = outcomes
        .iter()
        .filter(|outcome| matches!(outcome, JournalAppendOutcome::Inserted(_, _)))
        .count();
    if inserted_count == outcomes.len() {
        insert_message_in_transaction(&mut txn, message)?;
        for outbox in outboxes {
            if enqueue_outbox_in_transaction(&mut txn, outbox)?
                == OutboxEnqueueOutcome::IdentityConflict
            {
                return Err(ContractError::Conflict("event already enqueued".into()));
            }
        }
    } else if inserted_count == 0 {
        ensure_message_post_replay_matches(&mut txn, message, outboxes)?;
    } else {
        return Err(message_post_replay_conflict());
    }

    let positions = outcomes
        .into_iter()
        .map(JournalAppendOutcome::into_commit_position)
        .collect::<Result<Vec<_>, ContractError>>()?;
    txn.commit()
        .map_err(|error| postgres_unavailable_db("persist_message_post commit", error))?;
    Ok(positions)
}

fn validate_conversation_event(
    envelope: &CommitEnvelope,
    outbox: &OutboxEventRecord,
) -> Result<(), ContractError> {
    let organization_id = envelope.normalized_organization_id();
    let expected_outbox_event_id =
        format!("conversation:{}:{}", envelope.event_type, envelope.event_id);
    let expected_payload_hash = sdkwork_utils_rust::sha256_hash(envelope.payload.as_bytes());
    let outbox_payload_hash = sdkwork_utils_rust::sha256_hash(outbox.payload_json.as_bytes());
    let payload_json = postgres_jsonb_payload(envelope.payload.as_str())?;
    let outbox_payload_json = postgres_jsonb_payload(outbox.payload_json.as_str())?;
    let valid = !envelope.tenant_id.trim().is_empty()
        && !envelope.event_id.trim().is_empty()
        && !envelope.event_type.trim().is_empty()
        && !envelope.aggregate_id.trim().is_empty()
        && envelope.aggregate_type == AggregateType::Conversation
        && envelope.scope_type == CONVERSATION_SCOPE_TYPE
        && envelope.scope_id == envelope.aggregate_id
        && envelope.ordering_key
            == CommitEnvelope::ordering_key(
                envelope.tenant_id.as_str(),
                envelope.aggregate_id.as_str(),
            )
        && outbox.tenant_id == envelope.tenant_id
        && outbox.organization_id == organization_id
        && !outbox.outbox_id.trim().is_empty()
        && !outbox.event_id.trim().is_empty()
        && outbox.event_id == expected_outbox_event_id
        && outbox.aggregate_type == CONVERSATION_OUTBOX_AGGREGATE_TYPE
        && outbox.aggregate_id == envelope.aggregate_id
        && outbox.event_type == envelope.event_type
        && outbox.payload_hash == expected_payload_hash
        && outbox_payload_hash == expected_payload_hash
        && payload_json == outbox_payload_json
        && outbox.publish_status == OutboxPublishStatus::Pending
        && outbox.attempt_count == 0
        && outbox.published_at.is_none();
    if !valid {
        return Err(ContractError::Invalid(
            "durable conversation event journal/outbox identity is invalid".into(),
        ));
    }
    Ok(())
}

fn persist_conversation_event_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelope: &CommitEnvelope,
    outbox: &OutboxEventRecord,
) -> Result<CommitPosition, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_conversation_event")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("persist_conversation_event begin", error))?;

    let outcome = append_journal_in_transaction(&mut txn, prefix, envelope)?;
    ensure_conversation_outbox_in_transaction(&mut txn, outbox)?;
    let position = outcome.into_commit_position()?;
    txn.commit()
        .map_err(|error| postgres_unavailable_db("persist_conversation_event commit", error))?;
    Ok(position)
}

fn ensure_conversation_outbox_in_transaction(
    txn: &mut Transaction<'_>,
    outbox: &OutboxEventRecord,
) -> Result<(), ContractError> {
    let attempted = ConversationOutboxFingerprint::from_record(outbox)?;
    let existing = load_conversation_outbox_identity(txn, outbox)?;
    match existing.as_slice() {
        [] => match enqueue_outbox_in_transaction(txn, outbox)? {
            OutboxEnqueueOutcome::Inserted => Ok(()),
            OutboxEnqueueOutcome::IdentityConflict => {
                // A concurrent replay may have inserted the deterministic row
                // after the first lookup. Re-read it under the same transaction
                // and accept only an exact immutable match.
                let concurrent = load_conversation_outbox_identity(txn, outbox)?;
                ensure_conversation_outbox_match(concurrent.as_slice(), &attempted)
            }
        },
        rows => ensure_conversation_outbox_match(rows, &attempted),
    }
}

fn load_conversation_outbox_identity(
    txn: &mut Transaction<'_>,
    outbox: &OutboxEventRecord,
) -> Result<Vec<ConversationOutboxFingerprint>, ContractError> {
    let rows = txn
        .query(
            LOAD_CONVERSATION_OUTBOX_BY_IDENTITY_SQL,
            &[
                &outbox.tenant_id,
                &outbox.organization_id,
                &outbox.outbox_id,
                &outbox.event_id,
            ],
        )
        .map_err(|error| {
            postgres_unavailable_db("conversation event outbox identity lookup", error)
        })?;
    rows.iter()
        .map(ConversationOutboxFingerprint::from_row)
        .collect()
}

fn ensure_conversation_outbox_match(
    rows: &[ConversationOutboxFingerprint],
    attempted: &ConversationOutboxFingerprint,
) -> Result<(), ContractError> {
    if rows.len() != 1 || rows.first() != Some(attempted) {
        return Err(conversation_event_replay_conflict());
    }
    Ok(())
}

fn ensure_message_post_replay_matches(
    txn: &mut Transaction<'_>,
    message: &StoredMessageRecord,
    outboxes: &[OutboxEventRecord],
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

    for outbox in outboxes {
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
    for event_type in ["message.posted", AGENT_MENTION_DISPATCH_EVENT_TYPE] {
        let expected_count = outboxes
            .iter()
            .filter(|outbox| outbox.event_type == event_type)
            .count();
        ensure_message_post_replay_outbox_count(txn, message, event_type, expected_count)?;
    }

    Ok(())
}

fn ensure_message_post_replay_outbox_count(
    txn: &mut Transaction<'_>,
    message: &StoredMessageRecord,
    event_type: &str,
    expected_count: usize,
) -> Result<(), ContractError> {
    let message_id = message.message_id.to_string();
    let row = txn
        .query_one(
            LOAD_REPLAY_OUTBOX_COUNT_SQL,
            &[
                &message.tenant_id,
                &message.organization_id,
                &message.conversation_id,
                &event_type,
                &message_id,
            ],
        )
        .map_err(|error| {
            postgres_unavailable_db("message post replay outbox count lookup", error)
        })?;
    let existing_count: i64 =
        postgres_row_get(&row, 0, "message post replay outbox count", "count")?;
    if usize::try_from(existing_count).ok() != Some(expected_count) {
        return Err(message_post_replay_conflict());
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

fn conversation_outbox_row_get<T>(
    row: &postgres::Row,
    column: usize,
    field: &'static str,
) -> Result<T, ContractError>
where
    T: for<'a> postgres::types::FromSql<'a>,
{
    postgres_row_get(row, column, "conversation event replay outbox", field)
}

fn message_post_replay_conflict() -> ContractError {
    ContractError::Conflict(MESSAGE_POST_REPLAY_CONFLICT_MESSAGE.into())
}

fn conversation_event_replay_conflict() -> ContractError {
    ContractError::Conflict(CONVERSATION_EVENT_REPLAY_CONFLICT_MESSAGE.into())
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
) -> Result<OutboxEnqueueOutcome, ContractError> {
    let payload_json = postgres_jsonb_payload(event.payload_json.as_str())?;
    let attempt_count_i32 = i32::try_from(event.attempt_count).map_err(|_| {
        ContractError::Invalid(
            "durable outbox attempt count exceeds the PostgreSQL INTEGER range".into(),
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
        Ok(1) => Ok(OutboxEnqueueOutcome::Inserted),
        Ok(0) => Ok(OutboxEnqueueOutcome::IdentityConflict),
        Ok(_) => Err(ContractError::Unavailable(
            "postgres journal durable outbox enqueue returned an invalid row count".into(),
        )),
        Err(error) => Err(postgres_unavailable_db("durable outbox enqueue", error)),
    }
}

#[cfg(test)]
mod tests {
    use im_domain_events::EventActor;
    use serde_json::json;

    use super::*;

    fn conversation_event_fixture() -> (CommitEnvelope, OutboxEventRecord) {
        let tenant_id = "tenant-conversation-event";
        let organization_id = "0";
        let conversation_id = "group-conversation-event";
        let event_id = "evt_conversation_agents_replaced";
        let event_type = "conversation.agents_replaced";
        let occurred_at = "2026-07-12T10:00:00.000Z";
        let payload = json!({
            "conversationId": conversation_id,
            "previousGeneration": 1,
            "agentAssignments": {
                "generation": 2,
                "source": "conversation_override",
                "agents": [{
                    "agentId": "agent.im.writer",
                    "revisionId": "revision.im.writer.1"
                }]
            },
            "replacedAt": occurred_at
        })
        .to_string();
        let payload_hash = sdkwork_utils_rust::sha256_hash(payload.as_bytes());
        let envelope = CommitEnvelope {
            event_id: event_id.into(),
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: conversation_id.into(),
            scope_type: CONVERSATION_SCOPE_TYPE.into(),
            scope_id: conversation_id.into(),
            ordering_key: CommitEnvelope::ordering_key(tenant_id, conversation_id),
            ordering_seq: 2,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: "user-1".into(),
                actor_kind: "user".into(),
                actor_session_id: None,
            },
            occurred_at: occurred_at.into(),
            committed_at: occurred_at.into(),
            payload_schema: Some("conversation.agents_replaced.v1".into()),
            payload: payload.clone(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        let outbox = OutboxEventRecord {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            outbox_id: "conv_ob_conversation_agents_replaced".into(),
            aggregate_type: CONVERSATION_OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: conversation_id.into(),
            event_id: format!("conversation:{event_type}:{event_id}"),
            event_type: event_type.into(),
            payload_json: payload,
            payload_hash,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: occurred_at.into(),
            published_at: None,
            created_at: occurred_at.into(),
            updated_at: occurred_at.into(),
        };
        (envelope, outbox)
    }

    #[test]
    fn conversation_event_validation_accepts_one_canonical_pair() {
        let (envelope, outbox) = conversation_event_fixture();
        validate_conversation_event(&envelope, &outbox)
            .expect("canonical conversation event and outbox should validate");
    }

    #[test]
    fn conversation_event_validation_rejects_noncanonical_outbox_event_id() {
        let (envelope, mut outbox) = conversation_event_fixture();
        outbox.event_id = "conversation:conversation.agents_replaced:unrelated-event".into();

        assert!(matches!(
            validate_conversation_event(&envelope, &outbox),
            Err(ContractError::Invalid(_))
        ));
    }

    #[test]
    fn conversation_event_validation_rejects_cross_scope_and_payload_drift() {
        let (envelope, outbox) = conversation_event_fixture();

        let mut invalid_scope = envelope.clone();
        invalid_scope.scope_id = "different-conversation".into();
        assert!(matches!(
            validate_conversation_event(&invalid_scope, &outbox),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_aggregate_type = envelope.clone();
        invalid_aggregate_type.aggregate_type = AggregateType::Space;
        assert!(matches!(
            validate_conversation_event(&invalid_aggregate_type, &outbox),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_outbox_scope = outbox.clone();
        invalid_outbox_scope.organization_id = "different-organization".into();
        assert!(matches!(
            validate_conversation_event(&envelope, &invalid_outbox_scope),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_event_type = outbox.clone();
        invalid_event_type.event_type = "conversation.member_joined".into();
        assert!(matches!(
            validate_conversation_event(&envelope, &invalid_event_type),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_payload = outbox.clone();
        invalid_payload.payload_json = json!({
            "conversationId": envelope.aggregate_id,
            "agentAssignments": {"generation": 3}
        })
        .to_string();
        assert!(matches!(
            validate_conversation_event(&envelope, &invalid_payload),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_hash = outbox;
        invalid_hash.payload_hash = "different-producer-hash".into();
        assert!(matches!(
            validate_conversation_event(&envelope, &invalid_hash),
            Err(ContractError::Invalid(_))
        ));
    }

    #[test]
    fn conversation_outbox_replay_compares_immutable_identity_not_delivery_state() {
        let (_envelope, outbox) = conversation_event_fixture();
        let expected = ConversationOutboxFingerprint::from_record(&outbox)
            .expect("fixture fingerprint should be valid");

        let mut delivered = outbox.clone();
        delivered.publish_status = OutboxPublishStatus::Published;
        delivered.attempt_count = 4;
        delivered.available_at = "2026-07-12T10:01:00.000Z".into();
        delivered.published_at = Some("2026-07-12T10:01:01.000Z".into());
        delivered.updated_at = "2026-07-12T10:01:01.000Z".into();
        assert_eq!(
            ConversationOutboxFingerprint::from_record(&delivered)
                .expect("delivery lifecycle should not affect fingerprint"),
            expected
        );

        let mut conflicting_identity = outbox.clone();
        conflicting_identity.outbox_id = "different-outbox-id".into();
        assert_ne!(
            ConversationOutboxFingerprint::from_record(&conflicting_identity)
                .expect("identity drift should still produce a fingerprint"),
            expected
        );

        let mut conflicting_hash = outbox;
        conflicting_hash.payload_hash = "different-producer-hash".into();
        assert_ne!(
            ConversationOutboxFingerprint::from_record(&conflicting_hash)
                .expect("hash drift should still produce a fingerprint"),
            expected
        );
    }

    #[test]
    fn outbox_insert_and_identity_lookup_support_concurrent_idempotency() {
        let insert = ENQUEUE_OUTBOX_SQL.to_ascii_lowercase();
        assert!(insert.contains("on conflict do nothing"));

        let lookup = LOAD_CONVERSATION_OUTBOX_BY_IDENTITY_SQL.to_ascii_lowercase();
        assert!(lookup.contains("outbox_id = $3 or event_id = $4"));
        assert!(lookup.contains("for update"));
    }
}
