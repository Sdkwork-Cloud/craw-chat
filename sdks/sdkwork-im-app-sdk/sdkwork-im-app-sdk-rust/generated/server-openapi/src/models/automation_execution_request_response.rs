use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AutomationExecutionRequestResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,

    #[serde(rename = "principalId")]
    pub principal_id: String,

    #[serde(rename = "principalKind")]
    pub principal_kind: String,

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

    #[serde(rename = "outputPayload")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_payload: Option<String>,

    pub state: String,

    #[serde(rename = "retryCount")]
    pub retry_count: i64,

    #[serde(rename = "requestedAt")]
    pub requested_at: String,

    #[serde(rename = "completedAt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    #[serde(rename = "failureReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    #[serde(rename = "requestKey")]
    pub request_key: String,

    #[serde(rename = "deliveryStatus")]
    pub delivery_status: String,

    #[serde(rename = "proofVersion")]
    pub proof_version: String,
}
