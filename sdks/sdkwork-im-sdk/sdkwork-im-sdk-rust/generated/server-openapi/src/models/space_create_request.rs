use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceCreateRequest {
    #[serde(rename = "spaceName")]
    pub space_name: String,

    #[serde(rename = "spaceType")]
    pub space_type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
