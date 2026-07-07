//! Process-wide wiring for co-located session-gateway realtime plane.
//!
//! Conversation HTTP handlers resolve the embedded [`RealtimeEventPublisher`]
//! lazily so bootstrap order (application routes before session-gateway) does
//! not block typing fanout in unified-process deployments.

use std::sync::{Arc, OnceLock};

use im_platform_contracts::RealtimeEventPublisher;

use crate::runtime::{ConversationCommitJournal, ConversationRuntime, DirectMessageAccessGate};

static EMBEDDED_REALTIME_PUBLISHER: OnceLock<Arc<dyn RealtimeEventPublisher>> = OnceLock::new();
static EMBEDDED_CONVERSATION_RUNTIME: OnceLock<
    Arc<ConversationRuntime<ConversationCommitJournal>>,
> = OnceLock::new();
static EMBEDDED_DIRECT_MESSAGE_ACCESS_GATE: OnceLock<Arc<dyn DirectMessageAccessGate>> =
    OnceLock::new();

/// Register the embedded session-gateway ephemeral publisher for this process.
///
/// Safe to call once after the embedded realtime plane boots. Subsequent calls
/// are ignored when a publisher is already registered.
pub fn register_embedded_realtime_publisher(publisher: Arc<dyn RealtimeEventPublisher>) {
    let _ = EMBEDDED_REALTIME_PUBLISHER.set(publisher);
}

/// Register the conversation runtime built for embedded HTTP routes.
pub fn register_embedded_conversation_runtime(
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
) {
    let _ = EMBEDDED_CONVERSATION_RUNTIME.set(runtime);
}

/// Register the social direct-message access gate for embedded unified-process mode.
pub fn register_embedded_direct_message_access_gate(gate: Arc<dyn DirectMessageAccessGate>) {
    let _ = EMBEDDED_DIRECT_MESSAGE_ACCESS_GATE.set(gate);
}

/// Resolve the embedded realtime publisher when co-located with session-gateway.
pub fn resolve_embedded_realtime_publisher() -> Option<Arc<dyn RealtimeEventPublisher>> {
    EMBEDDED_REALTIME_PUBLISHER.get().cloned()
}

/// Resolve the embedded conversation runtime when co-located in unified-process mode.
pub fn resolve_embedded_conversation_runtime()
-> Option<Arc<ConversationRuntime<ConversationCommitJournal>>> {
    EMBEDDED_CONVERSATION_RUNTIME.get().cloned()
}

pub fn resolve_embedded_direct_message_access_gate() -> Option<Arc<dyn DirectMessageAccessGate>> {
    EMBEDDED_DIRECT_MESSAGE_ACCESS_GATE.get().cloned()
}
