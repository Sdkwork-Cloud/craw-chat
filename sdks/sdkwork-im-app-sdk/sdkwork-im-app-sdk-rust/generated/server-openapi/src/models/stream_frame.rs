use serde::{Deserialize, Serialize};

use crate::models::{Sender};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StreamFrame {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "streamId")]
    pub stream_id: String,

    #[serde(rename = "streamType")]
    pub stream_type: String,

    #[serde(rename = "scopeKind")]
    pub scope_kind: String,

    #[serde(rename = "scopeId")]
    pub scope_id: String,

    #[serde(rename = "frameSeq")]
    pub frame_seq: i64,

    #[serde(rename = "frameType")]
    pub frame_type: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    pub encoding: String,

    pub payload: String,

    pub sender: Sender,

    pub attributes: std::collections::HashMap<String, String>,

    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
}
