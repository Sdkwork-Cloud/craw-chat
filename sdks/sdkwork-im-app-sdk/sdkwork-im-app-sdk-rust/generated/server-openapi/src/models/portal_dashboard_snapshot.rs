use serde::{Deserialize, Serialize};

use crate::models::{PortalDataAvailability, PortalOperationalMetrics, PortalSnapshotMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalDashboardSnapshot {
    pub meta: PortalSnapshotMeta,

    pub availability: PortalDataAvailability,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PortalOperationalMetrics>,
}
