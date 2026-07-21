use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingDriftItem {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    pub domain: String,

    #[serde(rename = "baselineSelectedPluginId")]
    pub baseline_selected_plugin_id: String,

    #[serde(rename = "selectedPluginId")]
    pub selected_plugin_id: String,

    #[serde(rename = "baselineSelectionSource")]
    pub baseline_selection_source: String,

    #[serde(rename = "selectionSource")]
    pub selection_source: String,

    #[serde(rename = "driftKind")]
    pub drift_kind: String,
}
