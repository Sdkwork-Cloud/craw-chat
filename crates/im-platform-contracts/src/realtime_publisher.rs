//! Ephemeral realtime event publisher contract.
//!
//! Defines the boundary between durable journal-backed domain events and
//! ephemeral realtime pushes (typing indicators, presence pings, etc.).
//!
//! Implementations live in the session-gateway runtime (`RealtimeDeliveryRuntime`)
//! and are injected into conversation-service when co-located. Ephemeral events
//! are NOT persisted to the durable `RealtimeEventWindowStore` or
//! `RealtimeCheckpointStore`; only currently-connected WebSocket subscribers
//! receive them. Reconnecting clients do not replay ephemeral events.

use sdkwork_im_contract_core::ContractError;

/// A single recipient of an ephemeral realtime event.
///
/// Recipients are addressed by principal identity; the publisher resolves
/// registered device IDs internally and fans out to all connected devices
/// owned by each recipient.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeEventRecipient {
    pub principal_id: String,
    pub principal_kind: String,
}

impl RealtimeEventRecipient {
    pub fn new(principal_id: impl Into<String>, principal_kind: impl Into<String>) -> Self {
        Self {
            principal_id: principal_id.into(),
            principal_kind: principal_kind.into(),
        }
    }
}

/// Command payload for publishing a realtime scope event to selected
/// principals.
#[derive(Debug)]
pub struct RealtimeScopeEventPublishCommand<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub scope_type: &'a str,
    pub scope_id: &'a str,
    pub event_type: &'a str,
    pub payload: String,
    pub recipients: Vec<RealtimeEventRecipient>,
}

/// Publish boundary for ephemeral (non-durable) realtime events.
///
/// Ephemeral events:
/// - Bypass the durable `RealtimeEventWindowStore` (no 24h Redis window persist).
/// - Bypass the durable `RealtimeCheckpointStore` (no Postgres checkpoint row).
/// - Still update the in-memory event window and fire the per-device
///   `tokio::sync::watch` notifier so currently-subscribed WebSocket links
///   drain the event on their next read.
/// - Are NOT replayed on client reconnect — semantically correct for typing
///   indicators, presence pings, and similar transient signals.
///
/// The publisher is best-effort: if no devices are currently subscribed for
/// a recipient, that recipient is silently skipped. The return value is the
/// total number of devices that received the push.
pub trait RealtimeEventPublisher: Send + Sync {
    /// Publish an ephemeral scope event to all currently-connected subscribers
    /// among the given recipients.
    ///
    /// - `scope_type` / `scope_id`: the realtime subscription scope (e.g.
    ///   `"conversation"` / `conversation_id`). Recipients must have an active
    ///   subscription for this scope to receive the event.
    /// - `event_type`: free-form event type tag (e.g. `"conversation.typing"`).
    ///   Subscription matching checks this against `RealtimeSubscription.event_types`
    ///   (empty list means wildcard).
    /// - `payload`: JSON-encoded event payload.
    /// - `recipients`: list of principals who should receive the event.
    ///
    /// Returns the number of devices that received the push, or an error if
    /// the publisher is unavailable.
    fn publish_ephemeral_scope_event_to_recipients(
        &self,
        command: RealtimeScopeEventPublishCommand<'_>,
    ) -> Result<usize, ContractError>;

    /// Publish a durable scope event to conversation members for reconnect
    /// compensation and online push (TECH-16 message.posted/edited/recalled).
    ///
    /// Default implementation is a no-op for hosts without a realtime plane.
    fn publish_durable_scope_event_to_recipients(
        &self,
        command: RealtimeScopeEventPublishCommand<'_>,
    ) -> Result<usize, ContractError> {
        let _ = command;
        Ok(0)
    }

    /// Publish one durable inbox event on each recipient's own user scope.
    ///
    /// Conversation lists subscribe to `user/<principal_id>` instead of every
    /// conversation scope. The default implementation preserves compatibility
    /// for publishers by delegating one recipient at a time to the existing
    /// durable scope method.
    fn publish_durable_user_scope_event_to_recipients(
        &self,
        tenant_id: &str,
        organization_id: &str,
        event_type: &str,
        payload: String,
        recipients: Vec<RealtimeEventRecipient>,
    ) -> Result<usize, ContractError> {
        let mut delivered = 0usize;
        for recipient in recipients {
            let scope_id = recipient.principal_id.clone();
            delivered = delivered.saturating_add(self.publish_durable_scope_event_to_recipients(
                RealtimeScopeEventPublishCommand {
                    tenant_id,
                    organization_id,
                    scope_type: "user",
                    scope_id: scope_id.as_str(),
                    event_type,
                    payload: payload.clone(),
                    recipients: vec![recipient],
                },
            )?);
        }
        Ok(delivered)
    }
}
