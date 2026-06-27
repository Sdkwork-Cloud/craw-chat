use serde::{Deserialize, Serialize};

/// Provider binding mutation result after applying a control-plane policy change.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingCommitResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
