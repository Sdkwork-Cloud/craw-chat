use axum::body::Body;
use axum::http::{Request, StatusCode};
use session_gateway::RealtimePlaneBootstrap;
use tokio::net::TcpListener;
use tokio_tungstenite::{
    connect_async,
    tungstenite::client::ClientRequestBuilder,
};
use tower::ServiceExt;

async fn spawn_server(app: axum::Router) -> (String, tokio::task::JoinHandle<()>) {
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

fn default_bootstrap() -> RealtimePlaneBootstrap {
    RealtimePlaneBootstrap {
        assembly: session_gateway::RealtimePlaneAssembly::default(),
        node_id: "node_ws_upgrade".to_owned(),
        cluster_bus: None,
        iam_auth_pool: None,
    }
}

#[tokio::test]
async fn wrapped_realtime_open_api_router_preserves_websocket_upgrade_state() {
    ensure_dev_environment();
    let bootstrap = default_bootstrap();
    let app = sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap(
        &bootstrap,
    );
    let (address, handle) = spawn_server(app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{address}/im/v3/api/realtime/ws?deviceId=device_upgrade")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol("sdkwork-im.ccp.ws.v1");

    let (mut socket, response) = connect_async(request)
        .await
        .expect("wrapped realtime router must upgrade browser websocket without HTTP 426");
    assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);

    let _ = socket.close(None).await;
    handle.abort();
}

#[tokio::test]
async fn embedded_realtime_http_router_oneshot_dispatch_still_serves_healthz() {
    ensure_dev_environment();
    let bootstrap = default_bootstrap();
    let app = sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap(
        &bootstrap,
    );

    let request = Request::builder()
        .method("GET")
        .uri("/healthz")
        .body(Body::empty())
        .expect("healthz request should build");

    let response = app
        .oneshot(request)
        .await
        .expect("embedded gateway oneshot dispatch should succeed for HTTP routes");
    assert_eq!(response.status(), StatusCode::OK);
}
