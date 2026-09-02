mod edge_ip_rate_limit;

use std::net::{IpAddr, SocketAddr};
use std::path::Path;

use axum::Router;
use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_im_assembly::{
    apply_embedded_dependency_env, bootstrap_embedded_dependency_databases,
    bootstrap_embedded_dependency_routes,
};
use sdkwork_api_product_runtime::{
    RouterProductRuntimeOptions, build_product_runtime_router, resolve_product_site_dir_from_env,
};
use sdkwork_im_web_bootstrap::im_service_context_profile;
use sdkwork_web_axum::CanonicalCorsLayer;
use sdkwork_web_bootstrap::{ApiModuleRegistry, ComposedApiAssembly, WebFrameworkBuilder};

const DEFAULT_BIND: &str = "127.0.0.1:18079";

/// Distributed HTTP stores (rate limit, idempotency, concurrent admission)
/// backed by Redis when the shared gateway Redis configuration is present.
///
/// The framework production assembly gate refuses to boot with in-memory
/// stores in `WebEnvironment::Prod` (fail-closed), and the same env resolution
/// drives the WebSocket upgrade limiter so HTTP and realtime share one Redis
/// configuration. Without Redis, the in-memory defaults remain (single-replica
/// development and control-plane profiles).
fn wire_redis_http_stores(
    builder: WebFrameworkBuilder<sdkwork_iam_web_adapter::IamWebRequestContextResolver>,
) -> Result<WebFrameworkBuilder<sdkwork_iam_web_adapter::IamWebRequestContextResolver>, String> {
    let Some(redis_url) = im_adapters_redis_cache::resolve_gateway_rate_limit_redis_url_from_env()
    else {
        return Ok(builder);
    };
    let key_prefix = "sdkwork:im:gateway".to_owned();
    Ok(builder
        .rate_limit_store(
            sdkwork_web_store_redis::shared_rate_limit_store(
                redis_url.as_str(),
                key_prefix.clone(),
            )
            .map_err(|error| format!("failed to create Redis rate limit store: {error}"))?,
        )
        .idempotency_store(
            sdkwork_web_store_redis::shared_idempotency_store(
                redis_url.as_str(),
                key_prefix.clone(),
            )
            .map_err(|error| format!("failed to create Redis idempotency store: {error}"))?,
        )
        .concurrent_admission_store(
            sdkwork_web_store_redis::shared_concurrent_admission_store(
                redis_url.as_str(),
                key_prefix,
            )
            .map_err(|error| {
                format!("failed to create Redis concurrent admission store: {error}")
            })?,
        ))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sdkwork_im_service_readiness::enable_process_shared_database_pool();
    sdkwork_im_service_readiness::ensure_im_service_process_identity(
        "sdkwork-api-im-standalone-gateway",
    );
    sdkwork_im_service_readiness::init_im_service_tracing_from_env();

    let bind_address = resolve_bind_address()?;
    let base_url = format!("http://{}", display_listener_addr(bind_address));
    apply_standalone_process_environment(&base_url, bind_address);
    apply_embedded_dependency_env()
        .map_err(|error| format!("embedded dependency configuration failed: {error}"))?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(bind_address, base_url))
}

async fn async_main(
    bind_address: SocketAddr,
    base_url: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sdkwork_im_service_readiness::bootstrap_im_service_database_from_env()
        .await
        .map_err(|error| format!("failed to bootstrap IM database pools: {error}"))?;
    let retention_scheduler =
        im_adapters_postgres_journal::spawn_retention_purge_scheduler_from_env();
    sdkwork_im_web_bootstrap::shared_iam_web_request_context_resolver_from_env().await;
    let environment = resolve_environment();
    // IAM bootstrap runs through the IAM application assembly (API_ASSEMBLY_SPEC
    // §3/§6.1): it installs the IAM foundation schema and provisions the IAM
    // tenant application before the IM tenant provisioning below, preserving
    // the schema-before-provisioning invariant.
    let (iam_contribution, iam_host) =
        sdkwork_api_iam_assembly::bootstrap_iam_app_for_application()
            .await
            .map_err(|error| format!("failed to assemble IAM App API surface: {error}"))?;
    sdkwork_im_iam_application_bootstrap::ensure_im_tenant_application_runtime_from_env(
        environment.as_str(),
    )
    .await
    .map_err(|error| format!("failed to ensure IM IAM tenant application: {error}"))?;
    let iam_resolver = sdkwork_iam_web_adapter::IamWebRequestContextResolver::from_database_pool(
        Some(iam_host.pool().clone()),
    );

    let realtime_drain_timeout = session_gateway::resolve_session_gateway_drain_timeout()?;
    let realtime_plane = session_gateway::bootstrap_gateway_embedded_realtime_plane().await?;
    let realtime_state =
        session_gateway::AppState::from_realtime_bootstrap(&realtime_plane.bootstrap);

    bootstrap_embedded_dependency_databases()
        .await
        .map_err(|error| format!("synchronize embedded dependency databases failed: {error}"))?;

    let api_assembly = sdkwork_api_im_assembly::assemble_api_router_with_realtime_bootstrap(Some(
        &realtime_plane.bootstrap,
    ))
    .await?;
    let mut dependencies = bootstrap_embedded_dependency_routes()
        .await
        .map_err(|error| format!("failed to assemble dependency APIs: {error}"))?;
    let standalone_runtime = api_assembly
        .runtime
        .wire_standalone_runtime(&realtime_state, dependencies.agents_session_facade.clone())?;
    let product_runtime_router = build_gateway_product_runtime_router(base_url.as_str()).await?;

    let sdkwork_api_im_assembly::ApiAssembly {
        contribution: im_contribution,
        runtime: im_runtime,
    } = api_assembly;
    let mut contributions = vec![im_contribution, iam_contribution];
    contributions.append(&mut dependencies.contributions);
    let mut module_registry = ApiModuleRegistry::new();
    module_registry.add_modules(contributions);
    let composed = module_registry
        .try_compose("SDKWork IM Standalone API")
        .map_err(|error| format!("compose standalone API profile failed: {error}"))?;
    // The composed WebFramework defaults to a deny-all CORS policy; the
    // standalone profile reuses the canonical dev policy (loopback origins)
    // plus the explicitly configured browser origins.
    let security_policy = sdkwork_web_bootstrap::security_policy_for_environment(
        &sdkwork_web_bootstrap::web_environment_from_env(&["SDKWORK_IM_ENVIRONMENT"]),
        sdkwork_web_bootstrap::cors_allowed_origins_from_env(&["SDKWORK_CORS_ALLOWED_ORIGINS"]),
    );
    let hosted = composed.into_hosted(wire_redis_http_stores(
        WebFrameworkBuilder::new(iam_resolver)
            .profile(im_service_context_profile())
            .security_policy(security_policy),
    )?);
    let app = product_runtime_router
        .merge(hosted.router)
        .layer(build_cors_layer(environment.as_str()))
        .layer(axum::middleware::from_fn_with_state(
            edge_ip_rate_limit::EdgeIpRateLimiter::from_env(),
            edge_ip_rate_limit::edge_ip_rate_limit,
        ));

    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .map_err(|error| format!("failed to bind {bind_address}: {error}"))?;
    println!(
        "sdkwork-api-im-standalone-gateway listening on http://{}",
        display_listener_addr(bind_address)
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    if let Some(handle) = retention_scheduler {
        handle.shutdown();
    }
    standalone_runtime.shutdown().await;
    realtime_plane.shutdown(realtime_drain_timeout).await?;
    drop(im_runtime);
    Ok(())
}

async fn build_gateway_product_runtime_router(base_url: &str) -> Result<Router, String> {
    let (_loader, mut standalone_config) =
        StandaloneConfigLoader::from_env().map_err(|error| error.to_string())?;
    standalone_config.portal_api_base_url = base_url.trim_end_matches('/').to_owned();

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let site_dir =
        resolve_product_site_dir_from_env(&repo_root).map_err(|error| error.to_string())?;
    build_product_runtime_router(
        standalone_config,
        RouterProductRuntimeOptions::desktop_for_api_assembly_host(site_dir),
    )
    .await
    .map_err(|error| error.to_string())
}

fn build_cors_layer(environment: &str) -> CanonicalCorsLayer {
    let mut policy = if is_development_environment(environment) {
        sdkwork_web_core::CorsPolicy::development_private_network()
    } else {
        sdkwork_web_core::CorsPolicy::default()
    };
    if is_development_environment(environment) {
        policy.allowed_origins.extend([
            "tauri://localhost".to_owned(),
            "http://tauri.localhost".to_owned(),
            "https://tauri.localhost".to_owned(),
        ]);
    }
    if let Ok(origins) = std::env::var("SDKWORK_CORS_ALLOWED_ORIGINS") {
        for origin in origins
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let origin = origin.trim_end_matches('/').to_owned();
            if !policy.allowed_origins.contains(&origin) {
                policy.allowed_origins.push(origin);
            }
        }
    }
    sdkwork_web_axum::cors_layer_from_policy(policy)
}

fn resolve_bind_address() -> Result<SocketAddr, String> {
    std::env::var("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_BIND.to_owned())
        .parse()
        .map_err(|error| format!("invalid standalone gateway bind address: {error}"))
}

fn resolve_environment() -> String {
    // Fail closed: an unset environment variable must never downgrade the
    // gateway to the development security posture (CORS, origin checks, and
    // token validation all branch on this value). Production deployments
    // that omit SDKWORK_IM_ENVIRONMENT now get the production posture.
    std::env::var("SDKWORK_IM_ENVIRONMENT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "production".to_owned())
}

fn is_development_environment(environment: &str) -> bool {
    matches!(
        environment.trim().to_ascii_lowercase().as_str(),
        "dev" | "development" | "test" | "testing" | "local"
    )
}

fn display_listener_addr(address: SocketAddr) -> String {
    let host = match address.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".to_owned(),
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) if ip.is_unspecified() => "[::1]".to_owned(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    };
    format!("{host}:{}", address.port())
}

fn apply_standalone_process_environment(base_url: &str, bind_address: SocketAddr) {
    let environment = resolve_environment();
    let sdkwork_env = match environment.trim().to_ascii_lowercase().as_str() {
        "dev" | "development" => "development",
        "test" | "testing" => "test",
        "prod" | "production" => "production",
        _ => environment.as_str(),
    };
    let bind = bind_address.to_string();
    let websocket_url = format!("ws://{}", display_listener_addr(bind_address));
    for (key, value) in [
        ("SDKWORK_ENV", sdkwork_env),
        ("SDKWORK_IM_ENVIRONMENT", environment.as_str()),
        ("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND", bind.as_str()),
        ("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL", base_url),
        (
            "SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL",
            websocket_url.as_str(),
        ),
        ("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL", base_url),
        ("SDKWORK_IAM_APP_API_HOST_MOUNTED", "true"),
    ] {
        if std::env::var(key)
            .ok()
            .map(|current| current.trim().is_empty())
            .unwrap_or(true)
        {
            // SAFETY: main calls this before creating the Tokio runtime or any worker thread.
            unsafe { std::env::set_var(key, value) };
        }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::warn!(%error, "failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => tracing::warn!(%error, "failed to install terminate signal handler"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
