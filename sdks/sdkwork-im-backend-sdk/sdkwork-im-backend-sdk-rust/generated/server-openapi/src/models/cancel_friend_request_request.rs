use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CancelFriendRequestRequest {
    #[serde(rename = "canceledAt")]
    pub canceled_at: String,

    #[serde(rename = "canceledByUserId")]
    pub canceled_by_user_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,
}
