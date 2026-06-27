use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InviteRtcSessionRequest {
    #[serde(rename = "signalingStreamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signaling_stream_id: Option<String>,
}
