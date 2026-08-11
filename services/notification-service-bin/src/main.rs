use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_NOTIFICATION_SERVICE_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18087";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::enable_process_shared_database_pool();
    sdkwork_im_service_readiness::ensure_im_service_process_identity("notification-service");
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
    let runtime = notification_service::build_runtime_from_env()?;
    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("notification-service failed to bind local listener: {error}"))?;

    // Background delivery worker: claims `requested` notification tasks and
    // dispatches them into the recipients' realtime event windows.
    let delivery_worker =
        notification_service::bootstrap::spawn_notification_delivery_worker_from_env(
            runtime.clone(),
        );

    let server = axum::serve(
        listener,
        sdkwork_routes_im_notification_app_api::build_public_app_with_runtime(runtime),
    )
    .with_graceful_shutdown(async {
        sdkwork_im_service_readiness::shutdown_signal().await;
    })
    .await
    .map_err(|error| format!("notification-service server should run: {error}"));

    delivery_worker.abort();
    server
}
