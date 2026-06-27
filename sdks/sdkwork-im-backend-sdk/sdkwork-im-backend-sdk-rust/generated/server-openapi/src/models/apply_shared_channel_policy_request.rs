use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApplySharedChannelPolicyRequest {
    #[serde(rename = "appliedAt")]
    pub applied_at: String,

    #[serde(rename = "channelId")]
    pub channel_id: String,

    #[serde(rename = "connectionId")]
    pub connection_id: String,

    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "historyVisibility")]
    pub history_visibility: String,

    #[serde(rename = "policyId")]
    pub policy_id: String,

    #[serde(rename = "policyVersion")]
    pub policy_version: i64,
}
