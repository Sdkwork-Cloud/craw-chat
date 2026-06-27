use serde::{Deserialize, Serialize};

/// Friendship write result plus persistence metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendshipCommitResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
