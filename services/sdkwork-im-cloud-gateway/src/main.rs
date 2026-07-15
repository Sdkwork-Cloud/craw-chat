use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    RouterProductRuntimeOptions, build_product_runtime_router, resolve_product_site_dirs_from_env,
};
use sdkwork_im_cloud_gateway_config::{WebGatewayConfig, should_embed_session_gateway};
use sdkwork_im_cloud_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("sdkwork-im-cloud-gateway");
    sdkwork_im_service_readiness::init_im_service_tracing_from_env();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("{error}");
            ExitCode::FAILURE
        }
    }
}

enum StartupMode {
    Run(WebGatewayConfig),
    ExitSuccess,
}

struct EmbeddedStartup {
    runtime: web_gateway::EmbeddedSessionGatewayRuntime,
    retention_scheduler: Option<im_adapters_postgres_journal::RetentionPurgeSchedulerHandle>,
    realtime_relays: EmbeddedRealtimeRelayHandles,
    _application_assembly: sdkwork_im_gateway_assembly::ApplicationAssembly,
}

#[derive(Default)]
struct EmbeddedRealtimeRelayHandles {
    rtc: Option<sdkwork_im_gateway_assembly::RtcOutboxRelayHandle>,
    conversation: Option<sdkwork_im_gateway_assembly::ConversationOutboxRelayHandle>,
    social: Option<sdkwork_im_gateway_assembly::SocialOutboxRelayHandle>,
}

impl EmbeddedRealtimeRelayHandles {
    fn shutdown(self) {
        if let Some(handle) = self.rtc {
            handle.shutdown();
        }
        if let Some(handle) = self.conversation {
            handle.shutdown();
        }
        if let Some(handle) = self.social {
            handle.shutdown();
        }
    }
}

impl EmbeddedStartup {
    async fn shutdown(self) {
        if let Some(handle) = self.retention_scheduler {
            handle.shutdown();
        }
        self.realtime_relays.shutdown();
        self.runtime.shutdown().await;
    }
}

async fn run() -> Result<(), String> {
    let StartupMode::Run(config) = resolve_startup_mode()? else {
        return Ok(());
    };
    let base_url = resolve_gateway_base_url(&config)?;
    let ((app, startup_summary, embedded_runtime), listener) =
        sdkwork_im_service_readiness::complete_preflight_then_bind_tcp_listener(
            config.bind_addr.as_str(),
            "sdkwork-im-cloud-gateway",
            async {
                let registry = web_gateway::build_gateway_registry()?;
                sdkwork_im_service_readiness::bootstrap_im_service_database_from_env()
                    .await
                    .map_err(|error| {
                        format!("failed to bootstrap IM process database pools: {error}")
                    })?;
                let embedded_application = sdkwork_im_gateway_assembly::assemble_application_router()
                    .await
                    .map_err(|error| format!("failed to assemble IM application router: {error}"))?;
                let product_runtime_router =
                    build_gateway_product_runtime_router(base_url.as_str()).await?;
                let runtime_fallback_router = embedded_application
                    .router
                    .clone()
                    .merge(product_runtime_router);
                let mut embedded_runtime = if should_embed_session_gateway(&config) {
                    sdkwork_iam_database_host::bootstrap_iam_database_from_env()
                        .await
                        .map_err(|error| {
                            format!("failed to bootstrap IAM database lifecycle: {error}")
                        })?;
                    sdkwork_im_web_bootstrap::shared_iam_web_request_context_resolver_from_env()
                        .await;
                    let iam_environment =
                        match im_app_context::resolve_web_environment_from_process_env() {
                            sdkwork_web_core::WebEnvironment::Dev => "development",
                            sdkwork_web_core::WebEnvironment::Test => "test",
                            sdkwork_web_core::WebEnvironment::Prod => "production",
                        };
                    sdkwork_im_iam_application_bootstrap::ensure_im_tenant_application_runtime_from_env(
                        iam_environment,
                    )
                    .await
                    .map_err(|error| {
                        format!("failed to ensure IM IAM tenant application: {error}")
                    })?;
                    let retention_scheduler =
                        im_adapters_postgres_journal::spawn_retention_purge_scheduler_from_env();
                    let embedded =
                        web_gateway::bootstrap_embedded_session_gateway_runtime(&config).await?;
                    EmbeddedStartup {
                        runtime: embedded,
                        retention_scheduler,
                        realtime_relays: EmbeddedRealtimeRelayHandles::default(),
                        _application_assembly: embedded_application,
                    }
                } else {
                    EmbeddedStartup {
                        runtime: web_gateway::EmbeddedSessionGatewayRuntime::empty(),
                        retention_scheduler: None,
                        realtime_relays: EmbeddedRealtimeRelayHandles::default(),
                        _application_assembly: embedded_application,
                    }
                };
                if let Some(session_state) = embedded_runtime
                    .runtime
                    .embedded_realtime_app_state
                    .as_ref()
                {
                    embedded_runtime.realtime_relays = wire_embedded_realtime_plane(
                        session_state,
                        &embedded_runtime._application_assembly,
                    );
                }
                let session_router = embedded_runtime.runtime.session_router.take();
                let embedded_realtime_app_state =
                    embedded_runtime.runtime.embedded_realtime_app_state.take();
                let startup_summary = format_startup_summary(&build_startup_summary_with_registry(
                    &config,
                    &registry,
                    base_url.clone(),
                ));
                let app = web_gateway::build_app_with_registry_product_runtime_and_embedded_services_from_env(
                    config.clone(),
                    registry,
                    Some(runtime_fallback_router),
                    session_router,
                    embedded_realtime_app_state,
                )
                .await;
                Ok((app, startup_summary, embedded_runtime))
            },
        )
        .await?;
    println!("{startup_summary}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        sdkwork_im_service_readiness::shutdown_signal().await;
        embedded_runtime.shutdown().await;
    })
    .await
    .map_err(|error| format!("sdkwork-im-cloud-gateway server should run: {error}"))?;
    Ok(())
}

fn wire_embedded_realtime_plane(
    session_state: &session_gateway::AppState,
    application_assembly: &sdkwork_im_gateway_assembly::ApplicationAssembly,
) -> EmbeddedRealtimeRelayHandles {
    let realtime_runtime = session_state.realtime_runtime();
    conversation_runtime::register_embedded_realtime_publisher(realtime_runtime.clone());
    sdkwork_im_gateway_assembly::wire_social_runtime_embedded_plane(
        &application_assembly.social_runtime,
        realtime_runtime.clone(),
        conversation_runtime::resolve_embedded_conversation_runtime(),
    );
    EmbeddedRealtimeRelayHandles {
        rtc: sdkwork_im_gateway_assembly::spawn_rtc_outbox_relay_from_env(realtime_runtime.clone()),
        conversation: sdkwork_im_gateway_assembly::spawn_conversation_outbox_relay_from_env(
            realtime_runtime.clone(),
        ),
        social: sdkwork_im_gateway_assembly::spawn_social_outbox_relay_from_env(realtime_runtime),
    }
}

async fn build_gateway_product_runtime_router(base_url: &str) -> Result<axum::Router, String> {
    let (_loader, mut standalone_config) =
        StandaloneConfigLoader::from_env().map_err(|error| error.to_string())?;

    if !has_explicit_portal_api_base_url() {
        standalone_config.portal_api_base_url = base_url.trim_end_matches('/').to_owned();
    }

    let site_dirs = resolve_product_site_dirs_from_env(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(".."),
    );
    build_product_runtime_router(
        standalone_config,
        RouterProductRuntimeOptions::desktop(site_dirs),
    )
    .await
    .map_err(|error| error.to_string())
}

fn resolve_gateway_base_url(config: &WebGatewayConfig) -> Result<String, String> {
    if has_explicit_portal_api_base_url() {
        let (_loader, standalone_config) =
            StandaloneConfigLoader::from_env().map_err(|error| error.to_string())?;
        return Ok(standalone_config.portal_api_base_url);
    }

    let bind_addr = config.bind_addr.parse::<SocketAddr>().map_err(|error| {
        format!(
            "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL is required when cloud gateway bind address `{}` is not a socket address: {error}",
            config.bind_addr
        )
    })?;
    if bind_addr.port() == 0 {
        return Err(
            "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL is required when cloud gateway bind port is 0"
                .to_owned(),
        );
    }
    Ok(format!("http://{}", display_listener_addr(bind_addr)))
}

fn has_explicit_portal_api_base_url() -> bool {
    [
        "SDKWORK_IM_PORTAL_API_BASE_URL",
        "SDKWORK_PORTAL_API_BASE_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
    ]
    .iter()
    .any(|env_name| {
        std::env::var(env_name)
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    })
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

fn resolve_startup_mode() -> Result<StartupMode, String> {
    let mut args = std::env::args().skip(1);
    let mut config_path: Option<PathBuf> = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--config" => {
                let Some(path) = args.next() else {
                    return Err("missing value for --config".to_owned());
                };
                config_path = Some(PathBuf::from(path));
            }
            "-h" | "--help" => {
                println!("Usage: sdkwork-im-server [--config <chat.toml>]");
                println!(
                    "Start the Sdkwork IM unified gateway using env defaults or a chat.toml file."
                );
                return Ok(StartupMode::ExitSuccess);
            }
            unknown => {
                return Err(format!("unsupported argument: {unknown}"));
            }
        }
    }

    let config = match config_path {
        Some(path) => WebGatewayConfig::from_server_config_file(path)?,
        None => WebGatewayConfig::from_env(),
    };
    Ok(StartupMode::Run(config))
}

#[cfg(test)]
mod tests {
    use super::{has_explicit_portal_api_base_url, resolve_gateway_base_url};
    use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig};
    use std::sync::OnceLock;
    use tokio::sync::{Mutex, MutexGuard};

    fn global_env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn env_guard() -> MutexGuard<'static, ()> {
        global_env_guard().blocking_lock()
    }

    async fn env_guard_async() -> MutexGuard<'static, ()> {
        global_env_guard().lock().await
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

        fn remove(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => unsafe {
                    std::env::set_var(self.name, value);
                },
                None => unsafe {
                    std::env::remove_var(self.name);
                },
            }
        }
    }

    #[test]
    fn application_public_http_url_env_counts_as_explicit_portal_binding() {
        let _guard = env_guard();
        let _portal = ScopedEnvVar::remove("SDKWORK_IM_PORTAL_API_BASE_URL");
        let _sdkwork_portal = ScopedEnvVar::remove("SDKWORK_PORTAL_API_BASE_URL");
        let _application_public_http = ScopedEnvVar::set(
            "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
            "https://im.sdkwork.com",
        );
        let _bind = ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND");

        assert!(has_explicit_portal_api_base_url());
    }

    #[test]
    fn gateway_base_url_uses_configured_nonzero_bind_without_claiming_a_listener() {
        let _guard = env_guard();
        let _portal = ScopedEnvVar::remove("SDKWORK_IM_PORTAL_API_BASE_URL");
        let _sdkwork_portal = ScopedEnvVar::remove("SDKWORK_PORTAL_API_BASE_URL");
        let _application_public_http =
            ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL");
        let _bind = ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND");
        let config = WebGatewayConfig {
            bind_addr: "0.0.0.0:18079".to_owned(),
            runtime_mode: GatewayRuntimeMode::SingleIngress,
            strict_startup: false,
            upstreams: Vec::new(),
        };

        assert_eq!(
            resolve_gateway_base_url(&config).expect("configured bind should derive base url"),
            "http://127.0.0.1:18079"
        );
    }

    #[test]
    fn gateway_base_url_rejects_ephemeral_bind_without_explicit_public_url() {
        let _guard = env_guard();
        let _portal = ScopedEnvVar::remove("SDKWORK_IM_PORTAL_API_BASE_URL");
        let _sdkwork_portal = ScopedEnvVar::remove("SDKWORK_PORTAL_API_BASE_URL");
        let _application_public_http =
            ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL");
        let _bind = ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND");
        let config = WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::SingleIngress,
            strict_startup: false,
            upstreams: Vec::new(),
        };

        let error = resolve_gateway_base_url(&config)
            .expect_err("ephemeral bind needs an explicit browser-reachable URL");
        assert!(error.contains("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL"));
    }

    #[tokio::test]
    async fn missing_group_knowledgebase_rpc_client_preflight_does_not_bind_gateway_listener() {
        let _guard = env_guard_async().await;
        let _environment = ScopedEnvVar::set("SDKWORK_IM_ENVIRONMENT", "production");
        let _allow_all_principals = ScopedEnvVar::remove("SDKWORK_IM_ALLOW_ALL_PRINCIPALS");
        let _knowledgebase_rpc_endpoint =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_ENDPOINT");
        let _knowledgebase_rpc_ca =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CA_CERT_PATH");
        let _knowledgebase_rpc_certificate =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CERT_PATH");
        let _knowledgebase_rpc_key =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_KEY_PATH");
        let _knowledgebase_rpc_tls_domain =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_TLS_DOMAIN");
        let _knowledgebase_rpc_signing_key =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY");
        let _knowledgebase_rpc_signing_key_file = ScopedEnvVar::remove(
            "SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CALLER_CONTEXT_SIGNING_KEY_FILE",
        );
        let _knowledgebase_rpc_credential_ttl =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_CREDENTIAL_TTL_SECONDS");
        let _knowledgebase_rpc_timeout =
            ScopedEnvVar::remove("SDKWORK_IM_KNOWLEDGEBASE_RPC_CLIENT_TIMEOUT_MS");
        let catalog_path = std::env::temp_dir().join(format!(
            "sdkwork-im-group-knowledgebase-preflight-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock should be after the Unix epoch")
                .as_nanos(),
        ));
        std::fs::write(catalog_path.as_path(), r#"{"principals":[]}"#)
            .expect("write a temporary production principal directory catalog");
        let catalog_path_env = catalog_path
            .to_str()
            .expect("temporary catalog path should be valid Unicode");
        let _catalog = ScopedEnvVar::set(
            "SDKWORK_IM_PRINCIPAL_DIRECTORY_CATALOG_PATH",
            catalog_path_env,
        );
        let reservation = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("reserve a gateway listener address for the preflight regression");
        let bind_addr = reservation
            .local_addr()
            .expect("resolve the reserved gateway listener address")
            .to_string();
        drop(reservation);

        let result = sdkwork_im_service_readiness::complete_preflight_then_bind_tcp_listener(
            bind_addr.as_str(),
            "sdkwork-im-cloud-gateway",
            async {
                sdkwork_im_gateway_assembly::assemble_application_router()
                    .await
                    .map(|_| ())
                    .map_err(|error| format!("failed to assemble IM application router: {error}"))
            },
        )
        .await;
        let _ = std::fs::remove_file(catalog_path.as_path());

        let error = match result {
            Ok(_) => panic!("missing group knowledgebase RPC client must fail preflight"),
            Err(error) => error,
        };
        assert!(error.contains("Knowledgebase RPC client is not configured"));

        let rebound = tokio::net::TcpListener::bind(bind_addr.as_str())
            .await
            .expect(
                "a failed group knowledgebase RPC preflight must leave the gateway port unbound",
            );
        drop(rebound);
    }
}
