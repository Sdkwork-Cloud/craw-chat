//! HTTP handler functions for the automation service.

use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::response::{Html, Response};
use im_app_context::AppContext;
use im_domain_core::automation::{AgentToolCall, AutomationExecution};
use im_domain_core::stream::{StreamFrame, StreamSession};
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_web_core::WebRequestContext;

use crate::dto::*;
use crate::error::AutomationError;
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
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<RequestAutomationExecution>,
) -> Response {
    let result: ApiResult<AutomationExecutionRequestResponse> = (|| {
        let outcome = state.runtime.request_execution_with_outcome(&auth, request)?;
        Ok(outcome.into())
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_execution(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(execution_id): Path<String>,
) -> Response {
    let result: ApiResult<AutomationExecution> = (|| {
        Ok(state.runtime.get_execution(&auth, execution_id.as_str())?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_governance(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<AutomationGovernanceSnapshot> = (|| {
        Ok(state.runtime.governance_snapshot(&auth)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn start_agent_response(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<StartAgentResponseRequest>,
) -> Response {
    let result: ApiResult<StreamSession> = (|| {
        Ok(state.runtime.start_agent_response(&auth, request)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn append_agent_response_delta(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(stream_id): Path<String>,
    Json(request): Json<AppendAgentResponseDeltaRequest>,
) -> Response {
    let result: ApiResult<StreamFrame> = (|| {
        Ok(state.runtime.append_agent_response_delta(
            &auth,
            stream_id.as_str(),
            request,
        )?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn complete_agent_response(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path(stream_id): Path<String>,
    Json(request): Json<CompleteAgentResponseRequest>,
) -> Response {
    let result: ApiResult<StreamSession> = (|| {
        Ok(state.runtime.complete_agent_response(
            &auth,
            stream_id.as_str(),
            request,
        )?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn request_agent_tool_call(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<RequestAgentToolCallRequest>,
) -> Response {
    let result: ApiResult<AgentToolCall> = (|| {
        Ok(state.runtime.request_agent_tool_call(&auth, request)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn complete_agent_tool_call(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Path((execution_id, tool_call_id)): Path<(String, String)>,
    Json(request): Json<CompleteAgentToolCallRequest>,
) -> Response {
    let result: ApiResult<AgentToolCall> = (|| {
        Ok(state.runtime.complete_agent_tool_call(
            &auth,
            execution_id.as_str(),
            tool_call_id.as_str(),
            request,
        )?)
    })();
    finish_api_json(&ctx, result)
}
