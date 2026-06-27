use serde::{Deserialize, Serialize};

use crate::models::{MessagePinView, MessageReactionCountView};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageInteractionSummaryView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    #[serde(rename = "totalReactionCount")]
    pub total_reaction_count: i64,

    #[serde(rename = "reactionCounts")]
    pub reaction_counts: Vec<MessageReactionCountView>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<MessagePinView>,
}
