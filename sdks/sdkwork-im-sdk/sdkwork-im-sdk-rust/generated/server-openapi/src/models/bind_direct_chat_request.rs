use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BindDirectChatRequest {
    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "directChatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_chat_id: Option<String>,

    #[serde(rename = "leftActorId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_actor_id: Option<String>,

    #[serde(rename = "leftActorKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_actor_kind: Option<String>,

    #[serde(rename = "rightActorId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_actor_id: Option<String>,

    #[serde(rename = "rightActorKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_actor_kind: Option<String>,

    #[serde(rename = "targetUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
}
