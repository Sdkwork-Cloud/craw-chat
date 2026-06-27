use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RtcParticipantCredential {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "rtcSessionId")]
    pub rtc_session_id: String,

    #[serde(rename = "participantId")]
    pub participant_id: String,

    pub credential: String,

    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}
