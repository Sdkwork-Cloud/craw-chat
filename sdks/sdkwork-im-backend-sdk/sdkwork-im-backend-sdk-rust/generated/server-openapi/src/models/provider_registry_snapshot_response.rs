use serde::{Deserialize, Serialize};

/// Provider registry snapshot for the current control-plane view.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderRegistrySnapshotResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
