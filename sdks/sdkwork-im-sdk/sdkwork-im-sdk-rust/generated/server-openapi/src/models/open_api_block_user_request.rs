use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OpenApiBlockUserRequest {
    #[serde(rename = "blockedUserId")]
    pub blocked_user_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
