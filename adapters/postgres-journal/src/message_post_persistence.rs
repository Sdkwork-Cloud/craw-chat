//! Atomic journal + message truth + optional outbox enqueue in one Postgres transaction.

use chrono::{DateTime, Utc};
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{
    AGENT_MENTION_DISPATCH_EVENT_TYPE, AgentDispatchReplyCompletion, CommitPosition, ContractError,
    IdGenerator, NormalizedConversationCommit, OutboxEventRecord, OutboxPublishStatus,
    StoredMessageMutation, StoredMessageMutationTarget, StoredMessageRecord,
};
use r2d2_postgres::postgres::Transaction;
use sdkwork_im_contract_agent::AgentMentionDispatchRequest;
use sdkwork_im_contract_notification::NotificationTaskRecord;
use std::collections::HashSet;
use std::sync::Arc;

use crate::agent_integration_store::replace_conversation_agents_in_transaction;
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

const UPDATE_CONVERSATION_AFTER_MESSAGE_SQL: &str = r#"
update im_conversations
set message_count = message_count + 1,
    last_message_id = $4,
    last_message_seq = $5,
    last_sender_kind = $6,
    last_sender_id = $7,
    last_summary = $8::jsonb ->> 'summary',
    last_message_at = $9,
    last_activity_at = $9,
    commit_seq = greatest(commit_seq, $10),
    updated_at = $9
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and last_message_seq < $5
"#;

const LOCK_MESSAGE_MUTATION_TARGET_SQL: &str = r#"
select deleted_at
from im_conversation_messages
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4 and message_seq = $5
for update
"#;

const UPDATE_EDITED_MESSAGE_SQL: &str = r#"
update im_conversation_messages
set payload_json = $6::jsonb,
    payload_hash = $7,
    updated_at = $8
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4 and message_seq = $5 and deleted_at is null
"#;

const UPDATE_RECALLED_MESSAGE_SQL: &str = r#"
update im_conversation_messages
set deleted_at = $6,
    updated_at = $6
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4 and message_seq = $5 and deleted_at is null
"#;

const LOAD_MESSAGE_REACTION_SQL: &str = r#"
select 1
from im_message_reactions
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4 and actor_principal_kind = $5
  and actor_principal_id = $6 and reaction_type = $7
"#;

const INSERT_MESSAGE_REACTION_SQL: &str = r#"
insert into im_message_reactions (
    tenant_id, organization_id, conversation_id, message_id,
    actor_principal_kind, actor_principal_id, reaction_type, created_at
) values ($1, $2, $3, $4, $5, $6, $7, $8)
"#;

const DELETE_MESSAGE_REACTION_SQL: &str = r#"
delete from im_message_reactions
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4 and actor_principal_kind = $5
  and actor_principal_id = $6 and reaction_type = $7
"#;

const LOAD_MESSAGE_PIN_SQL: &str = r#"
select 1
from im_message_pins
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4
"#;

const INSERT_MESSAGE_PIN_SQL: &str = r#"
insert into im_message_pins (
    tenant_id, organization_id, conversation_id, message_id,
    pinned_by_principal_kind, pinned_by_principal_id, pinned_at
) values ($1, $2, $3, $4, $5, $6, $7)
"#;

const DELETE_MESSAGE_PIN_SQL: &str = r#"
delete from im_message_pins
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and message_id = $4
"#;

const UPDATE_CONVERSATION_AFTER_MESSAGE_MUTATION_SQL: &str = r#"
update im_conversations
set commit_seq = greatest(commit_seq, $4),
    updated_at = greatest(updated_at, $5)
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const UPSERT_NORMALIZED_CONVERSATION_SQL: &str = r#"
insert into im_conversations (
    tenant_id, organization_id, conversation_id, conversation_type, lifecycle_state,
    commit_seq, member_epoch, last_activity_at, payload_json, payload_hash,
    created_at, updated_at, retention_until, archived_at, archive_event_id,
    commit_fingerprint
) values ($1, $2, $3, $4, $5, $6, $7, $8, '{}'::jsonb, $9, $8, $8, $10, $11, $12, $13)
on conflict (tenant_id, organization_id, conversation_id)
do update set
    conversation_type = excluded.conversation_type,
    lifecycle_state = excluded.lifecycle_state,
    archived_at = excluded.archived_at,
    archive_event_id = excluded.archive_event_id,
    commit_seq = excluded.commit_seq,
    member_epoch = excluded.member_epoch,
    last_activity_at = excluded.last_activity_at,
    payload_hash = excluded.payload_hash,
    commit_fingerprint = excluded.commit_fingerprint,
    updated_at = excluded.updated_at,
    retention_until = excluded.retention_until
where $14::bigint is not null and im_conversations.commit_seq = $14
returning commit_seq
"#;

const LOAD_NORMALIZED_CONVERSATION_REPLAY_MATCH_SQL: &str = r#"
select conversation_type = $4
    and lifecycle_state = $5
    and commit_seq = $6
    and member_epoch = $7
    and last_activity_at = $8
    and retention_until is not distinct from $9
    and archived_at is not distinct from $10
    and archive_event_id is not distinct from $11
    and commit_fingerprint = $12 as exact_replay
from im_conversations
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
for update
"#;

const UPSERT_NORMALIZED_POLICY_SQL: &str = r#"
insert into im_conversation_policies (
    tenant_id, organization_id, conversation_id, policy_epoch, policy_version,
    capability_flags, history_visibility, retention_policy_ref, max_members, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
on conflict (tenant_id, organization_id, conversation_id)
do update set policy_epoch = excluded.policy_epoch,
    policy_version = excluded.policy_version,
    capability_flags = excluded.capability_flags,
    history_visibility = excluded.history_visibility,
    retention_policy_ref = excluded.retention_policy_ref,
    max_members = excluded.max_members,
    updated_at = excluded.updated_at
"#;

const DELETE_NORMALIZED_POLICY_SQL: &str = r#"
delete from im_conversation_policies
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const UPSERT_NORMALIZED_BUSINESS_BINDING_SQL: &str = r#"
insert into im_conversation_business_bindings (
    tenant_id, organization_id, conversation_id, business_type, business_id, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $6)
on conflict (tenant_id, organization_id, conversation_id)
do update set business_type = excluded.business_type,
    business_id = excluded.business_id,
    updated_at = excluded.updated_at
"#;

const DELETE_NORMALIZED_BUSINESS_BINDING_SQL: &str = r#"
delete from im_conversation_business_bindings
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const UPSERT_NORMALIZED_HANDOFF_SQL: &str = r#"
insert into im_conversation_handoffs (
    tenant_id, organization_id, conversation_id, handoff_status_epoch, status,
    source_principal_kind, source_principal_id, target_principal_kind, target_principal_id,
    handoff_session_id, handoff_reason, accepted_at, accepted_by_principal_kind,
    accepted_by_principal_id, resolved_at, resolved_by_principal_kind,
    resolved_by_principal_id, closed_at, closed_by_principal_kind, closed_by_principal_id,
    created_at, updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
    $16, $17, $18, $19, $20, $21, $21
)
on conflict (tenant_id, organization_id, conversation_id)
do update set handoff_status_epoch = excluded.handoff_status_epoch,
    status = excluded.status,
    source_principal_kind = excluded.source_principal_kind,
    source_principal_id = excluded.source_principal_id,
    target_principal_kind = excluded.target_principal_kind,
    target_principal_id = excluded.target_principal_id,
    handoff_session_id = excluded.handoff_session_id,
    handoff_reason = excluded.handoff_reason,
    accepted_at = excluded.accepted_at,
    accepted_by_principal_kind = excluded.accepted_by_principal_kind,
    accepted_by_principal_id = excluded.accepted_by_principal_id,
    resolved_at = excluded.resolved_at,
    resolved_by_principal_kind = excluded.resolved_by_principal_kind,
    resolved_by_principal_id = excluded.resolved_by_principal_id,
    closed_at = excluded.closed_at,
    closed_by_principal_kind = excluded.closed_by_principal_kind,
    closed_by_principal_id = excluded.closed_by_principal_id,
    updated_at = excluded.updated_at
"#;

const DELETE_NORMALIZED_HANDOFF_SQL: &str = r#"
delete from im_conversation_handoffs
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

const UPSERT_NORMALIZED_MEMBER_SQL: &str = r#"
insert into im_conversation_members (
    tenant_id, organization_id, conversation_id, principal_kind, principal_id,
    member_id, membership_role, membership_state, invited_by, joined_at, removed_at,
    attributes_json, payload_json, payload_hash, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::jsonb, '{}'::jsonb, $13, $10, $14)
on conflict (tenant_id, organization_id, conversation_id, principal_kind, principal_id)
do update set
    member_id = excluded.member_id,
    membership_role = excluded.membership_role,
    membership_state = excluded.membership_state,
    invited_by = excluded.invited_by,
    joined_at = excluded.joined_at,
    removed_at = excluded.removed_at,
    attributes_json = excluded.attributes_json,
    updated_at = excluded.updated_at
"#;

const UPSERT_NORMALIZED_READ_CURSOR_SQL: &str = r#"
insert into im_conversation_read_cursors (
    tenant_id, organization_id, conversation_id, member_id, device_id, principal_kind,
    principal_id, read_seq, last_read_message_id, payload_json, payload_hash, created_at, updated_at
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, '{}'::jsonb, $10, $11, $11)
on conflict (tenant_id, organization_id, conversation_id, member_id, device_id)
do update set
    principal_kind = excluded.principal_kind,
    principal_id = excluded.principal_id,
    read_seq = greatest(im_conversation_read_cursors.read_seq, excluded.read_seq),
    last_read_message_id = case
        when excluded.read_seq >= im_conversation_read_cursors.read_seq
            then excluded.last_read_message_id
        else im_conversation_read_cursors.last_read_message_id
    end,
    updated_at = greatest(im_conversation_read_cursors.updated_at, excluded.updated_at)
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
    id_generator: Arc<dyn IdGenerator>,
}

impl PostgresDurableConversationEventWriter {
    pub fn new(pool: PostgresJournalPool, partition_prefix: std::sync::Arc<str>) -> Self {
        Self::with_id_generator(
            pool,
            partition_prefix,
            sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "im-durable-conversation-event",
            ),
        )
    }

    pub fn with_id_generator(
        pool: PostgresJournalPool,
        partition_prefix: std::sync::Arc<str>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            pool,
            partition_prefix,
            id_generator,
        }
    }

    pub fn from_journal(journal: &crate::PostgresCommitJournal) -> Self {
        Self::new(journal.pool().clone(), journal.partition_prefix().clone())
    }

    pub fn from_journal_with_id_generator(
        journal: &crate::PostgresCommitJournal,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Self {
        Self::with_id_generator(
            journal.pool().clone(),
            journal.partition_prefix().clone(),
            id_generator,
        )
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

    pub fn persist_normalized_conversation_commit(
        &self,
        commit: NormalizedConversationCommit,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        validate_normalized_conversation_commit(&commit)?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        let id_generator = self.id_generator.clone();
        run_postgres_io(move || {
            persist_normalized_conversation_commit_txn(
                &pool,
                prefix.as_ref(),
                &commit,
                id_generator.as_ref(),
            )
        })
    }
}

/// Postgres-backed atomic message post writer (journal + message + outbox).
#[derive(Clone)]
pub struct PostgresDurableMessagePostWriter {
    pool: PostgresJournalPool,
    partition_prefix: std::sync::Arc<str>,
    id_generator: Arc<dyn IdGenerator>,
}

impl PostgresDurableMessagePostWriter {
    pub fn new(pool: PostgresJournalPool, partition_prefix: std::sync::Arc<str>) -> Self {
        Self {
            pool,
            partition_prefix,
            id_generator: sdkwork_im_runtime_id::build_runtime_id_generator_blocking(
                "im-durable-message-post",
            ),
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
        self.persist_message_post_batch_with_agent_dispatch(envelopes, message, outboxes, None, 10)
    }

    pub fn persist_message_post_batch_with_agent_dispatch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        dispatch_request: Option<AgentMentionDispatchRequest>,
        max_dispatch_attempts: u32,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        self.persist_message_post_batch_with_notification_fanout(
            envelopes,
            message,
            outboxes,
            dispatch_request,
            max_dispatch_attempts,
            Vec::new(),
        )
    }

    /// Atomic message post plus notification-request fanout: notification
    /// task rows are inserted in the same transaction as the journal, message,
    /// and outbox writes, so a committed message always has its notification
    /// requests persisted (and a failed post notifies nobody).
    pub fn persist_message_post_batch_with_notification_fanout(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        dispatch_request: Option<AgentMentionDispatchRequest>,
        max_dispatch_attempts: u32,
        notification_tasks: Vec<NotificationTaskRecord>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        if envelopes.is_empty() {
            return Err(ContractError::Invalid(
                "durable message post requires at least one journal envelope".into(),
            ));
        }
        validate_message_post_batch(envelopes.as_slice(), &message, outboxes.as_slice())?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        let id_generator = self.id_generator.clone();
        run_postgres_io(move || {
            let mut envelopes = envelopes;
            persist_message_post_txn(
                &pool,
                prefix.as_ref(),
                &mut envelopes,
                &message,
                outboxes.as_slice(),
                dispatch_request.as_ref(),
                max_dispatch_attempts,
                id_generator.as_ref(),
                notification_tasks.as_slice(),
            )
        })
    }

    pub fn persist_agent_reply_and_complete_dispatch(
        &self,
        envelopes: Vec<CommitEnvelope>,
        message: StoredMessageRecord,
        outboxes: Vec<OutboxEventRecord>,
        completion: AgentDispatchReplyCompletion,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        if envelopes.is_empty() {
            return Err(ContractError::Invalid(
                "durable agent reply requires at least one journal envelope".into(),
            ));
        }
        validate_message_post_batch(envelopes.as_slice(), &message, outboxes.as_slice())?;
        validate_agent_reply_completion(&message, &completion)?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || {
            persist_agent_reply_txn(
                &pool,
                prefix.as_ref(),
                envelopes.as_slice(),
                &message,
                outboxes.as_slice(),
                &completion,
            )
        })
    }
}

/// PostgreSQL writer for one authoritative message mutation commit.
///
/// The target message row is locked before the change decision so concurrent
/// add/remove and pin/unpin commands are serialized by normalized state rather
/// than by journal replay.
#[derive(Clone)]
pub struct PostgresDurableMessageMutationWriter {
    pool: PostgresJournalPool,
    partition_prefix: std::sync::Arc<str>,
}

impl PostgresDurableMessageMutationWriter {
    pub fn new(pool: PostgresJournalPool, partition_prefix: std::sync::Arc<str>) -> Self {
        Self {
            pool,
            partition_prefix,
        }
    }

    pub fn from_journal(journal: &crate::PostgresCommitJournal) -> Self {
        Self::new(journal.pool().clone(), journal.partition_prefix().clone())
    }

    /// Returns the durable journal position when the command changed state.
    /// A normalized-state no-op returns `None` and writes neither journal nor
    /// outbox rows.
    pub fn persist_message_mutation(
        &self,
        envelope: CommitEnvelope,
        mutation: StoredMessageMutation,
        outbox: OutboxEventRecord,
    ) -> Result<Option<CommitPosition>, ContractError> {
        validate_message_mutation_commit(&envelope, &mutation, &outbox)?;
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || {
            persist_message_mutation_txn(&pool, prefix.as_ref(), &envelope, &mutation, &outbox)
        })
    }
}

fn validate_message_mutation_commit(
    envelope: &CommitEnvelope,
    mutation: &StoredMessageMutation,
    outbox: &OutboxEventRecord,
) -> Result<(), ContractError> {
    let target = mutation.target();
    let organization_id =
        im_domain_events::normalize_commit_organization_id(target.organization_id.as_str());
    let payload = postgres_jsonb_payload(envelope.payload.as_str())?;
    let outbox_payload = postgres_jsonb_payload(outbox.payload_json.as_str())?;
    let expected_outbox_event_id = format!(
        "conversation:{}:{}",
        mutation.event_type(),
        envelope.event_id
    );
    let actor_field = match mutation {
        StoredMessageMutation::Edited { .. } => "editor",
        StoredMessageMutation::Recalled { .. } => "recalledBy",
        StoredMessageMutation::ReactionAdded { .. } => "reactedBy",
        StoredMessageMutation::ReactionRemoved { .. } => "removedBy",
        StoredMessageMutation::Pinned { .. } => "pinnedBy",
        StoredMessageMutation::Unpinned { .. } => "unpinnedBy",
    };
    let payload_message_id = payload.get("messageId").and_then(serde_json::Value::as_str);
    let payload_message_seq = payload
        .get("messageSeq")
        .and_then(serde_json::Value::as_u64);
    let payload_actor = payload.get(actor_field);
    let payload_actor_identity_matches = payload_actor
        .and_then(|actor| actor.get("kind"))
        .and_then(serde_json::Value::as_str)
        == Some(envelope.actor.actor_kind.as_str())
        && payload_actor
            .and_then(|actor| actor.get("id"))
            .and_then(serde_json::Value::as_str)
            == Some(envelope.actor.actor_id.as_str());
    let normalized_actor_identity_matches = match mutation {
        StoredMessageMutation::ReactionAdded { reaction, .. }
        | StoredMessageMutation::ReactionRemoved { reaction, .. } => {
            envelope.actor.actor_kind == reaction.actor_principal_kind
                && envelope.actor.actor_id == reaction.actor_principal_id
                && payload
                    .get("reactionKey")
                    .and_then(serde_json::Value::as_str)
                    == Some(reaction.reaction_key.as_str())
        }
        StoredMessageMutation::Pinned { pin, .. } => {
            envelope.actor.actor_kind == pin.pinned_by_principal_kind
                && envelope.actor.actor_id == pin.pinned_by_principal_id
        }
        _ => true,
    };
    if target.tenant_id.trim().is_empty()
        || target.organization_id.trim().is_empty()
        || target.conversation_id.trim().is_empty()
        || target.message_id.trim().is_empty()
        || target.message_seq == 0
        || target.message_id.parse::<i64>().is_err()
        || envelope.tenant_id != target.tenant_id
        || envelope.normalized_organization_id() != organization_id
        || envelope.aggregate_type != AggregateType::Conversation
        || envelope.aggregate_id != target.conversation_id
        || envelope.scope_type != CONVERSATION_SCOPE_TYPE
        || envelope.scope_id != target.conversation_id
        || envelope.ordering_key
            != CommitEnvelope::ordering_key(
                target.tenant_id.as_str(),
                target.conversation_id.as_str(),
            )
        || envelope.event_type != mutation.event_type()
        || envelope.event_id.trim().is_empty()
        || envelope.ordering_seq == 0
        || payload.get("tenantId").and_then(serde_json::Value::as_str)
            != Some(target.tenant_id.as_str())
        || payload
            .get("conversationId")
            .and_then(serde_json::Value::as_str)
            != Some(target.conversation_id.as_str())
        || payload_message_id != Some(target.message_id.as_str())
        || payload_message_seq != Some(target.message_seq)
        || !payload_actor_identity_matches
        || !normalized_actor_identity_matches
        || outbox.tenant_id != target.tenant_id
        || im_domain_events::normalize_commit_organization_id(outbox.organization_id.as_str())
            != organization_id
        || outbox.aggregate_type != CONVERSATION_OUTBOX_AGGREGATE_TYPE
        || outbox.aggregate_id != target.conversation_id
        || outbox.event_type != mutation.event_type()
        || outbox.outbox_id.trim().is_empty()
        || outbox.event_id != expected_outbox_event_id
        || outbox_payload
            .get("conversationId")
            .and_then(serde_json::Value::as_str)
            != Some(target.conversation_id.as_str())
        || outbox_payload
            .get("messageId")
            .and_then(serde_json::Value::as_str)
            != Some(target.message_id.as_str())
        || outbox_payload
            .get("messageSeq")
            .and_then(serde_json::Value::as_u64)
            != Some(target.message_seq)
        || outbox.payload_hash != sdkwork_utils_rust::sha256_hash(outbox.payload_json.as_bytes())
        || outbox.publish_status != OutboxPublishStatus::Pending
        || outbox.attempt_count != 0
        || outbox.published_at.is_some()
    {
        return Err(ContractError::Invalid(
            "durable message mutation identity is invalid".into(),
        ));
    }
    Ok(())
}

fn validate_agent_reply_completion(
    message: &StoredMessageRecord,
    completion: &AgentDispatchReplyCompletion,
) -> Result<(), ContractError> {
    let message_tenant_id = message
        .tenant_id
        .parse::<u64>()
        .map_err(|_| ContractError::Invalid("agent reply tenant id is invalid".into()))?;
    let message_organization_id = message
        .organization_id
        .parse::<u64>()
        .map_err(|_| ContractError::Invalid("agent reply organization id is invalid".into()))?;
    if message_tenant_id != completion.tenant_id
        || message_organization_id != completion.organization_id
        || message.conversation_id != completion.conversation_id
        || message.sender_principal_kind != "agent"
        || message.sender_principal_id != completion.agent_id
        || completion.dispatch_id.trim().is_empty()
        || completion.lease_owner.trim().is_empty()
        || completion.agents_session_id.trim().is_empty()
        || completion.agents_turn_id.trim().is_empty()
    {
        return Err(ContractError::Invalid(
            "agent reply dispatch completion identity is invalid".into(),
        ));
    }
    Ok(())
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

fn persist_message_mutation_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelope: &CommitEnvelope,
    mutation: &StoredMessageMutation,
    outbox: &OutboxEventRecord,
) -> Result<Option<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_message_mutation")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("persist_message_mutation begin", error))?;
    let target = mutation.target();
    let message_id = target_message_id(target)?;
    let message_seq = postgres_bigint_input(target.message_seq, "message sequence")?;
    let target_row = txn
        .query_opt(
            LOCK_MESSAGE_MUTATION_TARGET_SQL,
            &[
                &target.tenant_id,
                &target.organization_id,
                &target.conversation_id,
                &message_id,
                &message_seq,
            ],
        )
        .map_err(|error| postgres_unavailable_db("lock message mutation target", error))?
        .ok_or_else(|| ContractError::Conflict("message mutation target is missing".into()))?;
    let deleted_at: Option<DateTime<Utc>> = target_row.get(0);
    let should_apply =
        message_mutation_requires_change(&mut txn, mutation, message_id, deleted_at.is_some())?;
    if !should_apply {
        txn.rollback()
            .map_err(|error| postgres_unavailable_db("message mutation no-op rollback", error))?;
        return Ok(None);
    }

    let outcome = append_journal_in_transaction(&mut txn, prefix, envelope)?;
    if matches!(outcome, JournalAppendOutcome::Inserted(_, _)) {
        apply_message_mutation_in_transaction(&mut txn, mutation, message_id, message_seq)?;
        update_conversation_after_message_mutation_in_transaction(&mut txn, target, envelope)?;
    }
    ensure_conversation_outbox_in_transaction(&mut txn, outbox)?;
    let position = outcome.into_commit_position()?;
    txn.commit()
        .map_err(|error| postgres_unavailable_db("persist_message_mutation commit", error))?;
    Ok(Some(position))
}

fn target_message_id(target: &StoredMessageMutationTarget) -> Result<i64, ContractError> {
    target
        .message_id
        .parse::<i64>()
        .map_err(|_| ContractError::Invalid("message id must be a signed int64 string".into()))
}

fn message_mutation_requires_change(
    txn: &mut Transaction<'_>,
    mutation: &StoredMessageMutation,
    message_id: i64,
    recalled: bool,
) -> Result<bool, ContractError> {
    let target = mutation.target();
    match mutation {
        StoredMessageMutation::Edited { .. } => {
            if recalled {
                return Err(ContractError::Conflict(
                    "recalled messages cannot be edited".into(),
                ));
            }
            Ok(true)
        }
        StoredMessageMutation::Recalled { .. } => Ok(!recalled),
        StoredMessageMutation::ReactionAdded { reaction, .. }
        | StoredMessageMutation::ReactionRemoved { reaction, .. } => {
            if recalled {
                return Err(ContractError::Conflict(
                    "recalled messages cannot change reactions".into(),
                ));
            }
            let exists = txn
                .query_opt(
                    LOAD_MESSAGE_REACTION_SQL,
                    &[
                        &target.tenant_id,
                        &target.organization_id,
                        &target.conversation_id,
                        &message_id,
                        &reaction.actor_principal_kind,
                        &reaction.actor_principal_id,
                        &reaction.reaction_key,
                    ],
                )
                .map_err(|error| postgres_unavailable_db("load message reaction", error))?
                .is_some();
            Ok(match mutation {
                StoredMessageMutation::ReactionAdded { .. } => !exists,
                StoredMessageMutation::ReactionRemoved { .. } => exists,
                _ => unreachable!("reaction variants matched above"),
            })
        }
        StoredMessageMutation::Pinned { .. } | StoredMessageMutation::Unpinned { .. } => {
            if recalled {
                return Err(ContractError::Conflict(
                    "recalled messages cannot change pins".into(),
                ));
            }
            let exists = txn
                .query_opt(
                    LOAD_MESSAGE_PIN_SQL,
                    &[
                        &target.tenant_id,
                        &target.organization_id,
                        &target.conversation_id,
                        &message_id,
                    ],
                )
                .map_err(|error| postgres_unavailable_db("load message pin", error))?
                .is_some();
            Ok(match mutation {
                StoredMessageMutation::Pinned { .. } => !exists,
                StoredMessageMutation::Unpinned { .. } => exists,
                _ => unreachable!("pin variants matched above"),
            })
        }
    }
}

fn apply_message_mutation_in_transaction(
    txn: &mut Transaction<'_>,
    mutation: &StoredMessageMutation,
    message_id: i64,
    message_seq: i64,
) -> Result<(), ContractError> {
    let target = mutation.target();
    let affected = match mutation {
        StoredMessageMutation::Edited {
            payload_json,
            payload_hash,
            edited_at,
            ..
        } => {
            let payload_json = postgres_jsonb_payload(payload_json.as_str())?;
            let edited_at = postgres_timestamptz(edited_at.as_str(), "edited_at")?;
            txn.execute(
                UPDATE_EDITED_MESSAGE_SQL,
                &[
                    &target.tenant_id,
                    &target.organization_id,
                    &target.conversation_id,
                    &message_id,
                    &message_seq,
                    &payload_json,
                    payload_hash,
                    &edited_at,
                ],
            )
            .map_err(|error| postgres_unavailable_db("update edited message", error))?
        }
        StoredMessageMutation::Recalled { recalled_at, .. } => {
            let recalled_at = postgres_timestamptz(recalled_at.as_str(), "recalled_at")?;
            txn.execute(
                UPDATE_RECALLED_MESSAGE_SQL,
                &[
                    &target.tenant_id,
                    &target.organization_id,
                    &target.conversation_id,
                    &message_id,
                    &message_seq,
                    &recalled_at,
                ],
            )
            .map_err(|error| postgres_unavailable_db("update recalled message", error))?
        }
        StoredMessageMutation::ReactionAdded { reaction, .. } => {
            let reacted_at = postgres_timestamptz(reaction.reacted_at.as_str(), "reacted_at")?;
            txn.execute(
                INSERT_MESSAGE_REACTION_SQL,
                &[
                    &target.tenant_id,
                    &target.organization_id,
                    &target.conversation_id,
                    &message_id,
                    &reaction.actor_principal_kind,
                    &reaction.actor_principal_id,
                    &reaction.reaction_key,
                    &reacted_at,
                ],
            )
            .map_err(|error| postgres_unavailable_db("insert message reaction", error))?
        }
        StoredMessageMutation::ReactionRemoved { reaction, .. } => txn
            .execute(
                DELETE_MESSAGE_REACTION_SQL,
                &[
                    &target.tenant_id,
                    &target.organization_id,
                    &target.conversation_id,
                    &message_id,
                    &reaction.actor_principal_kind,
                    &reaction.actor_principal_id,
                    &reaction.reaction_key,
                ],
            )
            .map_err(|error| postgres_unavailable_db("delete message reaction", error))?,
        StoredMessageMutation::Pinned { pin, .. } => {
            let pinned_at = postgres_timestamptz(pin.pinned_at.as_str(), "pinned_at")?;
            txn.execute(
                INSERT_MESSAGE_PIN_SQL,
                &[
                    &target.tenant_id,
                    &target.organization_id,
                    &target.conversation_id,
                    &message_id,
                    &pin.pinned_by_principal_kind,
                    &pin.pinned_by_principal_id,
                    &pinned_at,
                ],
            )
            .map_err(|error| postgres_unavailable_db("insert message pin", error))?
        }
        StoredMessageMutation::Unpinned { .. } => txn
            .execute(
                DELETE_MESSAGE_PIN_SQL,
                &[
                    &target.tenant_id,
                    &target.organization_id,
                    &target.conversation_id,
                    &message_id,
                ],
            )
            .map_err(|error| postgres_unavailable_db("delete message pin", error))?,
    };
    if affected != 1 {
        return Err(ContractError::Conflict(
            "message mutation lost its normalized-state fence".into(),
        ));
    }
    Ok(())
}

fn update_conversation_after_message_mutation_in_transaction(
    txn: &mut Transaction<'_>,
    target: &StoredMessageMutationTarget,
    envelope: &CommitEnvelope,
) -> Result<(), ContractError> {
    let commit_seq = postgres_bigint_input(envelope.ordering_seq, "conversation commit sequence")?;
    let committed_at = postgres_timestamptz(envelope.committed_at.as_str(), "committed_at")?;
    let affected = txn
        .execute(
            UPDATE_CONVERSATION_AFTER_MESSAGE_MUTATION_SQL,
            &[
                &target.tenant_id,
                &target.organization_id,
                &target.conversation_id,
                &commit_seq,
                &committed_at,
            ],
        )
        .map_err(|error| {
            postgres_unavailable_db("update conversation after message mutation", error)
        })?;
    if affected != 1 {
        return Err(ContractError::Conflict(
            "normalized conversation is missing for message mutation".into(),
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn persist_message_post_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelopes: &mut [CommitEnvelope],
    message: &StoredMessageRecord,
    outboxes: &[OutboxEventRecord],
    dispatch_request: Option<&AgentMentionDispatchRequest>,
    max_dispatch_attempts: u32,
    id_generator: &dyn IdGenerator,
    notification_tasks: &[NotificationTaskRecord],
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_message_post")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("persist_message_post begin", error))?;

    // Serialize concurrent writers of the same journal partition: the
    // partition-level advisory xact lock (`pg_advisory_xact_lock`) is released
    // automatically when this transaction commits or rolls back. Without it,
    // two nodes posting into the same conversation can both allocate the same
    // next aggregate sequence and collide on the `(partition_key,
    // commit_offset)` primary key (HTTP 409) instead of being serialized.
    crate::lock_journal_partitions(&mut txn, prefix, envelopes, "message post journal lock")?;
    // Recompute the ordering sequences from the DB's latest aggregate_seq
    // while holding the partition lock, so every writer appends at the true
    // next position. Idempotent replay is preserved: an event_id that is
    // already committed keeps its original sequence and is absorbed by
    // `ON CONFLICT (event_id) DO NOTHING` below (returning the original
    // message position).
    //
    // NOTE for callers: when the caller's in-memory aggregate is stale, the
    // database may assign higher sequences than the caller pre-computed. The
    // caller's success path should observe the aggregate commit sequence from
    // the returned `CommitPosition` offsets (e.g. via
    // `aggregate.observe_commit_seq`) instead of the pre-computed
    // `envelope.ordering_seq`; the storage layer guarantees the journal is
    // correct regardless.
    crate::allocate_next_ordering_sequences(&mut txn, prefix, envelopes)?;

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
        update_conversation_after_message_in_transaction(&mut txn, message, envelopes)?;
        for outbox in outboxes {
            if enqueue_outbox_in_transaction(&mut txn, outbox)?
                == OutboxEnqueueOutcome::IdentityConflict
            {
                return Err(ContractError::Conflict("event already enqueued".into()));
            }
        }
        if !notification_tasks.is_empty() {
            crate::notification_task_store::enqueue_notification_tasks_in_transaction(
                &mut txn,
                notification_tasks,
            )?;
        }
        if let Some(dispatch_request) = dispatch_request {
            crate::agent_integration_store::enqueue_dispatches_in_transaction(
                &mut txn,
                dispatch_request,
                max_dispatch_attempts,
                id_generator,
            )?;
        }
    } else if inserted_count == 0 {
        ensure_message_post_replay_matches(&mut txn, message, outboxes)?;
        if let Some(dispatch_request) = dispatch_request {
            crate::agent_integration_store::enqueue_dispatches_in_transaction(
                &mut txn,
                dispatch_request,
                max_dispatch_attempts,
                id_generator,
            )?;
        }
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

fn persist_agent_reply_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelopes: &[CommitEnvelope],
    message: &StoredMessageRecord,
    outboxes: &[OutboxEventRecord],
    completion: &AgentDispatchReplyCompletion,
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_agent_reply")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable_db("persist_agent_reply begin", error))?;

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
        update_conversation_after_message_in_transaction(&mut txn, message, envelopes)?;
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

    complete_agent_dispatch_in_transaction(&mut txn, message, completion)?;
    let positions = outcomes
        .into_iter()
        .map(JournalAppendOutcome::into_commit_position)
        .collect::<Result<Vec<_>, ContractError>>()?;
    txn.commit()
        .map_err(|error| postgres_unavailable_db("persist_agent_reply commit", error))?;
    Ok(positions)
}

fn complete_agent_dispatch_in_transaction(
    txn: &mut Transaction<'_>,
    message: &StoredMessageRecord,
    completion: &AgentDispatchReplyCompletion,
) -> Result<(), ContractError> {
    let completed_at = postgres_timestamptz(&message.created_at, "completed_at")?;
    let affected = txn
        .execute(
            crate::agent_integration_store::COMPLETE_DISPATCH_SQL,
            &[
                &(completion.tenant_id as i64),
                &(completion.organization_id as i64),
                &completion.dispatch_id,
                &completion.lease_owner,
                &completion.agents_turn_id,
                &message.message_id,
                &(message.message_seq as i64),
                &completed_at,
            ],
        )
        .map_err(|error| postgres_unavailable_db("complete agent dispatch with reply", error))?;
    if affected == 1 {
        return Ok(());
    }

    let row = txn
        .query_opt(
            crate::agent_integration_store::SELECT_DISPATCH_COMPLETION_SQL,
            &[
                &(completion.tenant_id as i64),
                &(completion.organization_id as i64),
                &completion.dispatch_id,
            ],
        )
        .map_err(|error| postgres_unavailable_db("load completed agent dispatch", error))?
        .ok_or_else(|| {
            ContractError::Conflict("agent dispatch completion fence rejected".into())
        })?;
    let exact_replay = row.get::<_, i16>(0) == 4
        && row.get::<_, String>(1) == completion.agent_id
        && row.get::<_, Option<String>>(2).as_deref()
            == Some(completion.agents_session_id.as_str())
        && row.get::<_, Option<String>>(3).as_deref() == Some(completion.agents_turn_id.as_str())
        && row.get::<_, Option<i64>>(4) == Some(message.message_id)
        && row.get::<_, Option<i64>>(5) == Some(message.message_seq as i64)
        && row.get::<_, String>(6) == completion.conversation_id;
    if exact_replay {
        Ok(())
    } else {
        Err(ContractError::Conflict(
            "agent dispatch completion fence rejected".into(),
        ))
    }
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

fn validate_normalized_conversation_commit(
    commit: &NormalizedConversationCommit,
) -> Result<(), ContractError> {
    let conversation = &commit.conversation;
    if conversation.tenant_id.trim().is_empty()
        || conversation.organization_id.trim().is_empty()
        || conversation.conversation_id.trim().is_empty()
        || conversation.conversation_type.trim().is_empty()
        || !matches!(conversation.lifecycle_state.as_str(), "active" | "archived")
        || commit.envelopes.is_empty()
        || commit.envelopes.len() != commit.outboxes.len()
    {
        return Err(ContractError::Invalid(
            "normalized conversation commit identity is invalid".into(),
        ));
    }
    let mut expected_ordering_seq = commit.expected_commit_seq.unwrap_or_default();
    for (index, envelope) in commit.envelopes.iter().enumerate() {
        if commit.expected_commit_seq.is_some() || index > 0 {
            expected_ordering_seq = expected_ordering_seq.checked_add(1).ok_or_else(|| {
                ContractError::Invalid("normalized conversation commit sequence overflow".into())
            })?;
        }
        if envelope.ordering_seq != expected_ordering_seq {
            return Err(ContractError::Invalid(
                "normalized conversation journal sequence is not contiguous".into(),
            ));
        }
    }
    if conversation.commit_seq != expected_ordering_seq
        || conversation.member_epoch > conversation.commit_seq
        || match conversation.lifecycle_state.as_str() {
            "active" => {
                conversation.archived_at.is_some() || conversation.archive_event_id.is_some()
            }
            "archived" => {
                conversation.archived_at.is_none() || conversation.archive_event_id.is_none()
            }
            _ => true,
        }
    {
        return Err(ContractError::Invalid(
            "normalized conversation commit sequence is invalid".into(),
        ));
    }
    for (envelope, outbox) in commit.envelopes.iter().zip(&commit.outboxes) {
        validate_conversation_event(envelope, outbox)?;
        if envelope.tenant_id != conversation.tenant_id
            || envelope.normalized_organization_id() != conversation.organization_id
            || envelope.aggregate_id != conversation.conversation_id
        {
            return Err(ContractError::Invalid(
                "normalized conversation journal scope is invalid".into(),
            ));
        }
    }
    for member in &commit.members {
        if member.tenant_id != conversation.tenant_id
            || member.organization_id != conversation.organization_id
            || member.conversation_id != conversation.conversation_id
        {
            return Err(ContractError::Invalid(
                "normalized conversation member scope is invalid".into(),
            ));
        }
    }
    for cursor in &commit.read_cursors {
        if cursor.tenant_id != conversation.tenant_id
            || cursor.organization_id != conversation.organization_id
            || cursor.conversation_id != conversation.conversation_id
        {
            return Err(ContractError::Invalid(
                "normalized conversation read cursor scope is invalid".into(),
            ));
        }
    }
    if let Some(policy) = commit.policy.as_ref()
        && (policy.tenant_id != conversation.tenant_id
            || policy.organization_id != conversation.organization_id
            || policy.conversation_id != conversation.conversation_id
            || policy.policy_epoch > conversation.commit_seq
            || policy.policy_version.trim().is_empty()
            || policy.history_visibility.trim().is_empty()
            || policy.retention_policy_ref.trim().is_empty()
            || policy.max_members.is_some_and(|value| value <= 0))
        {
            return Err(ContractError::Invalid(
                "normalized conversation policy is invalid".into(),
            ));
        }
    if let Some(binding) = commit.business_binding.as_ref()
        && (binding.tenant_id != conversation.tenant_id
            || binding.organization_id != conversation.organization_id
            || binding.conversation_id != conversation.conversation_id
            || binding.business_type.trim().is_empty()
            || binding.business_id.trim().is_empty())
        {
            return Err(ContractError::Invalid(
                "normalized conversation business binding is invalid".into(),
            ));
        }
    if let Some(handoff) = commit.handoff.as_ref() {
        let actor_pair_is_valid = |kind: &Option<String>, id: &Option<String>| {
            kind.is_some() == id.is_some()
                && kind.as_ref().is_none_or(|value| !value.trim().is_empty())
                && id.as_ref().is_none_or(|value| !value.trim().is_empty())
        };
        if handoff.tenant_id != conversation.tenant_id
            || handoff.organization_id != conversation.organization_id
            || handoff.conversation_id != conversation.conversation_id
            || conversation.conversation_type != "agent_handoff"
            || handoff.handoff_status_epoch > conversation.commit_seq
            || !matches!(
                handoff.status.as_str(),
                "open" | "accepted" | "resolved" | "closed"
            )
            || handoff.source_principal_kind.trim().is_empty()
            || handoff.source_principal_id.trim().is_empty()
            || handoff.target_principal_kind.trim().is_empty()
            || handoff.target_principal_id.trim().is_empty()
            || handoff.handoff_session_id.trim().is_empty()
            || !actor_pair_is_valid(
                &handoff.accepted_by_principal_kind,
                &handoff.accepted_by_principal_id,
            )
            || !actor_pair_is_valid(
                &handoff.resolved_by_principal_kind,
                &handoff.resolved_by_principal_id,
            )
            || !actor_pair_is_valid(
                &handoff.closed_by_principal_kind,
                &handoff.closed_by_principal_id,
            )
        {
            return Err(ContractError::Invalid(
                "normalized conversation handoff is invalid".into(),
            ));
        }
    } else if conversation.conversation_type == "agent_handoff" {
        return Err(ContractError::Invalid(
            "agent handoff conversation requires normalized handoff state".into(),
        ));
    }
    if let Some(assignments) = commit.agent_assignments.as_ref()
        && (assignments.tenant_id.to_string() != conversation.tenant_id
            || assignments.organization_id.to_string() != conversation.organization_id
            || assignments.conversation_id != conversation.conversation_id
            || !commit
                .envelopes
                .iter()
                .any(|envelope| envelope.event_id == assignments.source_event_id))
        {
            return Err(ContractError::Invalid(
                "normalized conversation agent assignment scope is invalid".into(),
            ));
        }
    Ok(())
}

fn persist_normalized_conversation_commit_txn(
    pool: &PostgresJournalPool,
    prefix: &str,
    commit: &NormalizedConversationCommit,
    id_generator: &dyn IdGenerator,
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "persist_normalized_conversation_commit")?;
    let mut txn = client.transaction().map_err(|error| {
        postgres_unavailable_db("persist_normalized_conversation_commit begin", error)
    })?;
    let normalized_state_applied = upsert_normalized_conversation_in_transaction(&mut txn, commit)?;
    if normalized_state_applied
        && let Some(assignments) = commit.agent_assignments.as_ref() {
            replace_conversation_agents_in_transaction(&mut txn, assignments, id_generator)?;
        }
    let mut positions = Vec::with_capacity(commit.envelopes.len());
    for envelope in &commit.envelopes {
        positions.push(
            append_journal_in_transaction(&mut txn, prefix, envelope)?.into_commit_position()?,
        );
    }
    for outbox in &commit.outboxes {
        ensure_conversation_outbox_in_transaction(&mut txn, outbox)?;
    }
    txn.commit().map_err(|error| {
        postgres_unavailable_db("persist_normalized_conversation_commit commit", error)
    })?;
    Ok(positions)
}

fn normalized_conversation_commit_fingerprint(
    commit: &NormalizedConversationCommit,
) -> Result<String, ContractError> {
    // Outbox transport ids and retry timestamps are excluded. Their logical
    // event identity and payload are already validated against `envelopes`.
    let canonical_business_commit = (
        commit.expected_commit_seq,
        &commit.conversation,
        &commit.policy,
        &commit.business_binding,
        &commit.handoff,
        &commit.members,
        &commit.read_cursors,
        &commit.agent_assignments,
        &commit.envelopes,
    );
    let payload = serde_json::to_vec(&canonical_business_commit).map_err(|error| {
        ContractError::Invalid(format!(
            "normalized conversation commit fingerprint encode failed: {error}"
        ))
    })?;
    Ok(sdkwork_utils_rust::sha256_hash(payload.as_slice()))
}

fn upsert_normalized_conversation_in_transaction(
    txn: &mut Transaction<'_>,
    commit: &NormalizedConversationCommit,
) -> Result<bool, ContractError> {
    let conversation = &commit.conversation;
    let expected_commit_seq = commit
        .expected_commit_seq
        .map(|value| postgres_bigint_input(value, "expected conversation commit sequence"))
        .transpose()?;
    let commit_seq =
        postgres_bigint_input(conversation.commit_seq, "conversation commit sequence")?;
    let member_epoch =
        postgres_bigint_input(conversation.member_epoch, "conversation member epoch")?;
    let last_activity_at =
        postgres_timestamptz(&conversation.last_activity_at, "last_activity_at")?;
    let retention_until = conversation
        .retention_until
        .as_deref()
        .map(|value| postgres_timestamptz(value, "retention_until"))
        .transpose()?;
    let archived_at = conversation
        .archived_at
        .as_deref()
        .map(|value| postgres_timestamptz(value, "archived_at"))
        .transpose()?;
    let empty_payload_hash = sdkwork_utils_rust::sha256_hash(b"{}");
    let commit_fingerprint = normalized_conversation_commit_fingerprint(commit)?;
    let params: &[&(dyn postgres::types::ToSql + Sync)] = &[
        &conversation.tenant_id,
        &conversation.organization_id,
        &conversation.conversation_id,
        &conversation.conversation_type,
        &conversation.lifecycle_state,
        &commit_seq,
        &member_epoch,
        &last_activity_at,
        &empty_payload_hash,
        &retention_until,
        &archived_at,
        &conversation.archive_event_id,
        &commit_fingerprint,
        &expected_commit_seq,
    ];
    let applied = txn
        .query_opt(UPSERT_NORMALIZED_CONVERSATION_SQL, params)
        .map_err(|error| postgres_unavailable_db("upsert normalized conversation", error))?
        .is_some();
    if !applied {
        let replay_match = txn
            .query_opt(
                LOAD_NORMALIZED_CONVERSATION_REPLAY_MATCH_SQL,
                &[
                    &conversation.tenant_id,
                    &conversation.organization_id,
                    &conversation.conversation_id,
                    &conversation.conversation_type,
                    &conversation.lifecycle_state,
                    &commit_seq,
                    &member_epoch,
                    &last_activity_at,
                    &retention_until,
                    &archived_at,
                    &conversation.archive_event_id,
                    &commit_fingerprint,
                ],
            )
            .map_err(|error| {
                postgres_unavailable_db("load normalized conversation replay state", error)
            })?
            .is_some_and(|row| row.get::<_, bool>(0));
        if !replay_match {
            return Err(ContractError::Conflict(
                "normalized conversation commit sequence conflict".into(),
            ));
        }
        return Ok(false);
    }

    persist_normalized_conversation_capabilities(txn, commit, &last_activity_at)?;

    for member in &commit.members {
        let empty_payload_hash = sdkwork_utils_rust::sha256_hash(b"{}");
        let joined_at = postgres_timestamptz(&member.joined_at, "joined_at")?;
        let removed_at = member
            .removed_at
            .as_deref()
            .map(|value| postgres_timestamptz(value, "removed_at"))
            .transpose()?;
        let updated_at = removed_at.as_ref().unwrap_or(&joined_at);
        let attributes = postgres_jsonb_payload(&member.attributes_json)?;
        txn.execute(
            UPSERT_NORMALIZED_MEMBER_SQL,
            &[
                &member.tenant_id,
                &member.organization_id,
                &member.conversation_id,
                &member.principal_kind,
                &member.principal_id,
                &member.member_id,
                &member.membership_role,
                &member.membership_state,
                &member.invited_by,
                &joined_at,
                &removed_at,
                &attributes,
                &empty_payload_hash,
                updated_at,
            ],
        )
        .map_err(|error| postgres_unavailable_db("upsert normalized conversation member", error))?;
    }

    for cursor in &commit.read_cursors {
        let empty_payload_hash = sdkwork_utils_rust::sha256_hash(b"{}");
        let member_id = cursor.member_id;
        let read_seq = postgres_bigint_input(cursor.read_seq, "read sequence")?;
        let updated_at = postgres_timestamptz(&cursor.updated_at, "updated_at")?;
        txn.execute(
            UPSERT_NORMALIZED_READ_CURSOR_SQL,
            &[
                &cursor.tenant_id,
                &cursor.organization_id,
                &cursor.conversation_id,
                &member_id,
                &cursor.device_id,
                &cursor.principal_kind,
                &cursor.principal_id,
                &read_seq,
                &cursor.last_read_message_id,
                &empty_payload_hash,
                &updated_at,
            ],
        )
        .map_err(|error| postgres_unavailable_db("upsert normalized read cursor", error))?;
    }
    Ok(true)
}

fn persist_normalized_conversation_capabilities(
    txn: &mut Transaction<'_>,
    commit: &NormalizedConversationCommit,
    updated_at: &DateTime<Utc>,
) -> Result<(), ContractError> {
    let conversation = &commit.conversation;
    let scope_params: &[&(dyn postgres::types::ToSql + Sync)] = &[
        &conversation.tenant_id,
        &conversation.organization_id,
        &conversation.conversation_id,
    ];

    if let Some(policy) = commit.policy.as_ref() {
        let policy_epoch = postgres_bigint_input(policy.policy_epoch, "conversation policy epoch")?;
        txn.execute(
            UPSERT_NORMALIZED_POLICY_SQL,
            &[
                &policy.tenant_id,
                &policy.organization_id,
                &policy.conversation_id,
                &policy_epoch,
                &policy.policy_version,
                &policy.capability_flags,
                &policy.history_visibility,
                &policy.retention_policy_ref,
                &policy.max_members,
                updated_at,
            ],
        )
        .map_err(|error| postgres_unavailable_db("upsert normalized conversation policy", error))?;
    } else {
        txn.execute(DELETE_NORMALIZED_POLICY_SQL, scope_params)
            .map_err(|error| {
                postgres_unavailable_db("delete normalized conversation policy", error)
            })?;
    }

    if let Some(binding) = commit.business_binding.as_ref() {
        txn.execute(
            UPSERT_NORMALIZED_BUSINESS_BINDING_SQL,
            &[
                &binding.tenant_id,
                &binding.organization_id,
                &binding.conversation_id,
                &binding.business_type,
                &binding.business_id,
                updated_at,
            ],
        )
        .map_err(|error| {
            postgres_unavailable_db("upsert normalized conversation business binding", error)
        })?;
    } else {
        txn.execute(DELETE_NORMALIZED_BUSINESS_BINDING_SQL, scope_params)
            .map_err(|error| {
                postgres_unavailable_db("delete normalized conversation business binding", error)
            })?;
    }

    if let Some(handoff) = commit.handoff.as_ref() {
        let handoff_status_epoch = postgres_bigint_input(
            handoff.handoff_status_epoch,
            "conversation handoff status epoch",
        )?;
        let accepted_at = handoff
            .accepted_at
            .as_deref()
            .map(|value| postgres_timestamptz(value, "handoff accepted_at"))
            .transpose()?;
        let resolved_at = handoff
            .resolved_at
            .as_deref()
            .map(|value| postgres_timestamptz(value, "handoff resolved_at"))
            .transpose()?;
        let closed_at = handoff
            .closed_at
            .as_deref()
            .map(|value| postgres_timestamptz(value, "handoff closed_at"))
            .transpose()?;
        txn.execute(
            UPSERT_NORMALIZED_HANDOFF_SQL,
            &[
                &handoff.tenant_id,
                &handoff.organization_id,
                &handoff.conversation_id,
                &handoff_status_epoch,
                &handoff.status,
                &handoff.source_principal_kind,
                &handoff.source_principal_id,
                &handoff.target_principal_kind,
                &handoff.target_principal_id,
                &handoff.handoff_session_id,
                &handoff.handoff_reason,
                &accepted_at,
                &handoff.accepted_by_principal_kind,
                &handoff.accepted_by_principal_id,
                &resolved_at,
                &handoff.resolved_by_principal_kind,
                &handoff.resolved_by_principal_id,
                &closed_at,
                &handoff.closed_by_principal_kind,
                &handoff.closed_by_principal_id,
                updated_at,
            ],
        )
        .map_err(|error| {
            postgres_unavailable_db("upsert normalized conversation handoff", error)
        })?;
    } else {
        txn.execute(DELETE_NORMALIZED_HANDOFF_SQL, scope_params)
            .map_err(|error| {
                postgres_unavailable_db("delete normalized conversation handoff", error)
            })?;
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

fn update_conversation_after_message_in_transaction(
    txn: &mut Transaction<'_>,
    message: &StoredMessageRecord,
    envelopes: &[CommitEnvelope],
) -> Result<(), ContractError> {
    let message_seq = postgres_bigint_input(message.message_seq, "message sequence")?;
    let payload_json = postgres_jsonb_payload(message.payload_json.as_str())?;
    let created_at = postgres_timestamptz(message.created_at.as_str(), "created_at")?;
    let commit_seq = envelopes
        .iter()
        .map(|envelope| envelope.ordering_seq)
        .max()
        .ok_or_else(|| ContractError::Invalid("message journal batch is empty".into()))?;
    let commit_seq = postgres_bigint_input(commit_seq, "conversation commit sequence")?;
    let affected = txn
        .execute(
            UPDATE_CONVERSATION_AFTER_MESSAGE_SQL,
            &[
                &message.tenant_id,
                &message.organization_id,
                &message.conversation_id,
                &message.message_id,
                &message_seq,
                &message.sender_principal_kind,
                &message.sender_principal_id,
                &payload_json,
                &created_at,
                &commit_seq,
            ],
        )
        .map_err(|error| {
            postgres_unavailable_db("update normalized conversation message state", error)
        })?;
    if affected != 1 {
        return Err(ContractError::Conflict(
            "normalized conversation is missing or message sequence is stale".into(),
        ));
    }
    Ok(())
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

    fn normalized_conversation_commit_fixture() -> NormalizedConversationCommit {
        let (envelope, outbox) = conversation_event_fixture();
        NormalizedConversationCommit {
            expected_commit_seq: Some(1),
            conversation: im_platform_contracts::NormalizedConversationRecord {
                tenant_id: envelope.tenant_id.clone(),
                organization_id: envelope.organization_id.clone(),
                conversation_id: envelope.aggregate_id.clone(),
                conversation_type: "group".into(),
                lifecycle_state: "active".into(),
                archived_at: None,
                archive_event_id: None,
                commit_seq: envelope.ordering_seq,
                member_epoch: 1,
                last_activity_at: envelope.committed_at.clone(),
                retention_until: None,
            },
            policy: Some(im_platform_contracts::NormalizedConversationPolicyRecord {
                tenant_id: envelope.tenant_id.clone(),
                organization_id: envelope.organization_id.clone(),
                conversation_id: envelope.aggregate_id.clone(),
                policy_epoch: 1,
                policy_version: "group.v1".into(),
                capability_flags: Some(vec!["message.post".into()]),
                history_visibility: "joined".into(),
                retention_policy_ref: "tenant.standard".into(),
                max_members: Some(200),
            }),
            business_binding: Some(
                im_platform_contracts::NormalizedConversationBusinessBindingRecord {
                    tenant_id: envelope.tenant_id.clone(),
                    organization_id: envelope.organization_id.clone(),
                    conversation_id: envelope.aggregate_id.clone(),
                    business_type: "workspace".into(),
                    business_id: "workspace-42".into(),
                },
            ),
            handoff: None,
            members: vec![im_platform_contracts::ConversationMemberRecord {
                tenant_id: envelope.tenant_id.clone(),
                organization_id: envelope.organization_id.clone(),
                conversation_id: envelope.aggregate_id.clone(),
                principal_kind: "user".into(),
                principal_id: "user-1".into(),
                member_id: 1001,
                membership_role: "owner".into(),
                membership_state: "joined".into(),
                invited_by: None,
                joined_at: envelope.committed_at.clone(),
                removed_at: None,
                attributes_json: "{}".into(),
            }],
            read_cursors: vec![im_platform_contracts::ReadCursorRecord {
                tenant_id: envelope.tenant_id.clone(),
                organization_id: envelope.organization_id.clone(),
                conversation_id: envelope.aggregate_id.clone(),
                member_id: 1001,
                device_id: String::new(),
                principal_kind: "user".into(),
                principal_id: "user-1".into(),
                read_seq: 0,
                last_read_message_id: None,
                updated_at: envelope.committed_at.clone(),
            }],
            agent_assignments: None,
            envelopes: vec![envelope],
            outboxes: vec![outbox],
        }
    }

    fn message_mutation_fixture() -> (CommitEnvelope, StoredMessageMutation, OutboxEventRecord) {
        let tenant_id = "tenant-message-mutation";
        let organization_id = "0";
        let conversation_id = "group-message-mutation";
        let message_id = "9001";
        let event_id = "evt_message_reaction_added";
        let event_type = "message.reaction_added";
        let occurred_at = "2026-07-23T10:00:00.000Z";
        let payload = json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": 7,
            "reactionKey": "thumbs_up",
            "reactedBy": {
                "id": "agent-1",
                "kind": "agent"
            },
            "reactedAt": occurred_at
        })
        .to_string();
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
            ordering_seq: 8,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: "agent-1".into(),
                actor_kind: "agent".into(),
                actor_session_id: None,
            },
            occurred_at: occurred_at.into(),
            committed_at: occurred_at.into(),
            payload_schema: Some("message.reaction_added.v1".into()),
            payload: payload.clone(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        let mutation = StoredMessageMutation::ReactionAdded {
            target: StoredMessageMutationTarget {
                tenant_id: tenant_id.into(),
                organization_id: organization_id.into(),
                conversation_id: conversation_id.into(),
                message_id: message_id.into(),
                message_seq: 7,
            },
            reaction: im_platform_contracts::StoredMessageReactionRecord {
                actor_principal_kind: "agent".into(),
                actor_principal_id: "agent-1".into(),
                reaction_key: "thumbs_up".into(),
                reacted_at: occurred_at.into(),
            },
        };
        let outbox = OutboxEventRecord {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            outbox_id: "outbox-message-reaction-added".into(),
            aggregate_type: CONVERSATION_OUTBOX_AGGREGATE_TYPE.into(),
            aggregate_id: conversation_id.into(),
            event_id: format!("conversation:{event_type}:{event_id}"),
            event_type: event_type.into(),
            payload_hash: sdkwork_utils_rust::sha256_hash(payload.as_bytes()),
            payload_json: payload,
            publish_status: OutboxPublishStatus::Pending,
            attempt_count: 0,
            available_at: occurred_at.into(),
            published_at: None,
            created_at: occurred_at.into(),
            updated_at: occurred_at.into(),
        };
        (envelope, mutation, outbox)
    }

    #[test]
    fn message_mutation_validation_accepts_one_canonical_commit() {
        let (envelope, mutation, outbox) = message_mutation_fixture();

        validate_message_mutation_commit(&envelope, &mutation, &outbox)
            .expect("canonical message mutation commit should validate");
    }

    #[test]
    fn message_mutation_validation_rejects_scope_event_and_message_drift() {
        let (envelope, mutation, outbox) = message_mutation_fixture();

        let mut invalid_scope = envelope.clone();
        invalid_scope.scope_id = "different-conversation".into();
        assert!(validate_message_mutation_commit(&invalid_scope, &mutation, &outbox).is_err());

        let mut invalid_ordering_key = envelope.clone();
        invalid_ordering_key.ordering_key = "different-ordering-key".into();
        assert!(
            validate_message_mutation_commit(&invalid_ordering_key, &mutation, &outbox).is_err()
        );

        let mut invalid_event_type = envelope.clone();
        invalid_event_type.event_type = "message.reaction_removed".into();
        assert!(validate_message_mutation_commit(&invalid_event_type, &mutation, &outbox).is_err());

        let mut invalid_target = mutation.clone();
        if let StoredMessageMutation::ReactionAdded { target, .. } = &mut invalid_target {
            target.message_seq = 9;
        }
        assert!(validate_message_mutation_commit(&envelope, &invalid_target, &outbox).is_err());

        let mut invalid_payload = envelope.clone();
        let mut payload: serde_json::Value =
            serde_json::from_str(invalid_payload.payload.as_str()).expect("fixture JSON");
        payload["messageId"] = json!("9002");
        invalid_payload.payload = payload.to_string();
        assert!(validate_message_mutation_commit(&invalid_payload, &mutation, &outbox).is_err());
    }

    #[test]
    fn message_mutation_validation_rejects_actor_and_outbox_drift() {
        let (envelope, mutation, outbox) = message_mutation_fixture();

        let mut invalid_actor_kind = envelope.clone();
        invalid_actor_kind.actor.actor_kind = "user".into();
        assert!(validate_message_mutation_commit(&invalid_actor_kind, &mutation, &outbox).is_err());

        let mut invalid_actor_id = envelope.clone();
        invalid_actor_id.actor.actor_id = "agent-2".into();
        assert!(validate_message_mutation_commit(&invalid_actor_id, &mutation, &outbox).is_err());

        let mut invalid_mutation_actor = mutation.clone();
        if let StoredMessageMutation::ReactionAdded { reaction, .. } = &mut invalid_mutation_actor {
            reaction.actor_principal_id = "agent-2".into();
        }
        assert!(
            validate_message_mutation_commit(&envelope, &invalid_mutation_actor, &outbox).is_err()
        );

        let mut invalid_event_id = outbox.clone();
        invalid_event_id.event_id = "conversation:message.reaction_added:unrelated".into();
        assert!(validate_message_mutation_commit(&envelope, &mutation, &invalid_event_id).is_err());

        let mut invalid_lifecycle = outbox.clone();
        invalid_lifecycle.publish_status = OutboxPublishStatus::Published;
        invalid_lifecycle.attempt_count = 1;
        invalid_lifecycle.published_at = Some("2026-07-23T10:01:00.000Z".into());
        assert!(
            validate_message_mutation_commit(&envelope, &mutation, &invalid_lifecycle).is_err()
        );

        let mut invalid_payload = outbox;
        invalid_payload.payload_json = json!({
            "conversationId": "different-conversation",
            "messageId": "9001",
            "messageSeq": 7
        })
        .to_string();
        invalid_payload.payload_hash =
            sdkwork_utils_rust::sha256_hash(invalid_payload.payload_json.as_bytes());
        assert!(validate_message_mutation_commit(&envelope, &mutation, &invalid_payload).is_err());
    }

    #[test]
    fn message_interaction_sql_uses_typed_principal_columns_only() {
        for sql in [
            LOAD_MESSAGE_REACTION_SQL,
            INSERT_MESSAGE_REACTION_SQL,
            DELETE_MESSAGE_REACTION_SQL,
        ] {
            let sql = sql.to_ascii_lowercase();
            assert!(sql.contains("actor_principal_kind"));
            assert!(sql.contains("actor_principal_id"));
            assert!(!sql.contains("user_id"));
        }
        let insert_pin = INSERT_MESSAGE_PIN_SQL.to_ascii_lowercase();
        assert!(insert_pin.contains("pinned_by_principal_kind"));
        assert!(insert_pin.contains("pinned_by_principal_id"));
        for sql in [
            LOAD_MESSAGE_PIN_SQL,
            INSERT_MESSAGE_PIN_SQL,
            DELETE_MESSAGE_PIN_SQL,
        ] {
            let sql = sql.to_ascii_lowercase();
            assert!(!sql.contains("pinned_by_user_id"));
        }
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
    fn normalized_commit_fingerprint_covers_all_typed_current_state() {
        let commit = normalized_conversation_commit_fixture();
        validate_normalized_conversation_commit(&commit)
            .expect("normalized fixture should validate");
        let expected = normalized_conversation_commit_fingerprint(&commit)
            .expect("normalized fixture should hash");

        let mut changed_policy = commit.clone();
        changed_policy
            .policy
            .as_mut()
            .expect("fixture policy")
            .history_visibility = "shared".into();
        assert_ne!(
            normalized_conversation_commit_fingerprint(&changed_policy)
                .expect("changed policy should hash"),
            expected
        );

        let mut changed_binding = commit.clone();
        changed_binding
            .business_binding
            .as_mut()
            .expect("fixture binding")
            .business_id = "workspace-43".into();
        assert_ne!(
            normalized_conversation_commit_fingerprint(&changed_binding)
                .expect("changed binding should hash"),
            expected
        );

        let mut changed_member = commit.clone();
        changed_member.members[0].membership_role = "admin".into();
        assert_ne!(
            normalized_conversation_commit_fingerprint(&changed_member)
                .expect("changed member should hash"),
            expected
        );

        let mut changed_cursor = commit;
        changed_cursor.read_cursors[0].read_seq = 1;
        assert_ne!(
            normalized_conversation_commit_fingerprint(&changed_cursor)
                .expect("changed cursor should hash"),
            expected
        );
    }

    #[test]
    fn normalized_creation_commit_starts_at_zero_without_an_existing_version() {
        let mut commit = normalized_conversation_commit_fixture();
        commit.expected_commit_seq = None;
        commit.conversation.commit_seq = 0;
        commit.conversation.member_epoch = 0;
        commit.policy = None;
        commit.business_binding = None;
        commit.members.clear();
        commit.read_cursors.clear();
        commit.envelopes[0].ordering_seq = 0;

        validate_normalized_conversation_commit(&commit)
            .expect("a new normalized aggregate must be allowed to start at sequence zero");
    }

    #[test]
    fn normalized_conversation_cas_uses_dedicated_complete_fingerprint() {
        let upsert = UPSERT_NORMALIZED_CONVERSATION_SQL.to_ascii_lowercase();
        assert!(upsert.contains("commit_fingerprint"));
        assert!(
            upsert.contains("where $14::bigint is not null and im_conversations.commit_seq = $14")
        );

        let replay = LOAD_NORMALIZED_CONVERSATION_REPLAY_MATCH_SQL.to_ascii_lowercase();
        assert!(replay.contains("commit_fingerprint = $12"));
        assert!(replay.contains("for update"));
        assert!(!replay.contains("payload_json"));
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
