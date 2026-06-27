use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DirectChat {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "directChatId")]
    pub direct_chat_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    pub status: String,
}
