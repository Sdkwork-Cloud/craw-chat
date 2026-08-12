//! Outbound agent dispatch bridge: automation executions -> Agents runtime.
//!
//! The automation service records execution lifecycle and agent response
//! streams, but actual agent execution is owned by `sdkwork-agents`. This
//! bridge closes that gap: it polls executions still in `requested` state,
//! resolves an Agents session (`session_kind = automation`), completes a turn
//! with the execution input, and streams the turn output back into the
//! automation runtime (start -> deltas -> complete), transitioning the
//! execution to `succeeded`. Failures transition the execution to `failed`.
//!
//! Dispatch is idempotent per execution: the turn idempotency key is derived
//! from the execution id, so a completed turn is replayed instead of
//! re-executed after a bridge restart.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use automation_service::dto::{
    AppendAgentResponseDeltaRequest, CompleteAgentResponseRequest, StartAgentResponseRequest,
};
use im_app_context::local_service_app_context;
use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};
use im_time::utc_now_rfc3339_millis;
use sdkwork_agents_runtime_facade::{
    AgentsSessionActor, AgentsSessionEntrySurface, AgentsSessionFacade, AgentsSessionKind,
    AgentsTurnStatus, CompleteAgentsTurnRequest, GetAgentsTurnByIdempotencyRequest,
    ResolveAgentsSessionRequest,
};
use sdkwork_im_contract_agent::AgentSubject;
use sdkwork_utils_rust::sha256_hash;
use tokio::sync::watch;
use tokio::task::JoinHandle;

const BRIDGE_BATCH_SIZE_ENV: &str = "SDKWORK_IM_AUTOMATION_AGENT_BRIDGE_BATCH_SIZE";
const BRIDGE_POLL_INTERVAL_MS_ENV: &str = "SDKWORK_IM_AUTOMATION_AGENT_BRIDGE_POLL_INTERVAL_MS";
const BRIDGE_DEFAULT_BATCH_SIZE: usize = 8;
const BRIDGE_DEFAULT_POLL_INTERVAL_MS: u64 = 2_000;
/// Frames are bounded so the streamed response never carries an unbounded
/// single frame; the automation runtime enforces its own frame limits too.
const BRIDGE_MAX_FRAME_CHARS: usize = 6_000;
const BRIDGE_TURN_CONTENT_TYPE: &str = "text/markdown";
const BRIDGE_RESPONSE_STREAM_TYPE: &str = "agent.response";
const BRIDGE_ACTOR_SUBJECT: &str = "service.sdkwork-im.automation-agent-bridge";
const BRIDGE_ACTOR_ROLE: &str = "ai.agents.manage";

pub struct AutomationAgentBridgeHandle {
    shutdown: watch::Sender<()>,
    task: JoinHandle<()>,
}

impl AutomationAgentBridgeHandle {
    pub fn shutdown(self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

impl Drop for AutomationAgentBridgeHandle {
    fn drop(&mut self) {
        let _ = self.shutdown.send(());
        self.task.abort();
    }
}

/// Spawns the automation -> Agents dispatch bridge.
pub fn spawn_automation_agent_bridge(
    runtime: Arc<automation_service::AutomationRuntime>,
    agents: Arc<dyn AgentsSessionFacade>,
) -> AutomationAgentBridgeHandle {
    let batch_size = read_bounded_env_usize(
        BRIDGE_BATCH_SIZE_ENV,
        BRIDGE_DEFAULT_BATCH_SIZE,
        1,
        64,
    );
    let poll_interval_ms = read_bounded_env_u64(
        BRIDGE_POLL_INTERVAL_MS_ENV,
        BRIDGE_DEFAULT_POLL_INTERVAL_MS,
        100,
        60_000,
    );
    let (shutdown_tx, mut shutdown_rx) = watch::channel(());
    let task = tokio::spawn(async move {
        loop {
            let executions = runtime.requested_executions(batch_size);
            for execution in executions {
                if let Err(error) =
                    dispatch_execution(runtime.as_ref(), agents.as_ref(), &execution)
                {
                    tracing::warn!(
                        execution_id = execution.execution_id.as_str(),
                        error = %error,
                        "automation agent bridge dispatch failed"
                    );
                }
            }
            // The unit-value channel is a pure stop signal: `changed()`
            // resolves when the sender sends (shutdown) or is dropped
            // (handle destroyed); either way the bridge stops.
            let _ = shutdown_rx.changed().await;
            break;
            tokio::time::sleep(Duration::from_millis(poll_interval_ms)).await;
        }
    });
    AutomationAgentBridgeHandle {
        shutdown: shutdown_tx,
        task,
    }
}

fn dispatch_execution(
    runtime: &automation_service::AutomationRuntime,
    agents: &dyn AgentsSessionFacade,
    execution: &AutomationExecution,
) -> Result<(), String> {
    let tenant_id = parse_u64(&execution.tenant_id, "tenant_id")?;
    let owner_user_id = parse_u64(&execution.principal_id, "principal_id")?;
    // Automation executions are organization-agnostic; the default
    // organization id mirrors `local_service_app_context`.
    let organization_id = 0_u64;
    let agent_id = execution.target_ref.clone();
    let session_id = deterministic_session_id(execution);
    let idempotency_key = format!("automation:{tenant_id}:{}", execution.execution_id);
    let auth = local_service_app_context(
        execution.tenant_id.as_str(),
        execution.principal_id.as_str(),
        "user",
        None,
        ["automation.execute", "automation.read"],
    );

    // The execution may have been advanced or completed while queued; only
    // requested executions are dispatched.
    let current = runtime
        .get_execution(&auth, execution.execution_id.as_str())
        .map_err(|error| format!("execution lookup failed: {error:?}"))?;
    if current.state != AutomationExecutionState::Requested {
        return Ok(());
    }

    // Idempotent reconciliation: a turn already completed for this execution
    // is replayed instead of re-executed.
    let snapshot = agents
        .get_turn_by_idempotency(GetAgentsTurnByIdempotencyRequest {
            tenant_id,
            organization_id,
            owner_user_id,
            agent_id: agent_id.clone(),
            session_id: session_id.clone(),
            idempotency_key: idempotency_key.clone(),
            actor: bridge_actor(),
        })
        .map_err(|error| format!("Agents turn reconciliation lookup failed: {error}"))?;
    if let Some(snapshot) = snapshot {
        match snapshot.status {
            AgentsTurnStatus::Completed => {
                let content = snapshot
                    .response_content
                    .ok_or_else(|| "completed Agents turn snapshot is missing response content".to_owned())?;
                return commit_response(
                    runtime,
                    &auth,
                    execution,
                    &session_id,
                    snapshot.turn_id.as_str(),
                    content.as_str(),
                );
            }
            AgentsTurnStatus::Failed => {
                let reason = snapshot
                    .error_code
                    .clone()
                    .unwrap_or_else(|| "agents turn failed".to_owned());
                runtime
                    .fail_execution(&auth, execution.execution_id.as_str(), reason.as_str())
                    .map_err(|error| format!("execution failure transition failed: {error:?}"))?;
                return Ok(());
            }
            AgentsTurnStatus::Requested | AgentsTurnStatus::Running => {
                // Still in progress; the next poll picks it up.
                return Ok(());
            }
            AgentsTurnStatus::Cancelled => {
                runtime
                    .fail_execution(
                        &auth,
                        execution.execution_id.as_str(),
                        "agents turn reached cancelled state",
                    )
                    .map_err(|error| format!("execution failure transition failed: {error:?}"))?;
                return Ok(());
            }
        }
    }

    let payload_hash = sha256_hash(execution.input_payload.as_deref().unwrap_or("").as_bytes());
    let resolved = agents
        .resolve_or_create_session(ResolveAgentsSessionRequest {
            tenant_id,
            organization_id,
            owner_user_id,
            agent_id: agent_id.clone(),
            session_id: session_id.clone(),
            project_id: None,
            session_kind: AgentsSessionKind::Automation,
            entry_surface: AgentsSessionEntrySurface::Automation,
            source_module: Some("sdkwork-im".into()),
            source_context_kind: Some("automation".into()),
            source_context_id: Some(execution.execution_id.clone()),
            parent_session_id: None,
            forked_from_turn_id: None,
            title: format!("automation {}", execution.trigger_type),
            idempotency_key: idempotency_key.clone(),
            payload_hash,
            runtime_binding: None,
            actor: bridge_actor(),
            requested_at: utc_now_rfc3339_millis(),
        })
        .map_err(|error| format!("Agents session resolve failed: {error}"))?;

    let completed = agents
        .complete_turn(CompleteAgentsTurnRequest {
            tenant_id,
            organization_id,
            owner_user_id,
            agent_id,
            session_id,
            content: turn_content(execution),
            content_type: BRIDGE_TURN_CONTENT_TYPE.into(),
            idempotency_key,
            client_request_id: execution.execution_id.clone(),
            actor: bridge_actor(),
            requested_at: utc_now_rfc3339_millis(),
        })
        .map_err(|error| format!("Agents turn failed: {error}"))?;
    commit_response(
        runtime,
        &auth,
        execution,
        &resolved.session_id,
        completed.turn_id.as_str(),
        completed.response_content.as_str(),
    )
}

/// Streams the completed turn output into the automation runtime as an agent
/// response and completes the execution.
fn commit_response(
    runtime: &automation_service::AutomationRuntime,
    auth: &im_app_context::AppContext,
    execution: &AutomationExecution,
    session_id: &str,
    turn_id: &str,
    content: &str,
) -> Result<(), String> {
    let stream_id = deterministic_stream_id(execution);
    runtime
        .start_agent_response(
            auth,
            StartAgentResponseRequest {
                execution_id: execution.execution_id.clone(),
                stream_id: stream_id.clone(),
                stream_type: BRIDGE_RESPONSE_STREAM_TYPE.into(),
                conversation_id: execution.target_ref.clone(),
                schema_ref: None,
                member_id: None,
                agent: AgentSubject {
                    agent_id: execution.target_ref.clone(),
                    session_id: Some(session_id.to_owned()),
                    metadata: Default::default(),
                },
            },
        )
        .map_err(|error| format!("start agent response failed: {error:?}"))?;

    let mut frame_seq = 0_u64;
    for chunk in chunk_content(content) {
        frame_seq = frame_seq.saturating_add(1);
        runtime
            .append_agent_response_delta(
                auth,
                stream_id.as_str(),
                AppendAgentResponseDeltaRequest {
                    frame_seq,
                    frame_type: "text".into(),
                    schema_ref: None,
                    encoding: "utf-8".into(),
                    payload: chunk,
                    attributes: BTreeMap::new(),
                },
            )
            .map_err(|error| format!("append agent response delta failed: {error:?}"))?;
    }

    runtime
        .complete_agent_response(
            auth,
            stream_id.as_str(),
            CompleteAgentResponseRequest {
                frame_seq,
                result_message_id: Some(turn_id.to_owned()),
            },
        )
        .map_err(|error| format!("complete agent response failed: {error:?}"))?;
    Ok(())
}

/// The turn content is a deterministic JSON envelope of the execution so the
/// Agents runtime sees the automation request without any caller-controlled
/// structure ambiguity.
fn turn_content(execution: &AutomationExecution) -> String {
    serde_json::json!({
        "triggerType": execution.trigger_type,
        "targetKind": execution.target_kind,
        "targetRef": execution.target_ref,
        "inputPayload": execution.input_payload,
    })
    .to_string()
}

fn deterministic_session_id(execution: &AutomationExecution) -> String {
    let hash = sha256_hash(
        format!(
            "automation:session:{}:{}",
            execution.tenant_id, execution.execution_id
        )
        .as_bytes(),
    );
    format!("automation-{}", &hash[..24])
}

fn deterministic_stream_id(execution: &AutomationExecution) -> String {
    let hash = sha256_hash(
        format!(
            "automation:stream:{}:{}",
            execution.tenant_id, execution.execution_id
        )
        .as_bytes(),
    );
    format!("automation-response-{}", &hash[..24])
}

fn bridge_actor() -> AgentsSessionActor {
    AgentsSessionActor {
        subject_id: BRIDGE_ACTOR_SUBJECT.into(),
        roles: vec![BRIDGE_ACTOR_ROLE.into()],
    }
}

fn parse_u64(value: &str, field: &str) -> Result<u64, String> {
    value.trim().parse::<u64>().map_err(|_| {
        format!("automation execution {field} is not a valid numeric id: {value}")
    })
}

fn chunk_content(content: &str) -> Vec<String> {
    if content.is_empty() {
        return Vec::new();
    }
    let chars: Vec<char> = content.chars().collect();
    chars
        .chunks(BRIDGE_MAX_FRAME_CHARS)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

fn read_bounded_env_usize(name: &str, default: usize, min: usize, max: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

fn read_bounded_env_u64(name: &str, default: u64, min: u64, max: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execution(execution_id: &str, state: AutomationExecutionState) -> AutomationExecution {
        AutomationExecution {
            tenant_id: "100001".into(),
            principal_id: "42".into(),
            principal_kind: "user".into(),
            execution_id: execution_id.into(),
            trigger_type: "manual".into(),
            target_kind: "agent".into(),
            target_ref: "agent-demo".into(),
            input_payload: Some("run".into()),
            output_payload: None,
            state,
            retry_count: 0,
            requested_at: "2026-01-01T00:00:00.000Z".into(),
            completed_at: None,
            failure_reason: None,
        }
    }

    #[test]
    fn session_and_stream_ids_are_deterministic() {
        let first = execution("exec-1", AutomationExecutionState::Requested);
        let second = execution("exec-1", AutomationExecutionState::Requested);
        assert_eq!(deterministic_session_id(&first), deterministic_session_id(&second));
        assert_eq!(deterministic_stream_id(&first), deterministic_stream_id(&second));
        assert_ne!(
            deterministic_session_id(&first),
            deterministic_session_id(&execution("exec-2", AutomationExecutionState::Requested))
        );
    }

    #[test]
    fn turn_content_is_a_deterministic_envelope() {
        let content = turn_content(&execution("exec-1", AutomationExecutionState::Requested));
        assert!(content.contains("\"triggerType\":\"manual\""));
        assert!(content.contains("\"targetRef\":\"agent-demo\""));
        assert!(content.contains("\"inputPayload\":\"run\""));
    }

    #[test]
    fn chunk_content_respects_frame_bound() {
        let long = "x".repeat(12_500);
        let chunks = chunk_content(long.as_str());
        assert_eq!(chunks.len(), 3);
        assert!(chunks.iter().all(|chunk| chunk.len() <= BRIDGE_MAX_FRAME_CHARS));
        assert_eq!(chunks.concat().len(), 12_500);
        assert!(chunk_content("").is_empty());
    }

    #[test]
    fn numeric_id_parsing_rejects_non_numeric() {
        assert_eq!(parse_u64("100001", "tenant_id"), Ok(100_001));
        assert!(parse_u64("not-a-number", "tenant_id").is_err());
    }
}
