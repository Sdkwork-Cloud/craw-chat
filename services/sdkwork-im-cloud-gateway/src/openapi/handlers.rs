//! Axum HTTP handlers for the gateway OpenAPI surface: aggregate document,
//! service schema index, runtime summary, and docs UI endpoints.

use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Request, State},
    http::StatusCode,
    response::{Html, Response},
};
use sdkwork_im_api_registry::sdk_contract_summaries;
use sdkwork_im_cloud_gateway_observability::{
    GatewayStartupSummary, build_startup_summary_with_registry, route_summaries,
    surface_group_summaries,
};
use sdkwork_im_openapi::render_docs_html;
use serde_json::{Value, json};

use super::aggregate::{
    aggregate_openapi_document, fetch_service_openapi_document, service_schema_index_entries,
};
use super::sdk_contract_documents::{
    SdkContractDocument, SdkContractDocumentError, sdk_contract_document,
};
use super::spec::{aggregate_gateway_openapi_spec, service_openapi_spec};
use crate::response::{json_error_response, request_base_url};
use crate::state::GatewayState;

pub(crate) async fn im_sdk_openapi_json() -> Result<Json<Arc<Value>>, Response> {
    sdk_contract_document_json(SdkContractDocument::Im).map_err(sdk_contract_document_error)
}

pub(crate) async fn im_app_sdk_openapi_json() -> Result<Json<Arc<Value>>, Response> {
    sdk_contract_document_json(SdkContractDocument::App).map_err(sdk_contract_document_error)
}

pub(crate) async fn im_backend_sdk_openapi_json() -> Result<Json<Arc<Value>>, Response> {
    sdk_contract_document_json(SdkContractDocument::Backend).map_err(sdk_contract_document_error)
}

fn sdk_contract_document_json(
    contract: SdkContractDocument,
) -> Result<Json<Arc<Value>>, SdkContractDocumentError> {
    sdk_contract_document(contract).map(Json)
}

fn sdk_contract_document_error(error: SdkContractDocumentError) -> Response {
    tracing::error!(
        contract = error.contract_identifier(),
        error = %error,
        "failed to materialize embedded SDK contract document"
    );
    json_error_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        "embedded SDK contract document is unavailable",
    )
}

pub(crate) async fn openapi_json(
    State(state): State<GatewayState>,
    request: Request,
) -> Result<Json<Value>, Response> {
    let gateway_base_url = request_base_url(&request);
    Ok(Json(
        aggregate_openapi_document(&state, gateway_base_url.as_str()).await?,
    ))
}

pub(crate) async fn openapi_index_json(State(state): State<GatewayState>) -> Json<Value> {
    Json(json!({
        "sdkContracts": sdk_contract_summaries(""),
        "services": service_schema_index_entries(&state.config, &state.registry),
        "routes": route_summaries(&state.registry),
        "surfaceGroups": surface_group_summaries(&state.registry),
    }))
}

pub(crate) async fn openapi_runtime_summary_json(
    State(state): State<GatewayState>,
    request: Request,
) -> Json<GatewayStartupSummary> {
    Json(build_startup_summary_with_registry(
        &state.config,
        &state.registry,
        request_base_url(&request),
    ))
}

pub(crate) async fn service_openapi_json(
    Path(service_schema): Path<String>,
    State(state): State<GatewayState>,
    request: Request,
) -> Result<Json<Value>, Response> {
    let service_id = service_schema
        .strip_suffix(".openapi.json")
        .unwrap_or(service_schema.as_str());
    let gateway_base_url = request_base_url(&request);
    Ok(Json(
        fetch_service_openapi_document(&state, service_id, Some(gateway_base_url.as_str())).await?,
    ))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&aggregate_gateway_openapi_spec()))
}

pub(crate) async fn service_docs(Path(service_id): Path<String>) -> Html<String> {
    Html(render_docs_html(&service_openapi_spec(service_id.as_str())))
}
