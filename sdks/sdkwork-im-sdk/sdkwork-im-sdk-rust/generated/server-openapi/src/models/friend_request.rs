use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FriendRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "requestId")]
    pub request_id: String,

    #[serde(rename = "requesterUserId")]
    pub requester_user_id: String,

    #[serde(rename = "targetUserId")]
    pub target_user_id: String,

    pub status: String,

    #[serde(rename = "requestMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
