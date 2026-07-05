use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserBlock {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "blockId")]
    pub block_id: String,

    #[serde(rename = "blockerUserId")]
    pub blocker_user_id: String,

    #[serde(rename = "blockedUserId")]
    pub blocked_user_id: String,

    pub scope: String,

    pub status: String,

    #[serde(rename = "directChatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<String>,

    #[serde(rename = "expiresAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
