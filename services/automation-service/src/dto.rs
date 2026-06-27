//! Request and response DTOs for the automation service HTTP surface.

use std::collections::BTreeMap;

use im_domain_core::automation::AutomationExecution;
use sdkwork_im_contract_agent::AgentSubject;
use serde::{Deserialize, Serialize};

use crate::constants::AUTOMATION_EXECUTION_DELIVERY_PROOF_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAutomationExecution {
    pub execution_id: String,
    pub trigger_type: String,
    pub target_kind: String,
    pub target_ref: String,
    pub input_payload: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartAgentResponseRequest {
    pub execution_id: String,
    pub stream_id: String,
    pub stream_type: String,
    pub conversation_id: String,
    pub schema_ref: Option<String>,
    pub member_id: Option<String>,
    pub agent: AgentSubject,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendAgentResponseDeltaRequest {
    pub frame_seq: u64,
    pub frame_type: String,
    pub schema_ref: Option<String>,
    pub encoding: String,
    pub payload: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteAgentResponseRequest {
    pub frame_seq: u64,
    pub result_message_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAgentToolCallRequest {
    pub execution_id: String,
    pub tool_call_id: String,
    pub tool_name: String,
    pub arguments_payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteAgentToolCallRequest {
    pub result_payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomationExecutionRequestResult {
    pub execution: AutomationExecution,
    pub is_new: bool,
    pub request_key: String,
    pub delivery_status: AutomationExecutionDeliveryStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationExecutionDeliveryStatus {
    Accepted,
    Applied,
    Replayed,
    Failed,
}

impl AutomationExecutionDeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Applied => "applied",
            Self::Replayed => "replayed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomationExecutionRequestResponse {
    #[serde(flatten)]
    pub execution: AutomationExecution,
    pub request_key: String,
    pub delivery_status: AutomationExecutionDeliveryStatus,
    pub proof_version: String,
}

impl From<AutomationExecutionRequestResult> for AutomationExecutionRequestResponse {
    fn from(value: AutomationExecutionRequestResult) -> Self {
        Self {
            execution: value.execution,
            request_key: value.request_key,
            delivery_status: value.delivery_status,
            proof_version: AUTOMATION_EXECUTION_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomationGovernanceSnapshot {
    pub capability_profile_id: String,
    pub enabled_capabilities: Vec<String>,
    pub guardrail_policy_id: String,
    pub restricted_tool_prefixes: Vec<String>,
    pub operator_override_permission: String,
    pub operator_override_active: bool,
}
