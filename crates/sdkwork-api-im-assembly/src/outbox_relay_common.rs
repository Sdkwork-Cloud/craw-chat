//! Shared helpers for domain outbox relay workers.

use std::sync::Arc;
use std::time::Duration;

use im_platform_contracts::{
    ContractError, OutboxEventClaim, OutboxEventRecord, OutboxScopeDiscoveryRequest, OutboxStore,
    PrivilegedOperationActorKind, PrivilegedOperationContext,
};
use tracing::warn;

pub const DEFAULT_OUTBOX_CLAIM_LEASE: Duration = Duration::from_secs(30);

pub fn discover_outbox_scopes(
    outbox: &dyn OutboxStore,
    worker_id: &str,
    aggregate_type: &str,
    limit: usize,
) -> Result<Vec<(String, String)>, ContractError> {
    let context = PrivilegedOperationContext::try_new(
        PrivilegedOperationActorKind::ServiceWorker,
        worker_id,
        sdkwork_utils_rust::id::uuid(),
    )?;
    let request = OutboxScopeDiscoveryRequest::try_new(&context, aggregate_type, limit)?;
    outbox.discover_pending_scopes(request)
}

pub fn mark_outbox_failed(outbox: &Arc<dyn OutboxStore>, claim: &OutboxEventClaim, reason: &str) {
    let _ = outbox.mark_failed(claim, reason);
}

pub fn log_unexpected_aggregate_type(
    event: &OutboxEventRecord,
    expected_aggregate_type: &str,
    relay_name: &str,
) {
    warn!(
        outbox_id = event.outbox_id.as_str(),
        aggregate_type = event.aggregate_type.as_str(),
        expected_aggregate_type = expected_aggregate_type,
        "{relay_name} outbox relay skipped event with unexpected aggregate type"
    );
}

pub fn mark_missing_recipients(
    outbox: &Arc<dyn OutboxStore>,
    claim: &OutboxEventClaim,
    relay_name: &str,
    recipient_field: &str,
) {
    let event = &claim.event;
    warn!(
        outbox_id = event.outbox_id.as_str(),
        event_type = event.event_type.as_str(),
        aggregate_id = event.aggregate_id.as_str(),
        recipient_field = recipient_field,
        "{relay_name} outbox relay skipped publish because recipients are missing or empty"
    );
    mark_outbox_failed(
        outbox,
        claim,
        &format!("{relay_name} outbox relay missing {recipient_field}"),
    );
}
