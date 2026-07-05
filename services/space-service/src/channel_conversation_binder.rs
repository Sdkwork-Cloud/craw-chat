//! Optional conversation-service integration for space channel lifecycle.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateSpaceChannelConversationInput {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub creator_user_id: String,
}

/// Creates system-channel conversations when space channels are provisioned.
pub trait SpaceChannelConversationBinder: Send + Sync {
    fn create_channel_conversation(
        &self,
        input: CreateSpaceChannelConversationInput,
    ) -> Result<(), String>;
}
