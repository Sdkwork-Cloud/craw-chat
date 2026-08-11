use im_adapters_postgres_journal::{
    PostgresAgentIntegrationStore, PostgresAggregateStore, PostgresCommitJournal,
    PostgresConversationSeqAllocator, PostgresDurableConversationEventWriter,
    PostgresDurableMessageMutationWriter, PostgresDurableMessagePostWriter, PostgresJournalConfig,
    PostgresMessageStore, PostgresOutboxStore, PostgresRetentionScopeStore,
    PostgresWelcomeStateStore,
};
use im_adapters_redis_cache::RedisSeqAllocator;
use im_adapters_social_postgres::user_block_store::PostgresUserBlockStore;
use im_platform_contracts::{
    AgentIntegrationStore, ConversationAggregateStore, ConversationSeqAllocator, MessageStore,
    OutboxStore, RetentionScopeStore, WelcomeStateStore,
};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{
    CommitEnvelope, CommitJournal, CommitJournalAggregateEventTypeQuery, CommitPosition,
};
use sdkwork_im_runtime_id::build_runtime_id_generator_blocking;
use std::sync::Arc;
use tracing::info;

use super::{
    ConversationRuntime, DurableConversationEventWriter, DurableMessageMutationWriter,
    DurableMessagePostWriter, InMemoryJournal,
    postgres_direct_message_gate::PostgresDirectMessageAccessGate,
};

const IM_DATABASE_URL_ENV: &str = "SDKWORK_DATABASE_URL";

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
        // Refresh the disposable process cache after the authoritative commit. Cache refresh is
        // never part of transaction correctness and startup never replays the journal into it.
        crate::conversation_state::refresh_conversation_cache(&envelope);
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
        // Best-effort cache refresh after the authoritative batch commit.
        for envelope in &envelopes {
            crate::conversation_state::refresh_conversation_cache(envelope);
        }
        Ok(positions)
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded(journal),
            Self::Postgres(journal) => CommitJournal::recorded(journal),
        }
    }

    fn recorded_page(
        &self,
        cursor: Option<&sdkwork_im_contract_message::CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<sdkwork_im_contract_message::CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded_page(journal, cursor, limit),
            Self::Postgres(journal) => CommitJournal::recorded_page(journal, cursor, limit),
        }
    }

    fn recorded_page_for_aggregate(
        &self,
        scope: &sdkwork_im_contract_message::CommitJournalAggregateScope,
        cursor: Option<&sdkwork_im_contract_message::CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<sdkwork_im_contract_message::CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
            Self::Postgres(journal) => {
                CommitJournal::recorded_page_for_aggregate(journal, scope, cursor, limit)
            }
        }
    }

    fn recorded_page_for_aggregate_event_types(
        &self,
        query: &CommitJournalAggregateEventTypeQuery,
        cursor: Option<&sdkwork_im_contract_message::CommitJournalReplayCursor>,
        limit: usize,
    ) -> Result<sdkwork_im_contract_message::CommitJournalReplayPage, ContractError> {
        match self {
            Self::Memory(journal) => CommitJournal::recorded_page_for_aggregate_event_types(
                journal, query, cursor, limit,
            ),
            Self::Postgres(journal) => CommitJournal::recorded_page_for_aggregate_event_types(
                journal, query, cursor, limit,
            ),
        }
    }
}

pub fn resolve_conversation_commit_journal_from_env() -> Result<ConversationCommitJournal, String> {
    let config = DatabaseConfig::from_env("IM").map_err(|_| {
        format!("conversation runtime requires PostgreSQL configuration in {IM_DATABASE_URL_ENV}")
    })?;
    if config.engine != DatabaseEngine::Postgres {
        return Err(
            "conversation runtime requires PostgreSQL; SQLite is client-local only".to_owned(),
        );
    }

    let journal = PostgresJournalConfig::from_database_config(&config)
        .connect()
        .map_err(|_| "conversation PostgreSQL journal bootstrap failed".to_owned())?;
    info!("conversation-runtime using postgres commit journal");
    Ok(ConversationCommitJournal::Postgres(journal))
}

pub fn build_conversation_runtime_from_env()
-> Result<ConversationRuntime<ConversationCommitJournal>, String> {
    let journal = resolve_conversation_commit_journal_from_env()?;
    let mut runtime = ConversationRuntime::new(journal.clone());

    if let ConversationCommitJournal::Postgres(postgres_journal) = journal {
        let pool = postgres_journal.pool().clone();

        // Snowflake ID generator for message_id / event_id (not message_seq).
        let id_generator = build_runtime_id_generator_blocking("conversation-service");
        let seq_allocator = resolve_conversation_seq_allocator_from_env(pool.clone());

        runtime = runtime
            .with_message_store(
                Arc::new(PostgresMessageStore::from_pool(pool.clone())) as Arc<dyn MessageStore>
            )
            .with_seq_allocator(seq_allocator)
            .with_outbox_store(
                Arc::new(PostgresOutboxStore::from_pool(pool.clone())) as Arc<dyn OutboxStore>
            )
            .with_aggregate_store(Arc::new(PostgresAggregateStore::from_pool(pool.clone()))
                as Arc<dyn ConversationAggregateStore>)
            .with_agent_integration_store(Arc::new(PostgresAgentIntegrationStore::from_pool(
                pool.clone(),
                id_generator.clone(),
            )) as Arc<dyn AgentIntegrationStore>)
            .with_retention_scope_store(Arc::new(PostgresRetentionScopeStore::from_pool(
                pool.clone(),
            )) as Arc<dyn RetentionScopeStore>)
            .with_welcome_state_store(
                Arc::new(PostgresWelcomeStateStore::from_pool(pool)) as Arc<dyn WelcomeStateStore>
            )
            .with_id_generator(id_generator.clone())
            .with_durable_message_post_writer(Arc::new(
                PostgresDurableMessagePostWriter::from_journal(&postgres_journal),
            ) as Arc<dyn DurableMessagePostWriter>)
            .with_durable_message_mutation_writer(Arc::new(
                PostgresDurableMessageMutationWriter::from_journal(&postgres_journal),
            )
                as Arc<dyn DurableMessageMutationWriter>);
        runtime = runtime.with_durable_conversation_event_writer(Arc::new(
            PostgresDurableConversationEventWriter::from_journal_with_id_generator(
                &postgres_journal,
                id_generator,
            ),
        )
            as Arc<dyn DurableConversationEventWriter>);
        if let Ok(shared_pool) = sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool() {
            let block_store = Arc::new(PostgresUserBlockStore::new(Arc::new(shared_pool.clone())));
            runtime = runtime.with_direct_message_access_gate(Arc::new(
                PostgresDirectMessageAccessGate::new(block_store),
            ));
            runtime = runtime.with_user_profile_resolver(Arc::new(
                crate::conversation_state::PostgresUserProfileResolver::new(Arc::new(
                    im_adapters_social_postgres::user_profile_store::PostgresUserProfileStore::new(
                        Arc::new(shared_pool),
                    ),
                )),
            ));
            info!("conversation-runtime wired postgres direct message access gate");
        }
        info!("conversation-runtime wired postgres stores with per-conversation seq allocation");
    }

    Ok(runtime)
}

const IM_REDIS_SEQ_URL_ENV: &str = "SDKWORK_IM_REALTIME_ROUTE_STORE_URL";
const IM_CLUSTER_BUS_URL_ENV: &str = "SDKWORK_IM_REALTIME_CLUSTER_BUS_URL";

fn resolve_conversation_seq_allocator_from_env(
    pool: im_adapters_postgres_journal::PostgresJournalPool,
) -> Arc<dyn ConversationSeqAllocator> {
    if let Some(redis_url) = resolve_redis_seq_allocator_url_from_env() {
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => {
                info!("conversation-runtime using redis conversation seq allocator");
                return Arc::new(RedisSeqAllocator::new(client));
            }
            Err(error) => {
                tracing::warn!(
                    error = ?error,
                    "invalid redis url for conversation seq allocator; falling back to postgres"
                );
            }
        }
    }

    info!("conversation-runtime using postgres conversation seq allocator");
    Arc::new(PostgresConversationSeqAllocator::from_pool(pool))
}

fn resolve_redis_seq_allocator_url_from_env() -> Option<String> {
    std::env::var(IM_REDIS_SEQ_URL_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var(IM_CLUSTER_BUS_URL_ENV)
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
        })
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
