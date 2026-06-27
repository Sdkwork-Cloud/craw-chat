use serde::{Deserialize, Serialize};

/// Effective provider bindings resolved for the current tenant scope.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingsResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
