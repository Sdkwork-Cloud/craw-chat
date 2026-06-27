//! Axum HTTP handlers for the gateway OpenAPI surface: aggregate document,
//! service schema index, runtime summary, and docs UI endpoints.

use axum::{
    Json,
    extract::{Path, Request, State},
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
    build_aggregate_openapi_document, fetch_service_openapi_document, fetch_service_openapi_documents,
    service_schema_index_entries,
};
use super::spec::{aggregate_gateway_openapi_spec, service_openapi_spec};
use crate::response::request_base_url;
use crate::state::GatewayState;

pub(crate) async fn openapi_json(State(state): State<GatewayState>) -> Result<Json<Value>, Response> {
    let documents = fetch_service_openapi_documents(&state).await?;
    Ok(Json(build_aggregate_openapi_document(&documents)))
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
) -> Result<Json<Value>, Response> {
    let service_id = service_schema
        .strip_suffix(".openapi.json")
        .unwrap_or(service_schema.as_str());
    Ok(Json(fetch_service_openapi_document(&state, service_id).await?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&aggregate_gateway_openapi_spec()))
}

pub(crate) async fn service_docs(Path(service_id): Path<String>) -> Html<String> {
    Html(render_docs_html(&service_openapi_spec(service_id.as_str())))
}
