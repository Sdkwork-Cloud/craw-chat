use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalGovernanceRiskSample {
    #[serde(rename = "criticalCount")]
    pub critical_count: String,

    #[serde(rename = "highCount")]
    pub high_count: String,

    #[serde(rename = "warningCount")]
    pub warning_count: String,

    #[serde(rename = "informationalCount")]
    pub informational_count: String,
}
