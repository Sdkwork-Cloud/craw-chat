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
    pub left_actor_id: String,

    #[serde(rename = "leftActorKind")]
    pub left_actor_kind: String,

    #[serde(rename = "rightActorId")]
    pub right_actor_id: String,

    #[serde(rename = "rightActorKind")]
    pub right_actor_kind: String,
}
