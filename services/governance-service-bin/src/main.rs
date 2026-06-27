use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_GOVERNANCE_SERVICE_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18081";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("governance-service");
    sdkwork_im_service_readiness::init_im_service_tracing_from_env();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            tracing::error!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    if let Some(command) = args.next() {
        if command == "print-openapi" {
            let body = serde_json::to_string_pretty(&governance_service::render_openapi_document())
                .map_err(|error| format!("failed to serialize control-plane OpenAPI: {error}"))?;
            println!("{body}");
            return Ok(());
        }

        return Err(format!("Unknown command for governance-service: {command}"));
    }

    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("governance-service failed to bind local listener: {error}"))?;

    let app = sdkwork_routes_im_governance_backend_api::build_public_app();

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            sdkwork_im_service_readiness::shutdown_signal().await;
        })
        .await
        .map_err(|error| format!("governance-service server should run: {error}"))?;
    Ok(())
}
