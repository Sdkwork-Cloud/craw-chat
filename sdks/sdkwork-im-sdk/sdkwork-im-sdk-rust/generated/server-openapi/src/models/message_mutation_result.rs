use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageMutationResult {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(rename = "eventId")]
    pub event_id: String,
}
