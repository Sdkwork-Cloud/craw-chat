use serde::{Deserialize, Serialize};

use crate::models::{PortalDataAvailability, PortalRealtimeMetrics, PortalSnapshotMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalRealtimeSnapshot {
    pub meta: PortalSnapshotMeta,

    pub availability: PortalDataAvailability,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PortalRealtimeMetrics>,
}
