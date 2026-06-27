use im_domain_core::conversation::{ConversationActorView, ConversationAgentHandoffView};
use serde::Deserialize;

use super::ConversationSummaryView;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationCreatedPayload {
    pub(super) conversation_type: String,
    pub(super) source: Option<ProjectionActorView>,
    pub(super) target: Option<ProjectionActorView>,
    pub(super) handoff: Option<ConversationCreatedHandoffPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ConversationPolicyAppliedProjectionPayload {
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
pub(super) struct ProjectionActorView {
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
pub(super) struct AgentHandoffStatusChangedProjectionPayload {
    pub(super) changed_by: ProjectionActorView,
    pub(super) changed_at: String,
    pub(super) state: ProjectionAgentHandoffStatePayload,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProjectionAgentHandoffStatePayload {
    pub(super) conversation_id: String,
    pub(super) status: String,
    pub(super) source: ProjectionActorView,
    pub(super) target: ProjectionActorView,
    pub(super) handoff_session_id: String,
    pub(super) handoff_reason: Option<String>,
    pub(super) accepted_at: Option<String>,
    pub(super) accepted_by: Option<ProjectionActorView>,
    pub(super) resolved_at: Option<String>,
    pub(super) resolved_by: Option<ProjectionActorView>,
    pub(super) closed_at: Option<String>,
    pub(super) closed_by: Option<ProjectionActorView>,
}

#[derive(Debug)]
pub enum ProjectionError {
    InvalidPayload(serde_json::Error),
    InvalidSnapshot(serde_json::Error),
    InvalidEvent(String),
    StoreFailure(im_platform_contracts::ContractError),
}

impl std::fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectionError::InvalidPayload(error) => {
                write!(f, "projection payload invalid: {error}")
            }
            ProjectionError::InvalidSnapshot(error) => {
                write!(f, "projection snapshot invalid: {error}")
            }
            ProjectionError::InvalidEvent(message) => {
                write!(f, "projection event invalid: {message}")
            }
            ProjectionError::StoreFailure(error) => {
                write!(f, "projection store failure: {error:?}")
            }
        }
    }
}

impl std::error::Error for ProjectionError {}

pub(super) fn handoff_view_from_created_payload(
    payload: &ConversationCreatedPayload,
) -> Result<Option<ConversationAgentHandoffView>, ProjectionError> {
    if payload.conversation_type != "agent_handoff" {
        return Ok(None);
    }

    let source = payload
        .source
        .as_ref()
        .ok_or_else(|| ProjectionError::InvalidEvent("agent_handoff source missing".into()))?;
    let target = payload
        .target
        .as_ref()
        .ok_or_else(|| ProjectionError::InvalidEvent("agent_handoff target missing".into()))?;
    let handoff = payload
        .handoff
        .as_ref()
        .ok_or_else(|| ProjectionError::InvalidEvent("agent_handoff payload missing".into()))?;

    Ok(Some(ConversationAgentHandoffView {
        status: handoff.status.clone(),
        source: projection_actor_to_view(source),
        target: projection_actor_to_view(target),
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

pub(super) fn handoff_view_from_state_payload(
    state: &ProjectionAgentHandoffStatePayload,
) -> ConversationAgentHandoffView {
    ConversationAgentHandoffView {
        status: state.status.clone(),
        source: projection_actor_to_view(&state.source),
        target: projection_actor_to_view(&state.target),
        handoff_session_id: state.handoff_session_id.clone(),
        handoff_reason: state.handoff_reason.clone(),
        accepted_at: state.accepted_at.clone(),
        accepted_by: state.accepted_by.as_ref().map(projection_actor_to_view),
        resolved_at: state.resolved_at.clone(),
        resolved_by: state.resolved_by.as_ref().map(projection_actor_to_view),
        closed_at: state.closed_at.clone(),
        closed_by: state.closed_by.as_ref().map(projection_actor_to_view),
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

fn projection_actor_to_view(actor: &ProjectionActorView) -> ConversationActorView {
    ConversationActorView {
        id: actor.id.clone(),
        kind: actor.kind.clone(),
    }
}
