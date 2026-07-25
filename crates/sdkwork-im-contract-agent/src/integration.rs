use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

use crate::AgentMentionDispatchRequest;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentAssignmentSource {
    DefaultPolicy,
    ConversationOverride,
}

impl AgentAssignmentSource {
    pub fn db_code(self) -> i16 {
        match self {
            Self::DefaultPolicy => 0,
            Self::ConversationOverride => 1,
        }
    }

    pub fn from_db_code(value: i16) -> Result<Self, ContractError> {
        match value {
            0 => Ok(Self::DefaultPolicy),
            1 => Ok(Self::ConversationOverride),
            _ => Err(ContractError::Invalid(
                "invalid agent assignment source".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationAgentAssignmentItem {
    pub agent_id: String,
    pub agent_revision_ref: Option<String>,
    pub position: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaceConversationAgentAssignments {
    pub tenant_id: u64,
    pub organization_id: u64,
    pub conversation_id: String,
    pub assignment_source: AgentAssignmentSource,
    pub assignment_generation: u64,
    pub assigned_by: u64,
    pub assigned_at: String,
    pub source_event_id: String,
    pub source_aggregate_version: u64,
    pub payload_hash: String,
    pub items: Vec<ConversationAgentAssignmentItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConversationAgentAssignmentRecord {
    pub tenant_id: u64,
    pub organization_id: u64,
    pub conversation_id: String,
    pub agent_id: String,
    pub agent_revision_ref: Option<String>,
    pub assignment_source: AgentAssignmentSource,
    pub assignment_generation: u64,
    pub position: i32,
    pub enabled: bool,
    pub status: i16,
    pub source_aggregate_version: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentBindingStatus {
    Pending,
    Active,
    Failed,
    Closed,
    Superseded,
}

impl AgentBindingStatus {
    pub fn db_code(self) -> i16 {
        match self {
            Self::Pending => 0,
            Self::Active => 1,
            Self::Failed => 2,
            Self::Closed => 3,
            Self::Superseded => 4,
        }
    }

    pub fn from_db_code(value: i16) -> Result<Self, ContractError> {
        match value {
            0 => Ok(Self::Pending),
            1 => Ok(Self::Active),
            2 => Ok(Self::Failed),
            3 => Ok(Self::Closed),
            4 => Ok(Self::Superseded),
            _ => Err(ContractError::Invalid(
                "invalid agent binding status".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConversationAgentBindingRecord {
    pub binding_id: String,
    pub tenant_id: u64,
    pub organization_id: u64,
    pub conversation_id: String,
    pub agent_id: String,
    pub agent_revision_ref: Option<String>,
    pub assignment_generation: u64,
    pub agents_session_id: Option<String>,
    pub status: AgentBindingStatus,
    pub idempotency_key: String,
    pub payload_hash: String,
    pub created_by: u64,
    pub updated_by: u64,
    pub last_error_code: Option<String>,
    pub last_error_detail: Option<String>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentDispatchStatus {
    Pending,
    Leased,
    Dispatched,
    Running,
    Completed,
    Failed,
    Cancelled,
    DeadLetter,
}

impl AgentDispatchStatus {
    pub fn db_code(self) -> i16 {
        match self {
            Self::Pending => 0,
            Self::Leased => 1,
            Self::Dispatched => 2,
            Self::Running => 3,
            Self::Completed => 4,
            Self::Failed => 5,
            Self::Cancelled => 6,
            Self::DeadLetter => 7,
        }
    }

    pub fn from_db_code(value: i16) -> Result<Self, ContractError> {
        match value {
            0 => Ok(Self::Pending),
            1 => Ok(Self::Leased),
            2 => Ok(Self::Dispatched),
            3 => Ok(Self::Running),
            4 => Ok(Self::Completed),
            5 => Ok(Self::Failed),
            6 => Ok(Self::Cancelled),
            7 => Ok(Self::DeadLetter),
            _ => Err(ContractError::Invalid(
                "invalid agent dispatch status".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDispatchRecord {
    pub dispatch_id: String,
    pub tenant_id: u64,
    pub organization_id: u64,
    pub conversation_id: String,
    pub source_message_id: u64,
    pub source_message_seq: u64,
    pub agent_id: String,
    pub agent_revision_ref: Option<String>,
    pub assignment_generation: u64,
    pub binding_id: Option<String>,
    pub agents_session_id: Option<String>,
    pub agents_turn_id: Option<String>,
    pub status: AgentDispatchStatus,
    pub idempotency_key: String,
    pub payload_hash: String,
    pub attempt_count: u32,
    pub max_attempts: u32,
    pub lease_owner: Option<String>,
    pub lease_expires_at: Option<String>,
    pub next_attempt_at: String,
    pub requested_by: u64,
    pub reply_message_id: Option<u64>,
    pub reply_message_seq: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentReplyCommitResult {
    pub reply_message_id: u64,
    pub reply_message_seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentDispatchReplyCompletion {
    pub tenant_id: u64,
    pub organization_id: u64,
    pub conversation_id: String,
    pub dispatch_id: String,
    pub lease_owner: String,
    pub agent_id: String,
    pub agent_revision_ref: Option<String>,
    pub assignment_generation: u64,
    pub agents_session_id: String,
    pub agents_turn_id: String,
}

pub trait AgentIntegrationStore: Send + Sync {
    fn replace_conversation_agents(
        &self,
        command: ReplaceConversationAgentAssignments,
    ) -> Result<(), ContractError>;

    fn list_conversation_agents(
        &self,
        tenant_id: u64,
        organization_id: u64,
        conversation_id: &str,
        limit: usize,
    ) -> Result<Vec<ConversationAgentAssignmentRecord>, ContractError>;

    fn enqueue_dispatches(
        &self,
        request: &AgentMentionDispatchRequest,
        max_attempts: u32,
    ) -> Result<Vec<AgentDispatchRecord>, ContractError>;

    fn claim_dispatches(
        &self,
        tenant_id: u64,
        organization_id: u64,
        lease_owner: &str,
        now: &str,
        lease_expires_at: &str,
        limit: usize,
    ) -> Result<Vec<AgentDispatchRecord>, ContractError>;

    fn claim_dispatches_global(
        &self,
        lease_owner: &str,
        now: &str,
        lease_expires_at: &str,
        limit: usize,
    ) -> Result<Vec<AgentDispatchRecord>, ContractError>;

    fn resolve_binding(
        &self,
        tenant_id: u64,
        organization_id: u64,
        conversation_id: &str,
        agent_id: &str,
        assignment_generation: u64,
    ) -> Result<Option<ConversationAgentBindingRecord>, ContractError>;

    fn save_binding(
        &self,
        binding: ConversationAgentBindingRecord,
    ) -> Result<ConversationAgentBindingRecord, ContractError>;

    fn mark_dispatch_running(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        binding_id: &str,
        agents_session_id: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;

    fn complete_dispatch(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        agents_turn_id: &str,
        reply: AgentReplyCommitResult,
        completed_at: &str,
    ) -> Result<(), ContractError>;

    fn defer_dispatch_reconciliation(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        agents_turn_id: Option<&str>,
        detail: &str,
        next_attempt_at: &str,
        updated_at: &str,
    ) -> Result<(), ContractError>;

    fn fail_dispatch(
        &self,
        dispatch: &AgentDispatchRecord,
        lease_owner: &str,
        error_code: &str,
        error_detail: &str,
        next_attempt_at: &str,
        updated_at: &str,
    ) -> Result<AgentDispatchStatus, ContractError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_codes_match_database_constraints() {
        for (status, code) in [
            (AgentDispatchStatus::Pending, 0),
            (AgentDispatchStatus::Leased, 1),
            (AgentDispatchStatus::Dispatched, 2),
            (AgentDispatchStatus::Running, 3),
            (AgentDispatchStatus::Completed, 4),
            (AgentDispatchStatus::Failed, 5),
            (AgentDispatchStatus::Cancelled, 6),
            (AgentDispatchStatus::DeadLetter, 7),
        ] {
            assert_eq!(status.db_code(), code);
            assert_eq!(AgentDispatchStatus::from_db_code(code), Ok(status));
        }
    }

    #[test]
    fn assignment_source_codes_match_database_constraints() {
        for (source, code) in [
            (AgentAssignmentSource::DefaultPolicy, 0),
            (AgentAssignmentSource::ConversationOverride, 1),
        ] {
            assert_eq!(source.db_code(), code);
            assert_eq!(AgentAssignmentSource::from_db_code(code), Ok(source));
        }
        assert!(AgentAssignmentSource::from_db_code(2).is_err());
    }
}
