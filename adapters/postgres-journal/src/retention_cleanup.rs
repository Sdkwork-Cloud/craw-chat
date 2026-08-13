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

/// Invitation records carry invitee contact data (email/phone), so terminal
/// invitations (accepted/declined/expired/canceled) are purged when their
/// `retention_until` expires (`PRIVACY_SPEC.md`). Pending invitations are
/// never purged: they remain active until consumed, revoked, or expired.
const PURGE_INVITATIONS_SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
DELETE FROM im_invitations
WHERE ctid IN (
    SELECT ctid
    FROM im_invitations
    WHERE status <> 'pending'
      AND retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

/// Audit records are purged per `retention_class` in bounded batches so the
/// class-level index and the DDL-documented differentiated windows
/// (security=2y, access=180d, admin=1y, data_lifecycle=3y) drive expiry.
///
/// Note: the audit chain (`chain_prev_hash`/`chain_hash`) is not re-written
/// when expired rows are removed. Deletion is a privileged, audited
/// cross-organization operation, and chain verification reports the retained
/// state truthfully: a scope that has undergone retention purge reports
/// `chain_valid=false` once evidence before the retained window is gone.
const PURGE_AUDIT_RECORDS_BY_CLASS_SQL: &str = r#"
/* sdkwork:cross-organization-operation=retention-expiry-purge */
DELETE FROM im_audit_records
WHERE ctid IN (
    SELECT ctid
    FROM im_audit_records
    WHERE retention_class = $2
      AND retention_until IS NOT NULL
      AND retention_until <= NOW()
    ORDER BY retention_until ASC
    LIMIT $1
)
"#;

const AUDIT_RETENTION_CLASSES: [&str; 4] = ["security", "access", "admin", "data_lifecycle"];

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RetentionCleanupReport {
    pub commit_journal_deleted: u64,
    pub conversation_messages_deleted: u64,
    pub message_media_refs_deleted: u64,
    pub outbox_events_deleted: u64,
    pub inbox_events_deleted: u64,
    pub realtime_device_events_deleted: u64,
    pub rtc_sessions_deleted: u64,
    pub invitations_deleted: u64,
    pub audit_records_deleted: u64,
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
            + self.invitations_deleted
            + self.audit_records_deleted
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
    let limit = request.batch_size();
    let pool = pool.clone();
    let result = run_postgres_io(move || {
        let mut client = postgres_pool_client(&pool, "journal retention purge")?;
        let mut txn = client
            .transaction()
            .map_err(|error| postgres_unavailable("journal retention purge begin", error))?;
        let report = purge_retention_batch_on_txn(&mut txn, limit)?;
        txn.commit()
            .map_err(|error| postgres_unavailable("journal retention purge commit", error))?;
        Ok(report)
    });
    log_retention_purge_outcome(&actor_kind, &actor_id, &trace_id, limit, &result);
    result
}

/// Runs the retention deletes on an already-open transaction. Used by the
/// public single-batch entrypoint and by the retention scheduler, which holds
/// one transaction (and its advisory xact lock) across a whole tick.
pub(crate) fn purge_retention_batch_on_txn(
    txn: &mut postgres::Transaction<'_>,
    limit: i64,
) -> Result<RetentionCleanupReport, ContractError> {
    let commit_journal_deleted = execute_retention_delete(txn, PURGE_COMMIT_JOURNAL_SQL, limit)?;
    let conversation_messages_deleted =
        execute_retention_delete(txn, PURGE_CONVERSATION_MESSAGES_SQL, limit)?;
    let message_media_refs_deleted =
        execute_retention_delete(txn, PURGE_MESSAGE_MEDIA_REFS_SQL, limit)?;
    let outbox_events_deleted = execute_retention_delete(txn, PURGE_OUTBOX_EVENTS_SQL, limit)?;
    let inbox_events_deleted = execute_retention_delete(txn, PURGE_INBOX_EVENTS_SQL, limit)?;
    let realtime_device_events_deleted =
        execute_retention_delete(txn, PURGE_REALTIME_DEVICE_EVENTS_SQL, limit)?;
    let rtc_sessions_deleted = execute_retention_delete(txn, PURGE_RTC_SESSIONS_SQL, limit)?;
    let invitations_deleted = execute_retention_delete(txn, PURGE_INVITATIONS_SQL, limit)?;
    let mut audit_records_deleted = 0_u64;
    for retention_class in AUDIT_RETENTION_CLASSES {
        audit_records_deleted += execute_audit_retention_delete(txn, retention_class, limit)?;
    }

    Ok(RetentionCleanupReport {
        commit_journal_deleted,
        conversation_messages_deleted,
        message_media_refs_deleted,
        outbox_events_deleted,
        inbox_events_deleted,
        realtime_device_events_deleted,
        rtc_sessions_deleted,
        invitations_deleted,
        audit_records_deleted,
    })
}

/// Security audit log for one retention purge batch (cross-organization
/// operation evidence). Shared by the public entrypoint and the scheduler.
pub(crate) fn log_retention_purge_outcome(
    actor_kind: &str,
    actor_id: &str,
    trace_id: &str,
    limit: i64,
    result: &Result<RetentionCleanupReport, ContractError>,
) {
    match result {
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
}

fn execute_retention_delete(
    txn: &mut postgres::Transaction<'_>,
    sql: &str,
    limit: i64,
) -> Result<u64, ContractError> {
    txn.execute(sql, &[&limit])
        .map_err(|error| postgres_unavailable("journal retention purge delete", error))
}

fn execute_audit_retention_delete(
    txn: &mut postgres::Transaction<'_>,
    retention_class: &str,
    limit: i64,
) -> Result<u64, ContractError> {
    txn.execute(PURGE_AUDIT_RECORDS_BY_CLASS_SQL, &[&limit, &retention_class])
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
            PURGE_INVITATIONS_SQL,
            PURGE_AUDIT_RECORDS_BY_CLASS_SQL,
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

    #[test]
    fn test_audit_retention_purge_scoped_by_class() {
        let sql = PURGE_AUDIT_RECORDS_BY_CLASS_SQL;
        assert!(
            sql.contains("retention_class = $2"),
            "audit purge must scope by retention_class"
        );
        assert!(
            sql.contains("LIMIT $1"),
            "audit purge must keep the shared bounded-batch limit"
        );
        assert_eq!(AUDIT_RETENTION_CLASSES.len(), 4);
    }
}
