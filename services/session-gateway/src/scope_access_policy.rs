//! Realtime scope access policies backed by conversation membership.

use std::sync::Arc;

use im_platform_contracts::ConversationMemberAccessGate;
use im_domain_core::realtime::RealtimeEvent;
use sdkwork_im_contract_core::ContractError;

use crate::realtime::{RealtimeRuntimeError, RealtimeScopeAccessPolicy};

const USER_SCOPE_TYPE: &str = "user";
const CONVERSATION_SCOPE_TYPE: &str = "conversation";

/// Production policy: user scopes are self-only; conversation scopes require active membership.
#[derive(Clone)]
pub struct ConversationMemberRealtimeScopeAccessPolicy {
    member_gate: Arc<dyn ConversationMemberAccessGate>,
}

impl ConversationMemberRealtimeScopeAccessPolicy {
    pub fn new(member_gate: Arc<dyn ConversationMemberAccessGate>) -> Self {
        Self { member_gate }
    }
}

impl RealtimeScopeAccessPolicy for ConversationMemberRealtimeScopeAccessPolicy {
    fn validate_subscription_scope(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        scope_type: &str,
        scope_id: &str,
    ) -> Result<(), RealtimeRuntimeError> {
        if scope_type == USER_SCOPE_TYPE {
            if scope_id == principal_id {
                return Ok(());
            }
            return Err(RealtimeRuntimeError {
                code: "realtime_scope_access_denied",
                message: format!("user scope {scope_id} is not owned by principal {principal_id}"),
            });
        }

        if scope_type == CONVERSATION_SCOPE_TYPE {
            return self
                .member_gate
                .ensure_active_member(
                    tenant_id,
                    organization_id,
                    scope_id,
                    principal_kind,
                    principal_id,
                )
                .map_err(map_member_gate_error);
        }

        Err(RealtimeRuntimeError {
            code: "realtime_scope_access_denied",
            message: format!("unsupported realtime scope type: {scope_type}"),
        })
    }

    fn is_event_visible(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        event: &RealtimeEvent,
    ) -> bool {
        if event.scope_type == USER_SCOPE_TYPE {
            return event.scope_id == principal_id;
        }
        if event.scope_type == CONVERSATION_SCOPE_TYPE {
            return self
                .member_gate
                .ensure_active_member(
                    tenant_id,
                    organization_id,
                    event.scope_id.as_str(),
                    principal_kind,
                    principal_id,
                )
                .is_ok();
        }
        false
    }
}

fn map_member_gate_error(error: ContractError) -> RealtimeRuntimeError {
    let message = format!("{error:?}");
    if message.contains("conversation_permission_denied") {
        RealtimeRuntimeError {
            code: "conversation_permission_denied",
            message,
        }
    } else {
        RealtimeRuntimeError {
            code: "realtime_scope_access_denied",
            message,
        }
    }
}
