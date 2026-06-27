use serde::{Deserialize, Serialize};

/// Provider policy history snapshot for the current tenant scope.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderPolicyHistoryResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
