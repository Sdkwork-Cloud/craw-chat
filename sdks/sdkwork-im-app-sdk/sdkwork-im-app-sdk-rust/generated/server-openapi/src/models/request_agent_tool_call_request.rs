use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestAgentToolCallRequest {
    #[serde(rename = "executionId")]
    pub execution_id: String,

    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    #[serde(rename = "toolName")]
    pub tool_name: String,

    #[serde(rename = "argumentsPayload")]
    pub arguments_payload: String,
}
