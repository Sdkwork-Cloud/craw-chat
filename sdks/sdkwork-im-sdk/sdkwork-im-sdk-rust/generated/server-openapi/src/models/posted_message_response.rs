use serde::{Deserialize, Serialize};

use crate::models::{MessageBody};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostedMessageResponse {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    pub body: MessageBody,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
