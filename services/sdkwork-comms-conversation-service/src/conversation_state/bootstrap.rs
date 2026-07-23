use std::sync::{Arc, OnceLock};

use im_adapters_postgres_journal::{
    PostgresAgentIntegrationStore, PostgresJournalPool, PostgresOutboxStore, PostgresSearchProvider,
};
use im_platform_contracts::{AgentIntegrationStore, OutboxStore};

use crate::conversation_state::ConversationStateService;

/// Process-local cache and normalized-query dependencies used by Conversation HTTP/RPC handlers.
///
/// The service contains only disposable cache entries. PostgreSQL query providers read canonical
/// IM tables directly; no startup restore or journal replay participates in correctness.
pub struct ConversationStateRuntime {
    service: Arc<ConversationStateService>,
    search_provider: Option<Arc<PostgresSearchProvider>>,
}

impl ConversationStateRuntime {
    pub fn in_memory() -> Self {
        Self {
            service: Arc::new(ConversationStateService::default()),
            search_provider: None,
        }
    }

    pub fn service(&self) -> Arc<ConversationStateService> {
        self.service.clone()
    }

    pub fn search_provider(&self) -> Option<Arc<PostgresSearchProvider>> {
        self.search_provider.clone()
    }
}

static SHARED_CONVERSATION_STATE_RUNTIME: OnceLock<Arc<ConversationStateRuntime>> = OnceLock::new();

pub fn shared_conversation_state_runtime() -> Arc<ConversationStateRuntime> {
    SHARED_CONVERSATION_STATE_RUNTIME
        .get_or_init(|| Arc::new(build_conversation_state_runtime_from_env()))
        .clone()
}

pub fn try_init_conversation_state_runtime() -> Option<Arc<ConversationStateRuntime>> {
    Some(shared_conversation_state_runtime())
}

pub fn build_conversation_state_runtime_from_env() -> ConversationStateRuntime {
    let service = Arc::new(ConversationStateService::default());
    let Some(shared_pool) = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool() else {
        return ConversationStateRuntime {
            service,
            search_provider: None,
        };
    };

    let pool = PostgresJournalPool::from_pool(shared_pool);
    service.configure_conversation_event_outbox(Arc::new(PostgresOutboxStore::from_pool(
        pool.clone(),
    )) as Arc<dyn OutboxStore>);
    service.configure_agent_integration_store(Arc::new(
        PostgresAgentIntegrationStore::from_pool_with_runtime_ids(pool.clone()),
    ) as Arc<dyn AgentIntegrationStore>);

    ConversationStateRuntime {
        service,
        search_provider: Some(Arc::new(PostgresSearchProvider::from_pool(pool))),
    }
}
