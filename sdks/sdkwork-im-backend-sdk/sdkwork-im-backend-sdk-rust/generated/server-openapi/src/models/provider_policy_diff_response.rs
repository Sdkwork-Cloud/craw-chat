use serde::{Deserialize, Serialize};

/// Provider policy diff between two committed versions.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderPolicyDiffResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
