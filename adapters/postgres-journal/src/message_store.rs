//! PostgreSQL implementation of [`MessageStore`] trait.
//!
//! Writes message truth to `im_conversation_messages` table with Snowflake IDs.
//!
//! ## Message Sequence Allocation
//!
//! Message sequences (`message_seq`) are allocated using Snowflake IDs generated
//! locally by the [`RuntimeSnowflakeIdGenerator`], eliminating the database
//! round-trip hotspot that would occur with row-level sequence counters.
//!
//! The Snowflake ID provides:
//! - **Uniqueness**: Globally unique across all nodes
//! - **Monotonicity**: Roughly ordered by timestamp (within same node)
//! - **Performance**: No database round-trip required
//!
//! For ordering within a conversation, clients use `message_seq` which is
//! stored in the database but generated locally.

use im_platform_contracts::{ContractError, IdGenerator, MessageStore, MessageWindow, StoredMessageRecord};
use std::sync::Arc;

use crate::{
    now_rfc3339, postgres_jsonb_payload, postgres_pool_client, postgres_timestamptz,
    postgres_unavailable, run_postgres_io, PostgresJournalPool,
};

/// PostgreSQL implementation of [`MessageStore`].
///
/// Uses Snowflake ID generator for message sequence allocation,
/// avoiding database round-trip hotspots in high-throughput scenarios.
#[derive(Clone)]
pub struct PostgresMessageStore {
    pool: PostgresJournalPool,
    /// Snowflake ID generator for message sequence allocation.
    /// When `None`, falls back to database sequence (legacy mode).
    id_generator: Option<Arc<dyn IdGenerator>>,
}

impl PostgresMessageStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self {
            pool,
            id_generator: None,
        }
    }

    /// Create a message store with Snowflake ID generation for sequences.
    ///
    /// This is the recommended constructor for production deployments,
    /// eliminating the database round-trip hotspot for sequence allocation.
    pub fn with_id_generator(pool: PostgresJournalPool, id_generator: Arc<dyn IdGenerator>) -> Self {
        Self {
            pool,
            id_generator: Some(id_generator),
        }
    }
}

// SQL constants

const ALLOCATE_SEQ_SQL: &str = r#"
insert into im_conversation_seq_counters (tenant_id, organization_id, conversation_id, next_seq, updated_at)
values ($1, $2, $3, 1, $4)
on conflict (tenant_id, organization_id, conversation_id) do update
set next_seq = im_conversation_seq_counters.next_seq + 1, updated_at = $4
returning next_seq
"#;

const INSERT_MESSAGE_SQL: &str = r#"
insert into im_conversation_messages (
    tenant_id, organization_id, conversation_id, message_id, message_seq,
    sender_principal_kind, sender_principal_id, sender_device_id, client_msg_id,
    message_type, payload_json, payload_hash, created_at, updated_at, retention_until
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::jsonb, $12, $13, $14, $15)
"#;

const READ_WINDOW_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, message_id, message_seq,
    sender_principal_kind, sender_principal_id, sender_device_id, client_msg_id,
    message_type, payload_json::text, payload_hash, created_at, updated_at, deleted_at,
    retention_until
from im_conversation_messages
where tenant_id = $1 and organization_id = $2 and conversation_id = $3 and message_seq > $4
  and (retention_until is null or retention_until > now())
order by message_seq asc
limit $5
"#;

const READ_BY_ID_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, message_id, message_seq,
    sender_principal_kind, sender_principal_id, sender_device_id, client_msg_id,
    message_type, payload_json::text, payload_hash, created_at, updated_at, deleted_at,
    retention_until
from im_conversation_messages
where tenant_id = $1 and message_id = $2
  and (retention_until is null or retention_until > now())
"#;

const READ_BY_CLIENT_ID_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, message_id, message_seq,
    sender_principal_kind, sender_principal_id, sender_device_id, client_msg_id,
    message_type, payload_json::text, payload_hash, created_at, updated_at, deleted_at,
    retention_until
from im_conversation_messages
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
    and sender_principal_kind = $4 and sender_principal_id = $5 and client_msg_id = $6
  and (retention_until is null or retention_until > now())
"#;

const READ_HIGH_WATERMARK_SQL: &str = r#"
select coalesce(max(message_seq), 0) as high_watermark
from im_conversation_messages
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
"#;

impl MessageStore for PostgresMessageStore {
    /// Allocate a message sequence number.
    ///
    /// Uses Snowflake ID generator when available (production mode),
    /// falling back to database sequence counter (legacy mode).
    ///
    /// # Performance
    ///
    /// With Snowflake ID generation, this is a local operation with no
    /// database round-trip, enabling high-throughput message sending.
    fn allocate_message_seq(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _conversation_id: &str,
    ) -> Result<u64, ContractError> {
        // Use Snowflake ID generator when available (eliminates DB hotspot)
        if let Some(generator) = &self.id_generator {
            let id = generator.next_id()?;
            // Snowflake IDs are i64, convert to u64 for message_seq
            // The ID is positive and fits within u64 range
            return Ok(id as u64);
        }

        // Legacy fallback: database sequence counter
        // CRITICAL WARNING: This creates a row-level lock hotspot in high-throughput scenarios.
        // Production deployments MUST use Snowflake ID generation via PostgresMessageStore::with_id_generator()
        tracing::warn!(
            "CRITICAL: Using legacy database sequence counter for message_seq allocation. \
             This creates a performance hotspot. Configure Snowflake ID generator for production."
        );

        let pool = self.pool.clone();
        let tenant_id = _tenant_id.to_owned();
        let organization_id = _organization_id.to_owned();
        let conversation_id = _conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "allocate_seq")?;
            let now = now_rfc3339();
            let row = client
                .query_one(
                    ALLOCATE_SEQ_SQL,
                    &[&tenant_id, &organization_id, &conversation_id, &now],
                )
                .map_err(|error| postgres_unavailable("allocate_seq", error))?;
            let seq: i64 = row.get(0);
            Ok(seq as u64)
        })
    }

    fn insert_message(&self, message: StoredMessageRecord) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "insert_message")?;
            let message_seq_i64 = message.message_seq as i64;
            let payload_json = postgres_jsonb_payload(message.payload_json.as_str())?;
            // Convert RFC3339 timestamp strings to `DateTime<Utc>` so they
            // serialize as `TIMESTAMPTZ` (matching the column type). Passing
            // raw `String`s produces `VARCHAR`-typed parameters that fail
            // serialization against `TIMESTAMPTZ` columns.
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
            let result = client.execute(INSERT_MESSAGE_SQL, params);
            match result {
                Ok(_) => Ok(()),
                Err(error) => {
                    if error.code() == Some(&postgres::error::SqlState::UNIQUE_VIOLATION) {
                        Err(ContractError::Conflict("message already exists".into()))
                    } else {
                        Err(postgres_unavailable("insert_message", error))
                    }
                }
            }
        })
    }

    fn read_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<MessageWindow, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let after_seq_i64 = after_seq as i64;
        let limit_i32 = limit as i32;
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_window")?;
            let rows = client
                .query(
                    READ_WINDOW_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &after_seq_i64,
                        &limit_i32,
                    ],
                )
                .map_err(|error| postgres_unavailable("read_window", error))?;
            let items: Vec<StoredMessageRecord> = rows
                .iter()
                .map(stored_message_from_row)
                .collect();
            let high_watermark = items.last().map(|m| m.message_seq).unwrap_or(0);
            let has_more = items.len() == limit;
            let next_after_seq = items.last().map(|m| m.message_seq);
            Ok(MessageWindow {
                items,
                high_watermark,
                next_after_seq,
                has_more,
            })
        })
    }

    fn read_message_by_id(
        &self,
        tenant_id: &str,
        message_id: i64,
    ) -> Result<Option<StoredMessageRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_by_id")?;
            let row = client
                .query_opt(READ_BY_ID_SQL, &[&tenant_id, &message_id])
                .map_err(|error| postgres_unavailable("read_by_id", error))?;
            Ok(row.map(|row| stored_message_from_row(&row)))
        })
    }

    fn read_message_by_client_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        sender_principal_kind: &str,
        sender_principal_id: &str,
        client_msg_id: &str,
    ) -> Result<Option<StoredMessageRecord>, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let sender_principal_kind = sender_principal_kind.to_owned();
        let sender_principal_id = sender_principal_id.to_owned();
        let client_msg_id = client_msg_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_by_client_id")?;
            let row = client
                .query_opt(
                    READ_BY_CLIENT_ID_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &sender_principal_kind,
                        &sender_principal_id,
                        &client_msg_id,
                    ],
                )
                .map_err(|error| postgres_unavailable("read_by_client_id", error))?;
            Ok(row.map(|row| stored_message_from_row(&row)))
        })
    }

    fn read_high_watermark(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_high_watermark")?;
            let row = client
                .query_one(
                    READ_HIGH_WATERMARK_SQL,
                    &[&tenant_id, &organization_id, &conversation_id],
                )
                .map_err(|error| postgres_unavailable("read_high_watermark", error))?;
            let seq: i64 = row.get(0);
            Ok(seq as u64)
        })
    }
}

fn stored_message_from_row(row: &postgres::Row) -> StoredMessageRecord {
    StoredMessageRecord {
        tenant_id: row.get(0),
        organization_id: row.get(1),
        conversation_id: row.get(2),
        message_id: row.get::<_, i64>(3),
        message_seq: row.get::<_, i64>(4) as u64,
        sender_principal_kind: row.get(5),
        sender_principal_id: row.get(6),
        sender_device_id: row.get(7),
        client_msg_id: row.get(8),
        message_type: row.get(9),
        payload_json: row.get(10),
        payload_hash: row.get(11),
        created_at: row.get(12),
        updated_at: row.get(13),
        deleted_at: row.get(14),
        retention_until: retention_until_string_from_row(row),
    }
}

fn retention_until_string_from_row(row: &postgres::Row) -> Option<String> {
    row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(15)
        .map(|value| value.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}
