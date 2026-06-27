use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Friendship {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "friendshipId")]
    pub friendship_id: String,

    #[serde(rename = "initiatorUserId")]
    pub initiator_user_id: String,

    #[serde(rename = "leftUserId")]
    pub left_user_id: String,

    #[serde(rename = "rightUserId")]
    pub right_user_id: String,

    #[serde(rename = "userHighId")]
    pub user_high_id: String,

    #[serde(rename = "userLowId")]
    pub user_low_id: String,

    pub status: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}
