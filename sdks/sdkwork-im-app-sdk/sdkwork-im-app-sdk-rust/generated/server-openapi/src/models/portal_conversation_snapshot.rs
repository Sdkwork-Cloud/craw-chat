use serde::{Deserialize, Serialize};

use crate::models::{PortalConversationOperationalMetrics, PortalDataAvailability, PortalSnapshotMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalConversationSnapshot {
    pub meta: PortalSnapshotMeta,

    pub availability: PortalDataAvailability,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PortalConversationOperationalMetrics>,
}
