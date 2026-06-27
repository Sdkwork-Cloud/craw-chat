use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CompleteAgentToolCallRequest {
    #[serde(rename = "resultPayload")]
    pub result_payload: String,
}
