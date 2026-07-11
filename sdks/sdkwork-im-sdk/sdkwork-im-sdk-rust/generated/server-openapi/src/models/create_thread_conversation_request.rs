use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateThreadConversationRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "parentConversationId")]
    pub parent_conversation_id: String,

    #[serde(rename = "rootMessageId")]
    pub root_message_id: String,
}
