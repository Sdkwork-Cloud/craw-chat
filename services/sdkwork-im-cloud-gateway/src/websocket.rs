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
use tokio::sync::OwnedSemaphorePermit;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite,
    tungstenite::client::IntoClientRequest,
};

use crate::constants::{
    GATEWAY_MAX_WEBSOCKET_FRAME_BYTES, GATEWAY_MAX_WEBSOCKET_MESSAGE_BYTES,
    SDKWORK_INTERNAL_HEADER_PREFIX, WEBSOCKET_UPSTREAM_CONNECT_TIMEOUT_SECONDS,
};
use crate::gateway_protection::extract_client_ip_from_headers;
use crate::response::json_error_response;
use crate::state::GatewayState;
use crate::websocket_auth::{
    sanitized_gateway_websocket_path_and_query,
    should_authenticate_gateway_websocket_with_init_frame, websocket_auth_headers_from_query,
    websocket_dual_token_headers_for_auth_init,
};

async fn record_websocket_auth_failure(
    state: &GatewayState,
    headers: &HeaderMap,
    tenant_id: Option<&str>,
    user_id: Option<&str>,
) {
    let client_ip = extract_client_ip_from_headers(headers);
    // `check_auth_attempt` may perform blocking Redis IO (failed_auth
    // increment + ip_blocks.block_for_secs); run it on the blocking pool so
    // the async worker stays free and the gateway does not stall under
    // auth-failure storms.
    let detector = state.anomaly_detector.clone();
    let tenant_owned = tenant_id.unwrap_or("gateway").to_owned();
    let user_owned = user_id.map(str::to_owned);
    let event = tokio::task::spawn_blocking(move || {
        detector.check_auth_attempt(user_owned.as_deref(), &tenant_owned, client_ip, false)
    })
    .await
    .ok()
    .flatten();
    if let Some(event) = event {
        tracing::warn!(
            target: "sdkwork.im.gateway.anomaly",
            event = "im.gateway.websocket_auth_anomaly",
            anomaly_type = event.anomaly_type.as_str(),
            client_ip = %client_ip,
            "websocket authentication anomaly detected"
        );
    }
}

#[derive(Clone)]
struct WebsocketAnomalyMonitor {
    state: GatewayState,
    tenant_id: String,
    user_id: String,
    client_ip: std::net::IpAddr,
}

async fn record_websocket_connection_established(
    state: &GatewayState,
    tenant_id: &str,
    user_id: &str,
    client_ip: std::net::IpAddr,
) -> WebsocketAnomalyMonitor {
    // `check_connection` may perform blocking Redis IO (connection_rate
    // increment); run it on the blocking pool so the async worker stays free.
    let detector = state.anomaly_detector.clone();
    let tenant_owned = tenant_id.to_owned();
    let user_owned = user_id.to_owned();
    let event = tokio::task::spawn_blocking(move || {
        detector.check_connection(&user_owned, &tenant_owned, client_ip)
    })
    .await
    .ok()
    .flatten();
    if let Some(event) = event {
        tracing::warn!(
            target: "sdkwork.im.gateway.anomaly",
            event = "im.gateway.websocket_connection_anomaly",
            anomaly_type = event.anomaly_type.as_str(),
            tenant_id = %tenant_id,
            user_id = %user_id,
            client_ip = %client_ip,
            "websocket connection anomaly detected"
        );
    }
    WebsocketAnomalyMonitor {
        state: state.clone(),
        tenant_id: tenant_id.to_owned(),
        user_id: user_id.to_owned(),
        client_ip,
    }
}

async fn inspect_downstream_websocket_message(
    monitor: &WebsocketAnomalyMonitor,
    message: &Message,
) -> bool {
    let content = match message {
        Message::Text(text) => text.as_str(),
        Message::Binary(_) => "",
        _ => return true,
    };
    // `check_message`, `should_terminate_connection`, and
    // `enforce_recommended_action` may all perform blocking Redis IO
    // (message-rate increments + ip_blocks.block_for_secs); batch them
    // into a single spawn_blocking so only one blocking-thread hop is
    // needed per downstream message. Tracing also runs on the blocking
    // pool — it is non-blocking and avoids moving `event` back across
    // the thread boundary (which would partial-move `recommended_action`).
    // Returns `true` when the message is allowed to proceed.
    let detector = monitor.state.anomaly_detector.clone();
    let user_id = monitor.user_id.clone();
    let tenant_id = monitor.tenant_id.clone();
    let client_ip = monitor.client_ip;
    let content_owned = content.to_owned();
    tokio::task::spawn_blocking(move || {
        let event = match detector.check_message(&user_id, &tenant_id, client_ip, &content_owned) {
            Some(event) => event,
            None => return true,
        };
        let should_terminate = detector.should_terminate_connection(&event);
        tracing::warn!(
            target: "sdkwork.im.gateway.anomaly",
            event = "im.gateway.websocket_message_anomaly",
            anomaly_type = event.anomaly_type.as_str(),
            tenant_id = %tenant_id,
            user_id = %user_id,
            client_ip = %client_ip,
            recommended_action = %event.recommended_action.as_str(),
            "websocket downstream message anomaly detected"
        );
        detector.enforce_recommended_action(event.recommended_action, client_ip);
        !should_terminate
    })
    .await
    .unwrap_or(true)
}

pub(crate) async fn proxy_websocket_request(
    ws: WebSocketUpgrade,
    request: Request,
    state: &GatewayState,
    service_id: &str,
    websocket_subprotocols: &[String],
) -> Response {
    let client_ip = extract_client_ip_from_headers(request.headers());
    // `is_ip_blocked` may perform blocking Redis IO (ip_blocks.is_blocked);
    // run it on the blocking pool so the async worker stays free and the
    // gateway does not stall under auth-abuse storms from many IPs.
    let detector = state.anomaly_detector.clone();
    let is_blocked = tokio::task::spawn_blocking(move || detector.is_ip_blocked(client_ip))
        .await
        .unwrap_or(false);
    if is_blocked {
        return json_error_response(
            StatusCode::TOO_MANY_REQUESTS,
            "client IP is temporarily blocked due to authentication abuse",
        );
    }

    let connection_permit = match state
        .websocket_connection_semaphore
        .clone()
        .try_acquire_owned()
    {
        Ok(permit) => permit,
        Err(_) => {
            return json_error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                "gateway websocket connection capacity reached, please retry later",
            );
        }
    };

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
            )
            .as_str(),
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
                    connection_permit,
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
        let environment =
            std::env::var("SDKWORK_IM_ENVIRONMENT").unwrap_or_else(|_| "development".to_owned());
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
                    connection_permit,
                )
            })
            .into_response();
    }

    let Ok(upstream_url) =
        upstream_websocket_url(upstream_base_url.as_str(), &sanitized_path_and_query)
    else {
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
                    proxy_websocket_streams(
                        downstream_socket,
                        upstream_socket,
                        None,
                        connection_permit,
                    )
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
    _connection_permit: OwnedSemaphorePermit,
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
            record_websocket_auth_failure(&state, &original_headers, None, None).await;
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
            let client_ip = extract_client_ip_from_headers(&original_headers);
            if let Ok(auth_context) = state
                .realtime_auth
                .resolve_from_headers(&upstream_auth_headers)
                .await
            {
                let monitor = record_websocket_connection_established(
                    &state,
                    auth_context.tenant_id.as_str(),
                    auth_context.user_id.as_str(),
                    client_ip,
                )
                .await;
                proxy_websocket_streams(
                    downstream_socket,
                    upstream_socket,
                    Some(monitor),
                    _connection_permit,
                )
                .await;
                return;
            }
            proxy_websocket_streams(downstream_socket, upstream_socket, None, _connection_permit)
                .await;
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
    _connection_permit: OwnedSemaphorePermit,
) {
    let Some(auth_init) = read_websocket_auth_init_frame(&mut downstream_socket).await else {
        record_websocket_auth_failure(&state, &original_headers, None, None).await;
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
            record_websocket_auth_failure(&state, &original_headers, None, None).await;
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
    let effective_device_id =
        coalesce_websocket_device_id(auth_init.device_id.clone(), query_device_id);
    let upstream_auth_headers = match websocket_dual_token_headers_for_auth_init(
        &state.realtime_auth,
        &auth_headers,
        effective_device_id.as_deref(),
    )
    .await
    {
        Ok(headers) => headers,
        Err(_) => {
            record_websocket_auth_failure(&state, &original_headers, None, None).await;
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
            record_websocket_auth_failure(&state, &original_headers, None, None).await;
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
            record_websocket_auth_failure(
                &state,
                &original_headers,
                Some(auth_context.tenant_id.as_str()),
                Some(auth_context.user_id.as_str()),
            )
            .await;
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

    let path_and_query =
        websocket_path_and_query_with_device(path_and_query, Some(device_id.as_str()));
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
            let client_ip = extract_client_ip_from_headers(&original_headers);
            let monitor = record_websocket_connection_established(
                &state,
                auth_context.tenant_id.as_str(),
                auth_context.user_id.as_str(),
                client_ip,
            )
            .await;
            let _ = send_websocket_auth_ok(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                &auth_context,
                device_id.as_str(),
            )
            .await;
            proxy_websocket_streams(
                downstream_socket,
                upstream_socket,
                Some(monitor),
                _connection_permit,
            )
            .await;
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
    anomaly_monitor: Option<WebsocketAnomalyMonitor>,
    _connection_permit: OwnedSemaphorePermit,
) {
    let (mut downstream_sender, mut downstream_receiver) = downstream_socket.split();
    let (mut upstream_sender, mut upstream_receiver) = upstream_socket.split();

    loop {
        tokio::select! {
            downstream_message = downstream_receiver.next() => {
                match downstream_message {
                    Some(Ok(message)) => {
                        if let Some(monitor) = anomaly_monitor.as_ref()
                            && !inspect_downstream_websocket_message(monitor, &message).await
                        {
                            let _ = downstream_sender.send(Message::Close(None)).await;
                            let _ = upstream_sender.close().await;
                            break;
                        }
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
                        if !upstream_websocket_message_within_bounds(&message) {
                            let _ = downstream_sender.send(Message::Close(None)).await;
                            let _ = upstream_sender.close().await;
                            break;
                        }
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

fn upstream_websocket_message_within_bounds(message: &tungstenite::Message) -> bool {
    match message {
        tungstenite::Message::Text(text) => text.len() <= GATEWAY_MAX_WEBSOCKET_MESSAGE_BYTES,
        tungstenite::Message::Binary(bytes) => bytes.len() <= GATEWAY_MAX_WEBSOCKET_MESSAGE_BYTES,
        tungstenite::Message::Ping(payload) | tungstenite::Message::Pong(payload) => {
            payload.len() <= GATEWAY_MAX_WEBSOCKET_FRAME_BYTES
        }
        tungstenite::Message::Close(_) | tungstenite::Message::Frame(_) => true,
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
