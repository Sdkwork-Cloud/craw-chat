use serde::{Deserialize, Serialize};

use crate::models::{PortalDataAvailability, PortalSnapshotMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalModuleSnapshot {
    pub meta: PortalSnapshotMeta,

    pub availability: PortalDataAvailability,
}
