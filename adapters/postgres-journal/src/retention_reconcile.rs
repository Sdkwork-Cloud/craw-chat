//! Clears `retention_until` for a conversation scope when retention becomes indefinite.

use im_platform_contracts::{ContractError, RetentionScopeStore};
use tracing::info;

use crate::{
    postgres_pool_client, postgres_unavailable, run_postgres_io, PostgresJournalPool,
};

const CLEAR_CONVERSATION_MESSAGES_SQL: &str = r#"
UPDATE im_conversation_messages
SET retention_until = NULL, updated_at = NOW()
WHERE tenant_id = $1
  AND organization_id = $2
  AND conversation_id = $3
  AND retention_until IS NOT NULL
"#;

const CLEAR_MESSAGE_MEDIA_REFS_SQL: &str = r#"
UPDATE im_message_media_refs
SET retention_until = NULL, updated_at = NOW()
WHERE tenant_id = $1
  AND organization_id = $2
  AND conversation_id = $3
  AND retention_until IS NOT NULL
"#;

const CLEAR_PROJECTION_TIMELINE_SQL: &str = r#"
UPDATE im_projection_timeline_entries
SET retention_until = NULL, updated_at = NOW()
WHERE tenant_id = $1
  AND organization_id = $2
  AND conversation_id = $3
  AND retention_until IS NOT NULL
"#;

const CLEAR_COMMIT_JOURNAL_SQL: &str = r#"
UPDATE im_commit_journal
SET retention_until = NULL
WHERE tenant_id = $1
  AND aggregate_type = 'conversation'
  AND aggregate_id = $2
  AND retention_until IS NOT NULL
"#;

const CLEAR_OUTBOX_EVENTS_SQL: &str = r#"
UPDATE im_outbox_events
SET retention_until = NULL, updated_at = NOW()
WHERE tenant_id = $1
  AND organization_id = $2
  AND aggregate_type = 'conversation'
  AND aggregate_id = $3
  AND retention_until IS NOT NULL
"#;

const CLEAR_INBOX_EVENTS_SQL: &str = r#"
UPDATE im_inbox_events
SET retention_until = NULL, updated_at = NOW()
WHERE tenant_id = $1
  AND organization_id = $2
  AND retention_until IS NOT NULL
  AND (
    payload_json->>'conversationId' = $3
    OR (
      payload_json->>'scopeId' = $3
      AND COALESCE(payload_json->>'scopeType', 'conversation') = 'conversation'
    )
  )
"#;

const CLEAR_REALTIME_DEVICE_EVENTS_SQL: &str = r#"
UPDATE im_realtime_device_events
SET retention_until = NULL
WHERE tenant_id = $1
  AND scope_type = 'conversation'
  AND scope_id = $2
  AND retention_until IS NOT NULL
"#;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RetentionReconcileReport {
    pub conversation_messages_cleared: u64,
    pub message_media_refs_cleared: u64,
    pub projection_timeline_entries_cleared: u64,
    pub commit_journal_cleared: u64,
    pub outbox_events_cleared: u64,
    pub inbox_events_cleared: u64,
    pub realtime_device_events_cleared: u64,
}

impl RetentionReconcileReport {
    pub fn total_cleared(&self) -> u64 {
        self.conversation_messages_cleared
            + self.message_media_refs_cleared
            + self.projection_timeline_entries_cleared
            + self.commit_journal_cleared
            + self.outbox_events_cleared
            + self.inbox_events_cleared
            + self.realtime_device_events_cleared
    }
}

#[derive(Clone)]
pub struct PostgresRetentionScopeStore {
    pool: PostgresJournalPool,
}

impl PostgresRetentionScopeStore {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self { pool }
    }
}

impl RetentionScopeStore for PostgresRetentionScopeStore {
    fn clear_conversation_retention_until(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), ContractError> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let conversation_id = conversation_id.to_owned();
        run_postgres_io(move || {
            let report = clear_conversation_retention_until(
                &pool,
                tenant_id.as_str(),
                organization_id.as_str(),
                conversation_id.as_str(),
            )?;
            if report.total_cleared() > 0 {
                info!(
                    target: "sdkwork.im",
                    event = "im.retention.reconcile.legal_hold",
                    tenant_id = tenant_id.as_str(),
                    organization_id = organization_id.as_str(),
                    conversation_id = conversation_id.as_str(),
                    conversation_messages_cleared = report.conversation_messages_cleared,
                    message_media_refs_cleared = report.message_media_refs_cleared,
                    projection_timeline_entries_cleared = report.projection_timeline_entries_cleared,
                    commit_journal_cleared = report.commit_journal_cleared,
                    outbox_events_cleared = report.outbox_events_cleared,
                    inbox_events_cleared = report.inbox_events_cleared,
                    realtime_device_events_cleared = report.realtime_device_events_cleared,
                    "cleared expiring retention markers for indefinite retention policy"
                );
            }
            Ok(())
        })
    }
}

pub fn clear_conversation_retention_until(
    pool: &PostgresJournalPool,
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
) -> Result<RetentionReconcileReport, ContractError> {
    let mut client = postgres_pool_client(pool, "retention reconcile")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("retention reconcile begin", error))?;

    let conversation_messages_cleared = txn
        .execute(
            CLEAR_CONVERSATION_MESSAGES_SQL,
            &[&tenant_id, &organization_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile messages", error))?
        as u64;
    let message_media_refs_cleared = txn
        .execute(
            CLEAR_MESSAGE_MEDIA_REFS_SQL,
            &[&tenant_id, &organization_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile media refs", error))?
        as u64;
    let projection_timeline_entries_cleared = txn
        .execute(
            CLEAR_PROJECTION_TIMELINE_SQL,
            &[&tenant_id, &organization_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile projection timeline", error))?
        as u64;
    let commit_journal_cleared = txn
        .execute(
            CLEAR_COMMIT_JOURNAL_SQL,
            &[&tenant_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile commit journal", error))?
        as u64;
    let outbox_events_cleared = txn
        .execute(
            CLEAR_OUTBOX_EVENTS_SQL,
            &[&tenant_id, &organization_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile outbox", error))?
        as u64;
    let inbox_events_cleared = txn
        .execute(
            CLEAR_INBOX_EVENTS_SQL,
            &[&tenant_id, &organization_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile inbox", error))?
        as u64;
    let realtime_device_events_cleared = txn
        .execute(
            CLEAR_REALTIME_DEVICE_EVENTS_SQL,
            &[&tenant_id, &conversation_id],
        )
        .map_err(|error| postgres_unavailable("retention reconcile realtime device events", error))?
        as u64;

    txn.commit()
        .map_err(|error| postgres_unavailable("retention reconcile commit", error))?;

    Ok(RetentionReconcileReport {
        conversation_messages_cleared,
        message_media_refs_cleared,
        projection_timeline_entries_cleared,
        commit_journal_cleared,
        outbox_events_cleared,
        inbox_events_cleared,
        realtime_device_events_cleared,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_reconcile_report_totals() {
        let report = RetentionReconcileReport {
            conversation_messages_cleared: 2,
            message_media_refs_cleared: 1,
            projection_timeline_entries_cleared: 3,
            commit_journal_cleared: 4,
            outbox_events_cleared: 1,
            inbox_events_cleared: 0,
            realtime_device_events_cleared: 5,
        };
        assert_eq!(report.total_cleared(), 16);
    }
}
