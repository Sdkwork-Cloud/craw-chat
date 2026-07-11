use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConversationRequest {
    #[serde(rename = "conversationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,

    #[serde(rename = "conversationType")]
    pub conversation_type: String,

    #[serde(rename = "groupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    #[serde(rename = "clientRequestKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_request_key: Option<String>,

    #[serde(rename = "policyVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,

    #[serde(rename = "capabilityFlags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_flags: Option<Vec<String>>,

    #[serde(rename = "historyVisibility")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_visibility: Option<String>,

    #[serde(rename = "retentionPolicyRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_policy_ref: Option<String>,
}
