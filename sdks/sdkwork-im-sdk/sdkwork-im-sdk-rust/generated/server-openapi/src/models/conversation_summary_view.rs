use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationSummaryView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageCount")]
    pub message_count: i64,

    #[serde(rename = "lastMessageSeq")]
    pub last_message_seq: i64,

    #[serde(rename = "lastSummary")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_summary: Option<String>,

    #[serde(rename = "lastMessageAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<String>,
}
