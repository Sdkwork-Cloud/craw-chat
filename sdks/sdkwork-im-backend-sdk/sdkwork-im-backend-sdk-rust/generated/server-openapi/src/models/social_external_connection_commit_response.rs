use serde::{Deserialize, Serialize};

/// External connection write result plus persistence metadata.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialExternalConnectionCommitResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
