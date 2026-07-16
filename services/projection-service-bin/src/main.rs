use std::process::ExitCode;
use std::sync::Arc;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_PROJECTION_SERVICE_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18083";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("projection-service");
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
    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("projection-service failed to bind local listener: {error}"))?;

    let runtime = Arc::new(projection_service::build_projection_runtime_from_env()?);
    let projection_journal_consumer =
        projection_service::spawn_projection_journal_consumer_from_env(runtime.clone());
    let app =
        sdkwork_routes_im_projection_open_api::build_public_app_with_runtime(runtime.clone()).await;

    tracing::info!(
        "projection-service starting on {}",
        listener.local_addr().map_err(|e| e.to_string())?
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(sdkwork_im_service_readiness::shutdown_signal())
        .await
        .map_err(|error| format!("projection-service server should run: {error}"))?;
    if let Some(handle) = projection_journal_consumer {
        handle.shutdown();
    }
    Ok(())
}
