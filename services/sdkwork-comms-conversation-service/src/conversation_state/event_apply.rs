use im_domain_core::conversation::{
    ConversationActorView, ConversationAgentAssignment, ConversationAgentAssignmentSet,
    ConversationAgentAssignmentSource, ConversationAgentHandoffView,
};
use serde::Deserialize;

use super::ConversationSummaryView;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationCreatedPayload {
    pub(super) conversation_type: String,
    pub(super) agent_assignments: Option<ConversationAgentAssignmentsConversationStatePayload>,
    pub(super) title: Option<String>,
    pub(super) group_name: Option<String>,
    pub(super) display_name: Option<String>,
    pub(super) source: Option<ConversationStateActorView>,
    pub(super) target: Option<ConversationStateActorView>,
    pub(super) handoff: Option<ConversationCreatedHandoffPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationPolicyAppliedConversationStatePayload {
    pub(super) conversation_id: String,
    pub(super) policy_version: String,
    pub(super) history_visibility: String,
    pub(super) retention_policy_ref: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationMemberRoleChangedPayload {
    pub(super) updated_member: im_domain_core::conversation::ConversationMember,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationStateActorView {
    pub(super) id: String,
    pub(super) kind: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationCreatedHandoffPayload {
    pub(super) session_id: String,
    pub(super) reason: Option<String>,
    pub(super) status: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AgentHandoffStatusChangedConversationStatePayload {
    pub(super) changed_by: ConversationStateActorView,
    pub(super) changed_at: String,
    pub(super) state: ConversationStateAgentHandoffStatePayload,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationStateAgentHandoffStatePayload {
    pub(super) conversation_id: String,
    pub(super) status: String,
    pub(super) source: ConversationStateActorView,
    pub(super) target: ConversationStateActorView,
    pub(super) handoff_session_id: String,
    pub(super) handoff_reason: Option<String>,
    pub(super) accepted_at: Option<String>,
    pub(super) accepted_by: Option<ConversationStateActorView>,
    pub(super) resolved_at: Option<String>,
    pub(super) resolved_by: Option<ConversationStateActorView>,
    pub(super) closed_at: Option<String>,
    pub(super) closed_by: Option<ConversationStateActorView>,
}

#[derive(Debug)]
pub enum ConversationStateError {
    InvalidPayload(serde_json::Error),
    InvalidState(serde_json::Error),
    InvalidEvent(String),
    StoreFailure(im_platform_contracts::ContractError),
}

impl std::fmt::Display for ConversationStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversationStateError::InvalidPayload(error) => {
                write!(f, "conversation_state payload invalid: {error}")
            }
            ConversationStateError::InvalidState(error) => {
                write!(f, "conversation state invalid: {error}")
            }
            ConversationStateError::InvalidEvent(message) => {
                write!(f, "conversation_state event invalid: {message}")
            }
            ConversationStateError::StoreFailure(error) => {
                write!(f, "conversation_state store failure: {error:?}")
            }
        }
    }
}

impl std::error::Error for ConversationStateError {}

pub(super) fn handoff_view_from_created_payload(
    payload: &ConversationCreatedPayload,
) -> Result<Option<ConversationAgentHandoffView>, ConversationStateError> {
    if payload.conversation_type != "agent_handoff" {
        return Ok(None);
    }

    let source = payload.source.as_ref().ok_or_else(|| {
        ConversationStateError::InvalidEvent("agent_handoff source missing".into())
    })?;
    let target = payload.target.as_ref().ok_or_else(|| {
        ConversationStateError::InvalidEvent("agent_handoff target missing".into())
    })?;
    let handoff = payload.handoff.as_ref().ok_or_else(|| {
        ConversationStateError::InvalidEvent("agent_handoff payload missing".into())
    })?;

    Ok(Some(ConversationAgentHandoffView {
        status: handoff.status.clone(),
        source: conversation_state_actor_to_view(source),
        target: conversation_state_actor_to_view(target),
        handoff_session_id: handoff.session_id.clone(),
        handoff_reason: handoff.reason.clone(),
        accepted_at: None,
        accepted_by: None,
        resolved_at: None,
        resolved_by: None,
        closed_at: None,
        closed_by: None,
    }))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationAgentAssignmentsConversationStatePayload {
    pub(super) generation: u64,
    pub(super) source: ConversationAgentAssignmentSource,
    pub(super) agents: Vec<ConversationAgentAssignment>,
    pub(super) policy_id: Option<String>,
    pub(super) policy_version: Option<u32>,
}

impl ConversationAgentAssignmentsConversationStatePayload {
    pub(super) fn assignment_set(&self) -> ConversationAgentAssignmentSet {
        ConversationAgentAssignmentSet {
            generation: self.generation,
            source: self.source.clone(),
            agents: self.agents.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationAgentsReplacedConversationStatePayload {
    pub(super) conversation_id: String,
    pub(super) previous_generation: u64,
    pub(super) agent_assignments: ConversationAgentAssignmentsConversationStatePayload,
    pub(super) replaced_at: String,
}

pub(super) fn title_from_created_payload(payload: &ConversationCreatedPayload) -> Option<String> {
    [
        payload.title.as_deref(),
        payload.group_name.as_deref(),
        payload.display_name.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(str::trim)
    .find(|value| !value.is_empty())
    .map(str::to_owned)
}

pub(super) fn handoff_view_from_state_payload(
    state: &ConversationStateAgentHandoffStatePayload,
) -> ConversationAgentHandoffView {
    ConversationAgentHandoffView {
        status: state.status.clone(),
        source: conversation_state_actor_to_view(&state.source),
        target: conversation_state_actor_to_view(&state.target),
        handoff_session_id: state.handoff_session_id.clone(),
        handoff_reason: state.handoff_reason.clone(),
        accepted_at: state.accepted_at.clone(),
        accepted_by: state
            .accepted_by
            .as_ref()
            .map(conversation_state_actor_to_view),
        resolved_at: state.resolved_at.clone(),
        resolved_by: state
            .resolved_by
            .as_ref()
            .map(conversation_state_actor_to_view),
        closed_at: state.closed_at.clone(),
        closed_by: state
            .closed_by
            .as_ref()
            .map(conversation_state_actor_to_view),
    }
}

pub(super) fn latest_summary_activity_at(summary: &ConversationSummaryView) -> Option<String> {
    let mut candidates = Vec::new();
    if let Some(last_message_at) = summary.last_message_at.clone() {
        candidates.push(last_message_at);
    }
    if let Some(handoff) = summary.agent_handoff.as_ref() {
        if let Some(accepted_at) = handoff.accepted_at.clone() {
            candidates.push(accepted_at);
        }
        if let Some(resolved_at) = handoff.resolved_at.clone() {
            candidates.push(resolved_at);
        }
        if let Some(closed_at) = handoff.closed_at.clone() {
            candidates.push(closed_at);
        }
    }
    candidates.into_iter().max()
}

fn conversation_state_actor_to_view(actor: &ConversationStateActorView) -> ConversationActorView {
    ConversationActorView {
        id: actor.id.clone(),
        kind: actor.kind.clone(),
    }
}
