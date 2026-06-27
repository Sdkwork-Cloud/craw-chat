use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostRtcSignalRequest {
    #[serde(rename = "signalType")]
    pub signal_type: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    pub payload: String,

    #[serde(rename = "signalingStreamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signaling_stream_id: Option<String>,
}
