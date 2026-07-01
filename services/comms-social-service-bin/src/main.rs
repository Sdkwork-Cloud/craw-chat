use std::process::ExitCode;
use std::sync::Arc;

use social_service::SocialRuntime;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_COMMS_SOCIAL_SERVICE_BIND_ADDR";
const LEGACY_BIND_ADDR_ENV: &str = "SDKWORK_IM_SOCIAL_SERVICE_BIND_ADDR";
const RUNTIME_DIR_ENV: &str = "SDKWORK_IM_RUNTIME_DIR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18092";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("comms-social-service");
    sdkwork_im_service_readiness::init_im_service_tracing_from_env();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    sdkwork_im_service_readiness::bootstrap_im_service_database_from_env().await?;
    let bind_addr = std::env::var(BIND_ADDR_ENV)
        .or_else(|_| std::env::var(LEGACY_BIND_ADDR_ENV))
        .unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("comms-social-service failed to bind {bind_addr}: {error}"))?;

    let social_runtime = build_social_runtime()?;
    let _shared_channel_sync_scheduler =
        social_service::spawn_shared_channel_sync_stale_reclaim_scheduler_from_env(
            social_runtime.clone(),
        );
    // Initialize database-backed Snowflake node_id allocation for all
    // social-service ID generators (open-api + contact open-api).
    social_service::init_id_generators().await;
    let postgres_state = social_service::try_postgres_app_state_from_database_url_env().await;
    let mut app =
        sdkwork_routes_im_social_backend_api::build_control_public_app(social_runtime.clone());
    app = app.merge(sdkwork_routes_im_social_open_api::build_runtime_public_app(
        social_runtime.clone(),
    ));
    app = app.merge(social_service::build_app(social_runtime));
    if let Some(state) = postgres_state {
        app = app.merge(sdkwork_routes_im_social_open_api::build_supplemental_public_app(state));
    }

    tracing::info!(
        "comms-social-service listening on {}",
        listener.local_addr().map_err(|error| error.to_string())?
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            sdkwork_im_service_readiness::shutdown_signal().await;
        })
        .await
        .map_err(|error| format!("comms-social-service server should run: {error}"))?;
    Ok(())
}

fn build_social_runtime() -> Result<Arc<SocialRuntime>, String> {
    match std::env::var(RUNTIME_DIR_ENV) {
        Ok(runtime_dir) if !runtime_dir.trim().is_empty() => Ok(Arc::new(
            SocialRuntime::from_runtime_dir(runtime_dir.as_str()),
        )),
        _ => Ok(Arc::new(SocialRuntime::default())),
    }
}
