use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BindDirectChatRequest {
    #[serde(rename = "boundAt")]
    pub bound_at: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "directChatId")]
    pub direct_chat_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "leftActorId")]
    pub left_actor_id: String,

    #[serde(rename = "rightActorId")]
    pub right_actor_id: String,
}
