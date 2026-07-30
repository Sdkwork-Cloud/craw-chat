use std::sync::Arc;

use conversation_runtime::{
    AgentDispatchWorker, AgentDispatchWorkerConfig, AgentDispatchWorkerHandle,
    ConversationRuntimeAgentReplyCommitter, MessageStoreAgentDispatchSourceLoader,
    register_embedded_realtime_publisher, resolve_agent_dispatch_worker_id,
    resolve_embedded_conversation_runtime, spawn_agent_dispatch_worker,
};
use sdkwork_agents_runtime_facade::AgentsSessionFacade;
use session_gateway::AppState;

use crate::{
    ApiAssemblyRuntime, ConversationOutboxRelayHandle, RtcOutboxRelayHandle,
    SocialOutboxRelayHandle, spawn_conversation_outbox_relay_from_env,
    spawn_rtc_outbox_relay_from_env, spawn_social_outbox_relay_from_env,
    wire_social_runtime_embedded_plane,
};

pub struct StandaloneRuntimeWiring {
    agent_dispatch_worker: AgentDispatchWorkerHandle,
    rtc_outbox_relay: Option<RtcOutboxRelayHandle>,
    conversation_outbox_relay: Option<ConversationOutboxRelayHandle>,
    social_outbox_relay: Option<SocialOutboxRelayHandle>,
}

impl StandaloneRuntimeWiring {
    pub async fn shutdown(self) {
        self.agent_dispatch_worker.shutdown().await;
        if let Some(handle) = self.rtc_outbox_relay {
            handle.shutdown();
        }
        if let Some(handle) = self.conversation_outbox_relay {
            handle.shutdown();
        }
        if let Some(handle) = self.social_outbox_relay {
            handle.shutdown();
        }
    }
}

impl ApiAssemblyRuntime {
    pub fn wire_standalone_runtime(
        &self,
        session_state: &AppState,
        agents_session_facade: Arc<dyn AgentsSessionFacade>,
    ) -> Result<StandaloneRuntimeWiring, String> {
        let conversation_runtime = resolve_embedded_conversation_runtime().ok_or_else(|| {
            "conversation runtime is required by the IM agent dispatch worker".to_owned()
        })?;
        let shared_pool = sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool()
            .map_err(|error| {
                format!("IM agent dispatch worker requires the shared PostgreSQL pool: {error}")
            })?;
        let pool = im_adapters_postgres_journal::PostgresJournalPool::from_pool(shared_pool);
        let message_store =
            Arc::new(im_adapters_postgres_journal::PostgresMessageStore::from_pool(pool.clone()));
        let integration_store = Arc::new(
            im_adapters_postgres_journal::PostgresAgentIntegrationStore::from_pool_with_runtime_ids(
                pool,
            ),
        );
        let source_loader = Arc::new(MessageStoreAgentDispatchSourceLoader::new(message_store));
        let reply_committer = Arc::new(ConversationRuntimeAgentReplyCommitter::new(
            conversation_runtime.clone(),
        ));
        let worker = AgentDispatchWorker::new(
            integration_store,
            agents_session_facade,
            source_loader,
            reply_committer,
            resolve_agent_dispatch_worker_id()?,
        )?;
        let handle = spawn_agent_dispatch_worker(worker, AgentDispatchWorkerConfig::from_env()?);
        sdkwork_im_service_readiness::register_im_process_boolean_readiness_check(
            "im agent dispatch worker",
            handle.health_signal(),
        )?;

        register_embedded_realtime_publisher(session_state.realtime_runtime());
        wire_social_runtime_embedded_plane(
            &self.social_runtime,
            session_state.realtime_runtime(),
            Some(conversation_runtime),
        );

        Ok(StandaloneRuntimeWiring {
            agent_dispatch_worker: handle,
            rtc_outbox_relay: spawn_rtc_outbox_relay_from_env(session_state.realtime_runtime()),
            conversation_outbox_relay: spawn_conversation_outbox_relay_from_env(
                session_state.realtime_runtime(),
            ),
            social_outbox_relay: spawn_social_outbox_relay_from_env(
                session_state.realtime_runtime(),
            ),
        })
    }
}
