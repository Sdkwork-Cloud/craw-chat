use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;

use quinn::Endpoint;
use rustls::pki_types::CertificateDer;
use rustls_pemfile::{certs, private_key};
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::link_transport::LinkTransportRuntime;

const REALTIME_QUIC_BIND_ENV: &str = "SDKWORK_IM_REALTIME_QUIC_BIND_ADDR";
const REALTIME_QUIC_TLS_CERT_ENV: &str = "SDKWORK_IM_REALTIME_QUIC_TLS_CERT_PATH";
const REALTIME_QUIC_TLS_KEY_ENV: &str = "SDKWORK_IM_REALTIME_QUIC_TLS_KEY_PATH";

pub fn spawn_quic_listener(runtime: LinkTransportRuntime, bind_addr: SocketAddr) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = serve_quic_listener(runtime, bind_addr).await {
            warn!(target: "sdkwork.im", event = "im.link.quic.failed", %error);
        }
    })
}

async fn serve_quic_listener(
    runtime: LinkTransportRuntime,
    bind_addr: SocketAddr,
) -> Result<(), String> {
    let rustls_config = load_quic_tls_server_config()?;
    let quic_config = quinn::crypto::rustls::QuicServerConfig::try_from(rustls_config)
        .map_err(|error| format!("quic server crypto config failed: {error}"))?;
    let server_config = quinn::ServerConfig::with_crypto(Arc::new(quic_config));
    let endpoint = Endpoint::server(server_config, bind_addr)
        .map_err(|error| format!("quic link listener failed to bind {bind_addr}: {error}"))?;

    info!(
        target: "sdkwork.im",
        event = "im.link.quic.listen",
        bind = %bind_addr,
        node_id = %runtime.node_id(),
        "realtime quic link listener started"
    );

    loop {
        let Some(incoming) = endpoint.accept().await else {
            continue;
        };
        let permit = match runtime.connection_semaphore().clone().try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => {
                warn!(
                    target: "sdkwork.im",
                    event = "im.link.quic.overload",
                    "rejecting quic link connection at capacity"
                );
                continue;
            }
        };
        let runtime = runtime.clone();
        tokio::spawn(async move {
            let _permit = permit;
            match incoming.await {
                Ok(connection) => {
                    if let Err(error) = runtime.serve_quic_connection(connection).await {
                        warn!(
                            target: "sdkwork.im",
                            event = "im.link.quic.session_error",
                            %error,
                            "quic link session ended with error"
                        );
                    }
                }
                Err(error) => {
                    warn!(
                        target: "sdkwork.im",
                        event = "im.link.quic.handshake_failed",
                        %error,
                        "quic transport handshake failed"
                    );
                }
            }
        });
    }
}

fn load_quic_tls_server_config() -> Result<rustls::ServerConfig, String> {
    let cert_path = std::env::var(REALTIME_QUIC_TLS_CERT_ENV).map_err(|_| {
        format!("{REALTIME_QUIC_TLS_CERT_ENV} is required when QUIC link listener is enabled")
    })?;
    let key_path = std::env::var(REALTIME_QUIC_TLS_KEY_ENV).map_err(|_| {
        format!("{REALTIME_QUIC_TLS_KEY_ENV} is required when QUIC link listener is enabled")
    })?;

    let cert_bytes = fs::read(cert_path.trim())
        .map_err(|error| format!("read quic tls cert failed: {error}"))?;
    let key_bytes = fs::read(key_path.trim())
        .map_err(|error| format!("read quic tls key failed: {error}"))?;

    let certs: Vec<CertificateDer<'static>> = certs(&mut cert_bytes.as_slice())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("parse quic tls cert failed: {error}"))?;
    if certs.is_empty() {
        return Err("quic tls cert file contains no certificates".into());
    }

    let key = private_key(&mut key_bytes.as_slice())
        .map_err(|error| format!("parse quic tls private key failed: {error}"))?
        .ok_or_else(|| "quic tls key file contains no private key".to_owned())?;

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|error| format!("build quic tls server config failed: {error}"))
}

pub fn resolve_quic_bind_addr() -> Option<SocketAddr> {
    std::env::var(REALTIME_QUIC_BIND_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<SocketAddr>().ok())
}

#[cfg(test)]
mod link_quic_env_tests {
    use super::{REALTIME_QUIC_BIND_ENV, REALTIME_QUIC_TLS_CERT_ENV, REALTIME_QUIC_TLS_KEY_ENV};

    #[test]
    fn quic_env_keys_are_stable_for_contract_tests() {
        assert_eq!(REALTIME_QUIC_BIND_ENV, "SDKWORK_IM_REALTIME_QUIC_BIND_ADDR");
        assert_eq!(REALTIME_QUIC_TLS_CERT_ENV, "SDKWORK_IM_REALTIME_QUIC_TLS_CERT_PATH");
        assert_eq!(REALTIME_QUIC_TLS_KEY_ENV, "SDKWORK_IM_REALTIME_QUIC_TLS_KEY_PATH");
    }
}
