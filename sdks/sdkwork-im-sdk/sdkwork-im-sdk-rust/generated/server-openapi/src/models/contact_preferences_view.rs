use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContactPreferencesView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "ownerUserId")]
    pub owner_user_id: String,

    #[serde(rename = "targetUserId")]
    pub target_user_id: String,

    #[serde(rename = "isStarred")]
    pub is_starred: bool,

    pub remark: String,

    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
