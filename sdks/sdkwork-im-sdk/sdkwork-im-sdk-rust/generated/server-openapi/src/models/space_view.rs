use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceView {
    #[serde(rename = "spaceId")]
    pub space_id: String,

    #[serde(rename = "spaceName")]
    pub space_name: String,

    #[serde(rename = "spaceType")]
    pub space_type: String,

    #[serde(rename = "ownerUserId")]
    pub owner_user_id: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}
