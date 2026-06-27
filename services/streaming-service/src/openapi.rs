use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

use crate::error::StreamingError;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, StreamingError> {
    Ok(Json(build_streaming_service_openapi_document().map_err(
        |message| StreamingError::internal("openapi_export_failed", message),
    )?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&streaming_service_openapi_spec()))
}

fn build_streaming_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("app.rs"),
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &streaming_service_openapi_spec(),
        &routes,
        streaming_service_tag,
        streaming_service_requires_app_context,
        streaming_service_summary,
    ))
}

fn streaming_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Streaming Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the streaming-service router for stream session lifecycle and frame append/query flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn streaming_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "streams".to_owned(),
    }
}

fn streaming_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn streaming_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check streaming service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check streaming service readiness".to_owned(),
        _ => format!(
            "{} {}",
            streaming_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn streaming_service_method_display(method: HttpMethod) -> &'static str {
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
