use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RemoveFriendshipRequest {
    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "removedAt")]
    pub removed_at: String,

    #[serde(rename = "removedByUserId")]
    pub removed_by_user_id: String,
}
