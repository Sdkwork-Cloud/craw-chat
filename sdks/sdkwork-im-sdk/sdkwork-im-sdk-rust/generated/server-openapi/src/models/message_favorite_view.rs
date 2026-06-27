use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageFavoriteView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "favoriteId")]
    pub favorite_id: String,

    #[serde(rename = "favoriteType")]
    pub favorite_type: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "messageId")]
    pub message_id: String,

    #[serde(rename = "messageSeq")]
    pub message_seq: i64,

    pub title: String,

    #[serde(rename = "contentPreview")]
    pub content_preview: String,

    #[serde(rename = "sourceDisplayName")]
    pub source_display_name: String,

    #[serde(rename = "favoritedAt")]
    pub favorited_at: String,
}
