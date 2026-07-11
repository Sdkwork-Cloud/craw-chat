//! PostgreSQL implementation of [`MessageStore`] trait.
//!
//! Writes message truth to `im_conversation_messages` table.
//!
//! ## Message Sequence Allocation
//!
//! Per-conversation `message_seq` values are allocated through
//! [`ConversationSeqAllocator`] (Redis batch prefetch or Postgres counter).
//! Snowflake IDs are reserved for `message_id` / `event_id` only.

use im_platform_contracts::{ContractError, MessageStore, MessageWindow, StoredMessageRecord};

use crate::{
    PostgresJournalPool, now_rfc3339, postgres_jsonb_payload, postgres_pool_client,
    postgres_timestamptz, postgres_unavailable, run_postgres_io,
};

const MESSAGE_WINDOW_PAGE_SIZE_MAX: usize = 200;

/// PostgreSQL implementation of [`MessageStore`].
#[derive(Clone)]
pub struct PostgresMessageStore {
    pool: PostgresJournalPool,
}

impl PostgresMessageStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
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

const READ_HISTORY_WINDOW_SQL: &str = r#"
select tenant_id, organization_id, conversation_id, message_id, message_seq,
    sender_principal_kind, sender_principal_id, sender_device_id, client_msg_id,
    message_type, payload_json::text, payload_hash, created_at, updated_at, deleted_at,
    retention_until
from im_conversation_messages
where tenant_id = $1 and organization_id = $2 and conversation_id = $3
  and ($4::bigint is null or message_seq < $4)
  and (retention_until is null or retention_until > now())
order by message_seq desc
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
    /// Allocate the next per-conversation message sequence via Postgres counter.
    ///
    /// Production deployments SHOULD prefer [`ConversationSeqAllocator`] (Redis
    /// batch prefetch) wired through conversation runtime bootstrap.
    fn allocate_message_seq(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _conversation_id: &str,
    ) -> Result<u64, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = _tenant_id.to_owned();
        let organization_id = _organization_id.to_owned();
        let conversation_id = _conversation_id.to_owned();
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "allocate_seq")?;
            let now = postgres_timestamptz(&now_rfc3339(), "now")?;
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

    fn read_history_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        before_seq: Option<u64>,
        limit: usize,
    ) -> Result<MessageWindow, ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        let before_seq_i64 = before_seq
            .map(i64::try_from)
            .transpose()
            .map_err(|_| ContractError::Invalid("message history cursor is out of range".into()))?;
        let page_size = normalize_message_window_page_size(limit);
        let fetch_limit_i64 = message_window_fetch_limit(page_size);
        run_postgres_io(move || {
            let mut client = postgres_pool_client(&pool, "read_history_window")?;
            let rows = client
                .query(
                    READ_HISTORY_WINDOW_SQL,
                    &[
                        &tenant_id,
                        &organization_id,
                        &conversation_id,
                        &before_seq_i64,
                        &fetch_limit_i64,
                    ],
                )
                .map_err(|error| postgres_unavailable("read_history_window", error))?;
            let records: Vec<StoredMessageRecord> =
                rows.iter().map(stored_message_from_row).collect();
            let row = client
                .query_one(
                    READ_HIGH_WATERMARK_SQL,
                    &[&tenant_id, &organization_id, &conversation_id],
                )
                .map_err(|error| {
                    postgres_unavailable("read_history_window_high_watermark", error)
                })?;
            let high_watermark: i64 = row.get(0);
            Ok(message_history_window_from_desc_fetch_ahead(
                records,
                page_size,
                high_watermark as u64,
            ))
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
        created_at: timestamptz_string_from_row(row, 12),
        updated_at: timestamptz_string_from_row(row, 13),
        deleted_at: optional_timestamptz_string_from_row(row, 14),
        retention_until: optional_timestamptz_string_from_row(row, 15),
    }
}

fn timestamptz_string_from_row(row: &postgres::Row, column: usize) -> String {
    let value: chrono::DateTime<chrono::Utc> = row.get(column);
    value.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

fn optional_timestamptz_string_from_row(row: &postgres::Row, column: usize) -> Option<String> {
    row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(column)
        .map(|value| value.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

fn normalize_message_window_page_size(page_size: usize) -> usize {
    page_size.clamp(1, MESSAGE_WINDOW_PAGE_SIZE_MAX)
}

fn message_window_fetch_limit(page_size: usize) -> i64 {
    page_size.saturating_add(1) as i64
}

fn message_history_window_from_desc_fetch_ahead(
    mut items: Vec<StoredMessageRecord>,
    page_size: usize,
    high_watermark: u64,
) -> MessageWindow {
    let has_more = items.len() > page_size;
    if has_more {
        items.truncate(page_size);
    }
    let next_before_seq = has_more
        .then(|| items.last().map(|message| message.message_seq))
        .flatten();
    items.reverse();
    MessageWindow {
        items,
        high_watermark,
        next_before_seq,
        has_more,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored_message(message_seq: u64) -> StoredMessageRecord {
        StoredMessageRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "c_page".into(),
            message_id: message_seq as i64,
            message_seq,
            sender_principal_kind: "user".into(),
            sender_principal_id: "1".into(),
            sender_device_id: None,
            client_msg_id: None,
            message_type: "standard".into(),
            payload_json: "{}".into(),
            payload_hash: format!("hash_{message_seq}"),
            created_at: "2026-07-09T00:00:00.000Z".into(),
            updated_at: "2026-07-09T00:00:00.000Z".into(),
            deleted_at: None,
            retention_until: None,
        }
    }

    #[test]
    fn message_window_fetch_limit_reads_page_size_plus_one() {
        assert_eq!(message_window_fetch_limit(1), 2);
        assert_eq!(message_window_fetch_limit(20), 21);
        assert_eq!(message_window_fetch_limit(200), 201);
    }

    #[test]
    fn normalize_message_window_page_size_applies_sdkwork_bounds() {
        assert_eq!(normalize_message_window_page_size(0), 1);
        assert_eq!(normalize_message_window_page_size(20), 20);
        assert_eq!(normalize_message_window_page_size(200), 200);
        assert_eq!(normalize_message_window_page_size(201), 200);
        assert_eq!(normalize_message_window_page_size(1000), 200);
    }

    #[test]
    fn message_window_from_fetch_ahead_truncates_extra_row_and_returns_cursor() {
        let window = message_history_window_from_desc_fetch_ahead(
            vec![stored_message(3), stored_message(2), stored_message(1)],
            2,
            3,
        );

        assert_eq!(window.items.len(), 2);
        assert_eq!(window.items[0].message_seq, 2);
        assert_eq!(window.items[1].message_seq, 3);
        assert_eq!(window.high_watermark, 3);
        assert_eq!(window.next_before_seq, Some(2));
        assert!(window.has_more);
    }

    #[test]
    fn message_window_from_fetch_ahead_omits_cursor_on_final_full_page() {
        let window = message_history_window_from_desc_fetch_ahead(
            vec![stored_message(4), stored_message(3)],
            2,
            4,
        );

        assert_eq!(window.items.len(), 2);
        assert_eq!(window.high_watermark, 4);
        assert_eq!(window.next_before_seq, None);
        assert!(!window.has_more);
    }

    #[test]
    fn message_window_from_fetch_ahead_preserves_store_high_watermark_beyond_fetch_ahead_row() {
        let window = message_history_window_from_desc_fetch_ahead(
            vec![stored_message(2), stored_message(1)],
            1,
            4,
        );

        assert_eq!(window.items.len(), 1);
        assert_eq!(window.items[0].message_seq, 2);
        assert_eq!(window.high_watermark, 4);
        assert_eq!(window.next_before_seq, Some(2));
        assert!(window.has_more);
    }

    #[test]
    fn backward_history_window_returns_latest_page_in_chronological_order() {
        let window = message_history_window_from_desc_fetch_ahead(
            vec![
                stored_message(10),
                stored_message(9),
                stored_message(8),
                stored_message(7),
            ],
            3,
            10,
        );

        assert_eq!(
            window
                .items
                .iter()
                .map(|message| message.message_seq)
                .collect::<Vec<_>>(),
            vec![8, 9, 10]
        );
        assert_eq!(window.next_before_seq, Some(8));
        assert!(window.has_more);
    }

    #[test]
    fn backward_history_sql_uses_bounded_descending_keyset_pagination() {
        let normalized = READ_HISTORY_WINDOW_SQL.to_ascii_lowercase();
        assert!(normalized.contains("message_seq < $4"));
        assert!(normalized.contains("order by message_seq desc"));
        assert!(normalized.contains("limit $5"));
        assert!(!normalized.contains(" offset "));
    }
}
