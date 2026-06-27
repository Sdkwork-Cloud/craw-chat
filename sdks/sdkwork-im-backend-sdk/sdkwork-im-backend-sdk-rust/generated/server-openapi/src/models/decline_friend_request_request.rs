use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeclineFriendRequestRequest {
    #[serde(rename = "declinedAt")]
    pub declined_at: String,

    #[serde(rename = "declinedByUserId")]
    pub declined_by_user_id: String,

    #[serde(rename = "eventId")]
    pub event_id: String,
}
