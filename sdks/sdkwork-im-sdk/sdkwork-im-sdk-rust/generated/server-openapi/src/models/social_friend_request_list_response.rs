use serde::{Deserialize, Serialize};

use crate::models::{FriendRequest};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendRequestListResponse {
    pub items: Vec<FriendRequest>,

    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}
