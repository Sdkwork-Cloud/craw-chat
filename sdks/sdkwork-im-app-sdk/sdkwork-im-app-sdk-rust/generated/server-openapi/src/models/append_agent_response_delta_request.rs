use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppendAgentResponseDeltaRequest {
    #[serde(rename = "frameSeq")]
    pub frame_seq: i64,

    #[serde(rename = "frameType")]
    pub frame_type: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    pub encoding: String,

    pub payload: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}
