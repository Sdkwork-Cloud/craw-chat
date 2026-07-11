use std::sync::Arc;

use im_app_context::is_production_like_im_environment;
use im_domain_events::CommitEnvelope;

use crate::TimelineProjectionService;
use crate::bootstrap::{
    shared_projection_runtime, try_init_embedded_projection_runtime, try_shared_projection_runtime,
};

/// Resolve the embedded projection service for the current process.
///
/// **Production / cloud-separated deployments:** only an already-initialized
/// shared runtime is used. Separated services (e.g. `conversation-service`
/// running without projection HTTP handlers) never initialize one, so embedded
/// journal apply becomes a silent no-op and the journal consumer on
/// `projection-service` replicas drives read-model consistency instead. This
/// avoids building a phantom local runtime that no HTTP read path ever queries
/// and avoids misleading ERROR logs when the embedded service is intentionally
/// absent.
///
/// **Dev / test / unified standalone hosts:** lazily initializes the runtime so
/// journal append paths get immediate local projection feedback without
/// waiting for replay polling.
pub fn resolve_embedded_projection_service() -> Option<Arc<TimelineProjectionService>> {
    if is_production_like_im_environment() {
        return try_shared_projection_runtime().map(|runtime| runtime.service());
    }
    let _ = try_init_embedded_projection_runtime()?;
    Some(shared_projection_runtime().service())
}

/// Apply a committed domain event to the embedded projection runtime.
///
/// Unified-process hosts call this immediately after journal append so
/// projection read models stay consistent without waiting for replay polling.
/// In separated cloud deployments this is a silent no-op (see
/// [`resolve_embedded_projection_service`]); the journal consumer on
/// projection-service replicas remains authoritative.
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
///
/// Returns `Ok(())` when no embedded service is available — this is expected
/// in separated cloud deployments, not an error. Returns `Err` only when a
/// service was resolved but applying the event failed (a genuine projection
/// logic failure).
pub fn apply_embedded_projection_event(envelope: &CommitEnvelope) -> Result<(), String> {
    let Some(service) = resolve_embedded_projection_service() else {
        // No embedded projection runtime in this process. In separated cloud
        // deployments this is the expected steady state — the journal consumer
        // on projection-service replicas drives read-model consistency.
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
