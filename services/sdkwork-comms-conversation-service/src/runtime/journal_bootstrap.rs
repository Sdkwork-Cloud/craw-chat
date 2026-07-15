use im_adapters_postgres_journal::{
    PostgresAggregateStore, PostgresCommitJournal, PostgresConversationSeqAllocator,
    PostgresDurableConversationEventWriter, PostgresDurableMessagePostWriter,
    PostgresJournalConfig, PostgresMessageStore, PostgresOutboxStore, PostgresRetentionScopeStore,
};
use im_adapters_redis_cache::RedisSeqAllocator;
use im_adapters_social_postgres::user_block_store::PostgresUserBlockStore;
use im_app_context::resolve_web_environment_from_process_env;
use im_platform_contracts::{
    ConversationAggregateStore, ConversationSeqAllocator, MessageStore, OutboxStore,
    RetentionScopeStore,
};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{
    CommitEnvelope, CommitJournal, CommitJournalAggregateEventTypeQuery, CommitPosition,
};
use sdkwork_im_runtime_id::build_runtime_id_generator_blocking;
use sdkwork_web_core::WebEnvironment;
use std::sync::Arc;
use tracing::info;

use super::{
    ConversationRuntime, DurableConversationEventWriter, DurableMessagePostWriter, InMemoryJournal,
    postgres_direct_message_gate::PostgresDirectMessageAccessGate,
};

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
        // Apply the committed event to the embedded projection read-model on a best-effort
        // basis. The journal is the source of truth; the projection is a derived read-model
        // that must remain eventually consistent. If the projection store is temporarily
        // unavailable, the write must still succeed — the projection will catch up via
        // journal replay polling. Failing the journal commit here would cause cascading 503
        // errors (code 50301) for every message send when the projection dependency blips.
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
        // Best-effort projection apply — see `append` for rationale on decoupling the
        // projection read-model from the journal commit path.
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
            sdkwork_im_database_pool::log_im_core_ephemeral_non_postgres_authority(
                "conversation-runtime",
                config.engine,
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
            .with_retention_scope_store(Arc::new(PostgresRetentionScopeStore::from_pool(pool))
                as Arc<dyn RetentionScopeStore>)
            .with_id_generator(id_generator)
            .with_durable_message_post_writer(Arc::new(
                PostgresDurableMessagePostWriter::from_journal(&postgres_journal),
            ) as Arc<dyn DurableMessagePostWriter>);
        runtime = runtime.with_durable_conversation_event_writer(Arc::new(
            PostgresDurableConversationEventWriter::from_journal(&postgres_journal),
        )
            as Arc<dyn DurableConversationEventWriter>);
        if let Ok(shared_pool) = sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool() {
            let block_store = Arc::new(PostgresUserBlockStore::new(Arc::new(shared_pool)));
            runtime = runtime.with_direct_message_access_gate(Arc::new(
                PostgresDirectMessageAccessGate::new(block_store),
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
