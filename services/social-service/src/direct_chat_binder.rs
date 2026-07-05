//! Optional conversation-service integration for direct chat binding on friend accept.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindDirectChatConversationInput {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub direct_chat_id: String,
    pub left_actor_id: String,
    pub left_actor_kind: String,
    pub right_actor_id: String,
    pub right_actor_kind: String,
    pub bound_by: String,
}

/// Binds a direct chat conversation in conversation-service before social emits `direct_chat.bound`.
pub trait DirectChatConversationBinder: Send + Sync {
    fn bind_direct_chat_conversation(
        &self,
        input: BindDirectChatConversationInput,
    ) -> Result<(), String>;
}
