use std::process::ExitCode;

const DEFAULT_SESSION_GATEWAY_BIND_ADDR: &str = "127.0.0.1:18080";

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::ensure_im_service_process_identity("session-gateway");
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
    let bootstrap = session_gateway::bootstrap_realtime_plane_from_env().await?;
    let cluster_subscriber = session_gateway::spawn_cluster_route_event_subscriber(&bootstrap);
    let app = sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap(
        &bootstrap,
    );
    let link_transport_handles = session_gateway::spawn_link_transport_listeners(
        bootstrap.assembly.clone(),
        bootstrap.node_id.as_str(),
        session_gateway::RealtimeAuthContextResolver::new(bootstrap.iam_auth_pool.clone()),
    );

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|error| {
            format!("session-gateway failed to bind listener at {bind_addr}: {error}")
        })?;

    tracing::info!(
        target: "sdkwork.im",
        event = "im.session_gateway.listen",
        node_id = %bootstrap.node_id,
        bind = %bind_addr,
        cluster_bus = bootstrap.cluster_bus.is_some(),
        "session-gateway listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            for handle in link_transport_handles {
                handle.abort();
            }
            if let Some(handle) = cluster_subscriber {
                let _ = handle.join();
            }
        })
        .await
        .map_err(|error| format!("session-gateway server should run: {error}"))?;
    Ok(())
}

fn resolve_bind_addr() -> Result<std::net::SocketAddr, String> {
    let session_gateway_bind_addr = std::env::var("SESSION_GATEWAY_BIND_ADDR").ok();
    let topology_bind_addr = std::env::var("SDKWORK_IM_INTERNAL_SESSION_GATEWAY_BIND").ok();

    resolve_bind_addr_from_env(
        session_gateway_bind_addr.as_deref(),
        topology_bind_addr.as_deref(),
    )
}

fn resolve_bind_addr_from_env(
    session_gateway_bind_addr: Option<&str>,
    workspace_bind_addr: Option<&str>,
) -> Result<std::net::SocketAddr, String> {
    let bind_addr = [session_gateway_bind_addr, workspace_bind_addr]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|value| !value.is_empty())
        .unwrap_or(DEFAULT_SESSION_GATEWAY_BIND_ADDR);

    bind_addr
        .parse()
        .map_err(|error| format!("invalid session-gateway bind address `{bind_addr}`: {error}"))
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::resolve_bind_addr_from_env;

    #[test]
    fn resolve_bind_addr_prefers_service_specific_env_value() {
        let resolved = resolve_bind_addr_from_env(Some("0.0.0.0:28080"), Some("127.0.0.1:18080"))
            .expect("service-specific bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 28080)
        );
    }

    #[test]
    fn resolve_bind_addr_falls_back_to_topology_bind_env() {
        let resolved = resolve_bind_addr_from_env(None, Some("127.0.0.1:18080"))
            .expect("topology bind env should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18080)
        );
    }

    #[test]
    fn resolve_bind_addr_uses_default_when_no_env_values_are_present() {
        let resolved =
            resolve_bind_addr_from_env(None, None).expect("default bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18080)
        );
    }

    #[test]
    fn resolve_bind_addr_rejects_invalid_values() {
        let error = resolve_bind_addr_from_env(Some("not-a-socket-addr"), None)
            .expect_err("invalid bind addr should fail");

        assert!(
            error.contains("invalid session-gateway bind address"),
            "unexpected error: {error}"
        );
    }
}
