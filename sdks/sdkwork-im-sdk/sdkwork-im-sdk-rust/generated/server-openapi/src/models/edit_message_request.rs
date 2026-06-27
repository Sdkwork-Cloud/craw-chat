use serde::{Deserialize, Serialize};

use crate::models::{ContentPart, MessageReplyReference};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditMessageRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<ContentPart>>,

    #[serde(rename = "replyTo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageReplyReference>,
}
