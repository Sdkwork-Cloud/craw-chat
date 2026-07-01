use im_domain_events::CommitEnvelope;

use crate::http::default_projection_service;

/// Apply a committed domain event to the embedded projection runtime.
///
/// Unified-process hosts call this immediately after journal append so
/// projection read models stay consistent without waiting for replay polling.
pub fn try_apply_commit_envelope(envelope: &CommitEnvelope) {
    match default_projection_service().apply(envelope) {
        Ok(()) => {}
        Err(error) => {
            tracing::warn!(
                event_id = %envelope.event_id,
                event_type = %envelope.event_type,
                conversation_id = %envelope.aggregate_id,
                error = %error,
                "embedded projection apply failed"
            );
        }
    }
}
