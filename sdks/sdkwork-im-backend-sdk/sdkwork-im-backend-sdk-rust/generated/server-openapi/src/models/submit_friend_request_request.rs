use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubmitFriendRequestRequest {
    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "requestId")]
    pub request_id: String,

    #[serde(rename = "requestMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,

    #[serde(rename = "requestedAt")]
    pub requested_at: String,

    #[serde(rename = "requesterUserId")]
    pub requester_user_id: String,

    #[serde(rename = "targetUserId")]
    pub target_user_id: String,
}
