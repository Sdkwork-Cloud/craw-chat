use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalSnapshotMeta {
    pub section: String,

    #[serde(rename = "generatedAt")]
    pub generated_at: String,

    #[serde(rename = "opsStatus")]
    pub ops_status: String,
}
