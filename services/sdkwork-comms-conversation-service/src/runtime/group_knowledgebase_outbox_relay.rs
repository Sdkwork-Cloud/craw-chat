//! Reliable delivery worker for group knowledgebase ACL and archive events.
//!
//! The Conversation aggregate writes the link projection and an IM outbox row
//! atomically. This worker is the only consumer for that aggregate type and
//! invokes the injected generated Knowledgebase SDK adapter through the
//! coordinator. It intentionally never logs payloads because roster snapshots
//! contain principal identifiers.

use std::sync::Arc;
use std::time::Duration;

use im_adapters_postgres_journal::{PostgresJournalPool, PostgresOutboxStore};
use im_platform_contracts::{OutboxEventClaim, OutboxStore};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use super::knowledgebase::{
    GroupKnowledgebaseReconciliationCursor, run_group_knowledgebase_blocking,
};
use super::{
    ConversationCommitJournal, ConversationRuntime, GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE,
    GroupKnowledgebaseCoordinator, GroupKnowledgebaseOutboxPayload, RuntimeError,
};

const GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS";
const GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_TENANT_ID_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_TENANT_ID";
const GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_ORGANIZATION_ID_ENV: &str =
    "SDKWORK_IM_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_ORGANIZATION_ID";
const DEFAULT_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS: u64 = 250;
const MIN_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS: u64 = 10;
const MAX_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS: u64 = 60_000;
const GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_BATCH_SIZE: usize = 64;
const GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_SCOPE_LIMIT: usize = 32;
const GROUP_KNOWLEDGEBASE_RECONCILIATION_BATCH_SIZE: usize = 32;
const GROUP_KNOWLEDGEBASE_OUTBOX_LEASE: Duration = Duration::from_secs(30);

pub struct GroupKnowledgebaseOutboxRelayHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

impl GroupKnowledgebaseOutboxRelayHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

impl Drop for GroupKnowledgebaseOutboxRelayHandle {
    fn drop(&mut self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

struct GroupKnowledgebaseOutboxRelayConfig {
    poll_interval: Duration,
    scope_override: Option<(String, String)>,
}

impl GroupKnowledgebaseOutboxRelayConfig {
    fn from_env() -> Result<Self, RuntimeError> {
        let raw_poll_interval = std::env::var(GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS_ENV)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        let poll_millis = match raw_poll_interval {
            Some(value) => value.parse::<u64>().map_err(|_| {
                RuntimeError::InvalidInput(format!(
                    "{GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS_ENV} must be an integer"
                ))
            })?,
            None => DEFAULT_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS,
        };
        if !(MIN_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS
            ..=MAX_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS)
            .contains(&poll_millis)
        {
            return Err(RuntimeError::InvalidInput(format!(
                "{GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS_ENV} must be between \
                 {MIN_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS} and \
                 {MAX_GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_POLL_MS}"
            )));
        }

        let tenant_id = optional_env(GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_TENANT_ID_ENV);
        let organization_id = optional_env(GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_ORGANIZATION_ID_ENV);
        let scope_override = match (tenant_id, organization_id) {
            (Some(tenant_id), Some(organization_id)) => Some((tenant_id, organization_id)),
            (None, None) => None,
            _ => {
                return Err(RuntimeError::InvalidInput(format!(
                    "{GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_TENANT_ID_ENV} and \
                     {GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_ORGANIZATION_ID_ENV} must be configured together"
                )));
            }
        };

        Ok(Self {
            poll_interval: Duration::from_millis(poll_millis),
            scope_override,
        })
    }
}

/// Starts the worker after validating the generated SDK adapter and shared
/// PostgreSQL pool. Callers retain the returned handle for process shutdown.
pub async fn spawn_group_knowledgebase_outbox_relay(
    coordinator: Arc<GroupKnowledgebaseCoordinator>,
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
) -> Result<GroupKnowledgebaseOutboxRelayHandle, RuntimeError> {
    coordinator.ensure_outbox_delivery_ready().await?;
    let config = GroupKnowledgebaseOutboxRelayConfig::from_env()?;
    let outbox = resolve_outbox_store()?;
    let mut reconciliation_cursor = GroupKnowledgebaseReconciliationCursor::default();
    reconcile_group_knowledgebase_durable_state(
        coordinator.clone(),
        runtime.clone(),
        &mut reconciliation_cursor,
        &config,
    )
    .await?;
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let task = tokio::spawn(run_group_knowledgebase_outbox_relay(
        outbox,
        coordinator,
        runtime,
        config,
        reconciliation_cursor,
        shutdown_rx,
    ));
    info!(
        aggregate_type = GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE,
        "group knowledgebase outbox relay started"
    );
    Ok(GroupKnowledgebaseOutboxRelayHandle {
        shutdown: shutdown_tx,
        task,
    })
}

fn resolve_outbox_store() -> Result<Arc<dyn OutboxStore>, RuntimeError> {
    let shared_pool =
        sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool().ok_or_else(|| {
            RuntimeError::Contract(im_platform_contracts::ContractError::Unavailable(
                "group knowledgebase outbox relay requires the shared PostgreSQL process pool"
                    .into(),
            ))
        })?;
    let pool = PostgresJournalPool::from_pool(shared_pool);
    Ok(Arc::new(PostgresOutboxStore::from_pool(pool)) as Arc<dyn OutboxStore>)
}

async fn run_group_knowledgebase_outbox_relay(
    outbox: Arc<dyn OutboxStore>,
    coordinator: Arc<GroupKnowledgebaseCoordinator>,
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
    config: GroupKnowledgebaseOutboxRelayConfig,
    mut reconciliation_cursor: GroupKnowledgebaseReconciliationCursor,
    mut shutdown: watch::Receiver<()>,
) {
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }

        if let Err(error) = reconcile_group_knowledgebase_durable_state(
            coordinator.clone(),
            runtime.clone(),
            &mut reconciliation_cursor,
            &config,
        )
        .await
        {
            // The cursor is only returned after successful links, so this
            // retains the failed link for the next bounded pass while delivery
            // of already-durable independent outbox events can continue.
            warn!(
                error = ?error,
                "group knowledgebase durable reconciliation failed; retaining cursor for retry"
            );
        }

        for (tenant_id, organization_id) in resolve_scopes(&outbox, &config) {
            let claims = match outbox.claim_pending(
                tenant_id.as_str(),
                organization_id.as_str(),
                GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE,
                GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_BATCH_SIZE,
                GROUP_KNOWLEDGEBASE_OUTBOX_LEASE,
            ) {
                Ok(claims) => claims,
                Err(error) => {
                    warn!(
                        tenant_id = tenant_id.as_str(),
                        organization_id = organization_id.as_str(),
                        error = ?error,
                        "group knowledgebase outbox relay claim failed"
                    );
                    continue;
                }
            };

            for claim in claims {
                relay_claim(&outbox, coordinator.as_ref(), claim).await;
            }
        }

        tokio::select! {
            _ = shutdown.changed() => break,
            _ = tokio::time::sleep(config.poll_interval) => {}
        }
    }
}

async fn reconcile_group_knowledgebase_durable_state(
    coordinator: Arc<GroupKnowledgebaseCoordinator>,
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
    cursor: &mut GroupKnowledgebaseReconciliationCursor,
    config: &GroupKnowledgebaseOutboxRelayConfig,
) -> Result<usize, RuntimeError> {
    let prior_cursor = cursor.clone();
    let scope_override = config.scope_override.clone();
    let reconciliation_coordinator = coordinator.clone();
    let reconciliation_runtime = runtime.clone();
    let (next_cursor, reconciled) =
        run_group_knowledgebase_blocking("durable reconciliation", move || {
            let mut next_cursor = prior_cursor;
            let reconciled = match scope_override.as_ref() {
                Some((tenant_id, organization_id)) => reconciliation_coordinator
                    .reconcile_durable_state_in_scope(
                        reconciliation_runtime.as_ref(),
                        &mut next_cursor,
                        GROUP_KNOWLEDGEBASE_RECONCILIATION_BATCH_SIZE,
                        Some((tenant_id.as_str(), organization_id.as_str())),
                    )?,
                None => reconciliation_coordinator.reconcile_durable_state(
                    reconciliation_runtime.as_ref(),
                    &mut next_cursor,
                    GROUP_KNOWLEDGEBASE_RECONCILIATION_BATCH_SIZE,
                )?,
            };
            Ok((next_cursor, reconciled))
        })
        .await?;
    *cursor = next_cursor;
    coordinator
        .recover_pending_provisioning(runtime, cursor)
        .await?;
    Ok(reconciled)
}

fn resolve_scopes(
    outbox: &Arc<dyn OutboxStore>,
    config: &GroupKnowledgebaseOutboxRelayConfig,
) -> Vec<(String, String)> {
    if let Some(scope) = config.scope_override.as_ref() {
        return vec![scope.clone()];
    }

    match outbox.list_pending_scopes(
        GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE,
        GROUP_KNOWLEDGEBASE_OUTBOX_RELAY_SCOPE_LIMIT,
    ) {
        Ok(scopes) => scopes,
        Err(error) => {
            warn!(error = ?error, "group knowledgebase outbox relay scope discovery failed");
            Vec::new()
        }
    }
}

async fn relay_claim(
    outbox: &Arc<dyn OutboxStore>,
    coordinator: &GroupKnowledgebaseCoordinator,
    claim: OutboxEventClaim,
) {
    let event = &claim.event;
    if event.aggregate_type != GROUP_KNOWLEDGEBASE_OUTBOX_AGGREGATE_TYPE {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            aggregate_type = event.aggregate_type.as_str(),
            "group knowledgebase outbox relay rejected an unexpected aggregate type"
        );
        mark_failed(
            outbox,
            &claim,
            "unexpected group knowledgebase aggregate type",
        );
        return;
    }

    let payload = match serde_json::from_str::<GroupKnowledgebaseOutboxPayload>(
        event.payload_json.as_str(),
    ) {
        Ok(payload) => payload,
        Err(_) => {
            warn!(
                outbox_id = event.outbox_id.as_str(),
                event_id = event.event_id.as_str(),
                "group knowledgebase outbox relay rejected an invalid payload"
            );
            mark_failed(outbox, &claim, "invalid group knowledgebase outbox payload");
            return;
        }
    };

    if let Err(error) = coordinator.deliver_outbox_payload(payload).await {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            event_id = event.event_id.as_str(),
            error = ?error,
            "group knowledgebase outbox delivery failed"
        );
        mark_failed(outbox, &claim, "group knowledgebase outbox delivery failed");
        return;
    }

    if let Err(error) = outbox.mark_published(&claim) {
        warn!(
            outbox_id = event.outbox_id.as_str(),
            error = ?error,
            "group knowledgebase outbox relay could not mark event published"
        );
    }
}

fn mark_failed(outbox: &Arc<dyn OutboxStore>, claim: &OutboxEventClaim, reason: &str) {
    if let Err(error) = outbox.mark_failed(claim, reason) {
        warn!(
            outbox_id = claim.event.outbox_id.as_str(),
            error = ?error,
            "group knowledgebase outbox relay could not mark event failed"
        );
    }
}

fn optional_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
