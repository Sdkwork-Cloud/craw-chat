use serde::{Deserialize, Serialize};

/// Direct chat snapshot plus commit history.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialDirectChatSnapshotResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
