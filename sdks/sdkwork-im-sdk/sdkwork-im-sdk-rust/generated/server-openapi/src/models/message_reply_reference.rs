use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageReplyReference {
    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "senderDisplayName")]
    pub sender_display_name: String,

    #[serde(rename = "contentPreview")]
    pub content_preview: String,
}
