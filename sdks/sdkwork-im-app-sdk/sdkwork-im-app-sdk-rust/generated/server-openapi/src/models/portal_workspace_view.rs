use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalWorkspaceView {
    pub name: String,

    pub slug: String,

    pub environment: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "supportPlan")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_plan: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seats: Option<String>,

    #[serde(rename = "activeBrands")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_brands: Option<String>,
}
