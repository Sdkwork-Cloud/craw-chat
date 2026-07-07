use std::sync::Arc;

use im_app_context::is_production_like_im_environment;
use im_domain_events::CommitEnvelope;

use crate::TimelineProjectionService;
use crate::bootstrap::{shared_projection_runtime, try_init_embedded_projection_runtime};

/// Resolve the embedded projection service without panicking when Postgres is unavailable.
///
/// Unified-process journal append paths call this helper; HTTP handlers continue to use
/// [`crate::bootstrap::shared_projection_runtime`] which fail-closes in production.
pub fn resolve_embedded_projection_service() -> Option<Arc<TimelineProjectionService>> {
    let _ = try_init_embedded_projection_runtime()?;
    Some(shared_projection_runtime().service())
}

/// Apply a committed domain event to the embedded projection runtime.
///
/// Unified-process hosts call this immediately after journal append so
/// projection read models stay consistent without waiting for replay polling.
pub fn try_apply_commit_envelope(envelope: &CommitEnvelope) {
    if let Err(message) = apply_embedded_projection_event(envelope) {
        if is_production_like_im_environment() {
            tracing::error!(
                event_id = %envelope.event_id,
                event_type = %envelope.event_type,
                conversation_id = %envelope.aggregate_id,
                error = %message,
                "embedded projection apply failed in production"
            );
        } else {
            tracing::warn!(
                event_id = %envelope.event_id,
                event_type = %envelope.event_type,
                conversation_id = %envelope.aggregate_id,
                error = %message,
                "embedded projection apply failed"
            );
        }
    }
}

/// Structured result variant for callers that need explicit apply status.
pub fn apply_embedded_projection_event(envelope: &CommitEnvelope) -> Result<(), String> {
    let Some(service) = resolve_embedded_projection_service() else {
        if is_production_like_im_environment() {
            return Err(
                "embedded projection service unavailable in production-like environments".into(),
            );
        }
        return Ok(());
    };
    service
        .apply(envelope)
        .map_err(|error| format!("embedded projection apply failed: {error}"))
}

/// Acknowledge client-route sync feed progress for embedded/unified hosts.
///
/// Session-gateway realtime ack remains authoritative for push delivery; this
/// checkpoint drives projection-side delivery receipts on `MessageInteractionSummaryView`.
pub fn try_ack_client_route_sync_feed_for_principal(
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    acked_through_sync_seq: u64,
) -> Option<crate::ClientRouteSyncAckStateView> {
    let service = resolve_embedded_projection_service()?;
    Some(service.ack_client_route_sync_feed_for_principal_kind(
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
        acked_through_sync_seq,
    ))
}
