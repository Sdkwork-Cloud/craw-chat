use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

use crate::error::PortalError;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, PortalError> {
    Ok(Json(build_portal_service_openapi_document().map_err(
        |message| PortalError::internal("openapi_export_failed", message),
    )?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&portal_service_openapi_spec()))
}

fn build_portal_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes =
        extract_routes_from_function(include_str!("app.rs"), "build_domain_api_router", &[], &[])?;

    Ok(build_openapi_document(
        &portal_service_openapi_spec(),
        &routes,
        portal_service_tag,
        portal_service_requires_app_context,
        portal_service_summary,
    ))
}

fn portal_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Portal Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from portal-service router for console and workspace portal snapshot aggregation.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn portal_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "portal".to_owned(),
    }
}

fn portal_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn portal_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check portal service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check portal service readiness".to_owned(),
        _ => format!(
            "{} {}",
            portal_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn portal_service_method_display(method: HttpMethod) -> &'static str {
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
