use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BlockUserRequest {
    #[serde(rename = "blockId")]
    pub block_id: String,

    #[serde(rename = "blockedUserId")]
    pub blocked_user_id: String,

    #[serde(rename = "blockerUserId")]
    pub blocker_user_id: String,

    #[serde(rename = "directChatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<String>,

    #[serde(rename = "effectiveAt")]
    pub effective_at: String,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "expiresAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    pub scope: String,
}
