use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialUserSearchResult {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "chatId")]
    pub chat_id: String,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "relationshipState")]
    pub relationship_state: String,

    #[serde(rename = "avatarUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}
