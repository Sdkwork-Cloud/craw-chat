use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContactTagView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "ownerUserId")]
    pub owner_user_id: String,

    #[serde(rename = "tagId")]
    pub tag_id: String,

    pub name: String,

    pub color: String,

    pub count: i64,

    pub bg: String,

    pub border: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
