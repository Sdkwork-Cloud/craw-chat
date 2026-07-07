use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlockUserRequest {
    #[serde(rename = "blockedUserId")]
    pub blocked_user_id: String,

    pub scope: String,

    #[serde(rename = "directChatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<String>,

    #[serde(rename = "expiresAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
