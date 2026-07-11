use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateSystemChannelRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
}
