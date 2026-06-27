use serde::{Deserialize, Serialize};

/// Friend request write result plus persistence metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialFriendRequestCommitResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
