use serde::{Deserialize, Serialize};

use crate::models::{ConversationMember};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ListMembersResponse {
    pub items: Vec<ConversationMember>,

    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
