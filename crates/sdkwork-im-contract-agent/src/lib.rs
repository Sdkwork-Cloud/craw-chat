use im_domain_core::automation::AutomationExecution;
use im_domain_core::message::{MessageAttributes, Sender};
use im_time::{max_optional_rfc3339_string, max_rfc3339_string, rfc3339_cmp};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSubject {
    pub agent_id: String,
    pub session_id: Option<String>,
    pub metadata: MessageAttributes,
}

impl AgentSubject {
    pub fn sender(&self, member_id: Option<String>) -> Sender {
        Sender {
            id: self.agent_id.clone(),
            kind: "agent".into(),
            member_id,
            device_id: None,
            session_id: self.session_id.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSubjectRecord {
    pub tenant_id: String,
    pub agent: AgentSubject,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationExecutionRecord {
    pub tenant_id: String,
    pub principal_id: String,
    pub execution_id: String,
    pub execution: AutomationExecution,
    pub updated_at: String,
}

impl AutomationExecutionRecord {
    pub fn merge_monotonic(self, next: Self) -> Self {
        let mut selected = if automation_execution_record_precedes(&self, &next) {
            next.clone()
        } else {
            self.clone()
        };

        selected.updated_at = max_rfc3339_string(self.updated_at, next.updated_at);
        selected.execution.retry_count = self.execution.retry_count.max(next.execution.retry_count);
        selected.execution.completed_at = max_optional_rfc3339_string(
            selected.execution.completed_at,
            max_optional_rfc3339_string(self.execution.completed_at, next.execution.completed_at),
        );
        if selected.execution.output_payload.is_none() {
            selected.execution.output_payload = self
                .execution
                .output_payload
                .or(next.execution.output_payload);
        }
        if selected.execution.failure_reason.is_none() {
            selected.execution.failure_reason = self
                .execution
                .failure_reason
                .or(next.execution.failure_reason);
        }
        selected
    }
}

pub trait AgentSubjectStore: Send + Sync {
    fn load_subject(
        &self,
        tenant_id: &str,
        agent_id: &str,
    ) -> Result<Option<AgentSubjectRecord>, ContractError>;

    fn save_subject(&self, record: AgentSubjectRecord) -> Result<(), ContractError>;
}

pub trait AutomationExecutionStore: Send + Sync {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError>;

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError>;
}

fn automation_execution_record_precedes(
    left: &AutomationExecutionRecord,
    right: &AutomationExecutionRecord,
) -> bool {
    automation_execution_state_group_rank(&left.execution.state)
        .cmp(&automation_execution_state_group_rank(
            &right.execution.state,
        ))
        .then_with(|| rfc3339_cmp(left.updated_at.as_str(), right.updated_at.as_str()))
        .then_with(|| {
            automation_execution_state_tie_rank(&left.execution.state)
                .cmp(&automation_execution_state_tie_rank(&right.execution.state))
        })
        .is_lt()
}

fn automation_execution_state_group_rank(
    state: &im_domain_core::automation::AutomationExecutionState,
) -> u8 {
    match state {
        im_domain_core::automation::AutomationExecutionState::Requested => 0,
        im_domain_core::automation::AutomationExecutionState::Running => 1,
        im_domain_core::automation::AutomationExecutionState::Succeeded
        | im_domain_core::automation::AutomationExecutionState::Failed => 2,
    }
}

fn automation_execution_state_tie_rank(
    state: &im_domain_core::automation::AutomationExecutionState,
) -> u8 {
    match state {
        im_domain_core::automation::AutomationExecutionState::Requested => 0,
        im_domain_core::automation::AutomationExecutionState::Running => 1,
        im_domain_core::automation::AutomationExecutionState::Failed => 2,
        im_domain_core::automation::AutomationExecutionState::Succeeded => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};

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

    #[test]
    fn test_automation_execution_record_merge_rejects_stale_status_regression() {
        let current = automation_execution_record(
            AutomationExecutionState::Succeeded,
            2,
            Some("{\"accepted\":true}"),
            Some("2026-05-06T00:00:02.000Z"),
            None,
            "2026-05-06T00:00:02.000Z",
        );
        let stale = automation_execution_record(
            AutomationExecutionState::Running,
            1,
            None,
            None,
            None,
            "2026-05-06T00:00:01.000Z",
        );

        let merged = current.merge_monotonic(stale);

        assert_eq!(merged.execution.state, AutomationExecutionState::Succeeded);
        assert_eq!(merged.execution.retry_count, 2);
        assert_eq!(
            merged.execution.output_payload.as_deref(),
            Some("{\"accepted\":true}")
        );
        assert_eq!(
            merged.execution.completed_at.as_deref(),
            Some("2026-05-06T00:00:02.000Z")
        );
        assert_eq!(merged.updated_at, "2026-05-06T00:00:02.000Z");
    }

    #[test]
    fn test_automation_execution_record_merge_compares_rfc3339_by_instant() {
        let whole_second = automation_execution_record(
            AutomationExecutionState::Succeeded,
            1,
            Some("{\"accepted\":true}"),
            Some("2026-05-06T00:00:00Z"),
            None,
            "2026-05-06T00:00:00Z",
        );
        let later_fraction = automation_execution_record(
            AutomationExecutionState::Succeeded,
            2,
            Some("{\"accepted\":true}"),
            Some("2026-05-06T00:00:00.100Z"),
            None,
            "2026-05-06T00:00:00.100Z",
        );

        let merged = whole_second.merge_monotonic(later_fraction);

        assert_eq!(merged.execution.state, AutomationExecutionState::Succeeded);
        assert_eq!(merged.execution.retry_count, 2);
        assert_eq!(
            merged.execution.completed_at.as_deref(),
            Some("2026-05-06T00:00:00.100Z")
        );
        assert_eq!(merged.updated_at, "2026-05-06T00:00:00.100Z");
    }
}
