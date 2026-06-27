use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AcceptFriendRequestRequest {
    #[serde(rename = "acceptedAt")]
    pub accepted_at: String,

    #[serde(rename = "acceptedByUserId")]
    pub accepted_by_user_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,
}
