use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateAgentHandoffRequest {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "targetId")]
    pub target_id: String,

    #[serde(rename = "targetKind")]
    pub target_kind: String,

    #[serde(rename = "handoffSessionId")]
    pub handoff_session_id: String,

    #[serde(rename = "handoffReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff_reason: Option<String>,
}
