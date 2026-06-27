use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};

use crate::error::CallingError;

pub(crate) async fn openapi_json() -> Result<Json<serde_json::Value>, CallingError> {
    Ok(Json(build_calling_service_openapi_document().map_err(
        |message| CallingError::internal("openapi_export_failed", message),
    )?))
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&calling_service_openapi_spec()))
}

fn build_calling_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("app.rs"),
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &calling_service_openapi_spec(),
        &routes,
        calling_service_tag,
        calling_service_requires_app_context,
        calling_service_summary,
    ))
}

fn calling_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Call Signaling Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the calls-service router for RTC call session lifecycle and signal relay flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn calling_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "calls".to_owned(),
    }
}

fn calling_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn calling_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check call signaling service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check call signaling service readiness".to_owned(),
        ("/im/v3/api/calls/sessions", HttpMethod::Post) => {
            "Create an IM call signaling session".to_owned()
        }
        ("/im/v3/api/calls/sessions/{rtc_session_id}", HttpMethod::Get) => {
            "Retrieve IM call signaling session state".to_owned()
        }
        ("/im/v3/api/calls/sessions/{rtc_session_id}/invite", HttpMethod::Post) => {
            "Invite participants into an IM call signaling session".to_owned()
        }
        ("/im/v3/api/calls/sessions/{rtc_session_id}/accept", HttpMethod::Post) => {
            "Accept an IM call signaling session".to_owned()
        }
        ("/im/v3/api/calls/sessions/{rtc_session_id}/reject", HttpMethod::Post) => {
            "Reject an IM call signaling session".to_owned()
        }
        ("/im/v3/api/calls/sessions/{rtc_session_id}/end", HttpMethod::Post) => {
            "End an IM call signaling session".to_owned()
        }
        ("/im/v3/api/calls/sessions/{rtc_session_id}/signals", HttpMethod::Post) => {
            "Post an IM call signaling event".to_owned()
        }
        (
            "/im/v3/api/calls/sessions/{rtc_session_id}/credentials",
            HttpMethod::Post,
        ) => "Issue an RTC media participant credential for an IM call".to_owned(),
        _ => format!("{method:?} {path}"),
    }
}