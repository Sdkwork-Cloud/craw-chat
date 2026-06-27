use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubmitFriendRequestRequest {
    #[serde(rename = "targetUserId")]
    pub target_user_id: String,

    #[serde(rename = "requestMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
}
