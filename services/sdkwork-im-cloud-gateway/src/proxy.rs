//! HTTP proxy handlers for gateway GET and write-method requests, including
//! websocket upgrade dispatch and runtime router fallback.

use axum::{
    extract::{Request, State, ws::WebSocketUpgrade},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use sdkwork_im_api_registry::{HttpMethod, RouteProtocol};
use sdkwork_im_cloud_gateway_config::{
    GatewayRuntimeMode, is_assembly_embedded_im_service,
};

use crate::client::resolve_max_http_request_body_bytes;
use crate::response::{
    build_proxy_response, is_sdkwork_context_projection_header, json_error_response, map_http_method,
};
use crate::runtime::{
    delegate_to_runtime_router, dispatch_embedded_session_gateway_if_configured,
    runtime_router_for_path,
};
use crate::state::GatewayState;
use crate::websocket::proxy_websocket_request;

pub(crate) async fn proxy_get_request(
    websocket_upgrade: Result<
        WebSocketUpgrade,
        axum::extract::ws::rejection::WebSocketUpgradeRejection,
    >,
    State(state): State<GatewayState>,
    request: Request,
) -> Response {
    let request = match dispatch_embedded_session_gateway_if_configured(&state, request).await {
        Ok(response) => return response,
        Err(request) => request,
    };

    let route = state
        .registry
        .resolve(HttpMethod::Get, request.uri().path());
    if let Some(route) = route
        && route.protocol == RouteProtocol::Websocket
    {
        return match websocket_upgrade {
            Ok(websocket_upgrade) => {
                proxy_websocket_request(
                    websocket_upgrade,
                    request,
                    &state,
                    route.service_id.as_str(),
                    route.websocket_subprotocols.as_slice(),
                )
                .await
            }
            Err(rejection) => rejection.into_response(),
        };
    }

    if route.is_some() {
        return proxy_request(State(state), request).await;
    }

    delegate_to_runtime_router(
        runtime_router_for_path(&state, request.uri().path()),
        request,
    )
    .await
}

pub(crate) async fn proxy_request(State(state): State<GatewayState>, request: Request) -> Response {
    let request = match dispatch_embedded_session_gateway_if_configured(&state, request).await {
        Ok(response) => return response,
        Err(request) => request,
    };

    let Some(registry_method) = map_http_method(request.method()) else {
        return json_error_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "gateway does not support proxying this method",
        );
    };
    let Some(route) = state
        .registry
        .resolve(registry_method, request.uri().path())
    else {
        let (parts, body) = request.into_parts();
        return delegate_to_runtime_router(
            runtime_router_for_path(&state, parts.uri.path()),
            Request::from_parts(parts, body),
        )
        .await;
    };
    let service_id = route.service_id.clone();
    if !state.circuit_breakers.check(service_id.as_str()) {
        tracing::warn!(
            target: "sdkwork.im.gateway",
            event = "im.gateway.circuit_open",
            service = %service_id,
            path = %request.uri().path(),
            "request rejected by circuit breaker for {service_id}"
        );
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            format!(
                "upstream service {service_id} is temporarily unavailable. Please retry later."
            ).as_str(),
        );
    }
    let Some(upstream_base_url) = state.config.upstream_base_url(service_id.as_str()) else {
        if state.config.runtime_mode == GatewayRuntimeMode::Unified
            && (is_assembly_embedded_im_service(service_id.as_str())
                || sdkwork_im_cloud_gateway_config::is_standalone_embedded_dependency_service(
                    service_id.as_str(),
                ))
        {
            let (parts, body) = request.into_parts();
            return delegate_to_runtime_router(
                runtime_router_for_path(&state, parts.uri.path()),
                Request::from_parts(parts, body),
            )
            .await;
        }
        return json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("upstream target is not configured for {service_id}").as_str(),
        );
    };
    let (parts, body) = request.into_parts();
    let method = parts.method;
    let headers = parts.headers;
    let uri = parts.uri;
    let upstream_url = format!(
        "{}{}",
        upstream_base_url.trim_end_matches('/'),
        uri.path_and_query()
            .map(|value| value.as_str())
            .unwrap_or("/")
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST
            || *name == header::CONTENT_LENGTH
            || *name == header::CONNECTION
            || is_sdkwork_context_projection_header(name)
        {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }
    let body = match axum::body::to_bytes(body, resolve_max_http_request_body_bytes()).await {
        Ok(body) => body,
        Err(error) => {
            let status = if error.to_string().contains("length limit exceeded") {
                StatusCode::PAYLOAD_TOO_LARGE
            } else {
                StatusCode::BAD_REQUEST
            };
            return json_error_response(
                status,
                format!("gateway failed to read request body: {error}").as_str(),
            );
        }
    };

    match request_builder.body(body).send().await {
        Ok(upstream_response) => {
            let status = upstream_response.status();
            if status.is_server_error() {
                state.circuit_breakers.record_failure(service_id.as_str());
            } else {
                state.circuit_breakers.record_success(service_id.as_str());
            }
            build_proxy_response(service_id.as_str(), upstream_response).await
        }
        Err(error) => {
            state.circuit_breakers.record_failure(service_id.as_str());
            json_error_response(
                StatusCode::BAD_GATEWAY,
                format!("gateway upstream request to {service_id} failed: {error}").as_str(),
            )
        }
    }
}
