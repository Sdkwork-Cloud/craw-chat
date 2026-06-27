use serde::{Deserialize, Serialize};

use crate::models::{SocialUserSearchResult};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialUserSearchResponse {
    pub items: Vec<SocialUserSearchResult>,

    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
