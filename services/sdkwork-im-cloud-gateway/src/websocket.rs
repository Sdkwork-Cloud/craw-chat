//! WebSocket proxy pipeline: upgrade handling, upstream connection, bidirectional
//! stream forwarding, message conversion, and URL/header helpers.

use std::time::Duration;

use axum::{
    extract::{
        Request,
        ws::{CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use im_app_context::{coalesce_websocket_device_id, websocket_query_device_id_from_path_and_query};
use sdkwork_im_websocket_auth_gate::{
    close_websocket_with_auth_error, dual_token_headers_from_auth_init_frame,
    read_websocket_auth_init_frame, resolve_websocket_device_binding, send_websocket_auth_ok,
    should_require_auth_init_frame,
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite,
    tungstenite::client::IntoClientRequest,
};

use crate::constants::{
    GATEWAY_MAX_WEBSOCKET_FRAME_BYTES, GATEWAY_MAX_WEBSOCKET_MESSAGE_BYTES,
    SDKWORK_INTERNAL_HEADER_PREFIX, WEBSOCKET_UPSTREAM_CONNECT_TIMEOUT_SECONDS,
};
use crate::response::json_error_response;
use crate::state::GatewayState;
use crate::websocket_auth::{
    sanitized_gateway_websocket_path_and_query, should_authenticate_gateway_websocket_with_init_frame,
    websocket_auth_headers_from_query, websocket_dual_token_headers_for_auth_init,
};

pub(crate) async fn proxy_websocket_request(
    ws: WebSocketUpgrade,
    request: Request,
    state: &GatewayState,
    service_id: &str,
    websocket_subprotocols: &[String],
) -> Response {
    let Some(upstream_base_url) = websocket_upstream_base_url(state, service_id) else {
        return json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("upstream target is not configured for {service_id}").as_str(),
        );
    };
    if !state.circuit_breakers.check(service_id) {
        tracing::warn!(
            target: "sdkwork.im.gateway",
            event = "im.gateway.circuit_open",
            service = %service_id,
            "websocket request rejected by circuit breaker for {service_id}"
        );
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            format!(
                "upstream service {service_id} is temporarily unavailable. Please retry later."
            ).as_str(),
        );
    }
    if should_authenticate_gateway_websocket_with_init_frame(request.headers(), request.uri()) {
        let path_and_query = sanitized_gateway_websocket_path_and_query(request.uri());
        let original_headers = request.headers().clone();
        let state = state.clone();
        return bounded_websocket_upgrade(ws)
            .protocols(websocket_subprotocols.to_vec())
            .on_upgrade(move |downstream_socket| {
                proxy_websocket_after_auth_init(
                    downstream_socket,
                    state,
                    upstream_base_url,
                    path_and_query,
                    original_headers,
                )
            })
            .into_response();
    }

    let sanitized_path_and_query = sanitized_gateway_websocket_path_and_query(request.uri());
    if !should_require_auth_init_frame(
        request.headers(),
        websocket_auth_headers_from_query(request.uri()).is_some(),
    ) && let Some(query_auth_headers) = websocket_auth_headers_from_query(request.uri())
    {
        // Query-token auth is less secure than auth.init frame because tokens
        // may appear in access logs, browser history, or referrer headers.
        // In production, reject query-token auth entirely; in non-production,
        // allow it with a debug log for browser compatibility.
        let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_owned());
        if environment == "production" || environment == "prod" {
            tracing::warn!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.websocket_query_token_rejected",
                environment = %environment,
                "WebSocket query-token auth rejected in production — clients must use auth.init frame auth"
            );
            return json_error_response(
                StatusCode::UNAUTHORIZED,
                "WebSocket query-token authentication is not permitted in production. Use auth.init frame authentication instead.",
            );
        } else {
            tracing::debug!(
                target: "sdkwork.im.gateway",
                event = "im.gateway.websocket_query_token_auth",
                environment = %environment,
                "WebSocket query-token auth used (non-production only)"
            );
        }
        let original_headers = request.headers().clone();
        let state = state.clone();
        return bounded_websocket_upgrade(ws)
            .protocols(websocket_subprotocols.to_vec())
            .on_upgrade(move |downstream_socket| {
                proxy_websocket_after_query_token_auth(
                    downstream_socket,
                    state,
                    upstream_base_url,
                    sanitized_path_and_query,
                    original_headers,
                    query_auth_headers,
                )
            })
            .into_response();
    }

    let Ok(upstream_url) = upstream_websocket_url(
        upstream_base_url.as_str(),
        &sanitized_path_and_query,
    ) else {
        return json_error_response(
            StatusCode::BAD_GATEWAY,
            format!(
                "gateway websocket upstream URL is invalid for {}",
                service_id
            )
            .as_str(),
        );
    };
    let mut upstream_request = match upstream_url.as_str().into_client_request() {
        Ok(request) => request,
        Err(error) => {
            return json_error_response(
                StatusCode::BAD_GATEWAY,
                format!(
                    "gateway failed to prepare websocket upstream request for {}: {error}",
                    service_id
                )
                .as_str(),
            );
        }
    };
    copy_websocket_headers(request.headers(), upstream_request.headers_mut());

    match connect_upstream_websocket(upstream_request).await {
        Ok(upstream_socket) => {
            state.circuit_breakers.record_success(service_id);
            bounded_websocket_upgrade(ws)
            .protocols(websocket_subprotocols.to_vec())
            .on_upgrade(move |downstream_socket| {
                proxy_websocket_streams(downstream_socket, upstream_socket)
            })
            .into_response()
        }
        Err(error) => {
            state.circuit_breakers.record_failure(service_id);
            json_error_response(
                StatusCode::BAD_GATEWAY,
                format!(
                    "gateway websocket upstream request to {} failed: {error}",
                    service_id
                )
                .as_str(),
            )
        }
    }
}

async fn proxy_websocket_after_query_token_auth(
    downstream_socket: WebSocket,
    state: GatewayState,
    upstream_base_url: String,
    path_and_query: String,
    original_headers: HeaderMap,
    query_auth_headers: HeaderMap,
) {
    let auth_init_device_id = websocket_query_device_id_from_path_and_query(&path_and_query);
    let upstream_auth_headers = match websocket_dual_token_headers_for_auth_init(
        &state.realtime_auth,
        &query_auth_headers,
        auth_init_device_id.as_deref(),
    )
    .await
    {
        Ok(headers) => headers,
        Err(_) => {
            let mut socket = downstream_socket;
            close_websocket_with_auth_error(
                &mut socket,
                None,
                "websocket_auth_failed",
                "websocket query token context validation failed",
            )
            .await;
            return;
        }
    };

    let Ok(upstream_url) = upstream_websocket_url(upstream_base_url.as_str(), &path_and_query)
    else {
        let mut socket = downstream_socket;
        close_websocket_with_auth_error(
            &mut socket,
            None,
            "websocket_upstream_unavailable",
            "gateway websocket upstream URL is invalid",
        )
        .await;
        return;
    };
    let mut upstream_request = match upstream_url.as_str().into_client_request() {
        Ok(request) => request,
        Err(_) => {
            let mut socket = downstream_socket;
            close_websocket_with_auth_error(
                &mut socket,
                None,
                "websocket_upstream_unavailable",
                "gateway failed to prepare websocket upstream request",
            )
            .await;
            return;
        }
    };

    copy_websocket_headers(&original_headers, upstream_request.headers_mut());
    copy_dual_token_headers(&upstream_auth_headers, upstream_request.headers_mut());

    match connect_upstream_websocket(upstream_request).await {
        Ok(upstream_socket) => {
            proxy_websocket_streams(downstream_socket, upstream_socket).await;
        }
        Err(error) => {
            let mut socket = downstream_socket;
            close_websocket_with_auth_error(
                &mut socket,
                None,
                "websocket_upstream_unavailable",
                format!("gateway websocket upstream request failed: {error}").as_str(),
            )
            .await;
        }
    }
}

fn bounded_websocket_upgrade(ws: WebSocketUpgrade) -> WebSocketUpgrade {
    ws.max_message_size(GATEWAY_MAX_WEBSOCKET_MESSAGE_BYTES)
        .max_frame_size(GATEWAY_MAX_WEBSOCKET_FRAME_BYTES)
}

async fn connect_upstream_websocket(
    upstream_request: tungstenite::handshake::client::Request,
) -> Result<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, String> {
    match tokio::time::timeout(
        Duration::from_secs(WEBSOCKET_UPSTREAM_CONNECT_TIMEOUT_SECONDS),
        connect_async(upstream_request),
    )
    .await
    {
        Ok(Ok((upstream_socket, _))) => Ok(upstream_socket),
        Ok(Err(error)) => Err(error.to_string()),
        Err(_) => Err("upstream websocket connection timed out".to_owned()),
    }
}

async fn proxy_websocket_after_auth_init(
    mut downstream_socket: WebSocket,
    state: GatewayState,
    upstream_base_url: String,
    path_and_query: String,
    original_headers: HeaderMap,
) {
    let Some(auth_init) = read_websocket_auth_init_frame(&mut downstream_socket).await else {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            None,
            "websocket_auth_required",
            "auth.init frame is required before websocket frames",
        )
        .await;
        return;
    };
    let auth_headers = match dual_token_headers_from_auth_init_frame(&auth_init) {
        Ok(headers) => headers,
        Err(error) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                error.error_code(),
                error.message(),
            )
            .await;
            return;
        }
    };
    let query_device_id = websocket_query_device_id_from_path_and_query(&path_and_query);
    let effective_device_id = coalesce_websocket_device_id(
        auth_init.device_id.clone(),
        query_device_id,
    );
    let upstream_auth_headers = match websocket_dual_token_headers_for_auth_init(
        &state.realtime_auth,
        &auth_headers,
        effective_device_id.as_deref(),
    )
    .await
    {
        Ok(headers) => headers,
        Err(_) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                "websocket_auth_failed",
                "websocket auth.init token context validation failed",
            )
            .await;
            return;
        }
    };
    let auth_context = match state
        .realtime_auth
        .resolve_from_headers(&upstream_auth_headers)
        .await
    {
        Ok(context) => context,
        Err(_) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                "websocket_auth_failed",
                "websocket auth.init token context validation failed",
            )
            .await;
            return;
        }
    };
    let device_id = match resolve_websocket_device_binding(&auth_context, effective_device_id) {
        Ok(device_id) => device_id,
        Err(error) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                error.code,
                error.message.as_str(),
            )
            .await;
            return;
        }
    };

    let path_and_query = websocket_path_and_query_with_device(path_and_query, Some(device_id.as_str()));
    let Ok(upstream_url) = upstream_websocket_url(upstream_base_url.as_str(), &path_and_query)
    else {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            auth_init.request_id.as_deref(),
            "websocket_upstream_unavailable",
            "gateway websocket upstream URL is invalid",
        )
        .await;
        return;
    };
    let mut upstream_request = match upstream_url.as_str().into_client_request() {
        Ok(request) => request,
        Err(_) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                "websocket_upstream_unavailable",
                "gateway failed to prepare websocket upstream request",
            )
            .await;
            return;
        }
    };
    copy_websocket_headers(&original_headers, upstream_request.headers_mut());
    copy_dual_token_headers(&upstream_auth_headers, upstream_request.headers_mut());

    match connect_upstream_websocket(upstream_request).await {
        Ok(upstream_socket) => {
            let _ = send_websocket_auth_ok(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                &auth_context,
                device_id.as_str(),
            )
            .await;
            proxy_websocket_streams(downstream_socket, upstream_socket).await;
        }
        Err(error) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                "websocket_upstream_unavailable",
                format!("gateway websocket upstream request failed after auth.init: {error}")
                    .as_str(),
            )
            .await;
        }
    }
}

fn websocket_path_and_query_with_device(path_and_query: String, device_id: Option<&str>) -> String {
    if path_and_query.contains("deviceId=") {
        return path_and_query;
    }
    let Some(device_id) = device_id.map(str::trim).filter(|value| !value.is_empty()) else {
        return path_and_query;
    };
    let separator = if path_and_query.contains('?') {
        "&"
    } else {
        "?"
    };
    format!("{path_and_query}{separator}deviceId={device_id}")
}

fn copy_dual_token_headers(source_headers: &HeaderMap, target_headers: &mut HeaderMap) {
    if let Some(value) = source_headers.get(header::AUTHORIZATION) {
        target_headers.insert(header::AUTHORIZATION, value.clone());
    }
    if let Some(value) = source_headers
        .get("access-token")
        .or_else(|| source_headers.get("Access-Token"))
    {
        target_headers.insert("Access-Token", value.clone());
    }
}

fn websocket_upstream_base_url(state: &GatewayState, service_id: &str) -> Option<String> {
    state
        .config
        .upstream_base_url(service_id)
        .map(str::to_owned)
}

async fn proxy_websocket_streams(
    downstream_socket: WebSocket,
    upstream_socket: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
) {
    let (mut downstream_sender, mut downstream_receiver) = downstream_socket.split();
    let (mut upstream_sender, mut upstream_receiver) = upstream_socket.split();

    loop {
        tokio::select! {
            downstream_message = downstream_receiver.next() => {
                match downstream_message {
                    Some(Ok(message)) => {
                        let message = downstream_to_upstream_message(message);
                        let should_stop = matches!(message, tungstenite::Message::Close(_));
                        if upstream_sender.send(message).await.is_err() {
                            break;
                        }
                        if should_stop {
                            break;
                        }
                    }
                    Some(Err(_)) | None => {
                        let _ = upstream_sender.close().await;
                        break;
                    }
                }
            }
            upstream_message = upstream_receiver.next() => {
                match upstream_message {
                    Some(Ok(message)) => {
                        let should_stop = matches!(message, tungstenite::Message::Close(_));
                        let Some(message) = upstream_to_downstream_message(message) else {
                            continue;
                        };
                        if downstream_sender.send(message).await.is_err() {
                            break;
                        }
                        if should_stop {
                            break;
                        }
                    }
                    Some(Err(_)) | None => {
                        let _ = downstream_sender.send(Message::Close(None)).await;
                        break;
                    }
                }
            }
        }
    }
}

fn upstream_websocket_url(base_url: &str, path_and_query: &str) -> Result<String, String> {
    let upstream_base_url = if let Some(value) = base_url.strip_prefix("http://") {
        format!("ws://{value}")
    } else if let Some(value) = base_url.strip_prefix("https://") {
        format!("wss://{value}")
    } else if base_url.starts_with("ws://") || base_url.starts_with("wss://") {
        base_url.to_owned()
    } else {
        return Err(format!(
            "unsupported upstream websocket scheme in {base_url}"
        ));
    };

    Ok(format!(
        "{}{}",
        upstream_base_url.trim_end_matches('/'),
        path_and_query
    ))
}

fn copy_websocket_headers(source_headers: &HeaderMap, target_headers: &mut HeaderMap) {
    for (name, value) in source_headers.iter() {
        if websocket_header_should_forward(name) {
            target_headers.insert(name, value.clone());
        }
    }
}

fn websocket_header_should_forward(name: &header::HeaderName) -> bool {
    !matches!(
        *name,
        header::HOST
            | header::CONNECTION
            | header::UPGRADE
            | header::CONTENT_LENGTH
            | header::SEC_WEBSOCKET_ACCEPT
            | header::SEC_WEBSOCKET_EXTENSIONS
            | header::SEC_WEBSOCKET_KEY
            | header::SEC_WEBSOCKET_VERSION
    ) && !is_reserved_sdkwork_internal_header(name)
}

fn is_reserved_sdkwork_internal_header(name: &header::HeaderName) -> bool {
    name.as_str()
        .to_ascii_lowercase()
        .starts_with(SDKWORK_INTERNAL_HEADER_PREFIX)
}

fn downstream_to_upstream_message(message: Message) -> tungstenite::Message {
    match message {
        Message::Text(text) => tungstenite::Message::Text(text.to_string().into()),
        Message::Binary(bytes) => tungstenite::Message::Binary(bytes),
        Message::Ping(payload) => tungstenite::Message::Ping(payload),
        Message::Pong(payload) => tungstenite::Message::Pong(payload),
        Message::Close(frame) => {
            tungstenite::Message::Close(frame.map(|frame| tungstenite::protocol::CloseFrame {
                code: frame.code.into(),
                reason: frame.reason.to_string().into(),
            }))
        }
    }
}

fn upstream_to_downstream_message(message: tungstenite::Message) -> Option<Message> {
    match message {
        tungstenite::Message::Text(text) => Some(Message::Text(text.to_string().into())),
        tungstenite::Message::Binary(bytes) => Some(Message::Binary(bytes)),
        tungstenite::Message::Ping(payload) => Some(Message::Ping(payload)),
        tungstenite::Message::Pong(payload) => Some(Message::Pong(payload)),
        tungstenite::Message::Close(frame) => Some(Message::Close(frame.map(|frame| CloseFrame {
            code: frame.code.into(),
            reason: Utf8Bytes::from(frame.reason.to_string()),
        }))),
        tungstenite::Message::Frame(_) => None,
    }
}
