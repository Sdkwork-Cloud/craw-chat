use axum::{
    Json, Router,
    extract::{
        OriginalUri,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use im_app_context::{AppContext, build_dual_token_headers_for_context, local_service_app_context};
use sdkwork_im_api_registry::{
    HttpMethod, RouteDescriptor, RouteProtocol, RouteVisibility, SdkTarget, build_registry,
};
use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use sdkwork_im_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;
use serde_json::json;
use tokio::net::TcpListener;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message as TungsteniteMessage, client::ClientRequestBuilder},
};

const SDKWORK_INTERNAL_HEADER_PROBE: &str = "x-sdkwork-tenant-id";
const GATEWAY_WEBSOCKET_ALLOW_QUERY_TOKENS_ENV: &str =
    "SDKWORK_IM_GATEWAY_ALLOW_WEBSOCKET_QUERY_TOKENS";

fn ensure_gateway_dev_web_environment() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
            std::env::set_var("SDKWORK_ENV", "dev");
        }
    });
}

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    ensure_gateway_dev_web_environment();
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (format!("127.0.0.1:{}", address.port()), handle)
}

fn test_gateway_config(
    upstreams: Vec<sdkwork_im_cloud_gateway_config::ServiceUpstreamConfig>,
) -> WebGatewayConfig {
    WebGatewayConfig {
        bind_addr: "127.0.0.1:0".to_owned(),
        runtime_mode: GatewayRuntimeMode::Split,
        strict_startup: true,
        upstreams,
    }
}

fn gateway_test_app_context() -> AppContext {
    let mut context = local_service_app_context(
        "100001",
        "30",
        "user",
        Some("device_real"),
        ["*"],
    );
    context.session_id = Some("session_real".to_owned());
    context.app_id = Some("sdkwork-im-pc".to_owned());
    context
}

fn gateway_test_auth_headers() -> HeaderMap {
    let context = gateway_test_app_context();
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn gateway_test_authorization_header() -> String {
    gateway_test_auth_headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("test auth token should be present")
        .to_owned()
}

fn gateway_test_auth_token() -> String {
    gateway_test_authorization_header()
        .trim_start_matches("Bearer ")
        .to_owned()
}

fn gateway_test_access_token_header() -> String {
    gateway_test_auth_headers()
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .expect("test access token should be present")
        .to_owned()
}

fn has_sdkwork_internal_header(headers: &HeaderMap) -> bool {
    [
        "x-sdkwork-tenant-id",
        "x-sdkwork-organization-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-context-signature",
    ]
    .iter()
    .any(|name| headers.contains_key(*name))
}

async fn websocket_echo(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(handle_echo_socket)
}

async fn websocket_context_echo(headers: HeaderMap, ws: WebSocketUpgrade) -> impl IntoResponse {
    let context = im_app_context::resolve_app_context(&headers).ok();
    let sdkwork_internal_headers_forwarded = has_sdkwork_internal_header(&headers);
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let payload = match context {
                Some(context) => json!({
                    "tenantId": context.tenant_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "deviceId": context.device_id,
                    "sdkworkInternalHeadersForwarded": sdkwork_internal_headers_forwarded,
                }),
                None => json!({
                    "tenantId": null,
                    "userId": null,
                    "sessionId": null,
                    "deviceId": null,
                    "sdkworkInternalHeadersForwarded": sdkwork_internal_headers_forwarded,
                }),
            };
            let _ = socket.send(Message::Text(payload.to_string().into())).await;
            let _ = socket.close().await;
        })
}

async fn websocket_query_echo(
    OriginalUri(uri): OriginalUri,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let query = uri.query().unwrap_or_default().to_owned();
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let _ = socket
                .send(Message::Text(
                    json!({
                        "query": query,
                    })
                    .to_string()
                    .into(),
                ))
                .await;
            let _ = socket.close().await;
        })
}

async fn websocket_sdkwork_internal_probe(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let result = im_app_context::resolve_app_context(&headers);
    let sdkwork_internal_headers_forwarded = has_sdkwork_internal_header(&headers);
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let payload = match result {
                Ok(context) => json!({
                    "tenantId": context.tenant_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "sdkworkInternalHeadersForwarded": sdkwork_internal_headers_forwarded,
                }),
                Err(error) => json!({
                    "code": error.code(),
                    "message": error.message(),
                    "sdkworkInternalHeadersForwarded": sdkwork_internal_headers_forwarded,
                }),
            };
            let _ = socket.send(Message::Text(payload.to_string().into())).await;
            let _ = socket.close().await;
        })
}

async fn appbase_current_session(headers: HeaderMap) -> impl IntoResponse {
    let Ok(context) = im_app_context::resolve_app_context(&headers) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "type": "about:blank",
                "title": "Unauthorized",
                "status": 401,
                "detail": "dual token session is required"
            })),
        )
            .into_response();
    };

    (
        StatusCode::OK,
        Json(json!({
            "data": {
                "context": {
                    "tenantId": context.tenant_id,
                    "organizationId": context.organization_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "appId": context.app_id,
                    "actorKind": context.actor_kind
                }
            }
        })),
    )
        .into_response()
}

async fn handle_echo_socket(mut socket: WebSocket) {
    while let Some(message) = socket.next().await {
        let Ok(message) = message else {
            break;
        };

        match message {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Binary(bytes) => {
                if socket.send(Message::Binary(bytes)).await.is_err() {
                    break;
                }
            }
            Message::Ping(payload) => {
                if socket.send(Message::Pong(payload)).await.is_err() {
                    break;
                }
            }
            Message::Pong(_) => {}
            Message::Close(frame) => {
                let _ = socket.send(Message::Close(frame)).await;
                break;
            }
        }
    }
}

#[tokio::test]
async fn gateway_accepts_browser_realtime_websocket_auth_init_before_upstream_connect() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_context_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws?deviceId=device_real")
            .parse()
            .unwrap(),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-1",
                "authToken": gateway_test_auth_token(),
                "accessToken": gateway_test_access_token_header(),
                "deviceId": "device_real"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send");

    let auth_ok = socket
        .next()
        .await
        .expect("auth.ok frame should arrive")
        .expect("auth.ok frame should decode");
    let TungsteniteMessage::Text(text) = auth_ok else {
        panic!("expected auth.ok text frame, got {auth_ok:?}");
    };
    let auth_ok: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("auth.ok frame should be json");
    assert_eq!(auth_ok["type"], "auth.ok", "first frame: {auth_ok}");
    assert_eq!(auth_ok["requestId"], "auth-1");
    assert_eq!(auth_ok["tenantId"], "100001");
    assert_eq!(auth_ok["principalId"], "30");
    assert_eq!(auth_ok["sessionId"], "session_real");
    assert_eq!(auth_ok["deviceId"], "device_real");

    let context_frame = socket
        .next()
        .await
        .expect("upstream context frame should arrive after auth.ok")
        .expect("upstream context frame should decode");
    let TungsteniteMessage::Text(text) = context_frame else {
        panic!("expected context text frame, got {context_frame:?}");
    };
    let context: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("context frame should be json");
    assert_eq!(context["tenantId"], "100001");
    assert_eq!(context["userId"], "30");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["deviceId"], "device_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_accepts_realtime_websocket_ping_before_auth_init() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_context_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws?deviceId=device_real")
            .parse()
            .unwrap(),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Ping(vec![1, 2, 3].into()))
        .await
        .expect("ping frame should send");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-after-ping",
                "authToken": gateway_test_auth_token(),
                "accessToken": gateway_test_access_token_header(),
                "deviceId": "device_real"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send after ping");

    let auth_ok_text = loop {
        let frame = socket
            .next()
            .await
            .expect("auth.ok frame should arrive")
            .expect("auth.ok frame should decode");
        if let TungsteniteMessage::Text(text) = frame {
            break text;
        }
    };
    let auth_ok: serde_json::Value =
        serde_json::from_str(auth_ok_text.as_str()).expect("auth.ok frame should be json");
    assert_eq!(auth_ok["type"], "auth.ok");
    assert_eq!(auth_ok["requestId"], "auth-after-ping");

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_strips_sensitive_realtime_websocket_query_before_upstream_connect() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_query_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!(
            "ws://{gateway_address}/im/v3/api/realtime/ws?deviceId=device_real&conversationId=conversation-leak&authToken=query-auth-token&accessToken=query-access-token&token=query-token&authorization=query-authorization&refreshToken=query-refresh-token"
        )
        .parse()
        .unwrap(),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-sensitive-query",
                "authToken": gateway_test_auth_token(),
                "accessToken": gateway_test_access_token_header(),
                "deviceId": "device_real"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send");

    let auth_ok = socket
        .next()
        .await
        .expect("auth.ok frame should arrive")
        .expect("auth.ok frame should decode");
    assert!(matches!(auth_ok, TungsteniteMessage::Text(_)));

    let query_frame = socket
        .next()
        .await
        .expect("upstream query frame should arrive after auth.ok")
        .expect("upstream query frame should decode");
    let TungsteniteMessage::Text(text) = query_frame else {
        panic!("expected query text frame, got {query_frame:?}");
    };
    let query: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("query frame should be json");
    let query = query["query"]
        .as_str()
        .expect("query frame should expose upstream query");
    assert_eq!(query, "deviceId=device_real");
    assert!(!query.contains("conversationId"));
    assert!(!query.contains("authToken"));
    assert!(!query.contains("accessToken"));
    assert!(!query.contains("authorization"));
    assert!(!query.contains("refreshToken"));
    assert!(!query.contains("query-token"));

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_rejects_realtime_websocket_business_frame_before_auth_init() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_context_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "subscriptions.sync",
                "requestId": "sub-before-auth",
                "items": []
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("pre-auth business frame should send");

    let error_frame = socket
        .next()
        .await
        .expect("auth error frame should arrive")
        .expect("auth error frame should decode");
    let TungsteniteMessage::Text(text) = error_frame else {
        panic!("expected auth error text frame, got {error_frame:?}");
    };
    let error: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("auth error frame should be json");
    assert_eq!(error["type"], "error");
    assert_eq!(error["code"], "websocket_auth_required");
    assert_eq!(error["requestId"], "sub-before-auth");

    let close_frame = socket
        .next()
        .await
        .expect("gateway should close websocket after auth error")
        .expect("close frame should decode");
    assert!(matches!(close_frame, TungsteniteMessage::Close(_)));

    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_derives_realtime_websocket_context_from_appbase_dual_tokens_not_client_headers() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_context_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL)
    .with_header("Authorization", gateway_test_authorization_header())
    .with_header("access-token", gateway_test_access_token_header())
    .with_header(SDKWORK_INTERNAL_HEADER_PROBE, "100001")
    .with_header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
    .with_header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed");
    let received = socket
        .next()
        .await
        .expect("upstream context frame should arrive")
        .expect("upstream context frame should decode");
    let TungsteniteMessage::Text(text) = received else {
        panic!("expected text context frame, got {received:?}");
    };
    let context: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("context frame should be json");

    assert_eq!(context["tenantId"], "100001");
    assert_eq!(context["userId"], "30");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
    assert_ne!(context["tenantId"], "100001");
    assert_ne!(context["userId"], "user_test006_a_com");

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_drops_realtime_websocket_sdkwork_internal_headers_when_signature_secret_is_configured()
 {
    let _signature_secret = ScopedEnvVar::set(
        "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET",
        "gateway-signing-secret",
    );
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route(
        "/im/v3/api/realtime/ws",
        get(websocket_sdkwork_internal_probe),
    );
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL)
    .with_header("Authorization", gateway_test_authorization_header())
    .with_header("access-token", gateway_test_access_token_header())
    .with_header(SDKWORK_INTERNAL_HEADER_PROBE, "100001")
    .with_header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
    .with_header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed-signature");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed");
    let received = socket
        .next()
        .await
        .expect("upstream context frame should arrive")
        .expect("upstream context frame should decode");
    let TungsteniteMessage::Text(text) = received else {
        panic!("expected text context frame, got {received:?}");
    };
    let context: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("context frame should be json");

    assert_eq!(context["tenantId"], "100001");
    assert_eq!(context["userId"], "30");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_proxies_realtime_websocket_upgrade_and_frames() {
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-iam-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL)
    .with_header("Authorization", gateway_test_authorization_header())
    .with_header("access-token", gateway_test_access_token_header());

    let (mut socket, response) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("gateway should select websocket subprotocol"),
        LINK_WEBSOCKET_SUBPROTOCOL
    );

    socket
        .send(TungsteniteMessage::Text("hello gateway".into()))
        .await
        .expect("client text frame should send");
    let echoed = socket
        .next()
        .await
        .expect("echoed frame should arrive")
        .expect("echoed frame should decode");
    assert_eq!(echoed, TungsteniteMessage::Text("hello gateway".into()));

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<String>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            unsafe {
                std::env::set_var(self.name, previous);
            }
            return;
        }

        unsafe {
            std::env::remove_var(self.name);
        }
    }
}

#[tokio::test]
async fn gateway_proxies_registry_owned_websocket_routes_beyond_realtime_path() {
    let upstream_app = Router::new().route("/ws/custom/echo", get(websocket_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;

    let registry = build_registry(vec![RouteDescriptor {
        service_id: "session-gateway".to_owned(),
        methods: vec![HttpMethod::Get],
        path_pattern: "/ws/custom/{*path}".to_owned(),
        visibility: RouteVisibility::Public,
        sdk_targets: vec![SdkTarget::SdkworkImSdk],
        operation_group: "realtime".to_owned(),
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols: vec![LINK_WEBSOCKET_SUBPROTOCOL.to_owned()],
    }])
    .expect("custom websocket route registry should build");

    let gateway_app = web_gateway::build_app_with_registry(
        test_gateway_config(vec![
            service_upstream(
                "session-gateway",
                format!("http://{upstream_address}").as_str(),
            ),
            service_upstream(
                "sdkwork-iam-app-api",
                format!("http://{appbase_address}").as_str(),
            ),
        ]),
        registry,
    );
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/ws/custom/echo")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL);

    let (mut socket, response) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed for custom route");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("gateway should select websocket subprotocol"),
        LINK_WEBSOCKET_SUBPROTOCOL
    );

    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-custom-1",
                "authToken": gateway_test_auth_token(),
                "accessToken": gateway_test_access_token_header(),
                "deviceId": "device_real"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send");

    let auth_ok = socket
        .next()
        .await
        .expect("auth.ok frame should arrive")
        .expect("auth.ok frame should decode");
    let TungsteniteMessage::Text(text) = auth_ok else {
        panic!("expected auth.ok text frame, got {auth_ok:?}");
    };
    let auth_ok: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("auth.ok frame should be json");
    assert_eq!(auth_ok["type"], "auth.ok", "first frame: {auth_ok}");
    assert_eq!(auth_ok["requestId"], "auth-custom-1");

    socket
        .send(TungsteniteMessage::Text("hello custom route".into()))
        .await
        .expect("client text frame should send");
    let echoed = socket
        .next()
        .await
        .expect("echoed frame should arrive")
        .expect("echoed frame should decode");
    assert_eq!(
        echoed,
        TungsteniteMessage::Text("hello custom route".into())
    );

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_accepts_registry_owned_websocket_query_tokens_when_enabled_and_strips_sensitive_query()
{
    let _allow_query_tokens = ScopedEnvVar::set(GATEWAY_WEBSOCKET_ALLOW_QUERY_TOKENS_ENV, "true");
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/ws/custom/echo", get(websocket_query_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let registry = build_registry(vec![RouteDescriptor {
        service_id: "session-gateway".to_owned(),
        methods: vec![HttpMethod::Get],
        path_pattern: "/ws/custom/{*path}".to_owned(),
        visibility: RouteVisibility::Public,
        sdk_targets: vec![SdkTarget::SdkworkImSdk],
        operation_group: "realtime".to_owned(),
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols: vec![LINK_WEBSOCKET_SUBPROTOCOL.to_owned()],
    }])
    .expect("custom websocket route registry should build");

    let gateway_app = web_gateway::build_app_with_registry(
        test_gateway_config(vec![
            service_upstream(
                "session-gateway",
                format!("http://{upstream_address}").as_str(),
            ),
            service_upstream(
                "sdkwork-iam-app-api",
                format!("http://{appbase_address}").as_str(),
            ),
        ]),
        registry,
    );
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!(
            "ws://{gateway_address}/ws/custom/echo?foo=bar&authToken={}&accessToken={}&deviceId=device-frame",
            gateway_test_auth_token(),
            gateway_test_access_token_header()
        )
        .parse()
        .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL);

    let (mut socket, _) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed for custom route via query tokens");

    let query_frame = socket
        .next()
        .await
        .expect("upstream query frame should arrive")
        .expect("upstream query frame should decode");
    let TungsteniteMessage::Text(text) = query_frame else {
        panic!("expected query text frame, got {query_frame:?}");
    };
    let query: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("query frame should be json");
    let query = query["query"]
        .as_str()
        .expect("query frame should expose upstream query");

    assert!(query.contains("foo=bar"));
    assert!(query.contains("deviceId=device-frame"));
    assert!(!query.contains("authToken"));
    assert!(!query.contains("accessToken"));

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}
