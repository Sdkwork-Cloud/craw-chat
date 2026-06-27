use serde::{Deserialize, Serialize};

/// User block write result plus persistence metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialUserBlockCommitResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
