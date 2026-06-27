use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageReactionMutationResult {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "reactionKey")]
    pub reaction_key: String,

    pub count: i64,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
