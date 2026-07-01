//! Internal helpers: key encoding, payload validation, governance, access checks, and config resolvers.

use std::collections::BTreeMap;

use im_app_context::AppContext;
use im_domain_core::automation::AutomationExecution;

use crate::constants::*;
use crate::dto::*;
use crate::error::AutomationError;

// ---------------------------------------------------------------------------
// Key encoding helpers
// ---------------------------------------------------------------------------

pub(crate) fn execution_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    encode_automation_key_segments([tenant_id, principal_kind, principal_id, execution_id])
}

pub(crate) fn automation_execution_request_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    execution_scope_key(tenant_id, principal_kind, principal_id, execution_id)
}

pub(crate) fn execution_event_identity(execution: &AutomationExecution) -> String {
    execution_scope_key(
        execution.tenant_id.as_str(),
        execution.principal_kind.as_str(),
        execution.principal_id.as_str(),
        execution.execution_id.as_str(),
    )
}

pub(crate) fn agent_response_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    stream_id: &str,
) -> String {
    encode_automation_key_segments([tenant_id, principal_kind, principal_id, stream_id])
}

pub(crate) fn agent_response_execution_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    encode_automation_key_segments([tenant_id, principal_kind, principal_id, execution_id])
}

pub(crate) fn agent_tool_call_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
    tool_call_id: &str,
) -> String {
    encode_automation_key_segments([
        tenant_id,
        principal_kind,
        principal_id,
        execution_id,
        tool_call_id,
    ])
}

pub(crate) fn agent_tool_call_execution_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    encode_automation_key_segments([tenant_id, principal_kind, principal_id, execution_id])
}

pub(crate) fn automation_event_key(execution: &AutomationExecution, segments: &[&str]) -> String {
    let mut encoded_segments = vec![
        execution.tenant_id.as_str(),
        execution.principal_kind.as_str(),
        execution.principal_id.as_str(),
        execution.execution_id.as_str(),
    ];
    encoded_segments.extend_from_slice(segments);
    encode_automation_key_segments(encoded_segments)
}

fn encode_automation_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

// ---------------------------------------------------------------------------
// Match helpers
// ---------------------------------------------------------------------------

pub(crate) fn delivery_status_from_execution(state: &str) -> AutomationExecutionDeliveryStatus {
    match state {
        "requested" | "running" => AutomationExecutionDeliveryStatus::Accepted,
        "succeeded" => AutomationExecutionDeliveryStatus::Replayed,
        "failed" => AutomationExecutionDeliveryStatus::Failed,
        _ => AutomationExecutionDeliveryStatus::Failed,
    }
}

pub(crate) fn execution_matches_request(
    existing: &AutomationExecution,
    request: &RequestAutomationExecution,
) -> bool {
    existing.trigger_type == request.trigger_type
        && existing.target_kind == request.target_kind
        && existing.target_ref == request.target_ref
        && existing.input_payload == request.input_payload
}

pub(crate) fn execution_matches_principal_kind(existing: &AutomationExecution, actor_kind: &str) -> bool {
    existing.principal_kind == actor_kind
}

// ---------------------------------------------------------------------------
// Payload validation
// ---------------------------------------------------------------------------

pub(crate) fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), AutomationError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(AutomationError::payload_too_large(
            field,
            max_bytes,
            payload_len,
        ));
    }
    Ok(())
}

fn validate_string_map_payload_size(
    field: &'static str,
    values: &BTreeMap<String, String>,
    max_bytes: usize,
) -> Result<(), AutomationError> {
    let payload_bytes = values
        .iter()
        .map(|(key, value)| key.len() + value.len())
        .sum::<usize>();
    if payload_bytes > max_bytes {
        return Err(AutomationError::payload_too_large(
            field,
            max_bytes,
            payload_bytes,
        ));
    }
    Ok(())
}

pub(crate) fn validate_execution_request_payload_size(
    request: &RequestAutomationExecution,
) -> Result<(), AutomationError> {
    validate_payload_size(
        "executionId",
        request.execution_id.as_str(),
        AUTOMATION_EXECUTION_MAX_EXECUTION_ID_BYTES,
    )?;
    validate_payload_size(
        "triggerType",
        request.trigger_type.as_str(),
        AUTOMATION_EXECUTION_MAX_TRIGGER_TYPE_BYTES,
    )?;
    validate_payload_size(
        "targetKind",
        request.target_kind.as_str(),
        AUTOMATION_EXECUTION_MAX_TARGET_KIND_BYTES,
    )?;
    validate_payload_size(
        "targetRef",
        request.target_ref.as_str(),
        AUTOMATION_EXECUTION_MAX_TARGET_REF_BYTES,
    )?;
    if let Some(payload) = request.input_payload.as_deref() {
        validate_payload_size(
            "inputPayload",
            payload,
            AUTOMATION_EXECUTION_MAX_INPUT_PAYLOAD_BYTES,
        )?;
    }
    Ok(())
}

pub(crate) fn validate_agent_response_delta_payload_size(
    request: &AppendAgentResponseDeltaRequest,
) -> Result<(), AutomationError> {
    validate_payload_size(
        "frameType",
        request.frame_type.as_str(),
        AUTOMATION_AGENT_RESPONSE_FRAME_MAX_TYPE_BYTES,
    )?;
    validate_payload_size(
        "encoding",
        request.encoding.as_str(),
        AUTOMATION_AGENT_RESPONSE_FRAME_MAX_ENCODING_BYTES,
    )?;
    validate_payload_size(
        "payload",
        request.payload.as_str(),
        AUTOMATION_AGENT_RESPONSE_FRAME_MAX_PAYLOAD_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size(
            "schemaRef",
            schema_ref,
            AUTOMATION_AGENT_RESPONSE_MAX_SCHEMA_REF_BYTES,
        )?;
    }
    validate_string_map_payload_size(
        "attributes",
        &request.attributes,
        AUTOMATION_AGENT_RESPONSE_FRAME_MAX_ATTRIBUTES_BYTES,
    )?;
    Ok(())
}

pub(crate) fn validate_start_agent_response_request_payload_size(
    request: &StartAgentResponseRequest,
) -> Result<(), AutomationError> {
    validate_payload_size(
        "executionId",
        request.execution_id.as_str(),
        AUTOMATION_EXECUTION_MAX_EXECUTION_ID_BYTES,
    )?;
    validate_payload_size(
        "streamId",
        request.stream_id.as_str(),
        AUTOMATION_AGENT_RESPONSE_MAX_STREAM_ID_BYTES,
    )?;
    validate_payload_size(
        "streamType",
        request.stream_type.as_str(),
        AUTOMATION_AGENT_RESPONSE_MAX_STREAM_TYPE_BYTES,
    )?;
    validate_payload_size(
        "conversationId",
        request.conversation_id.as_str(),
        AUTOMATION_AGENT_RESPONSE_MAX_CONVERSATION_ID_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size(
            "schemaRef",
            schema_ref,
            AUTOMATION_AGENT_RESPONSE_MAX_SCHEMA_REF_BYTES,
        )?;
    }
    if let Some(member_id) = request.member_id.as_deref() {
        validate_payload_size(
            "memberId",
            member_id,
            AUTOMATION_AGENT_RESPONSE_MAX_MEMBER_ID_BYTES,
        )?;
    }
    validate_payload_size(
        "agent.agent_id",
        request.agent.agent_id.as_str(),
        AUTOMATION_AGENT_RESPONSE_MAX_AGENT_ID_BYTES,
    )?;
    if let Some(session_id) = request.agent.session_id.as_deref() {
        validate_payload_size(
            "agent.session_id",
            session_id,
            AUTOMATION_AGENT_RESPONSE_MAX_AGENT_SESSION_ID_BYTES,
        )?;
    }
    validate_string_map_payload_size(
        "agent.metadata",
        &request.agent.metadata,
        AUTOMATION_AGENT_RESPONSE_MAX_AGENT_METADATA_BYTES,
    )?;
    Ok(())
}

pub(crate) fn validate_agent_tool_call_request_payload_size(
    request: &RequestAgentToolCallRequest,
) -> Result<(), AutomationError> {
    validate_payload_size(
        "executionId",
        request.execution_id.as_str(),
        AUTOMATION_EXECUTION_MAX_EXECUTION_ID_BYTES,
    )?;
    validate_payload_size(
        "toolCallId",
        request.tool_call_id.as_str(),
        AUTOMATION_AGENT_TOOL_CALL_MAX_ID_BYTES,
    )?;
    validate_payload_size(
        "toolName",
        request.tool_name.as_str(),
        AUTOMATION_AGENT_TOOL_CALL_MAX_NAME_BYTES,
    )?;
    validate_payload_size(
        "argumentsPayload",
        request.arguments_payload.as_str(),
        AUTOMATION_AGENT_TOOL_CALL_MAX_ARGUMENTS_PAYLOAD_BYTES,
    )
}

pub(crate) fn validate_complete_agent_response_request_payload_size(
    request: &CompleteAgentResponseRequest,
) -> Result<(), AutomationError> {
    if let Some(result_message_id) = request.result_message_id.as_deref() {
        validate_payload_size(
            "resultMessageId",
            result_message_id,
            AUTOMATION_AGENT_RESPONSE_MAX_RESULT_MESSAGE_ID_BYTES,
        )?;
    }
    Ok(())
}

pub(crate) fn validate_agent_tool_call_completion_payload_size(
    request: &CompleteAgentToolCallRequest,
) -> Result<(), AutomationError> {
    validate_payload_size(
        "resultPayload",
        request.result_payload.as_str(),
        AUTOMATION_AGENT_TOOL_CALL_MAX_RESULT_PAYLOAD_BYTES,
    )
}

// ---------------------------------------------------------------------------
// Governance helpers
// ---------------------------------------------------------------------------

pub(crate) fn automation_governance_snapshot(auth: &AppContext) -> AutomationGovernanceSnapshot {
    AutomationGovernanceSnapshot {
        capability_profile_id: AUTOMATION_CAPABILITY_PROFILE_ID.into(),
        enabled_capabilities: AUTOMATION_ENABLED_CAPABILITIES
            .into_iter()
            .map(str::to_owned)
            .collect(),
        guardrail_policy_id: AUTOMATION_GUARDRAIL_POLICY_ID.into(),
        restricted_tool_prefixes: AUTOMATION_RESTRICTED_TOOL_PREFIXES
            .into_iter()
            .map(str::to_owned)
            .collect(),
        operator_override_permission: AUTOMATION_OPERATOR_OVERRIDE_PERMISSION.into(),
        operator_override_active: automation_operator_override_active(auth),
    }
}

pub fn automation_operator_override_permission() -> &'static str {
    AUTOMATION_OPERATOR_OVERRIDE_PERMISSION
}

pub fn automation_tool_requires_operator_override(tool_name: &str) -> bool {
    AUTOMATION_RESTRICTED_TOOL_PREFIXES
        .iter()
        .any(|prefix| tool_name.starts_with(prefix))
}

pub(crate) fn automation_operator_override_active(auth: &AppContext) -> bool {
    auth.has_permission(AUTOMATION_OPERATOR_OVERRIDE_PERMISSION)
}

// ---------------------------------------------------------------------------
// Access helpers
// ---------------------------------------------------------------------------

pub(crate) fn ensure_automation_execute_access(auth: &AppContext) -> Result<(), AutomationError> {
    if auth.has_permission("automation.execute") {
        return Ok(());
    }

    Err(AutomationError::forbidden("automation.execute"))
}

pub(crate) fn ensure_automation_read_access(auth: &AppContext) -> Result<(), AutomationError> {
    if auth.has_permission("automation.read") {
        return Ok(());
    }

    Err(AutomationError::forbidden("automation.read"))
}

// ---------------------------------------------------------------------------
// Config resolvers
// ---------------------------------------------------------------------------

pub(crate) fn resolve_max_in_flight_requests() -> usize {
    std::env::var(AUTOMATION_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(AUTOMATION_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(AUTOMATION_MAX_IN_FLIGHT_REQUESTS_MAX)
}

pub(crate) fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(AUTOMATION_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(AUTOMATION_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(AUTOMATION_MAX_REQUEST_BODY_BYTES_MAX)
}
