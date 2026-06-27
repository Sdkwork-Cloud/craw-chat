use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContactRecommendationView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "ownerUserId")]
    pub owner_user_id: String,

    #[serde(rename = "targetUserId")]
    pub target_user_id: String,

    #[serde(rename = "recommendationId")]
    pub recommendation_id: String,

    #[serde(rename = "targetConversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_conversation_id: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}
