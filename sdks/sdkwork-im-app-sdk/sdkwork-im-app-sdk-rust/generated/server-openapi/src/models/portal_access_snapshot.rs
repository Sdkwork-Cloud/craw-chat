use serde::{Deserialize, Serialize};

use crate::models::{PortalAuditRecordView, PortalDataAvailability, PortalSnapshotMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalAccessSnapshot {
    pub meta: PortalSnapshotMeta,

    pub availability: PortalDataAvailability,

    #[serde(rename = "tenantId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "principalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    #[serde(rename = "recentItems")]
    pub recent_items: Vec<PortalAuditRecordView>,

    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
