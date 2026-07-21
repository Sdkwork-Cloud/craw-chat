use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingItem {
    pub domain: String,

    #[serde(rename = "defaultPluginId")]
    pub default_plugin_id: String,

    #[serde(rename = "selectedPluginId")]
    pub selected_plugin_id: String,

    #[serde(rename = "selectionSource")]
    pub selection_source: String,

    #[serde(rename = "tenantOverrideAllowed")]
    pub tenant_override_allowed: bool,
}
