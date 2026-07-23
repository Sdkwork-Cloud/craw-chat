use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContactView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "ownerUserId")]
    pub owner_user_id: String,

    #[serde(rename = "targetUserId")]
    pub target_user_id: String,

    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "avatarUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "chatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,

    #[serde(rename = "contactType")]
    pub contact_type: String,

    #[serde(rename = "relationshipState")]
    pub relationship_state: String,

    #[serde(rename = "friendshipId")]
    pub friendship_id: String,

    #[serde(rename = "directChatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<String>,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "establishedAt")]
    pub established_at: String,

    #[serde(rename = "lastInteractionAt")]
    pub last_interaction_at: String,

    #[serde(rename = "isStarred")]
    pub is_starred: bool,

    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
