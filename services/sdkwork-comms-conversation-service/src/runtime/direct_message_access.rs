use im_domain_core::conversation::{ConversationMember, ConversationScenario};
use sdkwork_utils_rust::parse_bool;

use super::ConversationState;
use super::RuntimeError;

const SDKWORK_IM_REQUIRE_DM_ACCESS_GATE_ENV: &str = "SDKWORK_IM_REQUIRE_DM_ACCESS_GATE";

fn is_production_like_environment() -> bool {
    im_app_context::is_production_like_im_environment()
}

fn requires_dm_access_gate() -> bool {
    std::env::var(SDKWORK_IM_REQUIRE_DM_ACCESS_GATE_ENV)
        .ok()
        .and_then(|value| parse_bool(value.trim()))
        .unwrap_or(is_production_like_environment())
}

pub trait DirectMessageAccessGate: Send + Sync {
    fn ensure_direct_message_allowed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        sender_user_id: &str,
        peer_user_id: &str,
    ) -> Result<(), String>;
}

pub(super) fn ensure_direct_message_post_allowed(
    tenant_id: &str,
    organization_id: &str,
    conversation: &ConversationState,
    sender_member: &ConversationMember,
    gate: Option<&dyn DirectMessageAccessGate>,
) -> Result<(), RuntimeError> {
    if conversation.aggregate.scenario() != ConversationScenario::Direct {
        return Ok(());
    }
    let Some(gate) = gate else {
        if requires_dm_access_gate() {
            return Err(RuntimeError::PermissionDenied(
                "direct message access gate is required for direct conversations in production"
                    .into(),
            ));
        }
        return Ok(());
    };
    if sender_member.principal_kind != "user" {
        return Ok(());
    }
    let peer = conversation
        .roster
        .members()
        .values()
        .filter(|member: &&ConversationMember| member.is_active())
        .find(|member| {
            member.principal_kind == "user"
                && (member.principal_id != sender_member.principal_id
                    || member.principal_kind != sender_member.principal_kind)
        });
    let Some(peer) = peer else {
        return Ok(());
    };
    gate.ensure_direct_message_allowed(
        tenant_id,
        organization_id,
        sender_member.principal_id.as_str(),
        peer.principal_id.as_str(),
    )
    .map_err(RuntimeError::PermissionDenied)
}
