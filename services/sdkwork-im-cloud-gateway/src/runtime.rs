//! Runtime router delegation and embedded session-gateway dispatch helpers.

use axum::{Router, extract::Request, http::StatusCode, response::Response};
use sdkwork_im_realtime_api_paths::REALTIME_WS;
use tower::ServiceExt;

use crate::response::json_error_response;
use crate::state::GatewayState;

pub(crate) async fn delegate_to_runtime_router(
    runtime_router: Option<Router>,
    mut request: Request,
) -> Response {
    let Some(router) = runtime_router else {
        return json_error_response(StatusCode::NOT_FOUND, "gateway route owner not found");
    };

    request.extensions_mut().clear();
    match router.oneshot(request).await {
        Ok(response) => response,
        Err(error) => match error {},
    }
}

fn should_dispatch_embedded_session_gateway(path: &str) -> bool {
    if path == REALTIME_WS {
        return false;
    }
    path.starts_with("/im/v3/api/realtime") || path.starts_with("/im/v3/api/presence")
}

pub(crate) async fn dispatch_embedded_session_gateway_if_configured(
    state: &GatewayState,
    request: Request,
) -> Result<Response, Request> {
    if !should_dispatch_embedded_session_gateway(request.uri().path()) {
        return Err(request);
    }
    let Some(router) = state.embedded_session_gateway.as_ref() else {
        return Err(request);
    };
    match router.clone().oneshot(request).await {
        Ok(response) => Ok(response),
        Err(_) => Ok(json_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "embedded session-gateway dispatch failed",
        )),
    }
}

pub(crate) fn runtime_router_for_path(state: &GatewayState, path: &str) -> Option<Router> {
    if is_appbase_identity_namespace(path) {
        return None;
    }

    if should_delegate_to_product_runtime(path) || should_delegate_to_im_product_runtime(path) {
        return state.product_runtime_router.clone();
    }

    state.product_runtime_router.clone()
}

fn should_delegate_to_im_product_runtime(path: &str) -> bool {
    path == "/im/v3/openapi.json" || path.starts_with("/im/v3/api/")
}

fn should_delegate_to_product_runtime(path: &str) -> bool {
    path.starts_with("/app/v3/api/portal/")
}

fn is_appbase_identity_namespace(path: &str) -> bool {
    path == "/app/v3/api/open_platform/qr_auth"
        || path.starts_with("/app/v3/api/open_platform/qr_auth/")
}
