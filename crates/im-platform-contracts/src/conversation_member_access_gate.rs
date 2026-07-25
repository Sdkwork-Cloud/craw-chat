//! Conversation member access gate for cross-service authorization (realtime, RTC).

use std::sync::Arc;

use sdkwork_im_contract_core::ContractError;

use crate::ConversationAggregateStore;

const ACTIVE_MEMBER_STATES: [&str; 2] = ["joined", "linked"];

fn membership_state_is_active(state: &str) -> bool {
    ACTIVE_MEMBER_STATES
        .iter()
        .any(|active| active.eq_ignore_ascii_case(state))
}

/// Validates that a principal is an active conversation member before scoped writes/subscriptions.
pub trait ConversationMemberAccessGate: Send + Sync {
    fn ensure_active_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<(), ContractError>;
}

/// PostgreSQL normalized-member access gate.
#[derive(Clone)]
pub struct AggregateStoreConversationMemberAccessGate {
    store: Arc<dyn ConversationAggregateStore>,
}

impl AggregateStoreConversationMemberAccessGate {
    pub fn new(store: Arc<dyn ConversationAggregateStore>) -> Self {
        Self { store }
    }
}

impl ConversationMemberAccessGate for AggregateStoreConversationMemberAccessGate {
    fn ensure_active_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<(), ContractError> {
        let member = self.store.load_member(
            tenant_id,
            organization_id,
            conversation_id,
            principal_kind,
            principal_id,
        )?;
        if member.is_some_and(|record| membership_state_is_active(record.membership_state.as_str()))
        {
            return Ok(());
        }
        Err(ContractError::Invalid(format!(
            "conversation_permission_denied: principal {principal_id} is not an active conversation member"
        )))
    }
}

/// Fail-closed gate for deployments without a wired aggregate store (development only).
#[derive(Clone, Default)]
pub struct DenyConversationMemberAccessGate;

impl ConversationMemberAccessGate for DenyConversationMemberAccessGate {
    fn ensure_active_member(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        conversation_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
    ) -> Result<(), ContractError> {
        Err(ContractError::UnsupportedCapability(format!(
            "conversation member access gate is not configured for conversation {conversation_id}"
        )))
    }
}
