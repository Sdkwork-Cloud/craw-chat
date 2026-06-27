use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationPreferencesView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "isPinned")]
    pub is_pinned: bool,

    #[serde(rename = "isMuted")]
    pub is_muted: bool,

    #[serde(rename = "isMarkedUnread")]
    pub is_marked_unread: bool,

    #[serde(rename = "isHidden")]
    pub is_hidden: bool,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
