use std::process::ExitCode;
use std::time::{Duration, Instant};

use tokio::task::JoinHandle;

const DEFAULT_SESSION_GATEWAY_BIND_ADDR: &str = "127.0.0.1:28080";
const ROUTE_DRAIN_BATCH_SIZE: usize = 256;

#[tokio::main]
async fn main() -> ExitCode {
    sdkwork_im_service_readiness::enable_process_shared_database_pool();
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
    let drain_timeout = session_gateway::resolve_session_gateway_drain_timeout()?;
    let bootstrap = session_gateway::bootstrap_realtime_plane_from_env().await?;
    let cluster_subscriber = session_gateway::spawn_cluster_route_event_subscriber(&bootstrap);
    let maintenance_handle =
        session_gateway::spawn_realtime_maintenance_jobs(bootstrap.assembly.clone());
    let readiness = bootstrap.assembly.readiness();
    let realtime_cluster = bootstrap.assembly.realtime_cluster();
    let node_id = bootstrap.node_id.clone();
    let app =
        sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap(&bootstrap);
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

    let (server_shutdown_tx, server_shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let mut server_handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = server_shutdown_rx.await;
            })
            .await
            .map_err(|error| format!("session-gateway server should run: {error}"))
    });

    let signal = tokio::select! {
        signal = shutdown_signal() => signal?,
        result = &mut server_handle => return flatten_server_result(result),
    };
    let drain_started_at = Instant::now();
    let drain_deadline = drain_started_at + drain_timeout;
    readiness.mark_draining();
    let lifecycle_error = realtime_cluster
        .mark_node_draining(node_id.as_str())
        .err()
        .map(|error| error.message);
    let routes_before = realtime_cluster.route_count_for_node(node_id.as_str());
    tracing::info!(
        target: "sdkwork.im",
        event = "im.session_gateway.drain_started",
        node_id = %node_id,
        signal,
        routes_before,
        timeout_secs = drain_timeout.as_secs(),
        "session-gateway drain started"
    );

    let _ = server_shutdown_tx.send(());
    abort_tasks(link_transport_handles).await;
    if let Some(handle) = maintenance_handle {
        abort_tasks(vec![handle]).await;
    }

    let mut route_drain_error = None;
    loop {
        if !realtime_cluster.has_routes_for_node(node_id.as_str()) {
            break;
        }
        match realtime_cluster
            .fence_and_release_node_routes_batch(node_id.as_str(), ROUTE_DRAIN_BATCH_SIZE)
        {
            Ok(_) => route_drain_error = None,
            Err(error) => route_drain_error = Some(error.message),
        }
        if Instant::now() >= drain_deadline {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    if let Some(subscriber) = cluster_subscriber {
        subscriber
            .shutdown(remaining_duration(drain_deadline))
            .await;
    }

    let server_timed_out = if remaining_duration(drain_deadline).is_zero() {
        true
    } else {
        match tokio::time::timeout(remaining_duration(drain_deadline), &mut server_handle).await {
            Ok(result) => {
                flatten_server_result(result)?;
                false
            }
            Err(_) => true,
        }
    };
    if server_timed_out {
        server_handle.abort();
        let _ = server_handle.await;
    }

    let remaining_routes = realtime_cluster.route_count_for_node(node_id.as_str());
    realtime_cluster.unbind_node_runtime(node_id.as_str());
    tracing::info!(
        target: "sdkwork.im",
        event = "im.session_gateway.drain_completed",
        node_id = %node_id,
        remaining_routes,
        server_timed_out,
        elapsed_ms = drain_started_at.elapsed().as_millis() as u64,
        "session-gateway drain completed"
    );

    if let Some(error) = lifecycle_error {
        return Err(format!(
            "mark session-gateway node draining failed: {error}"
        ));
    }
    if remaining_routes > 0 {
        return Err(format!(
            "session-gateway drain deadline reached with {remaining_routes} route(s) remaining{}",
            route_drain_error
                .as_deref()
                .map(|error| format!(": {error}"))
                .unwrap_or_default()
        ));
    }
    if server_timed_out {
        return Err("session-gateway HTTP drain exceeded its deadline".into());
    }
    Ok(())
}

async fn abort_tasks(handles: Vec<JoinHandle<()>>) {
    for handle in &handles {
        handle.abort();
    }
    for handle in handles {
        let _ = handle.await;
    }
}

fn flatten_server_result(
    result: Result<Result<(), String>, tokio::task::JoinError>,
) -> Result<(), String> {
    result.map_err(|error| format!("session-gateway server task failed: {error}"))?
}

fn remaining_duration(deadline: Instant) -> Duration {
    deadline.saturating_duration_since(Instant::now())
}

#[cfg(unix)]
async fn shutdown_signal() -> Result<&'static str, String> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(|error| format!("install SIGTERM handler failed: {error}"))?;
    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            result.map_err(|error| format!("listen for SIGINT failed: {error}"))?;
            Ok("SIGINT")
        }
        _ = terminate.recv() => Ok("SIGTERM"),
    }
}

#[cfg(not(unix))]
async fn shutdown_signal() -> Result<&'static str, String> {
    tokio::signal::ctrl_c()
        .await
        .map_err(|error| format!("listen for shutdown signal failed: {error}"))?;
    Ok("CTRL_C")
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
        let resolved = resolve_bind_addr_from_env(Some("0.0.0.0:28081"), Some("127.0.0.1:28080"))
            .expect("service-specific bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 28081)
        );
    }

    #[test]
    fn resolve_bind_addr_falls_back_to_topology_bind_env() {
        let resolved = resolve_bind_addr_from_env(None, Some("127.0.0.1:28080"))
            .expect("topology bind env should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 28080)
        );
    }

    #[test]
    fn resolve_bind_addr_uses_default_when_no_env_values_are_present() {
        let resolved =
            resolve_bind_addr_from_env(None, None).expect("default bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 28080)
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
