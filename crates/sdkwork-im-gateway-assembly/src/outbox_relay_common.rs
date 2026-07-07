//! Shared helpers for domain outbox relay workers.

use std::sync::Arc;

use im_platform_contracts::{OutboxEventRecord, OutboxStore};
use tracing::warn;

pub fn mark_outbox_failed(outbox: &Arc<dyn OutboxStore>, event: &OutboxEventRecord, reason: &str) {
    let _ = outbox.mark_failed(
        event.tenant_id.as_str(),
        event.organization_id.as_str(),
        event.outbox_id.as_str(),
        reason,
    );
}

pub fn mark_unexpected_aggregate_type(
    outbox: &Arc<dyn OutboxStore>,
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
    mark_outbox_failed(
        outbox,
        event,
        &format!("{relay_name} outbox relay unexpected aggregate type"),
    );
}

pub fn mark_missing_recipients(
    outbox: &Arc<dyn OutboxStore>,
    event: &OutboxEventRecord,
    relay_name: &str,
    recipient_field: &str,
) {
    warn!(
        outbox_id = event.outbox_id.as_str(),
        event_type = event.event_type.as_str(),
        aggregate_id = event.aggregate_id.as_str(),
        recipient_field = recipient_field,
        "{relay_name} outbox relay skipped publish because recipients are missing or empty"
    );
    mark_outbox_failed(
        outbox,
        event,
        &format!("{relay_name} outbox relay missing {recipient_field}"),
    );
}
