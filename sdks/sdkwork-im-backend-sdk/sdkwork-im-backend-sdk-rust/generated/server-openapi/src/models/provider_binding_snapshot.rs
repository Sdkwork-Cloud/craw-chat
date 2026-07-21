use serde::{Deserialize, Serialize};

use crate::models::{ProviderBindingItem};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderBindingSnapshot {
    #[serde(rename = "interfaceVersion")]
    pub interface_version: String,

    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "effectiveBindings")]
    pub effective_bindings: Vec<ProviderBindingItem>,

    pub precedence: Vec<String>,
}
