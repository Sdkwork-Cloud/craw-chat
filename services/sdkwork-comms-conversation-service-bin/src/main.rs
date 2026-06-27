use std::process::ExitCode;
use std::sync::Arc;

const PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV: &str = "SDKWORK_IM_PRINCIPAL_DIRECTORY_CATALOG_PATH";
const ALLOW_ALL_PRINCIPALS_ENV: &str = "SDKWORK_IM_ALLOW_ALL_PRINCIPALS";
const BIND_ADDR_ENV: &str = "SDKWORK_IM_CONVERSATION_RUNTIME_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18082";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("sdkwork-comms-conversation-service");
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
    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("conversation-runtime failed to bind local listener: {error}"))?;

    let app = match std::env::var(PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        Some(catalog_path) => {
            let directory = conversation_runtime::StaticPrincipalDirectory::from_json_file(
                std::path::Path::new(catalog_path.as_str()),
            )?;
            sdkwork_routes_im_chat_open_api::build_public_app_with_principal_directory_from_env(
                Arc::new(directory),
            )
            .await
        }
        None => {
            let allow_all = std::env::var(ALLOW_ALL_PRINCIPALS_ENV)
                .ok()
                .is_some_and(|value| {
                    matches!(
                        value.trim().to_ascii_lowercase().as_str(),
                        "1" | "true" | "yes" | "on"
                    )
                });
            if allow_all {
                tracing::warn!(
                    "{} is enabled - all principals are allowed without verification; \
                     this must never be used in production",
                    ALLOW_ALL_PRINCIPALS_ENV
                );
                sdkwork_routes_im_chat_open_api::build_public_app_with_allow_all_principals_from_env(
                )
                .await
            } else {
                return Err(format!(
                    "principal directory is required: set {} to a JSON catalog file path, \
                     or set {}=true for development-only mode",
                    PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV, ALLOW_ALL_PRINCIPALS_ENV
                ));
            }
        }
    };

    tracing::info!(
        "conversation-runtime starting on {}",
        listener.local_addr().map_err(|e| e.to_string())?
    );
    axum::serve(listener, app)
.with_graceful_shutdown(async {
sdkwork_im_service_readiness::shutdown_signal().await;
})
        .await
        .map_err(|error| format!("conversation-runtime server should run: {error}"))?;
    Ok(())
}
