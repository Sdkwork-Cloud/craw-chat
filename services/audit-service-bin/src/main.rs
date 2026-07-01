use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_AUDIT_SERVICE_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18089";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("audit-service");
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
        .map_err(|error| format!("audit-service failed to bind local listener: {error}"))?;

    axum::serve(listener, sdkwork_routes_im_audit_backend_api::build_public_app())
        .with_graceful_shutdown(async {
            sdkwork_im_service_readiness::shutdown_signal().await;
        })
        .await
        .map_err(|error| format!("audit-service server should run: {error}"))?;
    Ok(())
}
