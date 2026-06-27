use serde::{Deserialize, Serialize};

/// External connection snapshot plus commit history.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialExternalConnectionSnapshotResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
