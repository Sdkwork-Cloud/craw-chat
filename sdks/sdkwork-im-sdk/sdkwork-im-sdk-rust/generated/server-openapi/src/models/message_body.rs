use serde::{Deserialize, Serialize};

use crate::models::{ContentPart, MessageReplyReference};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    pub parts: Vec<ContentPart>,

    #[serde(rename = "replyTo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageReplyReference>,

    #[serde(rename = "renderHints")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render_hints: Option<std::collections::HashMap<String, serde_json::Value>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}
