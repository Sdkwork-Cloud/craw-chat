//! Batch purge of rows whose `retention_until` timestamp is in the past.
//!
//! Uses the partial indexes declared in `001_im_core_schema.sql` and verified by
//! `database_schema_contract_test`.

use im_platform_contracts::{ContractError, PrivilegedOperationContext};
use r2d2_postgres::postgres;

use crate::{PostgresJournalPool, postgres_pool_client, postgres_unavailable, run_postgres_io};

const DEFAULT_PURGE_BATCH_SIZE: i64 = 500;
pub const RETENTION_PURGE_BATCH_SIZE_MAX: i64 = 5_000;

const PURGE_COMMIT_JOURNAL_SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
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
/* sdkwork:cross-organization-operation=retention-expiry-purge */
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
/* sdkwork:cross-organization-operation=retention-expiry-purge */
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
/* sdkwork:cross-organization-operation=retention-expiry-purge */
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
/* sdkwork:cross-organization-operation=retention-expiry-purge */
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

const PURGE_REALTIME_DEVICE_EVENTS_SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
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

const PURGE_RTC_SESSIONS_SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
DELETE FROM im_rtc_sessions
WHERE ctid IN (
    SELECT ctid
    FROM im_rtc_sessions
    WHERE retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const PURGE_RTC_SIGNALS_SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
DELETE FROM im_rtc_signals
WHERE ctid IN (
    SELECT ctid
    FROM im_rtc_signals
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
    pub realtime_device_events_deleted: u64,
    pub rtc_sessions_deleted: u64,
    pub rtc_signals_deleted: u64,
}

impl RetentionCleanupReport {
    pub fn total_deleted(&self) -> u64 {
        self.commit_journal_deleted
            + self.conversation_messages_deleted
            + self.message_media_refs_deleted
            + self.outbox_events_deleted
            + self.inbox_events_deleted
            + self.realtime_device_events_deleted
            + self.rtc_sessions_deleted
            + self.rtc_signals_deleted
    }
}

#[derive(Clone, Debug)]
pub struct RetentionPurgeRequest {
    context: PrivilegedOperationContext,
    batch_size: i64,
}

impl RetentionPurgeRequest {
    pub fn try_new(
        context: PrivilegedOperationContext,
        batch_size: Option<i64>,
    ) -> Result<Self, ContractError> {
        let batch_size = batch_size.unwrap_or(DEFAULT_PURGE_BATCH_SIZE);
        if !(1..=RETENTION_PURGE_BATCH_SIZE_MAX).contains(&batch_size) {
            return Err(ContractError::Invalid(format!(
                "retention purge batch size must be between 1 and {RETENTION_PURGE_BATCH_SIZE_MAX}"
            )));
        }
        Ok(Self {
            context,
            batch_size,
        })
    }

    pub fn context(&self) -> &PrivilegedOperationContext {
        &self.context
    }

    pub const fn batch_size(&self) -> i64 {
        self.batch_size
    }
}

pub fn purge_expired_retention_batch(
    pool: &PostgresJournalPool,
    request: RetentionPurgeRequest,
) -> Result<RetentionCleanupReport, ContractError> {
    let actor_kind = request.context().actor_kind().as_str().to_owned();
    let actor_id = request.context().actor_id().to_owned();
    let trace_id = request.context().trace_id().to_owned();
    let pool = pool.clone();
    let limit = request.batch_size();
    let result = run_postgres_io(move || purge_batch(&pool, limit));
    match &result {
        Ok(report) => tracing::info!(
            target: "sdkwork.im.security",
            event = "im.retention_purge.operation_completed",
            actor_kind,
            actor_id,
            trace_id,
            outcome = "succeeded",
            batch_size = limit,
            rows_deleted = report.total_deleted(),
            "cross-organization retention purge completed"
        ),
        Err(error) => tracing::warn!(
            target: "sdkwork.im.security",
            event = "im.retention_purge.operation_completed",
            actor_kind,
            actor_id,
            trace_id,
            outcome = "failed",
            batch_size = limit,
            error = ?error,
            "cross-organization retention purge failed"
        ),
    }
    result
}

fn purge_batch(
    pool: &PostgresJournalPool,
    limit: i64,
) -> Result<RetentionCleanupReport, ContractError> {
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
    let outbox_events_deleted = execute_retention_delete(&mut txn, PURGE_OUTBOX_EVENTS_SQL, limit)?;
    let inbox_events_deleted = execute_retention_delete(&mut txn, PURGE_INBOX_EVENTS_SQL, limit)?;
    let realtime_device_events_deleted =
        execute_retention_delete(&mut txn, PURGE_REALTIME_DEVICE_EVENTS_SQL, limit)?;
    let rtc_sessions_deleted = execute_retention_delete(&mut txn, PURGE_RTC_SESSIONS_SQL, limit)?;
    let rtc_signals_deleted = execute_retention_delete(&mut txn, PURGE_RTC_SIGNALS_SQL, limit)?;

    txn.commit()
        .map_err(|error| postgres_unavailable("journal retention purge commit", error))?;

    Ok(RetentionCleanupReport {
        commit_journal_deleted,
        conversation_messages_deleted,
        message_media_refs_deleted,
        outbox_events_deleted,
        inbox_events_deleted,
        realtime_device_events_deleted,
        rtc_sessions_deleted,
        rtc_signals_deleted,
    })
}

fn execute_retention_delete(
    txn: &mut postgres::Transaction<'_>,
    sql: &str,
    limit: i64,
) -> Result<u64, ContractError> {
    txn.execute(sql, &[&limit])
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
            PURGE_REALTIME_DEVICE_EVENTS_SQL,
            PURGE_RTC_SESSIONS_SQL,
            PURGE_RTC_SIGNALS_SQL,
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
