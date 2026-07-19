use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_CONVERSATION_RUNTIME_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:28082";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::enable_process_shared_database_pool();
    sdkwork_im_service_readiness::ensure_im_service_process_identity(
        "sdkwork-comms-conversation-service",
    );
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
    let ((app, _state), listener) =
        sdkwork_im_service_readiness::complete_preflight_then_bind_tcp_listener(
            bind_addr.as_str(),
            "conversation-runtime",
            async {
                sdkwork_im_service_readiness::bootstrap_im_service_database_from_env().await?;
                let state = conversation_runtime::http::bootstrap_conversation_app_state_from_env()?;
                state
                    .ensure_group_knowledgebase_outbox_relay_started()
                    .await
                    .map_err(|error| {
                        format!(
                            "conversation runtime group knowledgebase relay readiness failed: {error}"
                        )
                    })?;
                let app = sdkwork_routes_im_chat_open_api::gateway_mount_with_state(state.clone())
                    .await?;
                Ok((app, state))
            },
        )
        .await?;

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
