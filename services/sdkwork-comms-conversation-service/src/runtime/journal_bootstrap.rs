use im_adapters_postgres_journal::{
    PostgresAggregateStore, PostgresCommitJournal, PostgresJournalConfig, PostgresMessageStore,
    PostgresOutboxStore, PostgresRetentionScopeStore,
};
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::{ConversationAggregateStore, MessageStore, OutboxStore, RetentionScopeStore};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitEnvelope, CommitJournal, CommitPosition};
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
        match self {
            Self::Memory(journal) => CommitJournal::append(journal, envelope),
            Self::Postgres(journal) => CommitJournal::append(journal, envelope),
        }
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
        runtime = runtime
            .with_message_store(Arc::new(PostgresMessageStore::from_pool(pool.clone()))
                as Arc<dyn MessageStore>)
            .with_outbox_store(
                Arc::new(PostgresOutboxStore::from_pool(pool.clone())) as Arc<dyn OutboxStore>
            )
            .with_aggregate_store(
                Arc::new(PostgresAggregateStore::from_pool(pool.clone()))
                    as Arc<dyn ConversationAggregateStore>,
            )
            .with_retention_scope_store(Arc::new(PostgresRetentionScopeStore::from_pool(pool))
                as Arc<dyn RetentionScopeStore>);
        info!("conversation-runtime wired postgres message/outbox/aggregate stores");
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

    #[test]
    fn memory_journal_variant_delegates_append() {
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
