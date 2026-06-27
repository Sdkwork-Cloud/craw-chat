use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationInboxPeerView {
    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "userId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(rename = "chatId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,

    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "avatarUrl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "relationshipState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_state: Option<String>,
}
