use axum::Json;
use axum::response::Html;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use sdkwork_im_realtime_api_paths::{
    PRESENCE_HEARTBEAT, PRESENCE_ME, REALTIME_EVENTS, REALTIME_EVENTS_ACK,
    REALTIME_SUBSCRIPTIONS_SYNC, REALTIME_WS,
};

use crate::api_error::ApiError;
use crate::websocket::CCP_WEBSOCKET_SUBPROTOCOL;

pub async fn openapi_json() -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(build_session_gateway_openapi_document().map_err(
        |message| ApiError::internal("openapi_export_failed", message),
    )?))
}

pub async fn docs() -> Html<String> {
    Html(render_docs_html(&session_gateway_openapi_spec()))
}

pub fn build_session_gateway_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_domain_api_router",
        &[sdkwork_im_openapi::WebsocketRouteMetadata {
            path: REALTIME_WS.to_owned(),
            subprotocols: vec![CCP_WEBSOCKET_SUBPROTOCOL.to_owned()],
        }],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &session_gateway_openapi_spec(),
        &routes,
        session_gateway_tag,
        session_gateway_requires_app_context,
        session_gateway_summary,
    ))
}

fn session_gateway_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Realtime Gateway API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the session-gateway router for presence, realtime polling, and websocket upgrade flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn session_gateway_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" => "system".to_owned(),
        path if path.starts_with("/im/v3/api/presence/") => "presence".to_owned(),
        path if path.starts_with("/im/v3/api/realtime/") => "realtime".to_owned(),
        _ => "misc".to_owned(),
    }
}

fn session_gateway_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    path != "/healthz"
}

fn session_gateway_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check session gateway health".to_owned(),
        (PRESENCE_HEARTBEAT, HttpMethod::Post) => {
            "Refresh device presence heartbeat".to_owned()
        }
        (PRESENCE_ME, HttpMethod::Get) => {
            "Get current device presence snapshot".to_owned()
        }
        (REALTIME_SUBSCRIPTIONS_SYNC, HttpMethod::Post) => {
            "Sync realtime subscriptions".to_owned()
        }
        (REALTIME_WS, HttpMethod::Get) => {
            "Open realtime websocket client route".to_owned()
        }
        (REALTIME_EVENTS_ACK, HttpMethod::Post) => {
            "Acknowledge realtime events".to_owned()
        }
        (REALTIME_EVENTS, HttpMethod::Get) => "Pull realtime event window".to_owned(),
        _ => format!("{:?} {}", method, path),
    }
}
