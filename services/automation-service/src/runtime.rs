//! Automation runtime: in-memory execution state, agent response streams, tool call tracking,
//! event journaling, and the business logic that orchestrates the automation lifecycle.

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use im_app_context::AppContext;
use im_domain_core::automation::{
    AgentToolCall, AgentToolCallState, AutomationExecution, AutomationExecutionState,
};
use im_domain_core::stream::{
    StreamDurabilityClass, StreamFrame, StreamSession, StreamSessionState,
};
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
    frame_bytes: usize,
    estimated_bytes: usize,
    terminal_at: Option<Instant>,
}

pub struct AutomationRuntime {
    executions: Mutex<AutomationExecutionRuntimeStore>,
    agent_responses: Mutex<AgentResponseRuntimeStore>,
    tool_calls: Mutex<AgentToolCallRuntimeStore>,
    event_orders: Mutex<HashMap<String, u64>>,
    event_locks: [Mutex<()>; AUTOMATION_EVENT_LOCK_SHARDS],
    limits: AutomationRuntimeLimits,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    execution_store: Arc<dyn AutomationExecutionStore>,
}

const RUNTIME_ENTRY_OVERHEAD_BYTES: usize = 512;
const AUTOMATION_EVENT_LOCK_SHARDS: usize = 256;

#[derive(Clone, Debug)]
struct AutomationRuntimeLimits {
    max_executions: usize,
    max_execution_bytes: usize,
    max_agent_responses: usize,
    max_agent_response_bytes: usize,
    max_frames_per_response: usize,
    max_frame_bytes_per_response: usize,
    max_tool_calls: usize,
    max_tool_call_bytes: usize,
    terminal_ttl: Duration,
}

impl Default for AutomationRuntimeLimits {
    fn default() -> Self {
        Self {
            max_executions: AUTOMATION_RUNTIME_MAX_EXECUTIONS,
            max_execution_bytes: AUTOMATION_RUNTIME_MAX_EXECUTION_BYTES,
            max_agent_responses: AUTOMATION_RUNTIME_MAX_AGENT_RESPONSES,
            max_agent_response_bytes: AUTOMATION_RUNTIME_MAX_AGENT_RESPONSE_BYTES,
            max_frames_per_response: AUTOMATION_RUNTIME_MAX_FRAMES_PER_RESPONSE,
            max_frame_bytes_per_response: AUTOMATION_RUNTIME_MAX_FRAME_BYTES_PER_RESPONSE,
            max_tool_calls: AUTOMATION_RUNTIME_MAX_TOOL_CALLS,
            max_tool_call_bytes: AUTOMATION_RUNTIME_MAX_TOOL_CALL_BYTES,
            terminal_ttl: Duration::from_secs(AUTOMATION_RUNTIME_TERMINAL_TTL_SECONDS),
        }
    }
}

#[derive(Debug)]
struct CachedAutomationExecution {
    execution: AutomationExecution,
    estimated_bytes: usize,
    terminal_at: Option<Instant>,
}

#[derive(Default)]
struct AutomationExecutionRuntimeStore {
    by_execution: HashMap<String, CachedAutomationExecution>,
    terminal_order: VecDeque<String>,
    estimated_bytes: usize,
}

impl AutomationExecutionRuntimeStore {
    fn contains_key(&self, execution_key: &str) -> bool {
        self.by_execution.contains_key(execution_key)
    }

    fn get(&self, execution_key: &str) -> Option<&AutomationExecution> {
        self.by_execution
            .get(execution_key)
            .map(|cached| &cached.execution)
    }

    fn remove(&mut self, execution_key: &str) {
        if let Some(removed) = self.by_execution.remove(execution_key) {
            self.estimated_bytes = self.estimated_bytes.saturating_sub(removed.estimated_bytes);
        }
    }

    fn insert(
        &mut self,
        execution_key: String,
        execution: AutomationExecution,
        limits: &AutomationRuntimeLimits,
    ) -> Result<Vec<String>, AutomationError> {
        let previous = self.by_execution.remove(execution_key.as_str());
        if let Some(previous) = previous.as_ref() {
            self.estimated_bytes = self
                .estimated_bytes
                .saturating_sub(previous.estimated_bytes);
        }
        let estimated_bytes = estimate_execution_bytes(execution_key.as_str(), &execution);
        if estimated_bytes > limits.max_execution_bytes {
            self.restore(execution_key, previous);
            return Err(AutomationError::runtime_capacity("executions"));
        }

        let now = Instant::now();
        let mut evicted = self.prune_expired(now, limits.terminal_ttl);
        crate::metrics::record_terminal_evictions("executions", "ttl", evicted.len());
        while self.by_execution.len() >= limits.max_executions
            || self.estimated_bytes.saturating_add(estimated_bytes) > limits.max_execution_bytes
        {
            let Some(evicted_key) = self.evict_oldest_terminal() else {
                self.restore(execution_key, previous);
                return Err(AutomationError::runtime_capacity("executions"));
            };
            evicted.push(evicted_key);
            crate::metrics::record_terminal_evictions("executions", "capacity", 1);
        }

        let terminal_at = execution_is_terminal(&execution).then_some(now);
        if terminal_at.is_some() {
            self.terminal_order.push_back(execution_key.clone());
        }
        self.estimated_bytes = self.estimated_bytes.saturating_add(estimated_bytes);
        self.by_execution.insert(
            execution_key,
            CachedAutomationExecution {
                execution,
                estimated_bytes,
                terminal_at,
            },
        );
        Ok(evicted)
    }

    fn restore(&mut self, execution_key: String, previous: Option<CachedAutomationExecution>) {
        let Some(previous) = previous else {
            return;
        };
        if previous.terminal_at.is_some() {
            self.terminal_order.push_back(execution_key.clone());
        }
        self.estimated_bytes = self
            .estimated_bytes
            .saturating_add(previous.estimated_bytes);
        self.by_execution.insert(execution_key, previous);
    }

    fn prune_expired(&mut self, now: Instant, ttl: Duration) -> Vec<String> {
        let mut evicted = Vec::new();
        loop {
            let Some(key) = self.terminal_order.front().cloned() else {
                break;
            };
            let expired = self
                .by_execution
                .get(key.as_str())
                .and_then(|cached| cached.terminal_at)
                .is_none_or(|terminal_at| now.saturating_duration_since(terminal_at) >= ttl);
            if !expired {
                break;
            }
            self.terminal_order.pop_front();
            if self.by_execution.contains_key(key.as_str()) {
                self.remove(key.as_str());
                evicted.push(key);
            }
        }
        evicted
    }

    fn evict_oldest_terminal(&mut self) -> Option<String> {
        while let Some(key) = self.terminal_order.pop_front() {
            let is_terminal = self
                .by_execution
                .get(key.as_str())
                .is_some_and(|cached| cached.terminal_at.is_some());
            if is_terminal {
                self.remove(key.as_str());
                return Some(key);
            }
        }
        None
    }
}

#[derive(Default)]
struct AgentToolCallRuntimeStore {
    by_call: HashMap<String, AgentToolCall>,
    pending_tool_calls_by_execution: HashMap<String, BTreeSet<String>>,
    terminal_order: VecDeque<(String, Instant)>,
    estimated_bytes: usize,
}

impl AgentToolCallRuntimeStore {
    fn get(&self, tool_call_key: &str) -> Option<&AgentToolCall> {
        self.by_call.get(tool_call_key)
    }

    fn pending_tool_call_for_execution(&self, execution_key: &str) -> Option<String> {
        self.pending_tool_calls_by_execution
            .get(execution_key)
            .and_then(|tool_call_keys| tool_call_keys.iter().next())
            .and_then(|tool_call_key| self.by_call.get(tool_call_key))
            .map(|tool_call| tool_call.tool_call_id.clone())
    }

    fn insert(
        &mut self,
        execution_key: String,
        tool_call_key: String,
        tool_call: AgentToolCall,
        limits: &AutomationRuntimeLimits,
    ) -> Result<(), AutomationError> {
        if let Some(previous) = self.by_call.get(tool_call_key.as_str()).cloned() {
            self.remove_pending_index(execution_key.as_str(), tool_call_key.as_str(), &previous);
            self.estimated_bytes = self
                .estimated_bytes
                .saturating_sub(estimate_tool_call_bytes(tool_call_key.as_str(), &previous));
        }
        let estimated_bytes = estimate_tool_call_bytes(tool_call_key.as_str(), &tool_call);
        self.ensure_capacity(estimated_bytes, 1, limits)?;
        if tool_call.state == AgentToolCallState::Requested {
            self.pending_tool_calls_by_execution
                .entry(execution_key)
                .or_default()
                .insert(tool_call_key.clone());
        }
        self.estimated_bytes = self.estimated_bytes.saturating_add(estimated_bytes);
        self.by_call.insert(tool_call_key, tool_call);
        Ok(())
    }

    fn prepare_completion(
        &mut self,
        tool_call_key: &str,
        result_payload: String,
        completed_at: String,
        limits: &AutomationRuntimeLimits,
    ) -> Result<AgentToolCall, AutomationError> {
        let Some(existing) = self.get(tool_call_key).cloned() else {
            return Err(AutomationError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "agent_tool_call_not_found",
                message: format!("agent tool call not found: {tool_call_key}"),
            });
        };
        if existing.state == AgentToolCallState::Completed {
            if existing.result_payload.as_deref() == Some(result_payload.as_str()) {
                return Ok(existing);
            }
            return Err(AutomationError {
                status: axum::http::StatusCode::CONFLICT,
                code: "agent_tool_call_conflict",
                message: format!(
                    "agent tool call already completed: {}",
                    existing.tool_call_id
                ),
            });
        }
        let previous_bytes = estimate_tool_call_bytes(tool_call_key, &existing);
        let mut completed = existing;
        completed.result_payload = Some(result_payload);
        completed.state = AgentToolCallState::Completed;
        completed.completed_at = Some(completed_at);
        let completed_bytes = estimate_tool_call_bytes(tool_call_key, &completed);
        let additional_bytes = completed_bytes.saturating_sub(previous_bytes);
        self.ensure_capacity(additional_bytes, 0, limits)?;
        Ok(completed)
    }

    fn commit_completion(
        &mut self,
        execution_key: &str,
        tool_call_key: &str,
        completed: AgentToolCall,
    ) {
        let previous = self.by_call.get(tool_call_key).cloned();
        let was_requested = previous
            .as_ref()
            .is_some_and(|tool_call| tool_call.state == AgentToolCallState::Requested);
        let previous_bytes = previous.as_ref().map_or(0, |tool_call| {
            estimate_tool_call_bytes(tool_call_key, tool_call)
        });
        let completed_bytes = estimate_tool_call_bytes(tool_call_key, &completed);
        self.estimated_bytes = self
            .estimated_bytes
            .saturating_sub(previous_bytes)
            .saturating_add(completed_bytes);
        self.by_call
            .insert(tool_call_key.to_owned(), completed.clone());
        self.terminal_order
            .push_back((tool_call_key.to_owned(), Instant::now()));
        if was_requested
            && let Some(tool_call_keys) =
                self.pending_tool_calls_by_execution.get_mut(execution_key)
        {
            tool_call_keys.remove(tool_call_key);
            if tool_call_keys.is_empty() {
                self.pending_tool_calls_by_execution.remove(execution_key);
            }
        }
    }

    fn ensure_capacity(
        &mut self,
        additional_bytes: usize,
        additional_entries: usize,
        limits: &AutomationRuntimeLimits,
    ) -> Result<(), AutomationError> {
        if additional_bytes > limits.max_tool_call_bytes {
            return Err(AutomationError::runtime_capacity("agent tool calls"));
        }
        let expired = self.prune_expired(Instant::now(), limits.terminal_ttl);
        crate::metrics::record_terminal_evictions("agent_tool_calls", "ttl", expired);
        while self.by_call.len().saturating_add(additional_entries) > limits.max_tool_calls
            || self.estimated_bytes.saturating_add(additional_bytes) > limits.max_tool_call_bytes
        {
            if !self.evict_oldest_terminal() {
                return Err(AutomationError::runtime_capacity("agent tool calls"));
            }
            crate::metrics::record_terminal_evictions("agent_tool_calls", "capacity", 1);
        }
        Ok(())
    }

    fn prune_expired(&mut self, now: Instant, ttl: Duration) -> usize {
        let mut evicted = 0;
        while let Some((key, terminal_at)) = self.terminal_order.front().cloned() {
            if now.saturating_duration_since(terminal_at) < ttl {
                break;
            }
            self.terminal_order.pop_front();
            evicted += usize::from(self.remove_terminal(key.as_str()));
        }
        evicted
    }

    fn evict_oldest_terminal(&mut self) -> bool {
        while let Some((key, _)) = self.terminal_order.pop_front() {
            if self.remove_terminal(key.as_str()) {
                return true;
            }
        }
        false
    }

    fn remove_terminal(&mut self, tool_call_key: &str) -> bool {
        let is_terminal = self
            .by_call
            .get(tool_call_key)
            .is_some_and(|tool_call| tool_call.state != AgentToolCallState::Requested);
        if !is_terminal {
            return false;
        }
        if let Some(removed) = self.by_call.remove(tool_call_key) {
            self.estimated_bytes = self
                .estimated_bytes
                .saturating_sub(estimate_tool_call_bytes(tool_call_key, &removed));
            return true;
        }
        false
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
    terminal_order: VecDeque<String>,
    estimated_bytes: usize,
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
        limits: &AutomationRuntimeLimits,
    ) -> Result<(), AutomationError> {
        self.ensure_capacity(response.estimated_bytes, 1, limits)?;
        self.agent_responses_by_execution
            .insert(execution_key, stream_key.clone());
        self.estimated_bytes = self
            .estimated_bytes
            .saturating_add(response.estimated_bytes);
        self.by_stream.insert(stream_key, response);
        Ok(())
    }

    fn ensure_capacity(
        &mut self,
        additional_bytes: usize,
        additional_entries: usize,
        limits: &AutomationRuntimeLimits,
    ) -> Result<(), AutomationError> {
        if additional_bytes > limits.max_agent_response_bytes {
            return Err(AutomationError::runtime_capacity("agent response streams"));
        }
        let expired = self.prune_expired(Instant::now(), limits.terminal_ttl);
        crate::metrics::record_terminal_evictions("agent_responses", "ttl", expired);
        while self.by_stream.len().saturating_add(additional_entries) > limits.max_agent_responses
            || self.estimated_bytes.saturating_add(additional_bytes)
                > limits.max_agent_response_bytes
        {
            if !self.evict_oldest_terminal() {
                return Err(AutomationError::runtime_capacity("agent response streams"));
            }
            crate::metrics::record_terminal_evictions("agent_responses", "capacity", 1);
        }
        Ok(())
    }

    fn mark_terminal(&mut self, stream_key: &str) {
        if let Some(response) = self.by_stream.get_mut(stream_key) {
            response.terminal_at = Some(Instant::now());
            self.terminal_order.push_back(stream_key.to_owned());
        }
    }

    fn prune_expired(&mut self, now: Instant, ttl: Duration) -> usize {
        let mut evicted = 0;
        loop {
            let Some(key) = self.terminal_order.front().cloned() else {
                break;
            };
            let expired = self
                .by_stream
                .get(key.as_str())
                .and_then(|response| response.terminal_at)
                .is_none_or(|terminal_at| now.saturating_duration_since(terminal_at) >= ttl);
            if !expired {
                break;
            }
            self.terminal_order.pop_front();
            evicted += usize::from(self.remove_terminal(key.as_str()));
        }
        evicted
    }

    fn evict_oldest_terminal(&mut self) -> bool {
        while let Some(key) = self.terminal_order.pop_front() {
            if self.remove_terminal(key.as_str()) {
                return true;
            }
        }
        false
    }

    fn remove_terminal(&mut self, stream_key: &str) -> bool {
        let is_terminal = self
            .by_stream
            .get(stream_key)
            .is_some_and(|response| response.terminal_at.is_some());
        if !is_terminal {
            return false;
        }
        let Some(removed) = self.by_stream.remove(stream_key) else {
            return false;
        };
        self.estimated_bytes = self.estimated_bytes.saturating_sub(removed.estimated_bytes);
        let execution_key = self.agent_responses_by_execution.iter().find_map(
            |(execution_key, candidate_stream_key)| {
                (candidate_stream_key == stream_key).then(|| execution_key.clone())
            },
        );
        if let Some(execution_key) = execution_key {
            self.agent_responses_by_execution
                .remove(execution_key.as_str());
        }
        true
    }
}

fn estimate_optional_string_bytes(value: &Option<String>) -> usize {
    value.as_ref().map_or(0, String::len)
}

fn estimate_string_map_bytes(values: &std::collections::BTreeMap<String, String>) -> usize {
    values.iter().fold(0_usize, |estimated, (key, value)| {
        estimated
            .saturating_add(key.len())
            .saturating_add(value.len())
    })
}

fn estimate_execution_bytes(execution_key: &str, execution: &AutomationExecution) -> usize {
    [
        execution_key.len(),
        execution.tenant_id.len(),
        execution.principal_id.len(),
        execution.principal_kind.len(),
        execution.execution_id.len(),
        execution.trigger_type.len(),
        execution.target_kind.len(),
        execution.target_ref.len(),
        estimate_optional_string_bytes(&execution.input_payload),
        estimate_optional_string_bytes(&execution.output_payload),
        execution.requested_at.len(),
        estimate_optional_string_bytes(&execution.completed_at),
        estimate_optional_string_bytes(&execution.failure_reason),
        RUNTIME_ENTRY_OVERHEAD_BYTES,
    ]
    .into_iter()
    .fold(0_usize, usize::saturating_add)
}

fn estimate_execution_record_bytes(
    execution_key: &str,
    record: &AutomationExecutionRecord,
) -> usize {
    estimate_execution_bytes(execution_key, &record.execution)
        .saturating_add(record.organization_id.len())
        .saturating_add(record.updated_at.len())
        .saturating_add(RUNTIME_ENTRY_OVERHEAD_BYTES)
}

fn estimate_tool_call_bytes(tool_call_key: &str, tool_call: &AgentToolCall) -> usize {
    [
        tool_call_key.len(),
        tool_call.tenant_id.len(),
        tool_call.execution_id.len(),
        tool_call.agent_id.len(),
        tool_call.tool_call_id.len(),
        tool_call.tool_name.len(),
        tool_call.arguments_payload.len(),
        estimate_optional_string_bytes(&tool_call.result_payload),
        tool_call.requested_at.len(),
        estimate_optional_string_bytes(&tool_call.completed_at),
        RUNTIME_ENTRY_OVERHEAD_BYTES,
    ]
    .into_iter()
    .fold(0_usize, usize::saturating_add)
}

fn estimate_stream_session_bytes(session: &StreamSession) -> usize {
    [
        session.tenant_id.len(),
        session.stream_id.len(),
        session.owner_principal_id.len(),
        session.owner_principal_kind.len(),
        session.stream_type.len(),
        session.scope_kind.len(),
        session.scope_id.len(),
        session.ordering_scope.len(),
        estimate_optional_string_bytes(&session.schema_ref),
        estimate_optional_string_bytes(&session.result_message_id),
        estimate_optional_string_bytes(&session.abort_reason),
        session.opened_at.len(),
        estimate_optional_string_bytes(&session.closed_at),
        estimate_optional_string_bytes(&session.expires_at),
        RUNTIME_ENTRY_OVERHEAD_BYTES,
    ]
    .into_iter()
    .fold(0_usize, usize::saturating_add)
}

fn estimate_stream_frame_bytes(frame: &StreamFrame) -> usize {
    [
        frame.tenant_id.len(),
        frame.stream_id.len(),
        frame.stream_type.len(),
        frame.scope_kind.len(),
        frame.scope_id.len(),
        frame.frame_type.len(),
        estimate_optional_string_bytes(&frame.schema_ref),
        frame.encoding.len(),
        frame.payload.len(),
        frame.sender.id.len(),
        frame.sender.kind.len(),
        estimate_optional_string_bytes(&frame.sender.member_id),
        estimate_optional_string_bytes(&frame.sender.device_id),
        estimate_optional_string_bytes(&frame.sender.session_id),
        estimate_string_map_bytes(&frame.sender.metadata),
        estimate_string_map_bytes(&frame.attributes),
        frame.occurred_at.len(),
        RUNTIME_ENTRY_OVERHEAD_BYTES,
    ]
    .into_iter()
    .fold(0_usize, usize::saturating_add)
}

fn estimate_agent_response_bytes(stream_key: &str, response: &AgentResponseRuntimeState) -> usize {
    [
        stream_key.len(),
        response.principal_id.len(),
        response.principal_kind.len(),
        response.execution_id.len(),
        response.agent.agent_id.len(),
        estimate_optional_string_bytes(&response.agent.session_id),
        estimate_string_map_bytes(&response.agent.metadata),
        estimate_optional_string_bytes(&response.member_id),
        estimate_stream_session_bytes(&response.session),
        RUNTIME_ENTRY_OVERHEAD_BYTES,
    ]
    .into_iter()
    .fold(0_usize, usize::saturating_add)
}

fn execution_is_terminal(execution: &AutomationExecution) -> bool {
    matches!(
        execution.state,
        AutomationExecutionState::Succeeded | AutomationExecutionState::Failed
    )
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
    pub fn render_runtime_metrics_prometheus(&self) -> String {
        let (execution_entries, execution_bytes) = {
            let executions = self.executions.lock_automation();
            (executions.by_execution.len(), executions.estimated_bytes)
        };
        let (response_entries, response_bytes) = {
            let responses = self.agent_responses.lock_automation();
            (responses.by_stream.len(), responses.estimated_bytes)
        };
        let (tool_call_entries, tool_call_bytes) = {
            let tool_calls = self.tool_calls.lock_automation();
            (tool_calls.by_call.len(), tool_calls.estimated_bytes)
        };
        crate::metrics::render_prometheus(
            execution_entries,
            execution_bytes,
            response_entries,
            response_bytes,
            tool_call_entries,
            tool_call_bytes,
        )
    }

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
        Self::with_runtime_limits(journal, execution_store, AutomationRuntimeLimits::default())
    }

    fn with_runtime_limits(
        journal: Arc<dyn CommitJournal + Send + Sync>,
        execution_store: Arc<dyn AutomationExecutionStore>,
        limits: AutomationRuntimeLimits,
    ) -> Self {
        Self {
            executions: Mutex::new(AutomationExecutionRuntimeStore::default()),
            agent_responses: Mutex::new(AgentResponseRuntimeStore::default()),
            tool_calls: Mutex::new(AgentToolCallRuntimeStore::default()),
            event_orders: Mutex::new(HashMap::new()),
            event_locks: std::array::from_fn(|_| Mutex::new(())),
            limits,
            journal,
            execution_store,
        }
    }

    pub fn with_dyn_execution_store<J>(
        journal: Arc<J>,
        execution_store: Arc<dyn AutomationExecutionStore>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_runtime_limits(journal, execution_store, AutomationRuntimeLimits::default())
    }

    fn ensure_execution_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<(), AutomationError> {
        let scope_key = execution_scope_key(
            tenant_id,
            organization_id,
            principal_kind,
            principal_id,
            execution_id,
        );
        if self
            .executions
            .lock_automation()
            .contains_key(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .execution_store
            .load_execution(
                tenant_id,
                organization_id,
                principal_kind,
                principal_id,
                execution_id,
            )
            .map_err(AutomationError::automation_store)?;
        if let Some(record) = restored {
            let evicted = self.executions.lock_automation().insert(
                scope_key,
                record.execution,
                &self.limits,
            )?;
            self.clear_event_orders(evicted);
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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        )?;
        let execution_key = execution_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let request_key = automation_execution_request_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
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

        let evicted = {
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
            executions.insert(execution_key.clone(), requested.clone(), &self.limits)?
        };
        self.clear_event_orders(evicted);

        if let Err(error) = self.append_event(auth, &requested, "automation.execution_requested", 1)
        {
            self.clear_execution_state(execution_key.as_str());
            return Err(error);
        }

        self.event_orders
            .lock_automation()
            .insert(execution_key.clone(), 1);
        if let Err(error) = self
            .execution_store
            .save_execution(self.execution_record(auth, &requested))
        {
            self.clear_execution_state(execution_key.as_str());
            return Err(AutomationError::automation_store(error));
        }

        Ok(AutomationExecutionRequestResult {
            delivery_status: AutomationExecutionDeliveryStatus::Accepted,
            execution: requested,
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
        let scope_key = agent_response_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
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
            auth.organization_id.as_str(),
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
        let mut response = AgentResponseRuntimeState {
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
            execution_id: request.execution_id.clone(),
            session: session.clone(),
            agent: request.agent,
            member_id: request.member_id,
            frames: Vec::new(),
            frame_bytes: 0,
            estimated_bytes: 0,
            terminal_at: None,
        };
        response.estimated_bytes = estimate_agent_response_bytes(scope_key.as_str(), &response);
        responses.ensure_capacity(response.estimated_bytes, 1, &self.limits)?;

        let execution = self.transition_execution(
            auth,
            request.execution_id.as_str(),
            AutomationExecutionState::Running,
            None,
            None,
        )?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_response_started",
            "automation.agent_response_stream.v1",
            &payload,
        )?;
        responses.insert(scope_key, execution_response_key, response, &self.limits)?;
        drop(responses);

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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            stream_id,
        );
        let (frame, execution_id, frame_bytes) = {
            let state =
                responses
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
            if state.frames.len() >= self.limits.max_frames_per_response {
                return Err(AutomationError::runtime_capacity("agent response frames"));
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
            let frame_bytes = estimate_stream_frame_bytes(&frame);
            if state.frame_bytes.saturating_add(frame_bytes)
                > self.limits.max_frame_bytes_per_response
            {
                return Err(AutomationError::runtime_capacity(
                    "agent response frame bytes",
                ));
            }
            (frame, state.execution_id.clone(), frame_bytes)
        };
        responses.ensure_capacity(frame_bytes, 0, &self.limits)?;
        let execution = self.execution_for_actor(auth, execution_id.as_str())?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_response_delta",
            "automation.agent_response_frame.v1",
            &frame,
        )?;
        let state = responses.response_mut(scope_key.as_str()).ok_or_else(|| {
            AutomationError::internal(
                "agent_response_runtime_state_lost",
                format!("agent response stream disappeared during append: {stream_id}"),
            )
        })?;
        state.session.last_frame_seq = frame.frame_seq;
        state.session.state = StreamSessionState::Active;
        state.frames.push(frame.clone());
        state.frame_bytes = state.frame_bytes.saturating_add(frame_bytes);
        state.estimated_bytes = state.estimated_bytes.saturating_add(frame_bytes);
        responses.estimated_bytes = responses.estimated_bytes.saturating_add(frame_bytes);

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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            stream_id,
        );
        let state = responses
            .by_stream
            .get(scope_key.as_str())
            .ok_or_else(|| AutomationError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "agent_response_not_found",
                message: format!("agent response stream not found: {stream_id}"),
            })?;

        if state.session.state == StreamSessionState::Completed {
            let session = state.session.clone();
            let execution_id = state.execution_id.clone();
            drop(responses);
            self.transition_execution(
                auth,
                execution_id.as_str(),
                AutomationExecutionState::Succeeded,
                Some(serde_json::to_string(&session).map_err(|error| {
                    AutomationError::internal(
                        "automation_execution_output_serialize_failed",
                        format!("failed to serialize completed agent response: {error}"),
                    )
                })?),
                None,
            )?;
            return Ok(session);
        }
        if state.session.state == StreamSessionState::Aborted {
            return Ok(state.session.clone());
        }

        let execution_id = state.execution_id.clone();
        let tool_call_execution_key = agent_tool_call_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
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

        let previous_session_bytes = estimate_stream_session_bytes(&state.session);
        let mut session = state.session.clone();
        session.last_frame_seq = session.last_frame_seq.max(request.frame_seq);
        session.last_checkpoint_seq = Some(request.frame_seq);
        session.result_message_id = request.result_message_id;
        session.state = StreamSessionState::Completed;
        session.closed_at = Some(utc_now_rfc3339_millis());
        let session_bytes = estimate_stream_session_bytes(&session);
        let additional_bytes = session_bytes.saturating_sub(previous_session_bytes);
        responses.ensure_capacity(additional_bytes, 0, &self.limits)?;

        let execution = self.execution_for_actor(auth, execution_id.as_str())?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_response_completed",
            "automation.agent_response_stream.v1",
            &session,
        )?;
        self.transition_execution(
            auth,
            execution.execution_id.as_str(),
            AutomationExecutionState::Succeeded,
            Some(serde_json::to_string(&session).map_err(|error| {
                AutomationError::internal(
                    "automation_execution_output_serialize_failed",
                    format!("failed to serialize completed agent response: {error}"),
                )
            })?),
            None,
        )?;
        let state = responses.response_mut(scope_key.as_str()).ok_or_else(|| {
            AutomationError::internal(
                "agent_response_runtime_state_lost",
                format!("agent response stream disappeared during completion: {stream_id}"),
            )
        })?;
        state.session = session.clone();
        state.estimated_bytes = state.estimated_bytes.saturating_add(additional_bytes);
        responses.estimated_bytes = responses.estimated_bytes.saturating_add(additional_bytes);
        responses.mark_terminal(scope_key.as_str());

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
            auth.organization_id.as_str(),
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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
            request.tool_call_id.as_str(),
        );
        let tool_call_execution_key = agent_tool_call_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let mut tool_calls = self.tool_calls.lock_automation();
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
        if tool_requires_override {
            self.append_guardrail_event(
                auth,
                &execution,
                "automation.operator_override_applied",
                request.tool_name.as_str(),
                true,
            )?;
        }

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
        let estimated_bytes = estimate_tool_call_bytes(scope_key.as_str(), &tool_call);
        tool_calls.ensure_capacity(estimated_bytes, 1, &self.limits)?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_tool_call_requested",
            "automation.agent_tool_call.v1",
            &tool_call,
        )?;
        tool_calls.insert(
            tool_call_execution_key,
            scope_key,
            tool_call.clone(),
            &self.limits,
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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
            tool_call_id,
        );
        let tool_call_execution_key = agent_tool_call_execution_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        );
        let mut tool_calls = self.tool_calls.lock_automation();
        let tool_call = tool_calls.prepare_completion(
            scope_key.as_str(),
            request.result_payload,
            utc_now_rfc3339_millis(),
            &self.limits,
        )?;
        self.append_json_event(
            auth,
            &execution,
            "automation.agent_tool_call_completed",
            "automation.agent_tool_call.v1",
            &tool_call,
        )?;
        tool_calls.commit_completion(
            tool_call_execution_key.as_str(),
            scope_key.as_str(),
            tool_call.clone(),
        );

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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        )?;
        self.executions
            .lock_automation()
            .get(
                execution_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
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

    fn execution_record(
        &self,
        auth: &AppContext,
        execution: &AutomationExecution,
    ) -> AutomationExecutionRecord {
        AutomationExecutionRecord {
            tenant_id: execution.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
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

    fn clear_event_orders(&self, execution_keys: Vec<String>) {
        if execution_keys.is_empty() {
            return;
        }
        let mut event_orders = self.event_orders.lock_automation();
        for execution_key in execution_keys {
            event_orders.remove(execution_key.as_str());
        }
    }

    fn event_lock(&self, event_scope_key: &str) -> MutexGuard<'_, ()> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        event_scope_key.hash(&mut hasher);
        let shard = (hasher.finish() as usize) % AUTOMATION_EVENT_LOCK_SHARDS;
        self.event_locks[shard].lock_automation()
    }

    fn append_event(
        &self,
        auth: &AppContext,
        execution: &AutomationExecution,
        event_type: &str,
        ordering_seq: u64,
    ) -> Result<(), AutomationError> {
        let execution_identity = execution_event_identity(execution, auth.organization_id.as_str());
        let event_identity =
            automation_event_key(execution, auth.organization_id.as_str(), &[event_type]);
        let committed_at = execution
            .completed_at
            .clone()
            .unwrap_or_else(|| execution.requested_at.clone());
        let envelope = CommitEnvelope {
            event_id: format!("evt_{event_identity}"),
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
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
        if let Err(error) = self.journal.append(envelope) {
            crate::metrics::record_journal_append_failure();
            return Err(error.into());
        }
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
        let event_scope_key = execution_event_identity(execution, auth.organization_id.as_str());
        let _event_guard = self.event_lock(event_scope_key.as_str());
        let ordering_seq = self
            .event_orders
            .lock_automation()
            .get(event_scope_key.as_str())
            .copied()
            .unwrap_or(2)
            .saturating_add(1);
        let occurred_at = utc_now_rfc3339_millis();
        let ordering_seq_segment = ordering_seq.to_string();
        let event_identity = automation_event_key(
            execution,
            auth.organization_id.as_str(),
            &[event_type, ordering_seq_segment.as_str()],
        );
        let envelope = CommitEnvelope {
            event_id: format!("evt_{event_identity}"),
            tenant_id: auth.tenant_id.clone(),
            organization_id: auth.organization_id.clone(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::AutomationExecution,
            aggregate_id: event_scope_key.clone(),
            scope_type: "automation_execution".into(),
            scope_id: event_scope_key.clone(),
            ordering_key: CommitEnvelope::ordering_key(auth.tenant_id.as_str(), &event_scope_key),
            ordering_seq,
            causation_id: None,
            correlation_id: Some(event_scope_key.clone()),
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
        if let Err(error) = self.journal.append(envelope) {
            crate::metrics::record_journal_append_failure();
            return Err(error.into());
        }
        self.event_orders
            .lock_automation()
            .insert(event_scope_key, ordering_seq);
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
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        )?;
        self.executions
            .lock_automation()
            .get(
                execution_scope_key(
                    auth.tenant_id.as_str(),
                    auth.organization_id.as_str(),
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

    fn transition_execution(
        &self,
        auth: &AppContext,
        execution_id: &str,
        next_state: AutomationExecutionState,
        output_payload: Option<String>,
        failure_reason: Option<String>,
    ) -> Result<AutomationExecution, AutomationError> {
        let current = self.execution_for_actor(auth, execution_id)?;
        if current.state == next_state {
            return Ok(current);
        }
        let allowed = matches!(
            (&current.state, &next_state),
            (
                AutomationExecutionState::Requested,
                AutomationExecutionState::Running
            ) | (
                AutomationExecutionState::Requested,
                AutomationExecutionState::Succeeded
            ) | (
                AutomationExecutionState::Requested,
                AutomationExecutionState::Failed
            ) | (
                AutomationExecutionState::Running,
                AutomationExecutionState::Succeeded
            ) | (
                AutomationExecutionState::Running,
                AutomationExecutionState::Failed
            )
        );
        if !allowed {
            return Err(AutomationError::conflict(execution_id));
        }
        let terminal = matches!(
            next_state,
            AutomationExecutionState::Succeeded | AutomationExecutionState::Failed
        );
        let updated = AutomationExecution {
            state: next_state.clone(),
            output_payload,
            failure_reason,
            completed_at: terminal.then(utc_now_rfc3339_millis),
            ..current
        };
        let event_type = match next_state {
            AutomationExecutionState::Running => "automation.execution_started",
            AutomationExecutionState::Succeeded => "automation.execution_completed",
            AutomationExecutionState::Failed => "automation.execution_failed",
            AutomationExecutionState::Requested => {
                return Err(AutomationError::conflict(execution_id));
            }
        };
        self.append_json_event(
            auth,
            &updated,
            event_type,
            "automation.execution.v1",
            &updated,
        )?;
        self.execution_store
            .save_execution(self.execution_record(auth, &updated))
            .map_err(AutomationError::automation_store)?;
        let execution_key = execution_scope_key(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        );
        let evicted = self.executions.lock_automation().insert(
            execution_key,
            updated.clone(),
            &self.limits,
        )?;
        self.clear_event_orders(evicted);
        Ok(updated)
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
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        Ok(self
            .executions
            .lock_automation()
            .get(
                execution_scope_key(
                    tenant_id,
                    organization_id,
                    principal_kind,
                    principal_id,
                    execution_id,
                )
                .as_str(),
            )
            .cloned())
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        let key = execution_scope_key(
            record.tenant_id.as_str(),
            record.organization_id.as_str(),
            record.execution.principal_kind.as_str(),
            record.principal_id.as_str(),
            record.execution_id.as_str(),
        );
        let mut executions = self.executions.lock_automation();
        let previous = executions.remove(key.as_str());
        let next = previous
            .clone()
            .map(|current| current.merge_monotonic(record.clone()))
            .unwrap_or(record);
        let next_bytes = estimate_execution_record_bytes(key.as_str(), &next);
        if next_bytes > AUTOMATION_RUNTIME_MAX_EXECUTION_BYTES {
            if let Some(previous) = previous {
                executions.insert(key, previous);
            }
            return Err(ContractError::Unavailable(
                "in-memory automation execution store byte capacity exhausted".into(),
            ));
        }
        let mut current_bytes = executions
            .iter()
            .fold(0_usize, |estimated, (entry_key, entry)| {
                estimated.saturating_add(estimate_execution_record_bytes(entry_key, entry))
            });
        loop {
            let within_capacity = executions.len().saturating_add(1)
                <= AUTOMATION_RUNTIME_MAX_EXECUTIONS
                && current_bytes.saturating_add(next_bytes)
                    <= AUTOMATION_RUNTIME_MAX_EXECUTION_BYTES;
            if within_capacity {
                break;
            }
            let terminal_key = executions
                .iter()
                .filter(|(_, candidate)| execution_is_terminal(&candidate.execution))
                .min_by(|(left_key, left), (right_key, right)| {
                    left.updated_at
                        .cmp(&right.updated_at)
                        .then_with(|| left_key.cmp(right_key))
                })
                .map(|(candidate_key, _)| candidate_key.clone());
            let Some(terminal_key) = terminal_key else {
                if let Some(previous) = previous {
                    executions.insert(key, previous);
                }
                return Err(ContractError::Unavailable(
                    "in-memory automation execution store entry capacity exhausted".into(),
                ));
            };
            if let Some(removed) = executions.remove(terminal_key.as_str()) {
                current_bytes = current_bytes.saturating_sub(estimate_execution_record_bytes(
                    terminal_key.as_str(),
                    &removed,
                ));
            }
        }
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
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Default)]
    struct ToggleJournal {
        fail: AtomicBool,
    }

    impl CommitJournal for ToggleJournal {
        fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
            if self.fail.load(Ordering::Relaxed) {
                return Err(ContractError::Unavailable(
                    "journal failure for test".into(),
                ));
            }
            Ok(CommitPosition::new("test", 1))
        }
    }

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
            organization_id: "0".into(),
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

    fn execution_request(execution_id: &str) -> RequestAutomationExecution {
        RequestAutomationExecution {
            execution_id: execution_id.into(),
            trigger_type: "agent.manual".into(),
            target_kind: "conversation".into(),
            target_ref: "c_demo".into(),
            input_payload: Some(r#"{"prompt":"hello"}"#.into()),
        }
    }

    fn agent_response_request(execution_id: &str, stream_id: &str) -> StartAgentResponseRequest {
        StartAgentResponseRequest {
            execution_id: execution_id.into(),
            stream_id: stream_id.into(),
            stream_type: "agent.response.delta".into(),
            conversation_id: "c_demo".into(),
            schema_ref: Some("schema://agent/response.delta".into()),
            member_id: Some("cm_agent".into()),
            agent: AgentSubject {
                agent_id: "agent.demo".into(),
                session_id: Some("s_agent".into()),
                metadata: Default::default(),
            },
        }
    }

    fn runtime_with_limits(
        journal: Arc<ToggleJournal>,
        limits: AutomationRuntimeLimits,
    ) -> AutomationRuntime {
        AutomationRuntime::with_runtime_limits(
            journal,
            Arc::new(RuntimeMemoryAutomationExecutionStore::default()),
            limits,
        )
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
            store.load_execution("100001", "0", "user", "1", "ae_poison_store")
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
            .load_execution("100001", "0", "user", "1", "ae_demo")
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

    #[test]
    fn test_runtime_capacity_never_evicts_active_execution() {
        let mut limits = AutomationRuntimeLimits::default();
        limits.max_executions = 1;
        let runtime = runtime_with_limits(Arc::new(ToggleJournal::default()), limits);
        let auth = demo_auth_context();

        runtime
            .request_execution(&auth, execution_request("ae_active_1"))
            .expect("first active execution should fit");
        let error = runtime
            .request_execution(&auth, execution_request("ae_active_2"))
            .expect_err("capacity must reject instead of evicting an active execution");

        assert_eq!(error.code(), "automation_runtime_capacity_exhausted");
        assert!(
            runtime.executions.lock_automation().contains_key(
                execution_scope_key("100001", "0", "user", "1", "ae_active_1").as_str()
            )
        );
    }

    #[test]
    fn test_runtime_capacity_evicts_oldest_terminal_execution() {
        let mut limits = AutomationRuntimeLimits::default();
        limits.max_executions = 1;
        let runtime = runtime_with_limits(Arc::new(ToggleJournal::default()), limits);
        let auth = demo_auth_context();

        runtime
            .request_execution(&auth, execution_request("ae_terminal"))
            .expect("terminal candidate should be requested");
        runtime
            .start_agent_response(&auth, agent_response_request("ae_terminal", "st_terminal"))
            .expect("agent response should start");
        runtime
            .complete_agent_response(
                &auth,
                "st_terminal",
                CompleteAgentResponseRequest {
                    frame_seq: 0,
                    result_message_id: Some("m_terminal".into()),
                },
            )
            .expect("agent response should complete");
        runtime
            .request_execution(&auth, execution_request("ae_active"))
            .expect("new active execution should evict the terminal cache entry");

        let executions = runtime.executions.lock_automation();
        assert!(
            !executions.contains_key(
                execution_scope_key("100001", "0", "user", "1", "ae_terminal").as_str()
            )
        );
        assert!(
            executions.contains_key(
                execution_scope_key("100001", "0", "user", "1", "ae_active").as_str()
            )
        );
    }

    #[test]
    fn test_agent_response_frame_limit_preserves_last_committed_frame() {
        let mut limits = AutomationRuntimeLimits::default();
        limits.max_frames_per_response = 1;
        let runtime = runtime_with_limits(Arc::new(ToggleJournal::default()), limits);
        let auth = demo_auth_context();
        runtime
            .request_execution(&auth, execution_request("ae_frame_limit"))
            .expect("execution should be requested");
        runtime
            .start_agent_response(
                &auth,
                agent_response_request("ae_frame_limit", "st_frame_limit"),
            )
            .expect("agent response should start");

        runtime
            .append_agent_response_delta(
                &auth,
                "st_frame_limit",
                AppendAgentResponseDeltaRequest {
                    frame_seq: 1,
                    frame_type: "delta".into(),
                    schema_ref: None,
                    encoding: "utf-8".into(),
                    payload: "first".into(),
                    attributes: Default::default(),
                },
            )
            .expect("first frame should fit");
        let error = runtime
            .append_agent_response_delta(
                &auth,
                "st_frame_limit",
                AppendAgentResponseDeltaRequest {
                    frame_seq: 2,
                    frame_type: "delta".into(),
                    schema_ref: None,
                    encoding: "utf-8".into(),
                    payload: "second".into(),
                    attributes: Default::default(),
                },
            )
            .expect_err("second frame must be rejected by the per-response limit");
        assert_eq!(error.code(), "automation_runtime_capacity_exhausted");

        let responses = runtime.agent_responses.lock_automation();
        let state = responses
            .by_stream
            .get(agent_response_scope_key("100001", "0", "user", "1", "st_frame_limit").as_str())
            .expect("active response should remain cached");
        assert_eq!(state.frames.len(), 1);
        assert_eq!(state.session.last_frame_seq, 1);
    }

    #[test]
    fn test_journal_failure_does_not_commit_agent_response_frame_to_memory() {
        let journal = Arc::new(ToggleJournal::default());
        let runtime = runtime_with_limits(journal.clone(), AutomationRuntimeLimits::default());
        let auth = demo_auth_context();
        runtime
            .request_execution(&auth, execution_request("ae_journal_failure"))
            .expect("execution should be requested");
        runtime
            .start_agent_response(
                &auth,
                agent_response_request("ae_journal_failure", "st_journal_failure"),
            )
            .expect("agent response should start");
        journal.fail.store(true, Ordering::Relaxed);

        runtime
            .append_agent_response_delta(
                &auth,
                "st_journal_failure",
                AppendAgentResponseDeltaRequest {
                    frame_seq: 1,
                    frame_type: "delta".into(),
                    schema_ref: None,
                    encoding: "utf-8".into(),
                    payload: "must-not-commit".into(),
                    attributes: Default::default(),
                },
            )
            .expect_err("journal failure must fail the append");

        let responses = runtime.agent_responses.lock_automation();
        let state = responses
            .by_stream
            .get(
                agent_response_scope_key("100001", "0", "user", "1", "st_journal_failure").as_str(),
            )
            .expect("response should remain active for a safe retry");
        assert!(state.frames.is_empty());
        assert_eq!(state.session.last_frame_seq, 0);
        drop(responses);
        assert_eq!(
            runtime
                .event_orders
                .lock_automation()
                .get(
                    execution_scope_key("100001", "0", "user", "1", "ae_journal_failure",).as_str(),
                )
                .copied(),
            Some(3),
            "a failed journal append must not consume the next ordering sequence",
        );
    }

    #[test]
    fn test_journal_failure_does_not_commit_agent_tool_call_to_memory() {
        let journal = Arc::new(ToggleJournal::default());
        let runtime = runtime_with_limits(journal.clone(), AutomationRuntimeLimits::default());
        let auth = demo_auth_context();
        runtime
            .request_execution(&auth, execution_request("ae_tool_journal_failure"))
            .expect("execution should be requested");
        runtime
            .start_agent_response(
                &auth,
                agent_response_request("ae_tool_journal_failure", "st_tool_journal_failure"),
            )
            .expect("agent response should start");
        journal.fail.store(true, Ordering::Relaxed);

        runtime
            .request_agent_tool_call(
                &auth,
                RequestAgentToolCallRequest {
                    execution_id: "ae_tool_journal_failure".into(),
                    tool_call_id: "tc_journal_failure".into(),
                    tool_name: "knowledge.search".into(),
                    arguments_payload: r#"{"query":"hello"}"#.into(),
                },
            )
            .expect_err("journal failure must fail the tool-call request");

        let tool_calls = runtime.tool_calls.lock_automation();
        assert!(
            tool_calls
                .get(
                    agent_tool_call_scope_key(
                        "100001",
                        "0",
                        "user",
                        "1",
                        "ae_tool_journal_failure",
                        "tc_journal_failure",
                    )
                    .as_str(),
                )
                .is_none()
        );
    }
}
