use serde::{Deserialize, Serialize};

use crate::models::{AgentSubject};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StartAgentResponseRequest {
    #[serde(rename = "executionId")]
    pub execution_id: String,

    #[serde(rename = "streamId")]
    pub stream_id: String,

    #[serde(rename = "streamType")]
    pub stream_type: String,

    #[serde(rename = "conversationId")]
    pub conversation_id: String,

    #[serde(rename = "schemaRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    #[serde(rename = "memberId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,

    pub agent: AgentSubject,
}
