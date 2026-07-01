use std::process::ExitCode;
use std::sync::Arc;

use conversation_runtime::rpc_dispatch::{
    ConversationRpcDispatcher, CONVERSATION_RPC_SERVICE_KEYS,
};
use sdkwork_im_rpc_service_rust::{
    build_im_rpc_service_router_with_config_for_services, initialize_im_rpc_framework_from_env,
    register_im_discovery_instance, serve_im_rpc_with_discovery, ImRpcServerConfig,
};
use sdkwork_rpc_server::wait_for_ctrl_c;

const DEFAULT_CONVERSATION_RPC_BIND_ADDR: &str = "127.0.0.1:50052";
const CONVERSATION_RPC_BIND_ADDR_ENV: &str = "SDKWORK_IM_COMMS_CONVERSATION_RPC_BIND_ADDR";
const CONVERSATION_RPC_PUBLIC_ENDPOINT_ENV: &str =
    "SDKWORK_IM_COMMS_CONVERSATION_RPC_PUBLIC_ENDPOINT";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("comms-conversation-rpc");
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
    let bind_addr = resolve_bind_addr()?;
    let config = ImRpcServerConfig {
        bind_addr: bind_addr.to_string(),
        public_endpoint: resolve_public_endpoint(bind_addr),
        enable_health: true,
        ..ImRpcServerConfig::local_default()
    };

    let rpc_framework = initialize_im_rpc_framework_from_env()
        .map_err(|error| format!("im rpc framework bootstrap failed: {error}"))?;
    rpc_framework
        .verify_client_resolution()
        .await
        .map_err(|error| format!("im rpc client resolution verification failed: {error}"))?;

    let dispatcher = Arc::new(
        ConversationRpcDispatcher::bootstrap_from_env()
            .await
            .map_err(|error| format!("conversation rpc runtime bootstrap failed: {error}"))?,
    );
    let router = build_im_rpc_service_router_with_config_for_services(
        &config,
        dispatcher,
        CONVERSATION_RPC_SERVICE_KEYS,
    );

    let discovery = register_im_discovery_instance(&config)
        .await
        .map_err(|error| format!("conversation rpc discovery registration failed: {error}"))?;

    tracing::info!(
        target: "sdkwork.im",
        event = "im.conversation.rpc.listen",
        bind = %bind_addr,
        discovery_enabled = discovery.is_some(),
        resolver_profile = ?rpc_framework.resolver_profile,
        served_services = ?CONVERSATION_RPC_SERVICE_KEYS,
        "comms-conversation rpc listening"
    );

    serve_im_rpc_with_discovery(router, &config, discovery, async {
        let _ = wait_for_ctrl_c().await;
    })
        .await
        .map_err(|error| format!("comms-conversation-rpc server should run: {error}"))
}

fn resolve_bind_addr() -> Result<std::net::SocketAddr, String> {
    let bind_addr = std::env::var(CONVERSATION_RPC_BIND_ADDR_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CONVERSATION_RPC_BIND_ADDR.to_owned());

    bind_addr
        .parse()
        .map_err(|error| format!("invalid conversation rpc bind address `{bind_addr}`: {error}"))
}

fn resolve_public_endpoint(bind_addr: std::net::SocketAddr) -> Option<String> {
    std::env::var(CONVERSATION_RPC_PUBLIC_ENDPOINT_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .or_else(|| Some(format!("http://{bind_addr}")))
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::{resolve_public_endpoint, DEFAULT_CONVERSATION_RPC_BIND_ADDR};

    #[test]
    fn default_bind_addr_is_valid_socket_addr() {
        let resolved = DEFAULT_CONVERSATION_RPC_BIND_ADDR
            .parse::<SocketAddr>()
            .expect("default bind addr should parse");
        assert_eq!(resolved.port(), 50052);
    }

    #[test]
    fn resolve_public_endpoint_falls_back_to_http_bind_addr() {
        let endpoint = resolve_public_endpoint(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50052));
        assert_eq!(endpoint, Some("http://127.0.0.1:50052".to_owned()));
    }
}
