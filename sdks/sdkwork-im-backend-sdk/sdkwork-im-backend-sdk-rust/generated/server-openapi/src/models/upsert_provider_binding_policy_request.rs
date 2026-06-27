use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpsertProviderBindingPolicyRequest {
    pub domain: String,

    #[serde(rename = "expectedBaseVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_base_version: Option<i64>,

    #[serde(rename = "pluginId")]
    pub plugin_id: String,

    #[serde(rename = "tenantId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
