use serde::{Deserialize, Serialize};

/// Shared-channel policy write result plus persistence metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialSharedChannelPolicyCommitResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
