use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateContactRecommendationRequest {
    #[serde(rename = "targetConversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_conversation_id: Option<String>,
}
