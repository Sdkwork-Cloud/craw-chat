use serde::{Deserialize, Serialize};

/// User block snapshot plus commit history.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialUserBlockSnapshotResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
