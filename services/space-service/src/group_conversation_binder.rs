//! Optional conversation-service integration for space group lifecycle.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateSpaceGroupConversationInput {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub creator_user_id: String,
    pub max_members: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncSpaceGroupMemberInput {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub user_id: String,
    pub role: String,
    pub actor_user_id: String,
    pub mute_until: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferSpaceGroupOwnerInput {
    pub tenant_id: String,
    pub organization_id: String,
    pub conversation_id: String,
    pub current_owner_user_id: String,
    pub new_owner_user_id: String,
    pub actor_user_id: String,
}

/// Creates group conversations and keeps roster membership aligned with space groups.
pub trait SpaceGroupConversationBinder: Send + Sync {
    fn create_group_conversation(
        &self,
        input: CreateSpaceGroupConversationInput,
    ) -> Result<(), String>;

    fn add_group_member(&self, input: SyncSpaceGroupMemberInput) -> Result<(), String>;

    fn remove_group_member(&self, input: SyncSpaceGroupMemberInput) -> Result<(), String>;

    fn transfer_group_owner(&self, input: TransferSpaceGroupOwnerInput) -> Result<(), String>;
}
