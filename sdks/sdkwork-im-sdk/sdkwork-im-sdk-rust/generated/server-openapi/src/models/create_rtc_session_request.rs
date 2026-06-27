use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRtcSessionRequest {
    #[serde(rename = "rtcSessionId")]
    pub rtc_session_id: String,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "rtcMode")]
    pub rtc_mode: String,
}
