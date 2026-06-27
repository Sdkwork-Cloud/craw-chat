use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RtcSessionMutationResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "rtcSessionId")]
    pub rtc_session_id: String,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "initiatorId")]
    pub initiator_id: String,

    #[serde(rename = "initiatorKind")]
    pub initiator_kind: String,

    #[serde(rename = "providerPluginId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_plugin_id: Option<String>,

    #[serde(rename = "providerSessionId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_session_id: Option<String>,

    #[serde(rename = "accessEndpoint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,

    #[serde(rename = "providerRegion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_region: Option<String>,

    #[serde(rename = "rtcMode")]
    pub rtc_mode: String,

    pub state: String,

    #[serde(rename = "signalingStreamId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signaling_stream_id: Option<String>,

    #[serde(rename = "artifactMessageId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_message_id: Option<String>,

    #[serde(rename = "startedAt")]
    pub started_at: String,

    #[serde(rename = "endedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,

    #[serde(rename = "requestKey")]
    pub request_key: String,

    #[serde(rename = "deliveryStatus")]
    pub delivery_status: String,

    #[serde(rename = "proofVersion")]
    pub proof_version: String,
}
