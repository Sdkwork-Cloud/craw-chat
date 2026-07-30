//! Wires co-located domain runtimes to the embedded session-gateway realtime plane.

use std::sync::Arc;

use conversation_runtime::{
    register_embedded_realtime_publisher, resolve_embedded_conversation_runtime,
};
use sdkwork_api_im_assembly::{
    ConversationOutboxRelayHandle, RtcOutboxRelayHandle, SocialOutboxRelayHandle,
    spawn_conversation_outbox_relay_from_env, spawn_rtc_outbox_relay_from_env,
    spawn_social_outbox_relay_from_env, wire_social_runtime_embedded_plane,
};
use session_gateway::AppState;
use social_service::SocialRuntime;

pub struct EmbeddedPlaneWiring {
    rtc_outbox_relay: Option<RtcOutboxRelayHandle>,
    conversation_outbox_relay: Option<ConversationOutboxRelayHandle>,
    social_outbox_relay: Option<SocialOutboxRelayHandle>,
}

impl EmbeddedPlaneWiring {
    pub fn shutdown(self) {
        if let Some(handle) = self.rtc_outbox_relay {
            handle.shutdown();
        }
        if let Some(handle) = self.conversation_outbox_relay {
            handle.shutdown();
        }
        if let Some(handle) = self.social_outbox_relay {
            handle.shutdown();
        }
    }
}

/// Register the embedded session-gateway as the ephemeral realtime publisher
/// for conversation typing indicators and wire social commit fanout.
pub fn wire_embedded_realtime_plane(
    session_state: &AppState,
    social_runtime: &Arc<SocialRuntime>,
) -> EmbeddedPlaneWiring {
    register_embedded_realtime_publisher(session_state.realtime_runtime());
    wire_social_runtime_embedded_plane(
        social_runtime,
        session_state.realtime_runtime(),
        resolve_embedded_conversation_runtime(),
    );
    let rtc_outbox_relay = spawn_rtc_outbox_relay_from_env(session_state.realtime_runtime());
    let conversation_outbox_relay =
        spawn_conversation_outbox_relay_from_env(session_state.realtime_runtime());
    let social_outbox_relay = spawn_social_outbox_relay_from_env(session_state.realtime_runtime());
    EmbeddedPlaneWiring {
        rtc_outbox_relay,
        conversation_outbox_relay,
        social_outbox_relay,
    }
}
