use serde::{Deserialize, Serialize};

/// Derived social runtime repair report.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SocialRuntimeRepairResponse {
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
