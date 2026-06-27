use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

use crate::error::NotificationError;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, NotificationError> {
    Ok(Json(
        build_notification_service_openapi_document()
            .map_err(|message| NotificationError::internal("openapi_export_failed", message))?,
    ))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&notification_service_openapi_spec()))
}

fn build_notification_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("app.rs"),
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &notification_service_openapi_spec(),
        &routes,
        notification_service_tag,
        notification_service_requires_app_context,
        notification_service_summary,
    ))
}

fn notification_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Notification Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the notification-service router for notification request mutation and notification query flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn notification_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "notifications".to_owned(),
    }
}

fn notification_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn notification_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check notification service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check notification service readiness".to_owned(),
        _ => format!(
            "{} {}",
            notification_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn notification_service_method_display(method: HttpMethod) -> &'static str {
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
