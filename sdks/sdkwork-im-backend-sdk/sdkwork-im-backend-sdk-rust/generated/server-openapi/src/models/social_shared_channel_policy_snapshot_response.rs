use serde::{Deserialize, Serialize};

/// Shared-channel policy snapshot plus commit history.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelPolicySnapshotResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
