//! Batch purge of rows whose `retention_until` timestamp is in the past.
//!
//! Uses the partial indexes declared in `001_im_core_schema.sql` and verified by
//! `database_schema_contract_test`.

use im_platform_contracts::ContractError;
use r2d2_postgres::postgres;

use crate::{postgres_pool_client, postgres_unavailable, PostgresJournalPool, run_postgres_io};

const DEFAULT_PURGE_BATCH_SIZE: i64 = 500;

const PURGE_COMMIT_JOURNAL_SQL: &str = r#"
DELETE FROM im_commit_journal
WHERE ctid IN (
    SELECT ctid
    FROM im_commit_journal
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_CONVERSATION_MESSAGES_SQL: &str = r#"
DELETE FROM im_conversation_messages
WHERE ctid IN (
    SELECT ctid
    FROM im_conversation_messages
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_MESSAGE_MEDIA_REFS_SQL: &str = r#"
DELETE FROM im_message_media_refs
WHERE ctid IN (
    SELECT ctid
    FROM im_message_media_refs
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_OUTBOX_EVENTS_SQL: &str = r#"
DELETE FROM im_outbox_events
WHERE ctid IN (
    SELECT ctid
    FROM im_outbox_events
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_INBOX_EVENTS_SQL: &str = r#"
DELETE FROM im_inbox_events
WHERE ctid IN (
    SELECT ctid
    FROM im_inbox_events
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_PROJECTION_TIMELINE_SQL: &str = r#"
DELETE FROM im_projection_timeline_entries
WHERE ctid IN (
    SELECT ctid
    FROM im_projection_timeline_entries
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_REALTIME_DEVICE_EVENTS_SQL: &str = r#"
DELETE FROM im_realtime_device_events
WHERE ctid IN (
    SELECT ctid
    FROM im_realtime_device_events
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RetentionCleanupReport {
    pub commit_journal_deleted: u64,
    pub conversation_messages_deleted: u64,
    pub message_media_refs_deleted: u64,
    pub outbox_events_deleted: u64,
    pub inbox_events_deleted: u64,
    pub projection_timeline_entries_deleted: u64,
    pub realtime_device_events_deleted: u64,
}

pub fn purge_expired_retention_batch(
    pool: &PostgresJournalPool,
    batch_size: Option<i64>,
) -> Result<RetentionCleanupReport, ContractError> {
    let pool = pool.clone();
    let limit = batch_size.unwrap_or(DEFAULT_PURGE_BATCH_SIZE).max(1);
    run_postgres_io(move || purge_batch(&pool, limit))
}

fn purge_batch(pool: &PostgresJournalPool, limit: i64) -> Result<RetentionCleanupReport, ContractError> {
    let mut client = postgres_pool_client(pool, "journal retention purge")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("journal retention purge begin", error))?;

    let commit_journal_deleted =
        execute_retention_delete(&mut txn, PURGE_COMMIT_JOURNAL_SQL, limit)?;
    let conversation_messages_deleted =
        execute_retention_delete(&mut txn, PURGE_CONVERSATION_MESSAGES_SQL, limit)?;
    let message_media_refs_deleted =
        execute_retention_delete(&mut txn, PURGE_MESSAGE_MEDIA_REFS_SQL, limit)?;
    let outbox_events_deleted =
        execute_retention_delete(&mut txn, PURGE_OUTBOX_EVENTS_SQL, limit)?;
    let inbox_events_deleted =
        execute_retention_delete(&mut txn, PURGE_INBOX_EVENTS_SQL, limit)?;
    let projection_timeline_entries_deleted =
        execute_retention_delete(&mut txn, PURGE_PROJECTION_TIMELINE_SQL, limit)?;
    let realtime_device_events_deleted =
        execute_retention_delete(&mut txn, PURGE_REALTIME_DEVICE_EVENTS_SQL, limit)?;

    txn.commit()
        .map_err(|error| postgres_unavailable("journal retention purge commit", error))?;

    Ok(RetentionCleanupReport {
        commit_journal_deleted,
        conversation_messages_deleted,
        message_media_refs_deleted,
        outbox_events_deleted,
        inbox_events_deleted,
        projection_timeline_entries_deleted,
        realtime_device_events_deleted,
    })
}

fn execute_retention_delete(
    txn: &mut postgres::Transaction<'_>,
    sql: &str,
    limit: i64,
) -> Result<u64, ContractError> {
    txn.execute(sql, &[&limit])
        .map(|deleted| deleted as u64)
        .map_err(|error| postgres_unavailable("journal retention purge delete", error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_purge_sql_requires_expired_retention_until() {
        for sql in [
            PURGE_COMMIT_JOURNAL_SQL,
            PURGE_CONVERSATION_MESSAGES_SQL,
            PURGE_MESSAGE_MEDIA_REFS_SQL,
            PURGE_OUTBOX_EVENTS_SQL,
            PURGE_INBOX_EVENTS_SQL,
            PURGE_PROJECTION_TIMELINE_SQL,
            PURGE_REALTIME_DEVICE_EVENTS_SQL,
        ] {
            assert!(
                sql.contains("retention_until IS NOT NULL"),
                "purge SQL must skip indefinite retention rows"
            );
            assert!(
                sql.contains("retention_until <= NOW()"),
                "purge SQL must only delete expired rows"
            );
        }
    }
}
