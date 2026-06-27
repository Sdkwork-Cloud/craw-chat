use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AgentToolCall {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "executionId")]
    pub execution_id: String,

    #[serde(rename = "agentId")]
    pub agent_id: String,

    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    #[serde(rename = "toolName")]
    pub tool_name: String,

    #[serde(rename = "argumentsPayload")]
    pub arguments_payload: String,

    #[serde(rename = "resultPayload")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_payload: Option<String>,

    pub state: String,

    #[serde(rename = "requestedAt")]
    pub requested_at: String,

    #[serde(rename = "completedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}
