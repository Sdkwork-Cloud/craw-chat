use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalAuditRecordView {
    #[serde(rename = "recordId")]
    pub record_id: String,

    pub action: String,

    #[serde(rename = "actorId")]
    pub actor_id: String,

    #[serde(rename = "recordedAt")]
    pub recorded_at: String,

    pub severity: String,
}
