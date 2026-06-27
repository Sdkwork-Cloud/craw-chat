use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamView {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "streamId")]
    pub stream_id: String,

    pub state: String,

    #[serde(rename = "openedAt")]
    pub opened_at: String,
}
