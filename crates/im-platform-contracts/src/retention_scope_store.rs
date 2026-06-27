use crate::ContractError;

/// Clears expiring retention markers for a conversation when policy switches to indefinite
/// retention (for example `legal_hold`).
pub trait RetentionScopeStore: Send + Sync {
    fn clear_conversation_retention_until(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), ContractError>;
}
