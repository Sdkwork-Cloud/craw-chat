use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendRequestAcceptedConversation {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    pub kind: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}
