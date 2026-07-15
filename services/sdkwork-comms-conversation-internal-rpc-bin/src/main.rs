use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use conversation_runtime::internal_rpc_dispatch::{
    CONVERSATION_INTERNAL_RPC_SERVICE_KEYS, ConversationInternalRpcDispatcher,
};
use sdkwork_im_rpc_service_rust::{
    ImRpcServerConfig, build_im_rpc_mtls_service_router_with_config_for_services,
    initialize_im_rpc_framework_from_env, register_im_discovery_instance,
    serve_im_rpc_with_discovery,
};
use sdkwork_rpc_framework_core::{
    RpcCallerContextSigningKey, RpcCallerContextVerifier, RpcServiceIdentityPolicy,
};
use sdkwork_rpc_server::{RpcInternalServiceSecurity, RpcServerTlsConfig, wait_for_ctrl_c};

const DEFAULT_INTERNAL_RPC_BIND_ADDR: &str = "127.0.0.1:50053";
const INTERNAL_RPC_BIND_ADDR_ENV: &str = "SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_BIND_ADDR";
const INTERNAL_RPC_PUBLIC_ENDPOINT_ENV: &str =
    "SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_PUBLIC_ENDPOINT";
const INTERNAL_DISCOVERY_SERVICE_NAME: &str = "sdkwork-communication-internal-rpc";
const KNOWLEDGEBASE_RPC_TRUST_DOMAIN_ENV: &str = "SDKWORK_IM_KNOWLEDGEBASE_RPC_TRUST_DOMAIN";
const KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY";
const KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV: &str =
    "SDKWORK_IM_KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY_FILE";
const INTERNAL_RPC_TLS_SERVER_CERT_PATH_ENV: &str =
    "SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_TLS_SERVER_CERT_PATH";
const INTERNAL_RPC_TLS_SERVER_KEY_PATH_ENV: &str =
    "SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_TLS_SERVER_KEY_PATH";
const INTERNAL_RPC_TLS_CLIENT_CA_CERT_PATH_ENV: &str =
    "SDKWORK_IM_COMMS_CONVERSATION_INTERNAL_RPC_TLS_CLIENT_CA_CERT_PATH";
const KNOWLEDGEBASE_SERVICE_IDENTITY: &str = "sdkwork-knowledgebase";
const IM_SERVICE_IDENTITY: &str = "sdkwork-im";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity(
        "comms-conversation-internal-rpc",
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
    sdkwork_im_service_readiness::bootstrap_im_service_database_from_env().await?;
    let bind_addr = resolve_bind_addr()?;
    let config = ImRpcServerConfig {
        bind_addr: bind_addr.to_string(),
        public_endpoint: resolve_public_endpoint(bind_addr),
        enable_health: true,
        require_tls: true,
        require_mtls: true,
        ..ImRpcServerConfig::local_default()
    };
    let (tls_config, internal_security) = resolve_internal_rpc_security()?;

    let rpc_framework = initialize_im_rpc_framework_from_env()
        .map_err(|error| format!("im rpc framework bootstrap failed: {error}"))?;
    rpc_framework
        .verify_client_resolution()
        .await
        .map_err(|error| format!("im rpc client resolution verification failed: {error}"))?;

    let dispatcher = Arc::new(
        ConversationInternalRpcDispatcher::bootstrap_from_env()
            .await
            .map_err(|error| {
                format!("conversation internal rpc runtime bootstrap failed: {error}")
            })?,
    );
    let router = build_im_rpc_mtls_service_router_with_config_for_services(
        &config,
        dispatcher,
        CONVERSATION_INTERNAL_RPC_SERVICE_KEYS,
        &tls_config,
        &internal_security,
    )
    .map_err(|error| format!("conversation internal rpc mTLS bootstrap failed: {error}"))?;

    let discovery = register_im_discovery_instance(&config)
        .await
        .map_err(|error| {
            format!("conversation internal rpc discovery registration failed: {error}")
        })?;

    tracing::info!(
        target: "sdkwork.im",
        event = "im.conversation.internal.rpc.listen",
        bind = %bind_addr,
        discovery_enabled = discovery.is_some(),
        discovery_service = INTERNAL_DISCOVERY_SERVICE_NAME,
        resolver_profile = ?rpc_framework.resolver_profile,
        served_services = ?CONVERSATION_INTERNAL_RPC_SERVICE_KEYS,
        "comms-conversation internal rpc listening"
    );

    serve_im_rpc_with_discovery(router, &config, discovery, async {
        let _ = wait_for_ctrl_c().await;
    })
    .await
    .map_err(|error| format!("comms-conversation-internal-rpc server should run: {error}"))
}

fn resolve_bind_addr() -> Result<std::net::SocketAddr, String> {
    let bind_addr = std::env::var(INTERNAL_RPC_BIND_ADDR_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_INTERNAL_RPC_BIND_ADDR.to_owned());

    bind_addr.parse().map_err(|error| {
        format!("invalid conversation internal rpc bind address `{bind_addr}`: {error}")
    })
}

fn resolve_public_endpoint(bind_addr: std::net::SocketAddr) -> Option<String> {
    std::env::var(INTERNAL_RPC_PUBLIC_ENDPOINT_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .or_else(|| Some(format!("https://{bind_addr}")))
}

fn resolve_internal_rpc_security()
-> Result<(RpcServerTlsConfig, RpcInternalServiceSecurity), String> {
    let trust_domain = required_env(KNOWLEDGEBASE_RPC_TRUST_DOMAIN_ENV)?;
    let signing_key = RpcCallerContextSigningKey::from_base64url(
        resolve_secret_from_env_or_file(
            KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY_ENV,
            KNOWLEDGEBASE_RPC_CALLER_CONTEXT_SIGNING_KEY_FILE_ENV,
        )?
        .as_str(),
    )
    .map_err(|error| format!("invalid Knowledgebase RPC caller-context signing key: {error}"))?;
    let service_identity_policy =
        RpcServiceIdentityPolicy::new(trust_domain, [KNOWLEDGEBASE_SERVICE_IDENTITY]).map_err(
            |error| format!("invalid Knowledgebase mTLS service-identity policy: {error}"),
        )?;
    let caller_context_verifier = RpcCallerContextVerifier::new(
        IM_SERVICE_IDENTITY,
        [(KNOWLEDGEBASE_SERVICE_IDENTITY, signing_key)],
    )
    .map_err(|error| format!("invalid Knowledgebase caller-context verifier: {error}"))?;
    let tls_config = RpcServerTlsConfig {
        server_cert_path: PathBuf::from(required_env(INTERNAL_RPC_TLS_SERVER_CERT_PATH_ENV)?),
        server_key_path: PathBuf::from(required_env(INTERNAL_RPC_TLS_SERVER_KEY_PATH_ENV)?),
        client_ca_certificate_path: Some(PathBuf::from(required_env(
            INTERNAL_RPC_TLS_CLIENT_CA_CERT_PATH_ENV,
        )?)),
        client_auth_optional: false,
    };
    Ok((
        tls_config,
        RpcInternalServiceSecurity::new(service_identity_policy, Some(caller_context_verifier)),
    ))
}

fn required_env(name: &str) -> Result<String, String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{name} is required"))
}

fn resolve_secret_from_env_or_file(
    secret_env: &str,
    secret_file_env: &str,
) -> Result<String, String> {
    let configured_file = std::env::var(secret_file_env)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());
    let secret = if let Some(path) = configured_file {
        std::fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read {secret_file_env} configured caller-context signing key: {error}"
            )
        })?
    } else {
        required_env(secret_env)?
    };
    let secret = secret.trim().to_owned();
    if secret.is_empty() {
        return Err(format!(
            "{secret_env} caller-context signing key must not be blank"
        ));
    }
    Ok(secret)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::{DEFAULT_INTERNAL_RPC_BIND_ADDR, resolve_public_endpoint};

    #[test]
    fn default_bind_addr_is_valid_socket_addr() {
        let resolved = DEFAULT_INTERNAL_RPC_BIND_ADDR
            .parse::<SocketAddr>()
            .expect("default bind addr should parse");
        assert_eq!(resolved.port(), 50053);
    }

    #[test]
    fn resolve_public_endpoint_falls_back_to_https_bind_addr() {
        let endpoint =
            resolve_public_endpoint(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50053));
        assert_eq!(endpoint, Some("https://127.0.0.1:50053".to_owned()));
    }
}
