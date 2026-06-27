//! Automation runtime: in-memory execution state, agent response streams, tool call tracking,
//! event journaling, and the business logic that orchestrates the automation lifecycle.

use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use im_app_context::AppContext;
use im_domain_core::automation::{
    AgentToolCall, AgentToolCallState, AutomationExecution, AutomationExecutionState,
};
use im_domain_core::stream::{StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_agent::{
    AgentSubject, AutomationExecutionRecord, AutomationExecutionStore,
};
use sdkwork_im_contract_core::ContractError;
use sdkwork_im_contract_message::{CommitJournal, CommitPosition};
use serde::Serialize;

use crate::constants::*;
use crate::dto::*;
use crate::error::AutomationError;
use crate::helpers::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct AgentResponseRuntimeState {
    principal_id: String,
    principal_kind: String,
    execution_id: String,
    session: StreamSession,
    agent: AgentSubject,
    member_id: Option<String>,
    frames: Vec<StreamFrame>,
}

pub struct AutomationRuntime {
    executions: Mutex<HashMap<String, AutomationExecution>>,
    agent_responses: Mutex<AgentResponseRuntimeStore>,
    tool_calls: Mutex<AgentToolCallRuntimeStore>,
    event_orders: Mutex<HashMap<String, u64>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    execution_store: Arc<dyn AutomationExecutionStore>,
}

#[derive(Default)]
struct AgentToolCallRuntimeStore {
    by_call: HashMap<String, AgentToolCall>,
    pending_tool_calls_by_execution: HashMap<String, BTreeSet<String>>,
}

impl AgentToolCallRuntimeStore {
    fn get(&self, tool_call_key: &str) -> Option<&AgentToolCall> {
        self.by_call.get(tool_call_key)
    }

    fn get_mut(&mut self, tool_call_key: &str) -> Option<&mut AgentToolCall> {
        self.by_call.get_mut(tool_call_key)
    }

    fn pending_tool_call_for_execution(&self, execution_key: &str) -> Option<String> {
        self.pending_tool_calls_by_execution
            .get(execution_key)
            .and_then(|tool_call_keys| tool_call_keys.iter().next())
            .and_then(|tool_call_key| self.by_call.get(tool_call_key))
            .map(|tool_call| tool_call.tool_call_id.clone())
    }

    fn insert(&mut self, execution_key: String, tool_call_key: String, tool_call: AgentToolCall) {
        if let Some(previous) = self.by_call.get(tool_call_key.as_str()).cloned() {
            self.remove_pending_index(execution_key.as_str(), tool_call_key.as_str(), &previous);
        }
        if tool_call.state == AgentToolCallState::Requested {
            self.pending_tool_calls_by_execution
                .entry(execution_key)
                .or_default()
                .insert(tool_call_key.clone());
        }
        self.by_call.insert(tool_call_key, tool_call);
    }

    fn mark_completed(
        &mut self,
        execution_key: &str,
        tool_call_key: &str,
        result_payload: String,
        completed_at: String,
    ) -> Result<AgentToolCall, AutomationError> {
        let Some(tool_call) = self.get_mut(tool_call_key) else {
            return Err(AutomationError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "agent_tool_call_not_found",
                message: format!("agent tool call not found: {tool_call_key}"),
            });
        };
        if tool_call.state == AgentToolCallState::Completed {
            if tool_call.result_payload.as_deref() == Some(result_payload.as_str()) {
                return Ok(tool_call.clone());
            }
            return Err(AutomationError {
                status: axum::http::StatusCode::CONFLICT,
                code: "agent_tool_call_conflict",
                message: format!(
                    "agent tool call already completed: {}",
                    tool_call.tool_call_id
                ),
            });
        }
        let was_requested = tool_call.state == AgentToolCallState::Requested;
        tool_call.result_payload = Some(result_payload);
        tool_call.state = AgentToolCallState::Completed;
        tool_call.completed_at = Some(completed_at);
        let completed = tool_call.clone();
        if was_requested
            && let Some(tool_call_keys) =
                self.pending_tool_calls_by_execution.get_mut(execution_key)
        {
            tool_call_keys.remove(tool_call_key);
            if tool_call_keys.is_empty() {
                self.pending_tool_calls_by_execution.remove(execution_key);
            }
        }
        Ok(completed)
    }

    fn remove_pending_index(
        &mut self,
        execution_key: &str,
        tool_call_key: &str,
        tool_call: &AgentToolCall,
    ) {
        if tool_call.state != AgentToolCallState::Requested {
            return;
        }
        if let Some(tool_call_keys) = self.pending_tool_calls_by_execution.get_mut(execution_key) {
            tool_call_keys.remove(tool_call_key);
            if tool_call_keys.is_empty() {
                self.pending_tool_calls_by_execution.remove(execution_key);
            }
        }
    }
}

#[derive(Default)]
struct AgentResponseRuntimeStore {
    by_stream: HashMap<String, AgentResponseRuntimeState>,
    agent_responses_by_execution: HashMap<String, String>,
}

impl AgentResponseRuntimeStore {
    fn agent_response_key_for_execution(&self, execution_key: &str) -> Option<&str> {
        self.agent_responses_by_execution
            .get(execution_key)
            .map(String::as_str)
    }

    fn response_for_execution(&self, execution_key: &str) -> Option<&AgentResponseRuntimeState> {
        self.agent_response_key_for_execution(execution_key)
            .and_then(|stream_key| self.by_stream.get(stream_key))
    }

    fn response_mut(&mut self, stream_key: &str) -> Option<&mut AgentResponseRuntimeState> {
        self.by_stream.get_mut(stream_key)
    }

    fn insert(
        &mut self,
        stream_key: String,
        execution_key: String,
        response: AgentResponseRuntimeState,
    ) {
        self.agent_responses_by_execution
            .insert(execution_key, stream_key.clone());
        self.by_stream.insert(stream_key, response);
    }
}

#[derive(Default)]
struct NoopJournal;

impl CommitJournal for NoopJournal {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

impl Default for AutomationRuntime {
    fn default() -> Self {
        Self::with_journal(Arc::new(NoopJournal))
    }
}

trait AutomationMutexExt<T> {
    fn lock_automation(&self) -> MutexGuard<'_, T>;
}

impl<T> AutomationMutexExt<T> for Mutex<T> {
    fn lock_automation(&self) -> MutexGuard<'_, T> {
        match self.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!("recovering poisoned mutex in automation-service");
                poisoned.into_inner()
            }
        }
    }
}

impl AutomationRuntime {
    pub fn with_journal<J>(journal: Arc<J>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store(
            journal,
            Arc::new(RuntimeMemoryAutomationExecutionStore::default()),
        )
    }

    pub fn with_journal_and_store<J, S>(journal: Arc<J>, execution_store: Arc<S>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: AutomationExecutionStore + 'static,
    {
        Self {
            executions: Mutex::new(HashMap::new()),
            agent_responses: Mutex::new(AgentResponseRuntimeStore::default()),
            tool_calls: Mutex::new(AgentToolCallRuntimeStore::default()),
            event_orders: Mutex::new(HashMap::new()),
            journal,
            execution_store,
        }
    }

    fn ensure_execution_state(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<(), AutomationError> {
        let scope_key = execution_scope_key(tenant_id, principal_kind, principal_id, execution_id);
        if self
            .executions
            .lock_automation()
            .contains_key(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .execution_store
            .load_execution(tenant_id, principal_kind, principal_id, execution_id)
            .map_err(AutomationError::automation_store)?;
        if let Some(record) = restored {
            self.executions
                .lock_automation()
                .insert(scope_key, record.execution);
        }

        Ok(())
    }

    pub fn request_execution(
        &self,
        auth: &AppContext,
        request: RequestAutomationExecution,
    ) -> Result<AutomationExecution, AutomationError> {
        Ok(self
            .request_execution_with_outcome(auth, request)?
            .execution)
    }

    pub fn request_execution_with_outcome(
        &self,
        auth: &AppContext,
        request: RequestAutomationExecution,
    ) -> Result<AutomationExecutionRequestResult, AutomationError> {
        ensure_automation_execute_access(auth)?;
        validate_execution_request_payload_size(&request)?;
        self.ensure_execution_state(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        )?;
        let execution_key = execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let request_key = automation_execution_request_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let requested_at = utc_now_rfc3339_millis();
        let requested = AutomationExecution {
            tenant_id: auth.tenant_id.clone(),
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
            execution_id: request.execution_id.clone(),
            trigger_type: request.trigger_type.clone(),
            target_kind: request.target_kind.clone(),
            target_ref: request.target_ref.clone(),
            input_payload: request.input_payload.clone(),
            output_payload: None,
            state: AutomationExecutionState::Requested,
            retry_count: 0,
            requested_at: requested_at.clone(),
            completed_at: None,
            failure_reason: None,
        };

        {
            let mut executions = self.executions.lock_automation();

            if let Some(existing) = executions.get(execution_key.as_str()).cloned() {
                if !execution_matches_principal_kind(&existing, auth.actor_kind.as_str()) {
                    return Err(AutomationError::conflict(request.execution_id.as_str()));
                }
                if execution_matches_request(&existing, &request) {
                    return Ok(AutomationExecutionRequestResult {
                        delivery_status: delivery_status_from_execution(existing.state.as_str()),
                        execution: existing,
                        is_new: false,
                        request_key,
                    });
                }

                return Err(AutomationError::conflict(request.execution_id.as_str()));
            }
            executions.insert(execution_key.clone(), requested.clone());
        }

        if let Err(error) = self.append_event(auth, &requested, "automation.execution_requested", 1)
        {
            self.clear_execution_state(execution_key.as_str());
            return Err(error);
        }

        let completed_at = utc_now_rfc3339_millis();
        let output_payload = Some(
            serde_json::json!({
                "accepted": true,
                "targetRef": request.target_ref,
                "requestKey": request_key,
                "deliveryStatus": "applied",
            })
            .to_string(),
        );
        let completed = AutomationExecution {
            output_payload,
            state: AutomationExecutionState::Succeeded,
            completed_at: Some(completed_at),
            ..requested
        };
        if let Err(error) = self.append_event(auth, &completed, "automation.execution_completed", 2)
        {
            self.clear_execution_state(execution_key.as_str());
            return Err(error);
        }
        self.executions
            .lock_automation()
            .insert(execution_key.clone(), completed.clone());
        self.event_orders
            .lock_automation()
            .insert(execution_key.clone(), 2);
        if let Err(error) = self
            .execution_store
            .save_execution(self.execution_record(&completed))
        {
            self.clear_execution_state(execution_key.as_str());
            return Err(AutomationError::automation_store(error));
        }

        Ok(AutomationExecutionRequestResult {
            delivery_status: AutomationExecutionDeliveryStatus::Applied,
            execution: completed,
            is_new: true,
            request_key,
        })
    }

    pub fn governance_snapshot(
        &self,
        auth: &AppContext,
    ) -> Result<AutomationGovernanceSnapshot, AutomationError> {
        ensure_automation_read_access(auth)?;
        Ok(automation_governance_snapshot(auth))
    }

    pub fn start_agent_response(
        &self,
        auth: &AppContext,
        request: StartAgentResponseRequest,
    ) -> Result<StreamSession, AutomationError> {
        ensure_automation_execute_access(auth)?;
        validate_start_agent_response_request_payload_size(&request)?;
        let execution = self.execution_for_actor(auth, request.execution_id.as_str())?;
        let scope_key = agent_response_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.stream_id.as_str(),
        );
        let mut responses = self.agent_responses.lock_automation();
        if let Some(existing) = responses.by_stream.get(scope_key.as_str()) {
            if existing.execution_id == request.execution_id
                && existing.agent == request.agent
                && existing.member_id == request.member_id
                && existing.session.stream_type == request.stream_type
                && existing.session.scope_id == request.conversation_id
                && existing.session.schema_ref == request.schema_ref
            {
                return Ok(existing.session.clone());
            }
            return Err(AutomationError {
                status: axum::http::StatusCode::CONFLICT,
                code: "agent_response_conflict",
                message: format!(
                    "agent response stream conflicts with existing definition: {}",
                    request.stream_id
                ),
            });
        }
        let execution_response_key = agent_response_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        if responses
            .agent_response_key_for_execution(execution_response_key.as_str())
            .is_some()
        {
            return Err(AutomationError {
                status: axum::http::StatusCode::CONFLICT,
                code: "agent_response_conflict",
                message: format!(
                    "agent response execution already has an active stream: {}",
                    request.execution_id
                ),
            });
        }

        let session = StreamSession {
            tenant_id: auth.tenant_id.clone(),
            stream_id: request.stream_id.clone(),
            owner_principal_id: auth.actor_id.clone(),
            owner_principal_kind: auth.actor_kind.clone(),
            stream_type: request.stream_type.clone(),
            scope_kind: "conversation".into(),
            scope_id: request.conversation_id.clone(),
            durability_class: StreamDurabilityClass::EventLog,
            ordering_scope: "stream".into(),
            schema_ref: request.schema_ref.clone(),
            state: StreamSessionState::Opened,
            last_frame_seq: 0,
            last_checkpoint_seq: None,
            result_message_id: None,
            complete_frame_seq: None,
            abort_frame_seq: None,
            abort_reason: None,
            opened_at: utc_now_rfc3339_millis(),
            closed_at: None,
            expires_at: None,
        };
        let sender = request.agent.sender(request.member_id.clone());
        let payload = serde_json::json!({
            "executionId": request.execution_id,
            "streamId": session.stream_id,
            "streamType": session.stream_type,
            "conversationId": session.scope_id,
            "state": session.state.as_wire_value(),
            "sender": sender,
        });
        responses.insert(
            scope_key,
            execution_response_key,
            AgentResponseRuntimeState {
                principal_id: auth.actor_id.clone(),
                principal_kind: auth.actor_kind.clone(),
                execution_id: execution.execution_id.clone(),
                session: session.clone(),
                agent: request.agent,
                member_id: request.member_id,
                frames: Vec::new(),
            },
        );
        drop(responses);
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_response_started",
            "automation.agent_response_stream.v1",
            &payload,
        )?;

        Ok(session)
    }

    pub fn append_agent_response_delta(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: AppendAgentResponseDeltaRequest,
    ) -> Result<StreamFrame, AutomationError> {
        ensure_automation_execute_access(auth)?;
        validate_payload_size(
            "streamId",
            stream_id,
            AUTOMATION_AGENT_RESPONSE_MAX_STREAM_ID_BYTES,
        )?;
        validate_agent_response_delta_payload_size(&request)?;
        if request.frame_seq == 0 {
            return Err(AutomationError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_frame_seq",
                message: "frameSeq must start from 1".into(),
            });
        }

        let mut responses = self.agent_responses.lock_automation();
        let scope_key = agent_response_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            stream_id,
        );
        let state = responses
            .response_mut(scope_key.as_str())
            .ok_or_else(|| AutomationError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "agent_response_not_found",
                message: format!("agent response stream not found: {stream_id}"),
            })?;

        if matches!(
            state.session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(AutomationError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "agent_response_state_invalid",
                message: format!("agent response stream is already closed: {stream_id}"),
            });
        }

        let sender = state.agent.sender(state.member_id.clone());
        if let Some(existing) = state
            .frames
            .iter()
            .find(|frame| frame.frame_seq == request.frame_seq)
        {
            let is_same_retry = existing.frame_type == request.frame_type
                && existing.schema_ref == request.schema_ref
                && existing.encoding == request.encoding
                && existing.payload == request.payload
                && existing.sender == sender
                && existing.attributes == request.attributes;
            if is_same_retry {
                return Ok(existing.clone());
            }
            return Err(AutomationError {
                status: axum::http::StatusCode::CONFLICT,
                code: "agent_response_frame_conflict",
                message: format!("agent response frame seq conflict: {}", request.frame_seq),
            });
        }

        if request.frame_seq != state.session.last_frame_seq + 1 {
            return Err(AutomationError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "agent_response_frame_out_of_order",
                message: format!(
                    "expected next frame seq {}, got {}",
                    state.session.last_frame_seq + 1,
                    request.frame_seq
                ),
            });
        }

        let frame = StreamFrame {
            tenant_id: auth.tenant_id.clone(),
            stream_id: state.session.stream_id.clone(),
            stream_type: state.session.stream_type.clone(),
            scope_kind: state.session.scope_kind.clone(),
            scope_id: state.session.scope_id.clone(),
            frame_seq: request.frame_seq,
            frame_type: request.frame_type,
            schema_ref: request.schema_ref,
            encoding: request.encoding,
            payload: request.payload,
            sender,
            attributes: request.attributes,
            occurred_at: utc_now_rfc3339_millis(),
        };
        state.session.last_frame_seq = frame.frame_seq;
        state.session.state = StreamSessionState::Active;
        let execution_id = state.execution_id.clone();
        state.frames.push(frame.clone());
        drop(responses);

        let execution = self.execution_for_actor(auth, execution_id.as_str())?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_response_delta",
            "automation.agent_response_frame.v1",
            &frame,
        )?;

        Ok(frame)
    }

    pub fn complete_agent_response(
        &self,
        auth: &AppContext,
        stream_id: &str,
        request: CompleteAgentResponseRequest,
    ) -> Result<StreamSession, AutomationError> {
        ensure_automation_execute_access(auth)?;
        validate_payload_size(
            "streamId",
            stream_id,
            AUTOMATION_AGENT_RESPONSE_MAX_STREAM_ID_BYTES,
        )?;
        validate_complete_agent_response_request_payload_size(&request)?;
        let mut responses = self.agent_responses.lock_automation();
        let scope_key = agent_response_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            stream_id,
        );
        let state = responses
            .response_mut(scope_key.as_str())
            .ok_or_else(|| AutomationError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "agent_response_not_found",
                message: format!("agent response stream not found: {stream_id}"),
            })?;

        if matches!(
            state.session.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Ok(state.session.clone());
        }

        let execution_id = state.execution_id.clone();
        let tool_call_execution_key = agent_tool_call_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id.as_str(),
        );
        let pending_tool_call = self
            .tool_calls
            .lock_automation()
            .pending_tool_call_for_execution(tool_call_execution_key.as_str());
        if let Some(tool_call_id) = pending_tool_call {
            return Err(AutomationError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "agent_response_pending_tool_calls",
                message: format!(
                    "cannot complete agent response stream while tool call is pending: {tool_call_id}"
                ),
            });
        }

        state.session.last_frame_seq = state.session.last_frame_seq.max(request.frame_seq);
        state.session.last_checkpoint_seq = Some(request.frame_seq);
        state.session.result_message_id = request.result_message_id;
        state.session.state = StreamSessionState::Completed;
        state.session.closed_at = Some(utc_now_rfc3339_millis());
        let session = state.session.clone();
        let execution_id = state.execution_id.clone();
        drop(responses);

        let execution = self.execution_for_actor(auth, execution_id.as_str())?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_response_completed",
            "automation.agent_response_stream.v1",
            &session,
        )?;

        Ok(session)
    }

    pub fn request_agent_tool_call(
        &self,
        auth: &AppContext,
        request: RequestAgentToolCallRequest,
    ) -> Result<AgentToolCall, AutomationError> {
        ensure_automation_execute_access(auth)?;
        validate_agent_tool_call_request_payload_size(&request)?;
        let execution = self.execution_for_actor(auth, request.execution_id.as_str())?;
        let execution_response_key = agent_response_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let response_state = self.agent_responses.lock_automation();
        let response_state = response_state
            .response_for_execution(execution_response_key.as_str())
            .ok_or_else(|| AutomationError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "agent_response_not_started",
                message: format!(
                    "agent response stream must start before tool calls: {}",
                    request.execution_id
                ),
            })
            .map(|state| (state.agent.agent_id.clone(), state.session.clone()))?;
        if matches!(
            response_state.1.state,
            StreamSessionState::Completed | StreamSessionState::Aborted
        ) {
            return Err(AutomationError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "agent_response_state_invalid",
                message: format!(
                    "agent response stream is already closed: {}",
                    response_state.1.stream_id
                ),
            });
        }
        let agent_id = response_state.0;

        let tool_requires_override =
            automation_tool_requires_operator_override(request.tool_name.as_str());
        let operator_override_active = automation_operator_override_active(auth);
        if tool_requires_override && !operator_override_active {
            self.append_guardrail_event(
                auth,
                &execution,
                "automation.guardrail_denied",
                request.tool_name.as_str(),
                false,
            )?;
            return Err(AutomationError {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "automation_guardrail_denied",
                message: format!(
                    "tool call blocked by automation guardrail: {}",
                    request.tool_name
                ),
            });
        }

        let scope_key = agent_tool_call_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
            request.tool_call_id.as_str(),
        );
        let tool_call_execution_key = agent_tool_call_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let tool_calls = self.tool_calls.lock_automation();
        if let Some(existing) = tool_calls.get(scope_key.as_str()).cloned() {
            if existing.tool_name == request.tool_name
                && existing.arguments_payload == request.arguments_payload
            {
                return Ok(existing);
            }
            return Err(AutomationError {
                status: axum::http::StatusCode::CONFLICT,
                code: "agent_tool_call_conflict",
                message: format!("agent tool call conflict: {}", request.tool_call_id),
            });
        }
        drop(tool_calls);

        if tool_requires_override {
            self.append_guardrail_event(
                auth,
                &execution,
                "automation.operator_override_applied",
                request.tool_name.as_str(),
                true,
            )?;
        }

        let mut tool_calls = self.tool_calls.lock_automation();

        let tool_call = AgentToolCall {
            tenant_id: auth.tenant_id.clone(),
            execution_id: request.execution_id.clone(),
            agent_id,
            tool_call_id: request.tool_call_id.clone(),
            tool_name: request.tool_name,
            arguments_payload: request.arguments_payload,
            result_payload: None,
            state: AgentToolCallState::Requested,
            requested_at: utc_now_rfc3339_millis(),
            completed_at: None,
        };
        tool_calls.insert(tool_call_execution_key, scope_key, tool_call.clone());
        drop(tool_calls);
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_tool_call_requested",
            "automation.agent_tool_call.v1",
            &tool_call,
        )?;

        Ok(tool_call)
    }

    pub fn complete_agent_tool_call(
        &self,
        auth: &AppContext,
        execution_id: &str,
        tool_call_id: &str,
        request: CompleteAgentToolCallRequest,
    ) -> Result<AgentToolCall, AutomationError> {
        ensure_automation_execute_access(auth)?;
        validate_payload_size(
            "executionId",
            execution_id,
            AUTOMATION_EXECUTION_MAX_EXECUTION_ID_BYTES,
        )?;
        validate_payload_size(
            "toolCallId",
            tool_call_id,
            AUTOMATION_AGENT_TOOL_CALL_MAX_ID_BYTES,
        )?;
        validate_agent_tool_call_completion_payload_size(&request)?;
        let execution = self.execution_for_actor(auth, execution_id)?;
        let scope_key = agent_tool_call_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
            tool_call_id,
        );
        let tool_call_execution_key = agent_tool_call_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        );
        let mut tool_calls = self.tool_calls.lock_automation();
        let tool_call = tool_calls.mark_completed(
            tool_call_execution_key.as_str(),
            scope_key.as_str(),
            request.result_payload,
            utc_now_rfc3339_millis(),
        )?;
        drop(tool_calls);
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_tool_call_completed",
            "automation.agent_tool_call.v1",
            &tool_call,
        )?;

        Ok(tool_call)
    }

    pub fn get_execution(
        &self,
        auth: &AppContext,
        execution_id: &str,
    ) -> Result<AutomationExecution, AutomationError> {
        ensure_automation_read_access(auth)?;
        validate_payload_size(
            "executionId",
            execution_id,
            AUTOMATION_EXECUTION_MAX_EXECUTION_ID_BYTES,
        )?;
        self.ensure_execution_state(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        )?;
        self.executions
            .lock_automation()
            .get(
                execution_scope_key(
                    auth.tenant_id.as_str(),
                    auth.actor_kind.as_str(),
                    auth.actor_id.as_str(),
                    execution_id,
                )
                .as_str(),
            )
            .cloned()
            .filter(|execution| {
                execution_matches_principal_kind(execution, auth.actor_kind.as_str())
            })
            .ok_or_else(|| AutomationError::not_found(execution_id))
    }

    fn execution_record(&self, execution: &AutomationExecution) -> AutomationExecutionRecord {
        AutomationExecutionRecord {
            tenant_id: execution.tenant_id.clone(),
            principal_id: execution.principal_id.clone(),
            execution_id: execution.execution_id.clone(),
            execution: execution.clone(),
            updated_at: utc_now_rfc3339_millis(),
        }
    }

    fn clear_execution_state(&self, execution_key: &str) {
        self.executions.lock_automation().remove(execution_key);
        self.event_orders.lock_automation().remove(execution_key);
    }

    fn append_event(
        &self,
        auth: &AppContext,
        execution: &AutomationExecution,
        event_type: &str,
        ordering_seq: u64,
    ) -> Result<(), AutomationError> {
        let execution_identity = execution_event_identity(execution);
        let event_identity = automation_event_key(execution, &[event_type]);
        let committed_at = execution
            .completed_at
            .clone()
            .unwrap_or_else(|| execution.requested_at.clone());
        let envelope = CommitEnvelope {
            event_id: format!("evt_{event_identity}"),
            tenant_id: auth.tenant_id.clone(),
            organization_id: "0".into(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::AutomationExecution,
            aggregate_id: execution_identity.clone(),
            scope_type: "automation_execution".into(),
            scope_id: execution_identity.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                auth.tenant_id.as_str(),
                &execution_identity,
            ),
            ordering_seq,
            causation_id: None,
            correlation_id: Some(execution_identity),
            idempotency_key: Some(event_identity),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: execution.requested_at.clone(),
            committed_at,
            payload_schema: Some("automation.execution.v1".into()),
            payload: serde_json::to_string(execution).map_err(|error| {
                AutomationError::internal(
                    "automation_execution_serialize_failed",
                    format!("failed to serialize automation execution: {error}"),
                )
            })?,
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }

    fn append_json_event<P: Serialize>(
        &self,
        auth: &AppContext,
        execution: &AutomationExecution,
        event_type: &str,
        payload_schema: &str,
        payload: &P,
    ) -> Result<(), AutomationError> {
        let event_scope_key = execution_event_identity(execution);
        let ordering_seq = {
            let mut orders = self.event_orders.lock_automation();
            let next = orders.get(event_scope_key.as_str()).copied().unwrap_or(2) + 1;
            orders.insert(event_scope_key.clone(), next);
            next
        };
        let occurred_at = utc_now_rfc3339_millis();
        let ordering_seq_segment = ordering_seq.to_string();
        let event_identity =
            automation_event_key(execution, &[event_type, ordering_seq_segment.as_str()]);
        let envelope = CommitEnvelope {
            event_id: format!("evt_{event_identity}"),
            tenant_id: auth.tenant_id.clone(),
            organization_id: "0".into(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::AutomationExecution,
            aggregate_id: event_scope_key.clone(),
            scope_type: "automation_execution".into(),
            scope_id: event_scope_key.clone(),
            ordering_key: CommitEnvelope::ordering_key(auth.tenant_id.as_str(), &event_scope_key),
            ordering_seq,
            causation_id: None,
            correlation_id: Some(event_scope_key),
            idempotency_key: Some(event_identity),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: occurred_at.clone(),
            committed_at: occurred_at,
            payload_schema: Some(payload_schema.into()),
            payload: serde_json::to_string(payload).map_err(|error| {
                AutomationError::internal(
                    "automation_lifecycle_payload_serialize_failed",
                    format!("failed to serialize automation lifecycle payload: {error}"),
                )
            })?,
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }

    fn append_guardrail_event(
        &self,
        auth: &AppContext,
        execution: &AutomationExecution,
        event_type: &str,
        tool_name: &str,
        operator_override_active: bool,
    ) -> Result<(), AutomationError> {
        self.append_json_event(
            auth,
            execution,
            event_type,
            "automation.guardrail.v1",
            &serde_json::json!({
                "capabilityProfileId": AUTOMATION_CAPABILITY_PROFILE_ID,
                "guardrailPolicyId": AUTOMATION_GUARDRAIL_POLICY_ID,
                "toolName": tool_name,
                "restrictedToolPrefixes": AUTOMATION_RESTRICTED_TOOL_PREFIXES,
                "operatorOverridePermission": AUTOMATION_OPERATOR_OVERRIDE_PERMISSION,
                "operatorOverrideActive": operator_override_active,
            }),
        )
    }

    fn execution_for_actor(
        &self,
        auth: &AppContext,
        execution_id: &str,
    ) -> Result<AutomationExecution, AutomationError> {
        self.ensure_execution_state(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        )?;
        self.executions
            .lock_automation()
            .get(
                execution_scope_key(
                    auth.tenant_id.as_str(),
                    auth.actor_kind.as_str(),
                    auth.actor_id.as_str(),
                    execution_id,
                )
                .as_str(),
            )
            .cloned()
            .filter(|execution| {
                execution_matches_principal_kind(execution, auth.actor_kind.as_str())
            })
            .ok_or_else(|| AutomationError::not_found(execution_id))
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryAutomationExecutionStore {
    executions: Arc<Mutex<HashMap<String, AutomationExecutionRecord>>>,
}

impl AutomationExecutionStore for RuntimeMemoryAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        Ok(self
            .executions
            .lock_automation()
            .get(
                execution_scope_key(tenant_id, principal_kind, principal_id, execution_id).as_str(),
            )
            .cloned())
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let key = execution_scope_key(
            record.tenant_id.as_str(),
            record.execution.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.execution_id.as_str(),
        );
        let mut executions = self.executions.lock_automation();
        let next = executions
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        executions.insert(key, next);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_app_context::AppContext;
    use std::collections::BTreeSet;
    use std::panic::{self, AssertUnwindSafe};
    use std::sync::Mutex;

    fn automation_execution_record(
        state: AutomationExecutionState,
        retry_count: u32,
        output_payload: Option<&str>,
        completed_at: Option<&str>,
        failure_reason: Option<&str>,
        updated_at: &str,
    ) -> AutomationExecutionRecord {
        AutomationExecutionRecord {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            execution_id: "ae_demo".into(),
            execution: AutomationExecution {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                principal_kind: "user".into(),
                execution_id: "ae_demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                output_payload: output_payload.map(str::to_owned),
                state,
                retry_count,
                requested_at: "2026-05-06T00:00:00.000Z".into(),
                completed_at: completed_at.map(str::to_owned),
                failure_reason: failure_reason.map(str::to_owned),
            },
            updated_at: updated_at.into(),
        }
    }

    fn demo_auth_context() -> AppContext {
        AppContext {
            tenant_id: "100001".into(),
            organization_id: "0".to_owned(),
            user_id: "1".into(),
            session_id: Some("s_demo".into()),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: BTreeSet::from(["automation.execute".to_string()]),
            actor_id: "1".into(),
            actor_kind: "user".into(),
            device_id: Some("d_demo".into()),
        }
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_automation_runtime_uses_execution_index_for_agent_response_lookup() {
        let source = include_str!("runtime.rs").replace("\r\n", "\n");
        let implementation = source
            .split("#[cfg(test)]")
            .next()
            .expect("automation runtime implementation should be before tests");

        assert!(
            implementation.contains("agent_responses_by_execution: HashMap<String, String>"),
            "automation runtime should maintain a principal/execution -> agent-response stream index"
        );
        assert!(
            implementation.contains("agent_response_key_for_execution("),
            "automation runtime should resolve agent response streams by execution through an index"
        );
        assert!(
            !implementation.contains("responses.values().any(|state|"),
            "start_agent_response must not full-scan all agent response streams to detect existing execution streams"
        );
        assert!(
            !implementation.contains(
                ".agent_responses\n            .lock_automation()\n            .values()"
            ),
            "request_agent_tool_call must not full-scan all agent response streams to find the execution stream"
        );
        assert!(
            implementation
                .contains("pending_tool_calls_by_execution: HashMap<String, BTreeSet<String>>"),
            "automation runtime should maintain a principal/execution -> pending tool-call index"
        );
        assert!(
            implementation.contains("pending_tool_call_for_execution("),
            "complete_agent_response should resolve pending tool calls by execution through an index"
        );
        assert!(
            !implementation.contains("starts_with(tool_call_scope_prefix.as_str())"),
            "complete_agent_response must not full-scan tool calls by key prefix"
        );
    }

    #[test]
    fn test_request_execution_recovers_from_poisoned_executions_lock() {
        let runtime = AutomationRuntime::default();
        poison_mutex(&runtime.executions);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.request_execution(
                &demo_auth_context(),
                RequestAutomationExecution {
                    execution_id: "ae_poison_recovery".into(),
                    trigger_type: "webhook.manual".into(),
                    target_kind: "workflow".into(),
                    target_ref: "wf_demo".into(),
                    input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
                },
            )
        }));
        assert!(
            result.is_ok(),
            "request_execution should not panic when executions lock is poisoned"
        );
        let request_result = result.expect("panic status should be captured");
        assert!(
            request_result.is_ok(),
            "request_execution should recover from poisoned executions lock"
        );
    }

    #[test]
    fn test_runtime_memory_execution_store_load_recovers_from_poisoned_lock() {
        let store = RuntimeMemoryAutomationExecutionStore::default();
        poison_mutex(&store.executions);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            store.load_execution("100001", "user", "1", "ae_poison_store")
        }));
        assert!(
            result.is_ok(),
            "automation execution store load should not panic when lock is poisoned"
        );
        let load_result = result.expect("panic status should be captured");
        assert!(
            load_result.is_ok(),
            "automation execution store load should recover from poisoned lock"
        );
    }

    #[test]
    fn test_runtime_memory_execution_store_rejects_stale_status_regression_writes() {
        let store = RuntimeMemoryAutomationExecutionStore::default();
        store
            .save_execution(automation_execution_record(
                AutomationExecutionState::Succeeded,
                2,
                Some("{\"accepted\":true}"),
                Some("2026-05-06T00:00:02.000Z"),
                None,
                "2026-05-06T00:00:02.000Z",
            ))
            .expect("current automation execution save should succeed");
        store
            .save_execution(automation_execution_record(
                AutomationExecutionState::Running,
                1,
                None,
                None,
                None,
                "2026-05-06T00:00:01.000Z",
            ))
            .expect("stale automation execution save should not fail the caller");

        let restored = store
            .load_execution("100001", "user", "1", "ae_demo")
            .expect("automation execution load should succeed")
            .expect("automation execution should be present");
        assert_eq!(
            restored.execution.state,
            AutomationExecutionState::Succeeded
        );
        assert_eq!(restored.execution.retry_count, 2);
        assert_eq!(
            restored.execution.output_payload.as_deref(),
            Some("{\"accepted\":true}")
        );
        assert_eq!(
            restored.execution.completed_at.as_deref(),
            Some("2026-05-06T00:00:02.000Z")
        );
        assert_eq!(restored.updated_at, "2026-05-06T00:00:02.000Z");
    }
}
