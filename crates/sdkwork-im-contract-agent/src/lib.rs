use std::collections::HashSet;

use im_domain_core::automation::AutomationExecution;
use im_domain_core::conversation::{
    CONVERSATION_AGENT_ASSIGNMENT_MAX_COUNT, ConversationAgentAssignment,
    ConversationAgentAssignmentSource, ConversationAggregateState,
};
use im_domain_core::message::{ContentPart, MessageAttributes, MessageBody, Sender};
use im_time::{max_optional_rfc3339_string, max_rfc3339_string, rfc3339_cmp};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

pub const AGENT_MENTION_DISPATCH_EVENT_TYPE: &str = "conversation.agent_mention_dispatch_requested";
pub const AGENT_MENTION_DISPATCH_PAYLOAD_SCHEMA: &str =
    "conversation.agent_mention_dispatch_requested.v1";
pub const AGENT_MENTION_DISPATCH_OUTBOX_AGGREGATE_TYPE: &str = "conversation_agent_dispatch";
pub const AGENT_MENTION_DISPATCH_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgentMentionDispatchTarget {
    pub dispatch_id: String,
    pub agent_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

/// Durable handoff from the IM message plane to the agent execution plane.
///
/// One request represents one committed source message. `targets` is already
/// de-duplicated by authoritative agent id and each target carries a stable
/// `dispatch_id`, so downstream retries can be handled independently.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgentMentionDispatchRequest {
    pub schema_version: u16,
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub causation_event_id: String,
    pub sender_principal_id: String,
    pub sender_principal_kind: String,
    pub assignment_generation: u64,
    pub targets: Vec<AgentMentionDispatchTarget>,
    pub body: MessageBody,
    pub requested_at: String,
}

impl AgentMentionDispatchRequest {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != AGENT_MENTION_DISPATCH_SCHEMA_VERSION {
            return Err(ContractError::Invalid(
                "agent mention dispatch schema version is unsupported".into(),
            ));
        }
        if self.tenant_id.trim().is_empty()
            || self.organization_id.trim().is_empty()
            || self.conversation_id.trim().is_empty()
            || self.message_id.trim().is_empty()
            || self.message_seq == 0
            || self.causation_event_id.trim().is_empty()
            || self.sender_principal_id.trim().is_empty()
            || self.sender_principal_kind != "user"
            || self.assignment_generation == 0
            || self.requested_at.trim().is_empty()
        {
            return Err(ContractError::Invalid(
                "agent mention dispatch identity is invalid".into(),
            ));
        }
        if self.targets.is_empty() || self.targets.len() > CONVERSATION_AGENT_ASSIGNMENT_MAX_COUNT {
            return Err(ContractError::Invalid(format!(
                "agent mention dispatch target count must be between 1 and {CONVERSATION_AGENT_ASSIGNMENT_MAX_COUNT}"
            )));
        }
        let mut target_ids = HashSet::new();
        let mut dispatch_ids = HashSet::new();
        for target in &self.targets {
            if target.agent_id.trim().is_empty()
                || target.dispatch_id.trim().is_empty()
                || target
                    .revision_id
                    .as_deref()
                    .is_some_and(|revision_id| revision_id.trim().is_empty())
                || !target_ids.insert(target.agent_id.as_str())
                || !dispatch_ids.insert(target.dispatch_id.as_str())
            {
                return Err(ContractError::Invalid(
                    "agent mention dispatch targets are invalid or duplicated".into(),
                ));
            }
        }
        // Keep the dispatch boundary on the same canonical identifier rules as
        // the conversation assignment aggregate. This prevents a consumer
        // from accepting an outbox row that the authoritative assignment
        // store would never allow.
        ConversationAggregateState::new("group")
            .replace_agent_assignments(
                ConversationAgentAssignmentSource::ConversationOverride,
                self.targets
                    .iter()
                    .map(|target| {
                        ConversationAgentAssignment::new(
                            target.agent_id.clone(),
                            target.revision_id.clone(),
                        )
                    })
                    .collect(),
            )
            .map_err(|_| {
                ContractError::Invalid(
                    "agent mention dispatch targets contain an invalid agent or revision id".into(),
                )
            })?;
        let mentioned_ids = self
            .body
            .parts
            .iter()
            .filter_map(ContentPart::as_mention)
            .map(|mention| mention.target_id.as_str())
            .collect::<HashSet<_>>();
        if mentioned_ids != target_ids
            || self
                .body
                .parts
                .iter()
                .filter_map(ContentPart::as_mention)
                .any(|mention| mention.assignment_generation != self.assignment_generation)
        {
            return Err(ContractError::Invalid(
                "agent mention dispatch targets do not match the structured message mentions"
                    .into(),
            ));
        }
        Ok(())
    }
}

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
    use std::collections::BTreeMap;

    use super::*;
    use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};
    use im_domain_core::message::{MentionPart, MentionTargetKind};

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

    fn agent_mention_dispatch_request() -> AgentMentionDispatchRequest {
        AgentMentionDispatchRequest {
            schema_version: AGENT_MENTION_DISPATCH_SCHEMA_VERSION,
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            conversation_id: "g_demo".into(),
            message_id: "90001".into(),
            message_seq: 7,
            causation_event_id: "evt_90001_posted".into(),
            sender_principal_id: "1".into(),
            sender_principal_kind: "user".into(),
            assignment_generation: 3,
            targets: vec![AgentMentionDispatchTarget {
                dispatch_id: "amd_demo".into(),
                agent_id: "agent.im.writer".into(),
                revision_id: Some("revision.im.writer.1".into()),
            }],
            body: MessageBody {
                summary: Some("write this".into()),
                parts: vec![ContentPart::Mention(MentionPart {
                    target_kind: MentionTargetKind::Agent,
                    target_id: "agent.im.writer".into(),
                    display_text: "@Writer".into(),
                    assignment_generation: 3,
                })],
                render_hints: BTreeMap::new(),
                reply_to: None,
            },
            requested_at: "2026-07-12T00:00:00Z".into(),
        }
    }

    #[test]
    fn agent_mention_dispatch_contract_rejects_duplicate_or_mismatched_targets() {
        let valid = agent_mention_dispatch_request();
        assert_eq!(valid.validate(), Ok(()));

        let mut duplicate = valid.clone();
        duplicate.targets.push(duplicate.targets[0].clone());
        assert!(matches!(
            duplicate.validate(),
            Err(ContractError::Invalid(_))
        ));

        let mut mismatched = valid;
        mismatched.targets[0].agent_id = "agent.im.other".into();
        assert!(matches!(
            mismatched.validate(),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_agent_id = agent_mention_dispatch_request();
        invalid_agent_id.targets[0].agent_id = "Agent.Writer".into();
        assert!(matches!(
            invalid_agent_id.validate(),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_revision_id = agent_mention_dispatch_request();
        invalid_revision_id.targets[0].revision_id = Some("revision.Writer.1".into());
        assert!(matches!(
            invalid_revision_id.validate(),
            Err(ContractError::Invalid(_))
        ));

        let mut invalid_message_seq = agent_mention_dispatch_request();
        invalid_message_seq.message_seq = 0;
        assert!(matches!(
            invalid_message_seq.validate(),
            Err(ContractError::Invalid(_))
        ));
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
