//! Atomic journal + message truth + optional outbox enqueue in one Postgres transaction.

use chrono::Utc;
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitPosition, ContractError, OutboxEventRecord, StoredMessageRecord,
};
use r2d2_postgres::postgres::Transaction;

use crate::{
    compose_partition_key, journal_retention_until, postgres_jsonb_payload,
    postgres_pool_client, postgres_timestamptz, postgres_unavailable_db, run_postgres_io,
    PostgresJournalPool,
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
on conflict (tenant_id, organization_id, event_id) do nothing
"#;

enum JournalAppendOutcome {
    Inserted(String, i64),
    EventIdAbsorbed(String, i64),
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
            txn.commit()
                .map_err(|error| postgres_unavailable_db("persist_message_post commit", error))?;
            Ok(CommitPosition::new(partition, offset as u64))
        }
        JournalAppendOutcome::Inserted(partition, offset) => {
            insert_message_in_transaction(&mut txn, message)?;
            if let Some(outbox) = outbox {
                enqueue_outbox_in_transaction(&mut txn, outbox)?;
            }
            txn.commit()
                .map_err(|error| postgres_unavailable_db("persist_message_post commit", error))?;
            Ok(CommitPosition::new(partition, offset as u64))
        }
    }
}

fn append_journal_in_transaction(
    txn: &mut Transaction<'_>,
    prefix: &str,
    envelope: &CommitEnvelope,
) -> Result<JournalAppendOutcome, ContractError> {
    use crate::{is_unique_violation, APPEND_EVENT_SQL, LOAD_EVENT_BY_ID_SQL, LOAD_EVENT_BY_POSITION_SQL};
    use sdkwork_utils_rust::sha256_hash;

    let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
    let payload_json = postgres_jsonb_payload(envelope.payload.as_str())?;
    let payload_hash = sha256_hash(envelope.payload.as_bytes());
    let created_at = Utc::now();
    let aggregate_seq = i64::try_from(envelope.ordering_seq)
        .unwrap_or(0)
        .saturating_add(1);
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
                        let partition: String = row.get(0);
                        let offset: i64 = row.get(1);
                        JournalAppendOutcome::Inserted(partition, offset)
                    }
                    None => {
                        let row = txn
                            .query_one(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
                            .map_err(|error| {
                                postgres_unavailable_db("message post journal replay lookup", error)
                            })?;
                        let partition: String = row.get(0);
                        let offset: i64 = row.get(1);
                        JournalAppendOutcome::EventIdAbsorbed(partition, offset)
                    }
                }
            }
            Err(error) if is_unique_violation(&error) => {
                savepoint
                    .rollback()
                    .map_err(|error| postgres_unavailable_db("message post journal rollback", error))?;
                let row = txn
                    .query_one(LOAD_EVENT_BY_POSITION_SQL, &[&partition_key, &commit_offset])
                    .map_err(|error| {
                        postgres_unavailable_db("message post journal position lookup", error)
                    })?;
                let existing_event_id: String = row.get(0);
                let partition: String = row.get(1);
                let offset: i64 = row.get(2);
                if existing_event_id == envelope.event_id {
                    JournalAppendOutcome::EventIdAbsorbed(partition, offset)
                } else {
                    return Err(ContractError::Conflict(format!(
                        "journal position occupied by different event_id={existing_event_id}"
                    )));
                }
            }
            Err(error) => {
                return Err(postgres_unavailable_db("message post journal insert", error));
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

    let message_seq_i64 = message.message_seq as i64;
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
    let payload_json = postgres_jsonb_payload(event.payload_json.as_str())?;
    let attempt_count_i32 = event.attempt_count as i32;
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
    txn.execute(ENQUEUE_OUTBOX_SQL, params)
        .map_err(|error| postgres_unavailable_db("message post outbox enqueue", error))?;
    Ok(())
}
