use serde::{Deserialize, Serialize};

use crate::models::{PortalDataAvailability, PortalGovernanceRiskSample, PortalSnapshotMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalGovernanceSnapshot {
    pub meta: PortalSnapshotMeta,

    pub availability: PortalDataAvailability,

    #[serde(rename = "sampledEventCount")]
    pub sampled_event_count: String,

    #[serde(rename = "riskSample")]
    pub risk_sample: PortalGovernanceRiskSample,
}
