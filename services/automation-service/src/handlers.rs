//! HTTP handler functions for the automation service.

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::HeaderMap;
use axum::response::Html;
use im_domain_core::automation::{AgentToolCall, AutomationExecution};
use im_domain_core::stream::{StreamFrame, StreamSession};

use crate::dto::*;
use crate::error::AutomationError;
use crate::helpers::resolve_request_app_context;
use crate::openapi::{build_automation_service_openapi_document, render_automation_docs_html};
use crate::state::AppState;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, AutomationError> {
    Ok(Json(build_automation_service_openapi_document().map_err(
        |message| AutomationError::internal("openapi_export_failed", message),
    )?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_automation_docs_html())
}

pub(crate) async fn request_execution(
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestAutomationExecution>,
) -> Result<Json<AutomationExecutionRequestResponse>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let result = state
        .runtime
        .request_execution_with_outcome(&auth, request)?;
    Ok(Json(result.into()))
}

pub(crate) async fn get_execution(
    Path(execution_id): Path<String>,
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AutomationExecution>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state.runtime.get_execution(&auth, execution_id.as_str())?,
    ))
}

pub(crate) async fn get_governance(
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AutomationGovernanceSnapshot>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.governance_snapshot(&auth)?))
}

pub(crate) async fn start_agent_response(
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<StartAgentResponseRequest>,
) -> Result<Json<StreamSession>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.start_agent_response(&auth, request)?))
}

pub(crate) async fn append_agent_response_delta(
    Path(stream_id): Path<String>,
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendAgentResponseDeltaRequest>,
) -> Result<Json<StreamFrame>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.append_agent_response_delta(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

pub(crate) async fn complete_agent_response(
    Path(stream_id): Path<String>,
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteAgentResponseRequest>,
) -> Result<Json<StreamSession>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.complete_agent_response(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

pub(crate) async fn request_agent_tool_call(
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestAgentToolCallRequest>,
) -> Result<Json<AgentToolCall>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.request_agent_tool_call(&auth, request)?))
}

pub(crate) async fn complete_agent_tool_call(
    Path((execution_id, tool_call_id)): Path<(String, String)>,
    auth: Option<Extension<im_app_context::AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteAgentToolCallRequest>,
) -> Result<Json<AgentToolCall>, AutomationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.complete_agent_tool_call(
        &auth,
        execution_id.as_str(),
        tool_call_id.as_str(),
        request,
    )?))
}
