use serde::{Deserialize, Serialize};

/// Friendship snapshot plus commit history.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendshipSnapshotResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
