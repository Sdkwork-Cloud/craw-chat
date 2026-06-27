use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RequestAutomationExecution {
    #[serde(rename = "executionId")]
    pub execution_id: String,

    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "targetKind")]
    pub target_kind: String,

    #[serde(rename = "targetRef")]
    pub target_ref: String,

    #[serde(rename = "inputPayload")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_payload: Option<String>,
}
