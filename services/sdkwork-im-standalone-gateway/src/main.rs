mod config;
mod embedded_application_routes;
mod embedded_dependency_routes;

use std::net::{IpAddr, SocketAddr};
use std::path::Path;

use axum::Router;
use axum::middleware::from_fn_with_state;
use config::{load_gateway_config, resolve_config_path, resolve_gateway_config, ResolvedGatewayConfig};
use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    resolve_product_site_dirs_from_env, RouterProductRuntimeOptions, build_product_runtime_router,
};
use sdkwork_im_cloud_gateway_config::WebGatewayConfig;
use sdkwork_im_cloud_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};
use tower_http::cors::{Any, CorsLayer};
use web_gateway::gateway_protection::{self, RateLimitConfig, RateLimiter};
use web_gateway::{
    bootstrap_embedded_session_gateway_runtime, build_app_with_registry_product_runtime_and_embedded_services_from_env,
    build_gateway_registry,
};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("sdkwork-im-standalone-gateway");
    sdkwork_im_service_readiness::init_im_service_tracing_from_env();

    // Parse config and apply environment overrides BEFORE spawning the async runtime.
    // std::env::set_var is only safe on the main thread before any other threads exist.
    let args: Vec<String> = std::env::args().collect();
    let config_path = resolve_config_path(&args)?;
    let file_config = load_gateway_config(Path::new(&config_path))?;
    let gateway_config = resolve_gateway_config(file_config)?;
    apply_gateway_process_environment(&gateway_config);

    let bind_addr: SocketAddr = gateway_config
        .bind
        .parse()
        .map_err(|error| format!("invalid bind address `{}`: {error}", gateway_config.bind))?;
    let base_url = format!("http://{}", display_listener_addr(bind_addr));
    apply_collapsed_standalone_urls(&base_url, &bind_addr);

    // Apply embedded dependency environment variables before the async runtime
    // starts to ensure all SDKWORK_*_DATABASE_URL and related env vars are set
    // in a single-threaded context (see set_env_var safety contract).
    embedded_dependency_routes::apply_embedded_dependency_env();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async_main(gateway_config, bind_addr, base_url))
}

async fn async_main(
    gateway_config: ResolvedGatewayConfig,
    bind_addr: SocketAddr,
    base_url: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _im_db = sdkwork_im_database_pool::bootstrap_im_database_from_env()
        .await
        .map_err(|error| format!("failed to bootstrap IM database lifecycle: {error}"))?;
    let retention_scheduler =
        im_adapters_postgres_journal::spawn_retention_purge_scheduler_from_env();

    sdkwork_iam_database_host::bootstrap_iam_database_from_env()
        .await
        .map_err(|error| format!("failed to bootstrap IAM database lifecycle: {error}"))?;

    embedded_dependency_routes::bootstrap_embedded_dependency_databases()
        .await
        .map_err(|error| format!("failed to bootstrap embedded dependency databases: {error}"))?;

    sdkwork_im_web_bootstrap::shared_iam_web_request_context_resolver_from_env()
        .await;
    sdkwork_im_iam_application_bootstrap::ensure_im_tenant_application_runtime_from_env(
        gateway_config.environment.as_str(),
    )
    .await
    .map_err(|error| format!("failed to ensure IM IAM tenant application: {error}"))?;

    let iam_router = sdkwork_routes_iam_app_api::build_sdkwork_iam_app_api_router()
        .await
        .map_err(|error| format!("failed to build embedded IAM router: {error}"))?;

    let embedded_application =
        embedded_application_routes::bootstrap_embedded_application_routes().await;
    let embedded_dependencies = embedded_dependency_routes::bootstrap_embedded_dependency_routes().await;

    let web_config = WebGatewayConfig::from_env();
    let registry = build_gateway_registry()?;
    let product_runtime_router = build_gateway_product_runtime_router(base_url.as_str()).await?;
    let mut embedded_runtime =
        bootstrap_embedded_session_gateway_runtime(&web_config).await?;
    let session_router = embedded_runtime.session_router.take();
    let embedded_realtime_app_state = embedded_runtime.embedded_realtime_app_state.take();

    println!(
        "{}",
        format_startup_summary(&build_startup_summary_with_registry(
            &web_config,
            &registry,
            base_url.clone(),
        ))
    );

    let im_router = build_app_with_registry_product_runtime_and_embedded_services_from_env(
        web_config,
        registry,
        Some(product_runtime_router),
        session_router,
        embedded_realtime_app_state,
    )
    .await;

    // Dependency and IM assembly routes must win over cloud-gateway registry proxies.
    let application_router = embedded_dependencies
        .router
        .merge(embedded_application.router)
        .merge(im_router);
    // The IM cloud-gateway router already mounts /healthz, /livez, /readyz, and /metrics
    // through sdkwork-web-bootstrap. Do not mount infra routes again on the merged router.
    let app = iam_router
        .merge(application_router)
        .layer(build_cors_layer(&gateway_config))
        .layer(from_fn_with_state(
            RateLimiter::new(RateLimitConfig::from_env()),
            gateway_protection::rate_limit_middleware,
        ));

    println!("Assembling gateway router completed; binding {bind_addr}");
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    println!(
        "Listening on http://{} (healthz: http://{}/healthz)",
        display_listener_addr(bind_addr),
        display_listener_addr(bind_addr)
    );
    tracing::info!(
        target: "sdkwork.im",
        event = "im.standalone_gateway.listen",
        service = %gateway_config.service_name,
        environment = %gateway_config.environment,
        bind = %bind_addr,
        "listening"
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .with_graceful_shutdown(async move {
            shutdown_signal().await;
            if let Some(handle) = retention_scheduler {
                handle.shutdown();
            }
            embedded_runtime.shutdown().await;
        })
        .await?;
    Ok(())
}

/// Apply gateway process environment defaults.
///
/// # Safety
///
/// This function must only be called from the main thread before any other
/// threads (including the Tokio runtime) are spawned. The caller (fn main)
/// guarantees this by invoking it before `tokio::runtime::Builder::build`.
fn apply_gateway_process_environment(config: &ResolvedGatewayConfig) {
    if std::env::var("SDKWORK_IM_ENVIRONMENT")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        // SAFETY: Called from fn main() before the Tokio runtime is created.
        // No other threads exist at this point.
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", config.environment.as_str());
        }
    }
    if std::env::var("SDKWORK_ENV")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        let normalized_environment = config.environment.trim().to_ascii_lowercase();
        let sdkwork_env = match normalized_environment.as_str() {
            "dev" | "development" => "development",
            "test" | "testing" => "test",
            "prod" | "production" => "production",
            _ => normalized_environment.as_str(),
        };
        // SAFETY: Called from fn main() before the Tokio runtime is created.
        unsafe {
            std::env::set_var("SDKWORK_ENV", sdkwork_env);
        }
    }
}

/// Apply collapsed standalone URL environment overrides.
///
/// # Safety
///
/// This function must only be called from the main thread before any other
/// threads (including the Tokio runtime) are spawned. See
/// `apply_gateway_process_environment` for the safety contract.
fn apply_collapsed_standalone_urls(base_url: &str, bind_addr: &SocketAddr) {
    let bind = format!("{}:{}", bind_addr.ip(), bind_addr.port());
    let websocket_url = format!("ws://{}", display_listener_addr(*bind_addr));
    for (key, value) in [
        ("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND", bind.as_str()),
        ("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL", base_url),
        ("SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL", websocket_url.as_str()),
        ("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL", base_url),
        ("SDKWORK_API_CLOUD_GATEWAY_BASE_URL", base_url),
        ("SDKWORK_API_CLOUD_GATEWAY_BIND", bind.as_str()),
    ] {
        // SAFETY: Called from fn main() before the Tokio runtime is created.
        unsafe {
            std::env::set_var(key, value);
        }
    }
}

async fn build_gateway_product_runtime_router(base_url: &str) -> Result<Router, String> {
    let (_loader, mut standalone_config) =
        StandaloneConfigLoader::from_env().map_err(|error| error.to_string())?;
    standalone_config.portal_api_base_url = base_url.trim_end_matches('/').to_owned();

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    let site_dirs = resolve_product_site_dirs_from_env(&repo_root);
    build_product_runtime_router(
        standalone_config,
        RouterProductRuntimeOptions::desktop(site_dirs),
    )
    .await
    .map_err(|error| error.to_string())
}

fn build_cors_layer(config: &ResolvedGatewayConfig) -> CorsLayer {
    if config.allow_any_origin {
        return CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
    }

    let mut layer = CorsLayer::new().allow_methods(Any).allow_headers(Any);
    for origin in &config.allowed_origins {
        if let Ok(parsed) = origin.parse::<axum::http::HeaderValue>() {
            layer = layer.allow_origin(parsed);
        }
    }
    layer
}

fn display_listener_addr(addr: SocketAddr) -> String {
    let host = match addr.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".to_owned(),
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) if ip.is_unspecified() => "[::1]".to_owned(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    };
    format!("{host}:{}", addr.port())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if tokio::signal::ctrl_c().await.is_err() {
            tracing::warn!("failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => {
                tracing::warn!(error = ?error, "failed to install terminate signal handler");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use super::build_cors_layer;
    use super::ResolvedGatewayConfig;
    use web_gateway::build_app_with_registry_product_runtime_and_embedded_services_from_env;
    use axum::{
        Router,
        extract::ws::{Message, WebSocket, WebSocketUpgrade},
        response::IntoResponse,
        routing::get,
    };
    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio_tungstenite::{
        connect_async,
        tungstenite::{Message as TungsteniteMessage, client::ClientRequestBuilder},
    };

    async fn websocket_echo(ws: WebSocketUpgrade) -> impl IntoResponse {
        ws.protocols(["sdkwork-im.ccp.ws.v1"])
            .on_upgrade(handle_echo_socket)
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
                Message::Close(frame) => {
                    let _ = socket.send(Message::Close(frame)).await;
                    break;
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
            }
        }
    }

    fn test_gateway_config() -> ResolvedGatewayConfig {
        ResolvedGatewayConfig {
            service_name: "sdkwork-im-standalone-gateway".to_owned(),
            environment: "development".to_owned(),
            bind: "127.0.0.1:0".to_owned(),
            allow_any_origin: true,
            allowed_origins: Vec::new(),
        }
    }

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

    #[tokio::test]
    async fn standalone_merge_and_cors_preserve_websocket_upgrade_state() {
        let im_router =
            Router::new().route("/im/v3/api/realtime/ws", get(websocket_echo));
        let iam_router = Router::new().route(
            "/app/v3/api/auth/ping",
            get(|| async { "ok" }),
        );
        let app = iam_router
            .merge(im_router)
            .layer(build_cors_layer(&test_gateway_config()));
        let (address, handle) = spawn_server(app).await;

        let request = ClientRequestBuilder::new(
            format!("ws://{address}/im/v3/api/realtime/ws")
                .parse()
                .unwrap(),
        )
        .with_sub_protocol("sdkwork-im.ccp.ws.v1");

        let (mut socket, response) = connect_async(request)
            .await
            .expect("standalone websocket handshake should succeed");
        assert_eq!(response.status(), 101);

        socket
            .send(TungsteniteMessage::Text("hello standalone".into()))
            .await
            .expect("client frame should send");
        let echoed = socket
            .next()
            .await
            .expect("echo frame should arrive")
            .expect("echo frame should decode");
        assert_eq!(echoed, TungsteniteMessage::Text("hello standalone".into()));

        let _ = socket.close(None).await;
        handle.abort();
    }

    #[tokio::test]
    async fn standalone_real_iam_router_merge_preserves_websocket_upgrade_state() {
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
            std::env::set_var("SDKWORK_ENV", "test");
        }
        let im_router =
            Router::new().route("/im/v3/api/realtime/ws", get(websocket_echo));
        let iam_router = sdkwork_routes_iam_app_api::build_sdkwork_iam_app_api_router()
            .await
            .expect("iam router should build");
        let app = iam_router
            .merge(im_router)
            .layer(build_cors_layer(&test_gateway_config()));
        let (address, handle) = spawn_server(app).await;

        let request = ClientRequestBuilder::new(
            format!("ws://{address}/im/v3/api/realtime/ws")
                .parse()
                .unwrap(),
        )
        .with_sub_protocol("sdkwork-im.ccp.ws.v1");

        let connect_result = connect_async(request).await;
        handle.abort();
        connect_result.expect("real iam router merge should keep websocket handshake working");
    }

    #[tokio::test]
    async fn standalone_real_gateway_and_iam_assembly_preserves_websocket_upgrade_state() {
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
            std::env::set_var("SDKWORK_ENV", "test");
        }
        let iam_router = sdkwork_routes_iam_app_api::build_sdkwork_iam_app_api_router()
            .await
            .expect("iam router should build");
        let bootstrap = session_gateway::RealtimePlaneBootstrap {
            assembly: session_gateway::RealtimePlaneAssembly::default(),
            node_id: "node_embedded_ws".to_owned(),
            cluster_bus: None,
            iam_auth_pool: None,
        };
        let embedded_router =
            sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap(
                &bootstrap,
            );
        let embedded_app_state =
            session_gateway::AppState::from_realtime_bootstrap(&bootstrap);
        let im_router = build_app_with_registry_product_runtime_and_embedded_services_from_env(
            web_gateway_config(),
            web_gateway::build_gateway_registry().expect("gateway registry should build"),
            Some(Router::new()),
            Some(embedded_router),
            Some(embedded_app_state),
        )
        .await;
        let app = iam_router
            .merge(im_router)
            .layer(build_cors_layer(&test_gateway_config()));
        let (address, handle) = spawn_server(app).await;

        let request = ClientRequestBuilder::new(
            format!("ws://{address}/im/v3/api/realtime/ws?deviceId=test")
                .parse()
                .unwrap(),
        )
        .with_sub_protocol("sdkwork-im.ccp.ws.v1");

        let connect_result = connect_async(request).await;
        handle.abort();
        connect_result.expect("full standalone assembly should keep websocket handshake working");
    }

    #[tokio::test]
    async fn standalone_embedded_realtime_plane_preserves_websocket_upgrade_state() {
        unsafe {
            std::env::set_var("SDKWORK_IM_ENVIRONMENT", "test");
            std::env::set_var("SDKWORK_ENV", "test");
        }
        let bootstrap = session_gateway::RealtimePlaneBootstrap {
            assembly: session_gateway::RealtimePlaneAssembly::default(),
            node_id: "node_embedded_ws".to_owned(),
            cluster_bus: None,
            iam_auth_pool: None,
        };
        let embedded_router =
            sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap(
                &bootstrap,
            );
        let embedded_app_state =
            session_gateway::AppState::from_realtime_bootstrap(&bootstrap);
        let iam_router = sdkwork_routes_iam_app_api::build_sdkwork_iam_app_api_router()
            .await
            .expect("iam router should build");
        let im_router = build_app_with_registry_product_runtime_and_embedded_services_from_env(
            web_gateway_config(),
            web_gateway::build_gateway_registry().expect("gateway registry should build"),
            Some(Router::new()),
            Some(embedded_router),
            Some(embedded_app_state),
        )
        .await;
        let app = iam_router
            .merge(im_router)
            .layer(build_cors_layer(&test_gateway_config()));
        let (address, handle) = spawn_server(app).await;

        let request = ClientRequestBuilder::new(
            format!("ws://{address}/im/v3/api/realtime/ws?deviceId=test")
                .parse()
                .unwrap(),
        )
        .with_sub_protocol("sdkwork-im.ccp.ws.v1");

        let connect_result = connect_async(request).await;
        handle.abort();
        let (_, response) = connect_result
            .expect("embedded realtime plane must preserve websocket upgrade in unified gateway");
        assert_eq!(response.status(), 101);
    }

    fn web_gateway_config() -> sdkwork_im_cloud_gateway_config::WebGatewayConfig {
        sdkwork_im_cloud_gateway_config::WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: sdkwork_im_cloud_gateway_config::GatewayRuntimeMode::Unified,
            strict_startup: true,
            upstreams: Vec::new(),
        }
    }
}
