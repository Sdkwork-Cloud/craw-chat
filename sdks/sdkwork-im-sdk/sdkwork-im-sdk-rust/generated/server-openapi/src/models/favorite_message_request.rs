use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FavoriteMessageRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "favoriteType")]
    pub favorite_type: String,

    pub title: String,

    #[serde(rename = "contentPreview")]
    pub content_preview: String,

    #[serde(rename = "sourceDisplayName")]
    pub source_display_name: String,
}
