//! Wires social domain commits into the embedded session-gateway realtime plane.

use std::sync::Arc;

use conversation_runtime::{
    BindDirectChatConversationCommand, ConversationCommitJournal, ConversationRuntime,
    DirectMessageAccessGate,
};
use im_platform_contracts::CommitEnvelope;
use session_gateway::RealtimeDeliveryRuntime;
use social_service::{
    BindDirectChatConversationInput, DirectChatConversationBinder, SocialRealtimeFanout,
    social_realtime_recipients_for_commit,
};

pub struct SessionGatewaySocialRealtimeFanout {
    runtime: Arc<RealtimeDeliveryRuntime>,
}

impl SessionGatewaySocialRealtimeFanout {
    pub fn new(runtime: Arc<RealtimeDeliveryRuntime>) -> Self {
        Self { runtime }
    }
}

impl SocialRealtimeFanout for SessionGatewaySocialRealtimeFanout {
    fn publish_social_commit(&self, envelope: &CommitEnvelope) -> Result<(), String> {
        let (recipients, payload) = social_realtime_recipients_for_commit(envelope)?;
        self.runtime
            .publish_durable_user_scope_events_to_principals(
                envelope.tenant_id.as_str(),
                envelope.organization_id.as_str(),
                envelope.event_type.as_str(),
                payload,
                recipients,
            )
            .map_err(|error| format!("{error:?}"))
            .map(|_| ())
    }
}

pub struct ConversationServiceDirectChatBinder {
    runtime: Arc<ConversationRuntime<ConversationCommitJournal>>,
}

impl ConversationServiceDirectChatBinder {
    pub fn new(runtime: Arc<ConversationRuntime<ConversationCommitJournal>>) -> Self {
        Self { runtime }
    }
}

impl DirectChatConversationBinder for ConversationServiceDirectChatBinder {
    fn bind_direct_chat_conversation(
        &self,
        input: BindDirectChatConversationInput,
    ) -> Result<(), String> {
        self.runtime
            .bind_direct_chat_conversation_with_binder_kind(
                BindDirectChatConversationCommand {
                    tenant_id: input.tenant_id,
                    organization_id: input.organization_id,
                    conversation_id: input.conversation_id,
                    direct_chat_id: input.direct_chat_id,
                    left_actor_id: input.left_actor_id,
                    left_actor_kind: input.left_actor_kind,
                    right_actor_id: input.right_actor_id,
                    right_actor_kind: input.right_actor_kind,
                    bound_by: input.bound_by,
                },
                "system",
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(())
    }
}

pub struct SocialRuntimeDirectMessageAccessGate {
    social_runtime: Arc<social_service::SocialRuntime>,
}

impl SocialRuntimeDirectMessageAccessGate {
    pub fn new(social_runtime: Arc<social_service::SocialRuntime>) -> Self {
        Self { social_runtime }
    }
}

impl DirectMessageAccessGate for SocialRuntimeDirectMessageAccessGate {
    fn ensure_direct_message_allowed(
        &self,
        tenant_id: &str,
        organization_id: &str,
        sender_user_id: &str,
        peer_user_id: &str,
    ) -> Result<(), String> {
        self.social_runtime.ensure_direct_message_allowed(
            tenant_id,
            organization_id,
            sender_user_id,
            peer_user_id,
        )
    }
}

pub fn wire_social_runtime_embedded_plane(
    social_runtime: &Arc<social_service::SocialRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    conversation_runtime: Option<Arc<ConversationRuntime<ConversationCommitJournal>>>,
) {
    social_runtime.set_realtime_fanout(Arc::new(SessionGatewaySocialRealtimeFanout::new(
        realtime_runtime,
    )));
    if let Some(runtime) = conversation_runtime {
        social_runtime.set_direct_chat_conversation_binder(Arc::new(
            ConversationServiceDirectChatBinder::new(runtime),
        ));
        conversation_runtime::register_embedded_direct_message_access_gate(Arc::new(
            SocialRuntimeDirectMessageAccessGate::new(social_runtime.clone()),
        ));
    }
}
