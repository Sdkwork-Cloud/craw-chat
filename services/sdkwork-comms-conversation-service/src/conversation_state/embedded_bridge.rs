use std::sync::Arc;

use im_domain_events::CommitEnvelope;

use crate::conversation_state::ConversationStateService;
use crate::conversation_state::bootstrap::shared_conversation_state_runtime;

/// Returns the process-local disposable Conversation cache.
pub fn shared_conversation_state_service() -> Arc<ConversationStateService> {
    shared_conversation_state_runtime().service()
}

/// Refreshes a disposable local cache after a canonical transaction commits.
///
/// Failure is observable but never changes the outcome of the committed write. Ordinary
/// production reads use normalized repositories; this cache is an optimization only.
pub fn refresh_conversation_cache(envelope: &CommitEnvelope) {
    if let Err(error) = apply_committed_event_to_cache(envelope) {
        tracing::warn!(
            event_id = %envelope.event_id,
            event_type = %envelope.event_type,
            conversation_id = %envelope.aggregate_id,
            error = %error,
            "conversation cache refresh failed"
        );
    }
}

pub fn apply_committed_event_to_cache(envelope: &CommitEnvelope) -> Result<(), String> {
    shared_conversation_state_service()
        .apply(envelope)
        .map_err(|error| format!("conversation cache refresh failed: {error}"))
}

pub fn try_ack_client_route_sync_feed_for_principal(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    acked_through_sync_seq: u64,
) -> Option<crate::conversation_state::ClientRouteSyncAckStateView> {
    Some(
        shared_conversation_state_service().ack_client_route_sync_feed_for_principal_kind(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            acked_through_sync_seq,
        ),
    )
}
