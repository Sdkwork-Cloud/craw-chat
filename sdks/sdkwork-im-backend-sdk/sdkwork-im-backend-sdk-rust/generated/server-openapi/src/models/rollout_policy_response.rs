use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RolloutPolicyResponse {
    #[serde(rename = "cellSelector")]
    pub cell_selector: String,

    #[serde(rename = "operatorOverride")]
    pub operator_override: bool,

    #[serde(rename = "policyId")]
    pub policy_id: String,

    #[serde(rename = "regionSelector")]
    pub region_selector: String,

    #[serde(rename = "releaseChannel")]
    pub release_channel: String,

    #[serde(rename = "tenantAllowlist")]
    pub tenant_allowlist: Vec<String>,

    #[serde(rename = "trafficPercent")]
    pub traffic_percent: i64,
}
