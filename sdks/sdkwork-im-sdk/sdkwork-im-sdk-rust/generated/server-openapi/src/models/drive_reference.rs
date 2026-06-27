use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DriveReference {
    #[serde(rename = "driveUri")]
    pub drive_uri: String,

    #[serde(rename = "spaceId")]
    pub space_id: String,

    #[serde(rename = "nodeId")]
    pub node_id: String,

    #[serde(rename = "nodeVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_version: Option<String>,
}
