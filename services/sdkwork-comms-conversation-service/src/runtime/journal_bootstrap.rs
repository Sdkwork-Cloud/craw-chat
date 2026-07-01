use im_adapters_postgres_journal::{
    PostgresAggregateStore, PostgresCommitJournal, PostgresJournalConfig, PostgresMessageStore,
    PostgresOutboxStore, PostgresRetentionScopeStore,
};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::{ConversationAggregateStore, IdGenerator, MessageStore, OutboxStore, RetentionScopeStore};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitEnvelope, CommitJournal, CommitPosition};
use sdkwork_im_runtime_id::build_runtime_id_generator_blocking;
use sdkwork_web_core::WebEnvironment;
use std::sync::Arc;
use tracing::info;

use super::{ConversationRuntime, InMemoryJournal};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

/// Production-capable commit journal backend for conversation runtime processes.
#[derive(Clone)]
pub enum ConversationCommitJournal {
    Memory(InMemoryJournal),
    Postgres(PostgresCommitJournal),
}

impl CommitJournal for ConversationCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let position = match self {
            Self::Memory(journal) => CommitJournal::append(journal, envelope.clone()),
            Self::Postgres(journal) => CommitJournal::append(journal, envelope.clone()),
        }?;
        projection_service::try_apply_commit_envelope(&envelope);
        Ok(position)
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let positions = match self {
            Self::Memory(journal) => CommitJournal::append_batch(journal, envelopes.clone()),
            Self::Postgres(journal) => CommitJournal::append_batch(journal, envelopes.clone()),
        }?;
        for envelope in &envelopes {
            projection_service::try_apply_commit_envelope(envelope);
        }
        Ok(positions)
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded(journal),
            Self::Postgres(journal) => CommitJournal::recorded(journal),
        }
    }
}

pub fn resolve_conversation_commit_journal_from_env() -> Result<ConversationCommitJournal, String> {
    if let Ok(config) = DatabaseConfig::from_env("IM") {
        if config.engine == DatabaseEngine::Postgres {
            let journal = PostgresJournalConfig::from_database_config(&config)
                .connect()
                .map_err(|error| format!("postgres commit journal bootstrap failed: {error:?}"))?;
            info!("conversation-runtime using postgres commit journal");
            return Ok(ConversationCommitJournal::Postgres(journal));
        }

        let environment = resolve_web_environment_from_process_env();
        if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
            info!(
                "conversation-runtime using in-memory commit journal for non-postgres IM database in development"
            );
            return Ok(ConversationCommitJournal::Memory(InMemoryJournal::default()));
        }

        return Err(
            "postgres commit journal is required in production when IM database engine is not postgres"
                .into(),
        );
    }

    if let Some(database_url) = resolve_im_database_url_from_env() {
        let journal = PostgresJournalConfig::new(database_url)
            .connect()
            .map_err(|error| format!("postgres commit journal bootstrap failed: {error:?}"))?;
        info!("conversation-runtime using postgres commit journal");
        return Ok(ConversationCommitJournal::Postgres(journal));
    }

    let environment = resolve_web_environment_from_process_env();
    if matches!(environment, WebEnvironment::Dev | WebEnvironment::Test) {
        info!("conversation-runtime using in-memory commit journal (development only)");
        return Ok(ConversationCommitJournal::Memory(InMemoryJournal::default()));
    }

    Err(format!(
        "postgres commit journal is required in production: set {IM_DATABASE_URL_ENV}"
    ))
}

pub fn build_conversation_runtime_from_env() -> Result<ConversationRuntime<ConversationCommitJournal>, String> {
    let journal = resolve_conversation_commit_journal_from_env()?;
    let mut runtime = ConversationRuntime::new(journal.clone());

    if let ConversationCommitJournal::Postgres(postgres_journal) = journal {
        let pool = postgres_journal.pool().clone();

        // Build Snowflake ID generator for message sequence allocation (eliminates DB hotspot).
        // Uses the synchronous variant so this function stays callable from
        // synchronous bootstrap paths (e.g. `build_default_app` in tests and
        // the `bootstrap_conversation_app_state_from_env` entrypoint). Production
        // deployments that need database-backed node_id allocation should set
        // `SDKWORK_IM_ID_NODE_ID` explicitly or run the async
        // `build_runtime_id_generator` from an async bootstrap path.
        let id_generator = build_runtime_id_generator_blocking("conversation-service");

        runtime = runtime
            .with_message_store(Arc::new(PostgresMessageStore::with_id_generator(pool.clone(), id_generator.clone())) as Arc<dyn MessageStore>)
            .with_outbox_store(
                Arc::new(PostgresOutboxStore::from_pool(pool.clone())) as Arc<dyn OutboxStore>
            )
            .with_aggregate_store(
                Arc::new(PostgresAggregateStore::from_pool(pool.clone()))
                    as Arc<dyn ConversationAggregateStore>,
            )
            .with_retention_scope_store(Arc::new(PostgresRetentionScopeStore::from_pool(pool))
                as Arc<dyn RetentionScopeStore>)
            .with_id_generator(id_generator);
        info!("conversation-runtime wired postgres stores with Snowflake ID generation");
    }

    Ok(runtime)
}

fn resolve_im_database_url_from_env() -> Option<String> {
    std::env::var(IM_DATABASE_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_events::CommitEnvelope;

    #[tokio::test]
    async fn memory_journal_variant_delegates_append() {
        let journal = ConversationCommitJournal::Memory(InMemoryJournal::default());
        let envelope = CommitEnvelope::minimal(
            "evt-1",
            "100001",
            "ConversationCreated",
            "conversation",
            "conv-1",
            1,
        );
        let position = journal.append(envelope).expect("append should succeed");
        assert_eq!(position.offset, 1);
        assert_eq!(position.partition, "p0");
    }
}
