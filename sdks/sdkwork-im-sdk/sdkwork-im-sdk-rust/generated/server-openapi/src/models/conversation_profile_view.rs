use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationProfileView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,

    pub notice: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "updatedByPrincipalKind")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by_principal_kind: Option<String>,

    #[serde(rename = "updatedByPrincipalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by_principal_id: Option<String>,
}
