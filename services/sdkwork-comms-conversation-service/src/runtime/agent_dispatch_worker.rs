use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use im_domain_core::message::{ContentPart, MessageBody, MessageType, Sender};
use im_platform_contracts::{
    AgentBindingStatus, AgentDispatchRecord, AgentDispatchReplyCompletion, AgentIntegrationStore,
    AgentReplyCommitResult, ContractError, ConversationAgentBindingRecord, MessageStore,
};
use sdkwork_agents_runtime_facade::{
    AgentsSessionActor, AgentsSessionEntrySurface, AgentsSessionFacade, AgentsSessionKind,
    AgentsTurnSnapshot, AgentsTurnStatus, CompleteAgentsTurnRequest,
    GetAgentsTurnByIdempotencyRequest, ResolveAgentsSessionRequest,
};
use sdkwork_im_contract_message::CommitJournal;
use sdkwork_utils_rust::sha256_hash;
use tokio::sync::watch;
use tokio::task::JoinHandle;

const AGENT_DISPATCH_BATCH_SIZE_ENV: &str = "SDKWORK_IM_AGENT_DISPATCH_BATCH_SIZE";
const AGENT_DISPATCH_LEASE_SECONDS_ENV: &str = "SDKWORK_IM_AGENT_DISPATCH_LEASE_SECONDS";
const AGENT_DISPATCH_POLL_INTERVAL_MS_ENV: &str = "SDKWORK_IM_AGENT_DISPATCH_POLL_INTERVAL_MS";
const AGENT_DISPATCH_RETRY_SECONDS_ENV: &str = "SDKWORK_IM_AGENT_DISPATCH_RETRY_SECONDS";
const AGENT_DISPATCH_WORKER_ID_ENV: &str = "SDKWORK_IM_AGENT_DISPATCH_WORKER_ID";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDispatchWorkerConfig {
    pub batch_size: usize,
    pub lease_seconds: u64,
    pub poll_interval_ms: u64,
    pub retry_seconds: u64,
}

impl Default for AgentDispatchWorkerConfig {
    fn default() -> Self {
        Self {
            batch_size: 20,
            lease_seconds: 90,
            poll_interval_ms: 500,
            retry_seconds: 5,
        }
    }
}

impl AgentDispatchWorkerConfig {
    pub fn from_env() -> Result<Self, String> {
        let defaults = Self::default();
        Ok(Self {
            batch_size: read_bounded_env_usize(
                AGENT_DISPATCH_BATCH_SIZE_ENV,
                defaults.batch_size,
                1,
                100,
            )?,
            lease_seconds: read_bounded_env_u64(
                AGENT_DISPATCH_LEASE_SECONDS_ENV,
                defaults.lease_seconds,
                15,
                900,
            )?,
            poll_interval_ms: read_bounded_env_u64(
                AGENT_DISPATCH_POLL_INTERVAL_MS_ENV,
                defaults.poll_interval_ms,
                50,
                60_000,
            )?,
            retry_seconds: read_bounded_env_u64(
                AGENT_DISPATCH_RETRY_SECONDS_ENV,
                defaults.retry_seconds,
                1,
                3600,
            )?,
        })
    }
}

pub fn resolve_agent_dispatch_worker_id() -> Result<String, String> {
    if let Ok(value) = std::env::var(AGENT_DISPATCH_WORKER_ID_ENV) {
        let value = value.trim();
        if !value.is_empty() && value.len() <= 128 {
            return Ok(value.to_owned());
        }
        return Err(format!(
            "{AGENT_DISPATCH_WORKER_ID_ENV} must contain 1 to 128 bytes"
        ));
    }
    let host = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "local".into())
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || "._-".contains(*character))
        .take(80)
        .collect::<String>();
    Ok(format!("im-agent-dispatch:{host}:{}", std::process::id()))
}

pub struct AgentDispatchWorkerHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
    healthy: Arc<AtomicBool>,
}

impl AgentDispatchWorkerHandle {
    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Acquire)
    }

    pub fn health_signal(&self) -> Arc<AtomicBool> {
        self.healthy.clone()
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown.send(());
        let _ = self.task.await;
    }
}

pub fn spawn_agent_dispatch_worker(
    worker: AgentDispatchWorker,
    config: AgentDispatchWorkerConfig,
) -> AgentDispatchWorkerHandle {
    let worker = Arc::new(worker);
    let healthy = Arc::new(AtomicBool::new(true));
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let task_healthy = healthy.clone();
    let task = tokio::spawn(run_agent_dispatch_worker(
        worker,
        config,
        shutdown_rx,
        task_healthy,
    ));
    AgentDispatchWorkerHandle {
        shutdown: shutdown_tx,
        task,
        healthy,
    }
}

async fn run_agent_dispatch_worker(
    worker: Arc<AgentDispatchWorker>,
    config: AgentDispatchWorkerConfig,
    mut shutdown: watch::Receiver<()>,
    healthy: Arc<AtomicBool>,
) {
    loop {
        if shutdown.has_changed().unwrap_or(true) {
            break;
        }
        let now = im_time::utc_now_rfc3339_millis();
        let lease_expires_at = im_time::rfc3339_add_secs(&now, config.lease_seconds as i64)
            .expect("validated dispatch lease duration should produce an RFC3339 instant");
        let retry_at = im_time::rfc3339_add_secs(&now, config.retry_seconds as i64)
            .expect("validated dispatch retry duration should produce an RFC3339 instant");
        let batch_worker = worker.clone();
        let batch_size = config.batch_size;
        match tokio::task::spawn_blocking(move || {
            batch_worker.process_global_batch(&now, &lease_expires_at, &retry_at, batch_size)
        })
        .await
        {
            Ok(Ok(outcomes)) => {
                healthy.store(true, Ordering::Release);
                for outcome in outcomes {
                    tracing::info!(outcome = ?outcome, "processed IM agent dispatch");
                }
            }
            Ok(Err(error)) => {
                healthy.store(false, Ordering::Release);
                tracing::error!(error = ?error, "IM agent dispatch batch failed");
            }
            Err(error) => {
                healthy.store(false, Ordering::Release);
                tracing::error!(error = %error, "IM agent dispatch worker task failed");
            }
        }
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(config.poll_interval_ms)) => {}
            _ = shutdown.changed() => break,
        }
    }
    healthy.store(false, Ordering::Release);
}

fn read_bounded_env_u64(key: &str, default: u64, min: u64, max: u64) -> Result<u64, String> {
    let value = match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => value
            .trim()
            .parse::<u64>()
            .map_err(|_| format!("{key} must be an integer"))?,
        _ => default,
    };
    if !(min..=max).contains(&value) {
        return Err(format!("{key} must be between {min} and {max}"));
    }
    Ok(value)
}

fn read_bounded_env_usize(
    key: &str,
    default: usize,
    min: usize,
    max: usize,
) -> Result<usize, String> {
    let value = read_bounded_env_u64(key, default as u64, min as u64, max as u64)?;
    usize::try_from(value).map_err(|_| format!("{key} is outside usize range"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDispatchSource {
    pub content: String,
    pub content_type: String,
}

pub trait AgentDispatchSourceLoader: Send + Sync {
    fn load_source(&self, dispatch: &AgentDispatchRecord) -> Result<AgentDispatchSource, String>;
}

pub trait AgentReplyCommitter: Send + Sync {
    fn commit_reply_and_complete(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        agents_session_id: &str,
        agents_turn_id: &str,
        content: &str,
    ) -> Result<AgentReplyCommitResult, String>;
}

pub struct MessageStoreAgentDispatchSourceLoader {
    store: Arc<dyn MessageStore>,
}

impl MessageStoreAgentDispatchSourceLoader {
    pub fn new(store: Arc<dyn MessageStore>) -> Self {
        Self { store }
    }
}

impl AgentDispatchSourceLoader for MessageStoreAgentDispatchSourceLoader {
    fn load_source(&self, dispatch: &AgentDispatchRecord) -> Result<AgentDispatchSource, String> {
        let source_message_id = i64::try_from(dispatch.source_message_id)
            .map_err(|_| "source IM message id is outside int64 range".to_string())?;
        let tenant_id = dispatch.tenant_id.to_string();
        let stored = self
            .store
            .read_message_by_id(&tenant_id, source_message_id)
            .map_err(|error| format!("source IM message lookup failed: {error:?}"))?
            .ok_or_else(|| "source IM message was not found".to_string())?;
        let stored_organization_id = stored
            .organization_id
            .parse::<u64>()
            .map_err(|_| "source IM message organization is invalid".to_string())?;
        if stored.tenant_id != tenant_id
            || stored_organization_id != dispatch.organization_id
            || stored.conversation_id != dispatch.conversation_id
            || stored.message_id != source_message_id
            || stored.message_seq != dispatch.source_message_seq
            || stored.sender_principal_kind != "user"
            || stored.sender_principal_id != dispatch.requested_by.to_string()
            || stored.message_type != MessageType::Standard.as_wire_value()
            || stored.deleted_at.is_some()
        {
            return Err("source IM message identity does not match the dispatch".into());
        }
        let body = serde_json::from_str::<MessageBody>(&stored.payload_json)
            .map_err(|_| "source IM message body is invalid".to_string())?;
        let content = body
            .parts
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text(part) => {
                    let text = part.text.trim();
                    (!text.is_empty()).then_some(text)
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if content.is_empty() {
            return Err("source IM message has no dispatchable text".into());
        }
        if content.len() > 512 * 1024 {
            return Err("source IM message text exceeds the dispatch limit".into());
        }
        Ok(AgentDispatchSource {
            content,
            content_type: "text/plain".into(),
        })
    }
}

pub struct ConversationRuntimeAgentReplyCommitter<J> {
    runtime: Arc<super::ConversationRuntime<J>>,
}

impl<J> ConversationRuntimeAgentReplyCommitter<J> {
    pub fn new(runtime: Arc<super::ConversationRuntime<J>>) -> Self {
        Self { runtime }
    }
}

impl<J> AgentReplyCommitter for ConversationRuntimeAgentReplyCommitter<J>
where
    J: CommitJournal + Send + Sync + 'static,
{
    fn commit_reply_and_complete(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        agents_session_id: &str,
        agents_turn_id: &str,
        content: &str,
    ) -> Result<AgentReplyCommitResult, String> {
        if content.trim().is_empty() {
            return Err("Agents reply content is empty".into());
        }
        self.runtime
            .post_agent_dispatch_reply(
                super::PostMessageCommand {
                    tenant_id: dispatch.tenant_id.to_string(),
                    organization_id: dispatch.organization_id.to_string(),
                    conversation_id: dispatch.conversation_id.clone(),
                    sender: Sender {
                        id: dispatch.agent_id.clone(),
                        kind: "agent".into(),
                        member_id: None,
                        device_id: None,
                        session_id: Some(agents_session_id.to_owned()),
                        metadata: BTreeMap::new(),
                    },
                    client_msg_id: Some(format!("agent-dispatch-reply:{}", dispatch.dispatch_id)),
                    message_type: MessageType::Standard,
                    body: MessageBody {
                        summary: None,
                        parts: vec![ContentPart::text(content)],
                        render_hints: BTreeMap::new(),
                        reply_to: None,
                    },
                },
                AgentDispatchReplyCompletion {
                    tenant_id: dispatch.tenant_id,
                    organization_id: dispatch.organization_id,
                    conversation_id: dispatch.conversation_id.clone(),
                    dispatch_id: dispatch.dispatch_id.clone(),
                    lease_owner: lease_owner.to_owned(),
                    agent_id: dispatch.agent_id.clone(),
                    agent_revision_ref: dispatch.agent_revision_ref.clone(),
                    assignment_generation: dispatch.assignment_generation,
                    agents_session_id: agents_session_id.to_owned(),
                    agents_turn_id: agents_turn_id.to_owned(),
                },
            )
            .map_err(|error| format!("commit IM agent reply failed: {error:?}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentDispatchWorkerOutcome {
    Completed {
        dispatch_id: String,
        turn_id: String,
    },
    RetryScheduled {
        dispatch_id: String,
    },
    ReconciliationDeferred {
        dispatch_id: String,
        turn_id: Option<String>,
    },
    DeadLettered {
        dispatch_id: String,
    },
}

enum AgentDispatchExecution {
    Completed(String),
    ReconciliationDeferred {
        turn_id: Option<String>,
        detail: String,
    },
}

pub struct AgentDispatchWorker {
    store: Arc<dyn AgentIntegrationStore>,
    agents: Arc<dyn AgentsSessionFacade>,
    source_loader: Arc<dyn AgentDispatchSourceLoader>,
    reply_committer: Arc<dyn AgentReplyCommitter>,
    lease_owner: String,
}

impl AgentDispatchWorker {
    pub fn new(
        store: Arc<dyn AgentIntegrationStore>,
        agents: Arc<dyn AgentsSessionFacade>,
        source_loader: Arc<dyn AgentDispatchSourceLoader>,
        reply_committer: Arc<dyn AgentReplyCommitter>,
        lease_owner: impl Into<String>,
    ) -> Result<Self, String> {
        let lease_owner = lease_owner.into();
        if lease_owner.trim().is_empty() || lease_owner.len() > 128 {
            return Err("agent dispatch lease owner is invalid".into());
        }
        Ok(Self {
            store,
            agents,
            source_loader,
            reply_committer,
            lease_owner,
        })
    }

    pub fn process_batch(
        &self,
        tenant_id: u64,
        organization_id: u64,
        now: &str,
        lease_expires_at: &str,
        retry_at: &str,
        limit: usize,
    ) -> Result<Vec<AgentDispatchWorkerOutcome>, ContractError> {
        self.store
            .claim_dispatches(
                tenant_id,
                organization_id,
                &self.lease_owner,
                now,
                lease_expires_at,
                limit,
            )?
            .into_iter()
            .map(|dispatch| self.process_claim(dispatch, now, retry_at))
            .collect()
    }

    pub fn process_global_batch(
        &self,
        now: &str,
        lease_expires_at: &str,
        retry_at: &str,
        limit: usize,
    ) -> Result<Vec<AgentDispatchWorkerOutcome>, ContractError> {
        self.store
            .claim_dispatches_global(&self.lease_owner, now, lease_expires_at, limit)?
            .into_iter()
            .map(|dispatch| self.process_claim(dispatch, now, retry_at))
            .collect()
    }

    fn process_claim(
        &self,
        dispatch: AgentDispatchRecord,
        now: &str,
        retry_at: &str,
    ) -> Result<AgentDispatchWorkerOutcome, ContractError> {
        match self.execute_claim(&dispatch, now) {
            Ok(AgentDispatchExecution::Completed(turn_id)) => {
                Ok(AgentDispatchWorkerOutcome::Completed {
                    dispatch_id: dispatch.dispatch_id.clone(),
                    turn_id,
                })
            }
            Ok(AgentDispatchExecution::ReconciliationDeferred { turn_id, detail }) => {
                let detail = sanitize_error_detail(&detail);
                self.store.defer_dispatch_reconciliation(
                    &dispatch,
                    &self.lease_owner,
                    turn_id.as_deref(),
                    &detail,
                    retry_at,
                    now,
                )?;
                Ok(AgentDispatchWorkerOutcome::ReconciliationDeferred {
                    dispatch_id: dispatch.dispatch_id.clone(),
                    turn_id,
                })
            }
            Err(error) => {
                let detail = sanitize_error_detail(&error);
                let status = self.store.fail_dispatch(
                    &dispatch,
                    &self.lease_owner,
                    "agents_dispatch_failed",
                    &detail,
                    retry_at,
                    now,
                )?;
                if status == im_platform_contracts::AgentDispatchStatus::DeadLetter {
                    Ok(AgentDispatchWorkerOutcome::DeadLettered {
                        dispatch_id: dispatch.dispatch_id.clone(),
                    })
                } else {
                    Ok(AgentDispatchWorkerOutcome::RetryScheduled {
                        dispatch_id: dispatch.dispatch_id.clone(),
                    })
                }
            }
        }
    }

    fn execute_claim(
        &self,
        dispatch: &AgentDispatchRecord,
        now: &str,
    ) -> Result<AgentDispatchExecution, String> {
        let source = self.source_loader.load_source(dispatch)?;
        if source.content.trim().is_empty() {
            return Err("source IM message has no dispatchable text".into());
        }
        let binding = self.resolve_or_create_binding(dispatch, now)?;
        let agents_session_id = binding
            .agents_session_id
            .as_deref()
            .ok_or_else(|| "active binding is missing Agents session id".to_string())?;
        if dispatch
            .agents_session_id
            .as_deref()
            .is_some_and(|existing| existing != agents_session_id)
        {
            return Err("dispatch Agents session does not match the active binding".into());
        }
        self.store
            .mark_dispatch_running(
                dispatch,
                &self.lease_owner,
                &binding.binding_id,
                agents_session_id,
                now,
            )
            .map_err(|error| format!("mark dispatch running failed: {error:?}"))?;

        if dispatch.agents_session_id.is_some() {
            match self.lookup_turn(dispatch, agents_session_id) {
                Ok(Some(snapshot)) => {
                    return self.handle_turn_snapshot(dispatch, agents_session_id, snapshot);
                }
                Ok(None) => {}
                Err(error) => {
                    return Ok(AgentDispatchExecution::ReconciliationDeferred {
                        turn_id: dispatch.agents_turn_id.clone(),
                        detail: format!("Agents turn reconciliation lookup failed: {error}"),
                    });
                }
            }
        }

        let request = CompleteAgentsTurnRequest {
            tenant_id: dispatch.tenant_id,
            organization_id: dispatch.organization_id,
            owner_user_id: dispatch.requested_by,
            agent_id: dispatch.agent_id.clone(),
            session_id: agents_session_id.to_owned(),
            content: source.content,
            content_type: source.content_type,
            idempotency_key: dispatch.idempotency_key.clone(),
            client_request_id: dispatch.dispatch_id.clone(),
            actor: trusted_actor(),
            requested_at: dispatch.created_at.clone(),
        };
        let completed = match self.agents.complete_turn(request) {
            Ok(completed) => completed,
            Err(completion_error) => {
                return match self.lookup_turn(dispatch, agents_session_id) {
                    Ok(Some(snapshot)) => {
                        self.handle_turn_snapshot(dispatch, agents_session_id, snapshot)
                    }
                    Ok(None) => Err(format!("Agents turn failed: {completion_error}")),
                    Err(lookup_error) => Ok(AgentDispatchExecution::ReconciliationDeferred {
                        turn_id: dispatch.agents_turn_id.clone(),
                        detail: format!(
                            "Agents turn outcome is indeterminate after completion error; lookup failed: {lookup_error}"
                        ),
                    }),
                };
            }
        };
        self.reply_committer.commit_reply_and_complete(
            dispatch,
            &self.lease_owner,
            agents_session_id,
            &completed.turn_id,
            &completed.response_content,
        )?;
        Ok(AgentDispatchExecution::Completed(completed.turn_id))
    }

    fn lookup_turn(
        &self,
        dispatch: &AgentDispatchRecord,
        agents_session_id: &str,
    ) -> sdkwork_agents_runtime_facade::RuntimeFacadeResult<Option<AgentsTurnSnapshot>> {
        self.agents
            .get_turn_by_idempotency(GetAgentsTurnByIdempotencyRequest {
                tenant_id: dispatch.tenant_id,
                organization_id: dispatch.organization_id,
                owner_user_id: dispatch.requested_by,
                agent_id: dispatch.agent_id.clone(),
                session_id: agents_session_id.to_owned(),
                idempotency_key: dispatch.idempotency_key.clone(),
                actor: trusted_actor(),
            })
    }

    fn handle_turn_snapshot(
        &self,
        dispatch: &AgentDispatchRecord,
        agents_session_id: &str,
        snapshot: AgentsTurnSnapshot,
    ) -> Result<AgentDispatchExecution, String> {
        if snapshot.session_id != agents_session_id {
            return Err("Agents turn snapshot session mismatch".into());
        }
        match snapshot.status {
            AgentsTurnStatus::Requested | AgentsTurnStatus::Running => {
                Ok(AgentDispatchExecution::ReconciliationDeferred {
                    turn_id: Some(snapshot.turn_id),
                    detail: "Agents turn remains in progress".into(),
                })
            }
            AgentsTurnStatus::Completed => {
                let content = snapshot.response_content.as_deref().ok_or_else(|| {
                    "completed Agents turn snapshot is missing response content".to_string()
                })?;
                self.reply_committer.commit_reply_and_complete(
                    dispatch,
                    &self.lease_owner,
                    agents_session_id,
                    &snapshot.turn_id,
                    content,
                )?;
                Ok(AgentDispatchExecution::Completed(snapshot.turn_id))
            }
            AgentsTurnStatus::Failed => Err(format!(
                "Agents turn reached failed state{}",
                snapshot
                    .error_code
                    .as_deref()
                    .map(|code| format!(" ({code})"))
                    .unwrap_or_default()
            )),
            AgentsTurnStatus::Cancelled => Err("Agents turn reached cancelled state".into()),
        }
    }

    fn resolve_or_create_binding(
        &self,
        dispatch: &AgentDispatchRecord,
        now: &str,
    ) -> Result<ConversationAgentBindingRecord, String> {
        if let Some(binding) = self
            .store
            .resolve_binding(
                dispatch.tenant_id,
                dispatch.organization_id,
                &dispatch.conversation_id,
                &dispatch.agent_id,
                dispatch.assignment_generation,
            )
            .map_err(|error| format!("binding lookup failed: {error:?}"))?
            && binding.status == AgentBindingStatus::Active
        {
            return Ok(binding);
        }

        let binding_id = deterministic_binding_id(dispatch);
        let binding_payload_hash = sha256_hash(
            format!(
                "{}:{}:{}:{}:{}",
                dispatch.tenant_id,
                dispatch.organization_id,
                dispatch.conversation_id,
                dispatch.agent_id,
                dispatch.assignment_generation
            )
            .as_bytes(),
        );
        let mut binding = self
            .store
            .save_binding(ConversationAgentBindingRecord {
                binding_id: binding_id.clone(),
                tenant_id: dispatch.tenant_id,
                organization_id: dispatch.organization_id,
                conversation_id: dispatch.conversation_id.clone(),
                agent_id: dispatch.agent_id.clone(),
                agent_revision_ref: dispatch.agent_revision_ref.clone(),
                assignment_generation: dispatch.assignment_generation,
                agents_session_id: None,
                status: AgentBindingStatus::Pending,
                idempotency_key: format!("im-agent-binding:{binding_id}"),
                payload_hash: binding_payload_hash.clone(),
                created_by: dispatch.requested_by,
                updated_by: dispatch.requested_by,
                last_error_code: None,
                last_error_detail: None,
                version: 0,
                created_at: now.to_owned(),
                updated_at: now.to_owned(),
            })
            .map_err(|error| format!("binding create failed: {error:?}"))?;
        if binding.status == AgentBindingStatus::Active {
            return Ok(binding);
        }
        let session_id = deterministic_agents_session_id(dispatch);
        let resolved = self
            .agents
            .resolve_or_create_session(ResolveAgentsSessionRequest {
                tenant_id: dispatch.tenant_id,
                organization_id: dispatch.organization_id,
                owner_user_id: dispatch.requested_by,
                agent_id: dispatch.agent_id.clone(),
                session_id,
                project_id: None,
                session_kind: AgentsSessionKind::ImDispatch,
                entry_surface: AgentsSessionEntrySurface::ImDispatch,
                source_module: Some("sdkwork-im".into()),
                source_context_kind: Some("conversation".into()),
                source_context_id: Some(dispatch.conversation_id.clone()),
                parent_session_id: None,
                forked_from_turn_id: None,
                title: format!("IM {}", dispatch.conversation_id),
                idempotency_key: binding.idempotency_key.clone(),
                payload_hash: binding.payload_hash.clone(),
                runtime_binding: None,
                actor: trusted_actor(),
                requested_at: now.to_owned(),
            })
            .map_err(|error| format!("Agents session resolve failed: {error}"))?;
        binding.status = AgentBindingStatus::Active;
        binding.agents_session_id = Some(resolved.session_id);
        binding.version = binding.version.saturating_add(1);
        binding.updated_at = now.to_owned();
        self.store
            .save_binding(binding)
            .map_err(|error| format!("binding activation failed: {error:?}"))
    }
}

fn trusted_actor() -> AgentsSessionActor {
    AgentsSessionActor {
        subject_id: "service.sdkwork-im.agent-dispatch".into(),
        roles: vec!["ai.agents.manage".into()],
    }
}

fn deterministic_binding_id(dispatch: &AgentDispatchRecord) -> String {
    let hash = sha256_hash(
        format!(
            "{}:{}:{}:{}:{}",
            dispatch.tenant_id,
            dispatch.organization_id,
            dispatch.conversation_id,
            dispatch.agent_id,
            dispatch.assignment_generation
        )
        .as_bytes(),
    );
    format!("binding.{}", &hash[..32])
}

fn deterministic_agents_session_id(dispatch: &AgentDispatchRecord) -> String {
    let hash = sha256_hash(
        format!(
            "{}:{}:{}:{}",
            dispatch.tenant_id,
            dispatch.organization_id,
            dispatch.conversation_id,
            dispatch.agent_id
        )
        .as_bytes(),
    );
    format!("session.im.{}", &hash[..32])
}

fn sanitize_error_detail(error: &str) -> String {
    error
        .replace(['\r', '\n'], " ")
        .chars()
        .take(2048)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use im_platform_contracts::{
        AgentAssignmentSource, AgentDispatchStatus, ConversationAgentConversationStateRecord,
        MessageWindow, ReplaceConversationAgentConversationState, StoredMessageRecord,
    };
    use sdkwork_agents_runtime_facade::{
        CompletedAgentsTurn, ResolvedAgentsSession, RuntimeFacadeError,
    };

    use super::*;

    struct FakeStore {
        dispatch: Mutex<Option<AgentDispatchRecord>>,
        binding: Mutex<Option<ConversationAgentBindingRecord>>,
        failure_detail: Mutex<Option<String>>,
        deferred_turn_id: Mutex<Option<String>>,
        defer_calls: AtomicUsize,
        complete_calls: AtomicUsize,
    }

    impl FakeStore {
        fn new(dispatch: AgentDispatchRecord) -> Self {
            Self {
                dispatch: Mutex::new(Some(dispatch)),
                binding: Mutex::new(None),
                failure_detail: Mutex::new(None),
                deferred_turn_id: Mutex::new(None),
                defer_calls: AtomicUsize::new(0),
                complete_calls: AtomicUsize::new(0),
            }
        }
    }

    impl AgentIntegrationStore for FakeStore {
        fn replace_conversation_agents(
            &self,
            _command: ReplaceConversationAgentConversationState,
        ) -> Result<(), ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn list_conversation_agents(
            &self,
            _tenant_id: u64,
            _organization_id: u64,
            _conversation_id: &str,
            _limit: usize,
        ) -> Result<Vec<ConversationAgentConversationStateRecord>, ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn enqueue_dispatches(
            &self,
            _request: &im_platform_contracts::AgentMentionDispatchRequest,
            _max_attempts: u32,
        ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn claim_dispatches(
            &self,
            _tenant_id: u64,
            _organization_id: u64,
            lease_owner: &str,
            _now: &str,
            lease_expires_at: &str,
            _limit: usize,
        ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
            self.claim_dispatches_global(lease_owner, "", lease_expires_at, 1)
        }

        fn claim_dispatches_global(
            &self,
            lease_owner: &str,
            _now: &str,
            lease_expires_at: &str,
            _limit: usize,
        ) -> Result<Vec<AgentDispatchRecord>, ContractError> {
            let Some(mut dispatch) = self.dispatch.lock().expect("dispatch lock").take() else {
                return Ok(Vec::new());
            };
            dispatch.status = AgentDispatchStatus::Leased;
            dispatch.lease_owner = Some(lease_owner.into());
            dispatch.lease_expires_at = Some(lease_expires_at.into());
            dispatch.attempt_count += 1;
            Ok(vec![dispatch])
        }

        fn resolve_binding(
            &self,
            _tenant_id: u64,
            _organization_id: u64,
            _conversation_id: &str,
            _agent_id: &str,
            _assignment_generation: u64,
        ) -> Result<Option<ConversationAgentBindingRecord>, ContractError> {
            Ok(self.binding.lock().expect("binding lock").clone())
        }

        fn save_binding(
            &self,
            binding: ConversationAgentBindingRecord,
        ) -> Result<ConversationAgentBindingRecord, ContractError> {
            *self.binding.lock().expect("binding lock") = Some(binding.clone());
            Ok(binding)
        }

        fn mark_dispatch_running(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _binding_id: &str,
            _agents_session_id: &str,
            _updated_at: &str,
        ) -> Result<(), ContractError> {
            Ok(())
        }

        fn complete_dispatch(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _agents_turn_id: &str,
            _reply: AgentReplyCommitResult,
            _completed_at: &str,
        ) -> Result<(), ContractError> {
            self.complete_calls.fetch_add(1, Ordering::SeqCst);
            Err(ContractError::Conflict(
                "worker must use the atomic reply committer".into(),
            ))
        }

        fn defer_dispatch_reconciliation(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            agents_turn_id: Option<&str>,
            _detail: &str,
            _next_attempt_at: &str,
            _updated_at: &str,
        ) -> Result<(), ContractError> {
            self.defer_calls.fetch_add(1, Ordering::SeqCst);
            *self.deferred_turn_id.lock().expect("deferred turn lock") =
                agents_turn_id.map(ToOwned::to_owned);
            Ok(())
        }

        fn fail_dispatch(
            &self,
            dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _error_code: &str,
            error_detail: &str,
            _next_attempt_at: &str,
            _updated_at: &str,
        ) -> Result<AgentDispatchStatus, ContractError> {
            *self.failure_detail.lock().expect("failure lock") = Some(error_detail.into());
            Ok(if dispatch.attempt_count >= dispatch.max_attempts {
                AgentDispatchStatus::DeadLetter
            } else {
                AgentDispatchStatus::Failed
            })
        }
    }

    struct FakeAgents {
        fail_complete: bool,
        lookup_snapshot: Option<AgentsTurnSnapshot>,
        complete_calls: Arc<AtomicUsize>,
    }

    impl AgentsSessionFacade for FakeAgents {
        fn resolve_or_create_session(
            &self,
            request: ResolveAgentsSessionRequest,
        ) -> Result<ResolvedAgentsSession, RuntimeFacadeError> {
            assert_eq!(
                request.actor.subject_id,
                "service.sdkwork-im.agent-dispatch"
            );
            assert_eq!(request.owner_user_id, 1001);
            Ok(ResolvedAgentsSession {
                session_id: request.session_id,
                created: true,
                version: 1,
            })
        }

        fn complete_turn(
            &self,
            request: CompleteAgentsTurnRequest,
        ) -> Result<CompletedAgentsTurn, RuntimeFacadeError> {
            self.complete_calls.fetch_add(1, Ordering::SeqCst);
            assert_eq!(
                request.actor.subject_id,
                "service.sdkwork-im.agent-dispatch"
            );
            assert_eq!(request.owner_user_id, 1001);
            if self.fail_complete {
                return Err(RuntimeFacadeError::Kernel(format!(
                    "provider timeout\n{}",
                    "x".repeat(4096)
                )));
            }
            Ok(CompletedAgentsTurn {
                session_id: request.session_id,
                turn_id: "turn.worker.1".into(),
                request_message_id: "message.request.1".into(),
                response_message_id: "message.response.1".into(),
                response_content: "agent answer".into(),
            })
        }

        fn get_turn_by_idempotency(
            &self,
            request: GetAgentsTurnByIdempotencyRequest,
        ) -> Result<Option<AgentsTurnSnapshot>, RuntimeFacadeError> {
            assert_eq!(
                request.actor.subject_id,
                "service.sdkwork-im.agent-dispatch"
            );
            assert_eq!(request.owner_user_id, 1001);
            Ok(self.lookup_snapshot.clone())
        }
    }

    fn fake_agents(
        fail_complete: bool,
        lookup_snapshot: Option<AgentsTurnSnapshot>,
    ) -> (Arc<FakeAgents>, Arc<AtomicUsize>) {
        let complete_calls = Arc::new(AtomicUsize::new(0));
        (
            Arc::new(FakeAgents {
                fail_complete,
                lookup_snapshot,
                complete_calls: complete_calls.clone(),
            }),
            complete_calls,
        )
    }

    struct FakeSource;

    impl AgentDispatchSourceLoader for FakeSource {
        fn load_source(
            &self,
            _dispatch: &AgentDispatchRecord,
        ) -> Result<AgentDispatchSource, String> {
            Ok(AgentDispatchSource {
                content: "hello agent".into(),
                content_type: "text/plain".into(),
            })
        }
    }

    struct FakeReply {
        calls: AtomicUsize,
    }

    impl AgentReplyCommitter for FakeReply {
        fn commit_reply_and_complete(
            &self,
            _dispatch: &AgentDispatchRecord,
            _lease_owner: &str,
            _agents_session_id: &str,
            _agents_turn_id: &str,
            _content: &str,
        ) -> Result<AgentReplyCommitResult, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(AgentReplyCommitResult {
                reply_message_id: 9002,
                reply_message_seq: 8,
            })
        }
    }

    fn dispatch(max_attempts: u32) -> AgentDispatchRecord {
        AgentDispatchRecord {
            dispatch_id: "amd_worker_1".into(),
            tenant_id: 100001,
            organization_id: 0,
            conversation_id: "conversation.worker.1".into(),
            source_message_id: 9001,
            source_message_seq: 7,
            agent_id: "agent.worker.1".into(),
            agent_revision_ref: Some("revision.worker.1".into()),
            assignment_generation: 3,
            binding_id: None,
            agents_session_id: None,
            agents_turn_id: None,
            status: AgentDispatchStatus::Pending,
            idempotency_key: "im-agent-dispatch:amd_worker_1".into(),
            payload_hash: "payload-hash".into(),
            attempt_count: 0,
            max_attempts,
            lease_owner: None,
            lease_expires_at: None,
            next_attempt_at: "2026-07-19T00:00:00Z".into(),
            requested_by: 1001,
            reply_message_id: None,
            reply_message_seq: None,
            created_at: "2026-07-19T00:00:00Z".into(),
            updated_at: "2026-07-19T00:00:00Z".into(),
        }
    }

    #[test]
    fn worker_completes_only_through_atomic_reply_committer() {
        let store = Arc::new(FakeStore::new(dispatch(3)));
        let reply = Arc::new(FakeReply {
            calls: AtomicUsize::new(0),
        });
        let (agents, complete_calls) = fake_agents(false, None);
        let worker = AgentDispatchWorker::new(
            store.clone(),
            agents,
            Arc::new(FakeSource),
            reply.clone(),
            "worker.test",
        )
        .expect("worker should build");

        let outcomes = worker
            .process_global_batch(
                "2026-07-19T00:00:01Z",
                "2026-07-19T00:01:31Z",
                "2026-07-19T00:00:06Z",
                10,
            )
            .expect("batch should complete");

        assert_eq!(
            outcomes,
            vec![AgentDispatchWorkerOutcome::Completed {
                dispatch_id: "amd_worker_1".into(),
                turn_id: "turn.worker.1".into(),
            }]
        );
        assert_eq!(reply.calls.load(Ordering::SeqCst), 1);
        assert_eq!(complete_calls.load(Ordering::SeqCst), 1);
        assert_eq!(store.complete_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn worker_dead_letters_last_attempt_and_sanitizes_failure_detail() {
        let store = Arc::new(FakeStore::new(dispatch(1)));
        let (agents, _) = fake_agents(true, None);
        let worker = AgentDispatchWorker::new(
            store.clone(),
            agents,
            Arc::new(FakeSource),
            Arc::new(FakeReply {
                calls: AtomicUsize::new(0),
            }),
            "worker.test",
        )
        .expect("worker should build");

        let outcomes = worker
            .process_global_batch(
                "2026-07-19T00:00:01Z",
                "2026-07-19T00:01:31Z",
                "2026-07-19T00:00:06Z",
                10,
            )
            .expect("failure should become a terminal outcome");

        assert_eq!(
            outcomes,
            vec![AgentDispatchWorkerOutcome::DeadLettered {
                dispatch_id: "amd_worker_1".into(),
            }]
        );
        let detail = store
            .failure_detail
            .lock()
            .expect("failure lock")
            .clone()
            .expect("failure detail should be recorded");
        assert!(!detail.contains(['\r', '\n']));
        assert!(detail.chars().count() <= 2048);
    }

    #[test]
    fn worker_defers_running_turn_without_entering_failure_budget() {
        let mut record = dispatch(1);
        let session_id = deterministic_agents_session_id(&record);
        record.agents_session_id = Some(session_id.clone());
        let store = Arc::new(FakeStore::new(record));
        let reply = Arc::new(FakeReply {
            calls: AtomicUsize::new(0),
        });
        let (agents, complete_calls) = fake_agents(
            false,
            Some(AgentsTurnSnapshot {
                session_id,
                turn_id: "turn.worker.running".into(),
                status: AgentsTurnStatus::Running,
                request_message_id: "message.request.running".into(),
                response_message_id: None,
                response_content: None,
                error_code: None,
            }),
        );
        let worker = AgentDispatchWorker::new(
            store.clone(),
            agents,
            Arc::new(FakeSource),
            reply.clone(),
            "worker.test",
        )
        .expect("worker should build");

        let outcomes = worker
            .process_global_batch(
                "2026-07-19T00:00:01Z",
                "2026-07-19T00:01:31Z",
                "2026-07-19T00:00:06Z",
                10,
            )
            .expect("running turn should be deferred");

        assert_eq!(
            outcomes,
            vec![AgentDispatchWorkerOutcome::ReconciliationDeferred {
                dispatch_id: "amd_worker_1".into(),
                turn_id: Some("turn.worker.running".into()),
            }]
        );
        assert_eq!(store.defer_calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            store
                .deferred_turn_id
                .lock()
                .expect("deferred turn lock")
                .as_deref(),
            Some("turn.worker.running")
        );
        assert!(store.failure_detail.lock().expect("failure lock").is_none());
        assert_eq!(reply.calls.load(Ordering::SeqCst), 0);
        assert_eq!(complete_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn worker_commits_completed_reconciled_turn_without_reexecution() {
        let mut record = dispatch(1);
        let session_id = deterministic_agents_session_id(&record);
        record.agents_session_id = Some(session_id.clone());
        let store = Arc::new(FakeStore::new(record));
        let reply = Arc::new(FakeReply {
            calls: AtomicUsize::new(0),
        });
        let (agents, complete_calls) = fake_agents(
            false,
            Some(AgentsTurnSnapshot {
                session_id,
                turn_id: "turn.worker.reconciled".into(),
                status: AgentsTurnStatus::Completed,
                request_message_id: "message.request.reconciled".into(),
                response_message_id: Some("message.response.reconciled".into()),
                response_content: Some("reconciled answer".into()),
                error_code: None,
            }),
        );
        let worker = AgentDispatchWorker::new(
            store,
            agents,
            Arc::new(FakeSource),
            reply.clone(),
            "worker.test",
        )
        .expect("worker should build");

        let outcomes = worker
            .process_global_batch(
                "2026-07-19T00:00:01Z",
                "2026-07-19T00:01:31Z",
                "2026-07-19T00:00:06Z",
                10,
            )
            .expect("completed turn should reconcile");

        assert_eq!(
            outcomes,
            vec![AgentDispatchWorkerOutcome::Completed {
                dispatch_id: "amd_worker_1".into(),
                turn_id: "turn.worker.reconciled".into(),
            }]
        );
        assert_eq!(reply.calls.load(Ordering::SeqCst), 1);
        assert_eq!(complete_calls.load(Ordering::SeqCst), 0);
    }

    struct SingleMessageStore {
        record: StoredMessageRecord,
    }

    impl MessageStore for SingleMessageStore {
        fn allocate_message_seq(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<u64, ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn insert_message(&self, _message: StoredMessageRecord) -> Result<(), ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn read_history_window(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _before_seq: Option<u64>,
            _limit: usize,
        ) -> Result<MessageWindow, ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn read_message_by_id(
            &self,
            tenant_id: &str,
            message_id: i64,
        ) -> Result<Option<StoredMessageRecord>, ContractError> {
            Ok(
                (self.record.tenant_id == tenant_id && self.record.message_id == message_id)
                    .then(|| self.record.clone()),
            )
        }

        fn read_message_by_client_id(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
            _sender_principal_kind: &str,
            _sender_principal_id: &str,
            _client_msg_id: &str,
        ) -> Result<Option<StoredMessageRecord>, ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }

        fn read_high_watermark(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _conversation_id: &str,
        ) -> Result<u64, ContractError> {
            Err(ContractError::UnsupportedCapability("test".into()))
        }
    }

    #[test]
    fn source_loader_extracts_only_text_and_rejects_scope_mismatch() {
        let body = MessageBody {
            summary: Some("do not dispatch summary".into()),
            parts: vec![
                ContentPart::text("first"),
                ContentPart::Mention(im_domain_core::message::MentionPart {
                    target_kind: im_domain_core::message::MentionTargetKind::Agent,
                    target_id: "agent.worker.1".into(),
                    display_text: "@agent".into(),
                    assignment_generation: 3,
                }),
                ContentPart::text("second"),
            ],
            render_hints: BTreeMap::from([("private".into(), "not-dispatched".into())]),
            reply_to: None,
        };
        let loader = MessageStoreAgentDispatchSourceLoader::new(Arc::new(SingleMessageStore {
            record: StoredMessageRecord {
                tenant_id: "100001".into(),
                organization_id: "0".into(),
                conversation_id: "conversation.worker.1".into(),
                message_id: 9001,
                message_seq: 7,
                sender_principal_kind: "user".into(),
                sender_principal_id: "1001".into(),
                sender_device_id: None,
                client_msg_id: Some("client.9001".into()),
                message_type: "standard".into(),
                payload_json: serde_json::to_string(&body).expect("body should encode"),
                payload_hash: "body-hash".into(),
                created_at: "2026-07-19T00:00:00Z".into(),
                updated_at: "2026-07-19T00:00:00Z".into(),
                deleted_at: None,
                retention_until: None,
            },
        }));

        let source = loader
            .load_source(&dispatch(3))
            .expect("matching source should load");
        assert_eq!(source.content, "first\nsecond");
        assert!(!source.content.contains("private"));

        let mut wrong_scope = dispatch(3);
        wrong_scope.organization_id = 9;
        assert!(loader.load_source(&wrong_scope).is_err());
    }

    #[test]
    fn configuration_bounds_are_commercially_bounded() {
        let defaults = AgentDispatchWorkerConfig::default();
        assert!((1..=100).contains(&defaults.batch_size));
        assert!((15..=900).contains(&defaults.lease_seconds));
        assert!((50..=60_000).contains(&defaults.poll_interval_ms));
        assert_eq!(AgentAssignmentSource::DefaultPolicy.db_code(), 0);
    }
}
