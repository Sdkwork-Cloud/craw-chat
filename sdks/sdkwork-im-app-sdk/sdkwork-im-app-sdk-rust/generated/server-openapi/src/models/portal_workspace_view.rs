use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalWorkspaceView {
    pub name: String,

    pub slug: String,

    pub tier: String,

    pub region: String,

    #[serde(rename = "supportPlan")]
    pub support_plan: String,

    pub seats: i64,

    #[serde(rename = "activeBrands")]
    pub active_brands: i64,

    pub uptime: String,
}
