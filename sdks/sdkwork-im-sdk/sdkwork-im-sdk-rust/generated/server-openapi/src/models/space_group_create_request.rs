use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SpaceGroupCreateRequest {
    #[serde(rename = "groupName")]
    pub group_name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
