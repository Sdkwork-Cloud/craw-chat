use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    resolve_product_site_dirs_from_env, RouterProductRuntimeOptions, build_product_runtime_router,
};
use sdkwork_im_cloud_gateway_config::{should_embed_session_gateway, WebGatewayConfig};
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
}

async fn run() -> Result<(), String> {
    let StartupMode::Run(config) = resolve_startup_mode()? else {
        return Ok(());
    };
    let listener = tokio::net::TcpListener::bind(config.bind_addr.as_str())
        .await
        .map_err(|error| format!("sdkwork-im-cloud-gateway failed to bind listener: {error}"))?;
    let local_addr = listener
        .local_addr()
        .map_err(|error| format!("sdkwork-im-cloud-gateway failed to resolve listener addr: {error}"))?;
    let base_url = format!("http://{}", display_listener_addr(local_addr));
    let registry = web_gateway::build_gateway_registry()?;
    let product_runtime_router = build_gateway_product_runtime_router(base_url.as_str()).await?;
    let mut embedded_runtime = if should_embed_session_gateway(&config) {
        sdkwork_im_database_pool::bootstrap_im_database_from_env()
            .await
            .map_err(|error| format!("failed to bootstrap IM database lifecycle: {error}"))?;
        sdkwork_iam_database_host::bootstrap_iam_database_from_env()
            .await
            .map_err(|error| format!("failed to bootstrap IAM database lifecycle: {error}"))?;
        sdkwork_im_web_bootstrap::shared_iam_web_request_context_resolver_from_env()
            .await;
        let iam_environment = match im_app_context::resolve_web_environment_from_process_env() {
            sdkwork_web_core::WebEnvironment::Dev => "development",
            sdkwork_web_core::WebEnvironment::Test => "test",
            sdkwork_web_core::WebEnvironment::Prod => "production",
        };
        sdkwork_im_iam_application_bootstrap::ensure_im_tenant_application_runtime_from_env(
            iam_environment,
        )
        .await
        .map_err(|error| format!("failed to ensure IM IAM tenant application: {error}"))?;
        let retention_scheduler =
            im_adapters_postgres_journal::spawn_retention_purge_scheduler_from_env();
        let embedded =
            web_gateway::bootstrap_embedded_session_gateway_runtime(&config).await?;
        EmbeddedStartup {
            runtime: embedded,
            retention_scheduler,
        }
    } else {
        EmbeddedStartup {
            runtime: web_gateway::EmbeddedSessionGatewayRuntime::empty(),
            retention_scheduler: None,
        }
    };
    let session_router = embedded_runtime.runtime.session_router.take();
    let embedded_realtime_app_state = embedded_runtime.runtime.embedded_realtime_app_state.take();
    println!(
        "{}",
        format_startup_summary(&build_startup_summary_with_registry(
            &config,
            &registry,
            base_url.clone(),
        ))
    );

    axum::serve(
        listener,
        web_gateway::build_app_with_registry_product_runtime_and_embedded_services_from_env(
            config,
            registry,
            Some(product_runtime_router),
            session_router,
            embedded_realtime_app_state,
        )
        .await
        .into_make_service_with_connect_info::<SocketAddr>(),
    )
.with_graceful_shutdown(async move {
sdkwork_im_service_readiness::shutdown_signal().await;
if let Some(handle) = embedded_runtime.retention_scheduler {
handle.shutdown();
}
embedded_runtime.runtime.shutdown().await;
})
    .await
    .map_err(|error| format!("sdkwork-im-cloud-gateway server should run: {error}"))?;
    Ok(())
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

fn has_explicit_portal_api_base_url() -> bool {
    [
        "SDKWORK_IM_PORTAL_API_BASE_URL",
        "SDKWORK_PORTAL_API_BASE_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
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
    use super::has_explicit_portal_api_base_url;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("web gateway main env guard should not be poisoned")
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
}
