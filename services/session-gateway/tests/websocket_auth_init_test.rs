use axum::Router;
use futures_util::{SinkExt, StreamExt};
use im_app_context::{AppContext, build_dual_token_headers_for_context, local_service_app_context};
use serde_json::json;
use tokio::net::TcpListener;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message as TungsteniteMessage, client::ClientRequestBuilder},
};

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
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

fn ensure_dev_environment() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| unsafe {
        std::env::set_var("SDKWORK_IM_ENVIRONMENT", "dev");
        std::env::set_var("SDKWORK_ENV", "dev");
    });
}

fn test_app_context() -> AppContext {
    let mut context = local_service_app_context(
        "100001",
        "30",
        "user",
        Some("device_real"),
        ["*"],
    );
    context.session_id = Some("session_real".to_owned());
    context
}

fn test_auth_init_tokens() -> (String, String) {
    let context = test_app_context();
    let headers = build_dual_token_headers_for_context(&context, context.permission_scope.iter());
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("auth header should be present")
        .trim()
        .to_owned();
    let access_token = headers
        .get("access-token")
        .or_else(|| headers.get("Access-Token"))
        .and_then(|value| value.to_str().ok())
        .expect("access-token should be present")
        .trim()
        .to_owned();
    let auth_token = auth_header.trim_start_matches("Bearer ").to_owned();
    (auth_token, access_token)
}

#[tokio::test]
async fn session_gateway_accepts_browser_auth_init_frame_before_ccp_handshake() {
    ensure_dev_environment();
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{address}/im/v3/api/realtime/ws?deviceId=device_real")
            .parse()
            .unwrap(),
    );
    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should upgrade before auth.init");

    let (auth_token, access_token) = test_auth_init_tokens();
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-1",
                "authToken": auth_token,
                "accessToken": access_token,
                "deviceId": "device_real"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send");

    let received = socket
        .next()
        .await
        .expect("auth.ok frame should arrive")
        .expect("auth.ok frame should decode");
    let TungsteniteMessage::Text(text) = received else {
        panic!("expected auth.ok text frame, got {received:?}");
    };
    let auth_ok: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("auth.ok frame should be json");
    if auth_ok["type"] != "auth.ok" {
        panic!("expected auth.ok frame, got: {auth_ok}");
    }
    assert_eq!(auth_ok["requestId"], "auth-1");
    assert_eq!(auth_ok["tenantId"], "100001");
    assert_eq!(auth_ok["principalId"], "30");
    assert_eq!(auth_ok["sessionId"], "session_real");
    assert_eq!(auth_ok["deviceId"], "device_real");

    let _ = socket.close(None).await;
    handle.abort();
}

