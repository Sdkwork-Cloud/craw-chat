use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_COMMS_SPACE_SERVICE_BIND_ADDR";
const LEGACY_BIND_ADDR_ENV: &str = "SDKWORK_IM_SPACE_SERVICE_BIND_ADDR";
const DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18093";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("space-service");
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
    let state = space_service::try_app_state_from_database_url_env()
        .await
        .ok_or_else(|| format!("{DATABASE_URL_ENV} is required for comms-space-service"))?;
    let app = sdkwork_routes_im_space_open_api::build_public_app(state);

    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("comms-space-service failed to bind {bind_addr}: {error}"))?;
    tracing::info!(
        "comms-space-service listening on {}",
        listener.local_addr().map_err(|error| error.to_string())?
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            sdkwork_im_service_readiness::shutdown_signal().await;
        })
        .await
        .map_err(|error| format!("comms-space-service server should run: {error}"))?;
    Ok(())
}
