//! PostgreSQL repository adapters for SDKWork IM.

mod agent_integration_store;
mod aggregate_store;
mod automation_execution_store;
mod journal_queries;
mod journal_repository;
mod message_post_persistence;
mod message_store;
mod notification_task_store;
mod outbox_store;
mod postgres_support;
mod principal_directory;
mod retention_cleanup;
mod retention_metrics;
mod retention_reconcile;
mod retention_scheduler;
mod search_store;
mod seq_allocator;
mod stream_state_store;
mod welcome_state_store;

pub use agent_integration_store::PostgresAgentIntegrationStore;
pub use aggregate_store::PostgresAggregateStore;
pub use automation_execution_store::PostgresAutomationExecutionStore;
pub use im_platform_contracts::CommitJournalReplayCursor as JournalReplayCursor;
pub use journal_repository::{CommitJournalReplayState, PostgresCommitJournal, JournalReplayStateRequest};
pub use message_post_persistence::{
    PostgresDurableConversationEventWriter, PostgresDurableMessageMutationWriter,
    PostgresDurableMessagePostWriter,
};
pub use message_store::PostgresMessageStore;
pub use notification_task_store::PostgresNotificationTaskStore;
pub use outbox_store::PostgresOutboxStore;
pub use postgres_support::{
    PostgresJournalConfig, PostgresJournalConnectionManager, PostgresJournalPool,
    PostgresJournalTlsConnector, conversation_member_access_gate_from_pool,
};
pub use principal_directory::PostgresPrincipalDirectory;
pub use retention_cleanup::{
    RETENTION_PURGE_BATCH_SIZE_MAX, RetentionCleanupReport, RetentionPurgeRequest,
    purge_expired_retention_batch,
};
pub(crate) use retention_cleanup::{log_retention_purge_outcome, purge_retention_batch_on_txn};
pub use retention_metrics::{RetentionPurgeMetrics, retention_purge_metrics};
pub use retention_reconcile::{
    PostgresRetentionScopeStore, RetentionReconcileReport, clear_conversation_retention_until,
};
pub use retention_scheduler::{
    RetentionPurgeSchedulerConfig, RetentionPurgeSchedulerHandle, spawn_retention_purge_scheduler,
    spawn_retention_purge_scheduler_from_env,
};
pub use search_store::{MemberSearchQuery, PostgresSearchProvider};
pub use seq_allocator::PostgresConversationSeqAllocator;
pub use stream_state_store::PostgresStreamStateStore;
pub use welcome_state_store::PostgresWelcomeStateStore;

pub(crate) use journal_queries::{APPEND_EVENT_SQL, LOAD_EVENT_BY_POSITION_SQL};
pub(crate) use journal_repository::{
    allocate_next_ordering_sequences, compose_partition_key, is_unique_violation,
    journal_aggregate_seq, journal_position_conflict, journal_retention_until,
    lock_journal_partitions, resolve_journal_event_id_replay,
};
pub(crate) use postgres_support::{
    now_rfc3339, postgres_bigint_input, postgres_bigint_output, postgres_jsonb_payload,
    postgres_pool_client, postgres_row_get, postgres_timestamptz, postgres_unavailable,
    postgres_unavailable_db, run_postgres_io,
};
