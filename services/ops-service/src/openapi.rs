use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

use crate::error::OpsError;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, OpsError> {
    Ok(Json(build_ops_service_openapi_document().map_err(
        |message| OpsError::internal("openapi_export_failed", message),
    )?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&ops_service_openapi_spec()))
}

fn build_ops_service_openapi_document() -> Result<serde_json::Value, String> {
    let source = include_str!("app.rs");
    let mut routes = extract_routes_from_function(
        source,
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;
    routes.extend(extract_routes_from_function(
        source,
        "build_domain_api_router",
        &[],
        &[],
    )?);

    Ok(build_openapi_document(
        &ops_service_openapi_spec(),
        &routes,
        ops_service_tag,
        ops_service_requires_app_context,
        ops_service_summary,
    ))
}

fn ops_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Ops Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the ops-service router for cluster, lag, diagnostics, runtime_dir, replay status, and provider binding inspections.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn ops_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" | "/metrics" => "system".to_owned(),
        path if path.contains("provider_bindings") => "provider_bindings".to_owned(),
        path if path.contains("diagnostics") => "diagnostics".to_owned(),
        path if path.contains("retention") => "retention".to_owned(),
        _ => "ops".to_owned(),
    }
}

fn ops_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz" | "/metrics")
}

fn ops_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check ops service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check ops service readiness".to_owned(),
        _ => format!(
            "{} {}",
            ops_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn ops_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}
