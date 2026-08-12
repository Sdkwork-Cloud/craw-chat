use std::sync::Arc;
use std::time::Duration;

use conversation_runtime::{
    AgentDispatchWorker, AgentDispatchWorkerConfig, AgentDispatchWorkerHandle,
    ConversationRuntimeAgentReplyCommitter, MessageStoreAgentDispatchSourceLoader,
    register_embedded_realtime_publisher, resolve_agent_dispatch_worker_id,
    resolve_embedded_conversation_runtime, spawn_agent_dispatch_worker,
};
use im_platform_contracts::{ContractError, OutboxStore};
use ops_service::{LagItem, OpsRuntime, SideEffectOutboxDiagnosticsView};
use sdkwork_agents_runtime_facade::AgentsSessionFacade;
use session_gateway::AppState;
use tokio::task::JoinHandle;

use crate::outbox_relay_common::discover_outbox_scopes;
use crate::{
    ApiAssemblyRuntime, ConversationOutboxRelayHandle, RtcOutboxRelayHandle,
    SocialOutboxRelayHandle, spawn_conversation_outbox_relay_from_env,
    spawn_rtc_outbox_relay_from_env, spawn_social_outbox_relay_from_env,
    wire_social_runtime_embedded_plane,
};

const OPS_OUTBOX_LAG_SAMPLE_INTERVAL: Duration = Duration::from_secs(5);
const OPS_OUTBOX_LAG_SCOPE_LIMIT: usize = 32;
const OUTBOX_LAG_SAMPLER_WORKER_ID: &str = "ops-outbox-lag-sampler";
/// Aggregate types relayed through `im_outbox_events` by the embedded relays.
const OUTBOX_LAG_AGGREGATE_TYPES: [&str; 3] = ["conversation", "rtc_session", "social"];

pub struct StandaloneRuntimeWiring {
    agent_dispatch_worker: AgentDispatchWorkerHandle,
    rtc_outbox_relay: Option<RtcOutboxRelayHandle>,
    conversation_outbox_relay: Option<ConversationOutboxRelayHandle>,
    social_outbox_relay: Option<SocialOutboxRelayHandle>,
    ops_outbox_lag_sampler: Option<JoinHandle<()>>,
    automation_agent_bridge: Option<crate::AutomationAgentBridgeHandle>,
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
        if let Some(task) = self.ops_outbox_lag_sampler {
            task.abort();
        }
        if let Some(handle) = self.automation_agent_bridge {
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
                pool.clone(),
            ),
        );
        let source_loader = Arc::new(MessageStoreAgentDispatchSourceLoader::new(message_store));
        let reply_committer = Arc::new(ConversationRuntimeAgentReplyCommitter::new(
            conversation_runtime.clone(),
        ));
        let worker = AgentDispatchWorker::new(
            integration_store,
            agents_session_facade.clone(),
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
            ops_outbox_lag_sampler: spawn_ops_outbox_lag_sampler(
                self.ops_runtime.clone(),
                Arc::new(im_adapters_postgres_journal::PostgresOutboxStore::from_pool(pool)),
            ),
            automation_agent_bridge: Some(crate::spawn_automation_agent_bridge(
                self.automation_runtime.clone(),
                agents_session_facade,
            )),
        })
    }
}

/// Samples the `im_outbox_events` backlog into real ops `/lag` items and the
/// side-effect outbox diagnostics view.
///
/// Runs on a dedicated task every [`OPS_OUTBOX_LAG_SAMPLE_INTERVAL`]: pending
/// scopes are discovered per relayed aggregate type (bounded by
/// [`OPS_OUTBOX_LAG_SCOPE_LIMIT`]), then each scope's pending count is read
/// from the store. `current_offset` is the pending backlog itself and
/// `committed_offset` is zero because the outbox position is a count, not a
/// sequence watermark; `lag` is the pending count either way. A `cluster`
/// item reports the summed backlog. The same counts feed
/// `side_effect_outboxes` so the health view reports real delivery pressure.
fn spawn_ops_outbox_lag_sampler(
    ops_runtime: Arc<OpsRuntime>,
    outbox: Arc<dyn OutboxStore>,
) -> Option<JoinHandle<()>> {
    Some(tokio::spawn(async move {
        loop {
            let result = sample_outbox_lag(ops_runtime.as_ref(), outbox.as_ref());
            if let Err(error) = result {
                tracing::warn!(
                    error = %format!("{error:?}"),
                    "ops outbox lag sampler tick failed"
                );
            }
            tokio::time::sleep(OPS_OUTBOX_LAG_SAMPLE_INTERVAL).await;
        }
    }))
}

fn sample_outbox_lag(
    ops_runtime: &OpsRuntime,
    outbox: &dyn OutboxStore,
) -> Result<(), ContractError> {
    let mut scopes: Vec<(String, String)> = Vec::new();
    for aggregate_type in OUTBOX_LAG_AGGREGATE_TYPES {
        for (tenant_id, organization_id) in discover_outbox_scopes(
            outbox,
            OUTBOX_LAG_SAMPLER_WORKER_ID,
            aggregate_type,
            OPS_OUTBOX_LAG_SCOPE_LIMIT,
        )? {
            if !scopes
                .iter()
                .any(|(existing_tenant, existing_org)| {
                    existing_tenant == &tenant_id && existing_org == &organization_id
                })
            {
                scopes.push((tenant_id, organization_id));
            }
        }
    }

    let mut items = Vec::with_capacity(scopes.len().saturating_add(1));
    let mut pending_total = 0_u64;
    for (tenant_id, organization_id) in scopes {
        let pending = outbox.count_pending(tenant_id.as_str(), organization_id.as_str())?;
        pending_total = pending_total.saturating_add(pending);
        items.push(LagItem {
            component: "outbox".to_owned(),
            scope_id: format!("{tenant_id}:{organization_id}"),
            current_offset: pending,
            committed_offset: 0,
            lag: pending,
        });
    }
    items.push(LagItem {
        component: "outbox".to_owned(),
        scope_id: "cluster".to_owned(),
        current_offset: pending_total,
        committed_offset: 0,
        lag: pending_total,
    });
    ops_runtime.upsert_lag_items(items);
    ops_runtime.update_side_effect_outboxes(vec![SideEffectOutboxDiagnosticsView {
        name: "outbox".to_owned(),
        status: if pending_total == 0 { "idle" } else { "pending" }.to_owned(),
        pending_count: pending_total,
        delivered_count: 0,
        failed_attempt_count: 0,
        oldest_pending_created_at: None,
    }]);
    Ok(())
}
