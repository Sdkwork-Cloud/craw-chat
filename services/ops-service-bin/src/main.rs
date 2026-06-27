use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_OPS_SERVICE_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18091";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("ops-service");
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
    let retention_scheduler =
        im_adapters_postgres_journal::spawn_retention_purge_scheduler_from_env();
    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("ops-service failed to bind local listener: {error}"))?;

    axum::serve(listener, sdkwork_routes_im_ops_backend_api::build_public_app())
        .with_graceful_shutdown(async move {
            sdkwork_im_service_readiness::shutdown_signal().await;
            if let Some(handle) = retention_scheduler {
                handle.shutdown();
            }
        })
        .await
        .map_err(|error| format!("ops-service server should run: {error}"))?;
    Ok(())
}
