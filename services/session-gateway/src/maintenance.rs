//! Periodic maintenance for embedded realtime cluster state.

use std::time::Duration;

use chrono::{Duration as ChronoDuration, Utc};
use tokio::task::JoinHandle;

use crate::RealtimePlaneAssembly;

const REALTIME_MAINTENANCE_INTERVAL_SECS: u64 = 300;
const WEBSOCKET_IDLE_TIMEOUT_SECS_ENV: &str = "SDKWORK_IM_WEBSOCKET_IDLE_TIMEOUT_SECS";
const WEBSOCKET_IDLE_TIMEOUT_DEFAULT_SECS: u64 = 90;
const PRESENCE_STALE_MULTIPLIER: i64 = 2;

/// Spawn background jobs that reclaim stale route epoch notifiers, disconnect
/// fence cache entries, and expire stale online presence devices.
/// Returns `None` when maintenance is disabled via env.
pub fn spawn_realtime_maintenance_jobs(assembly: RealtimePlaneAssembly) -> Option<JoinHandle<()>> {
    if std::env::var("SDKWORK_IM_REALTIME_MAINTENANCE_DISABLED")
        .ok()
        .and_then(|value| sdkwork_utils_rust::parse_bool(value.as_str()))
        .unwrap_or(false)
    {
        return None;
    }

    Some(tokio::spawn(async move {
        let mut interval =
            tokio::time::interval(Duration::from_secs(REALTIME_MAINTENANCE_INTERVAL_SECS));
        interval.tick().await;
        loop {
            interval.tick().await;
            let cluster = assembly.realtime_cluster();
            cluster.cleanup_stale_route_epoch_notifiers();
            cluster.cleanup_stale_disconnect_fences();
            expire_stale_presence_devices(assembly.presence_runtime().as_ref());
        }
    }))
}

fn resolve_presence_stale_cutoff_rfc3339() -> String {
    let idle_secs = std::env::var(WEBSOCKET_IDLE_TIMEOUT_SECS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(WEBSOCKET_IDLE_TIMEOUT_DEFAULT_SECS as i64);
    let stale_secs = idle_secs.saturating_mul(PRESENCE_STALE_MULTIPLIER);
    (Utc::now() - ChronoDuration::seconds(stale_secs))
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn expire_stale_presence_devices(presence_runtime: &crate::PresenceRuntime) {
    let cutoff = resolve_presence_stale_cutoff_rfc3339();
    let expired_at = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    match presence_runtime.expire_stale_online_devices(cutoff.as_str(), expired_at.as_str()) {
        Ok(expired_count) if expired_count > 0 => {
            tracing::info!(
                expired_count,
                cutoff_seen_at = cutoff.as_str(),
                "expired stale online presence devices"
            );
        }
        Ok(_) => {}
        Err(error) => {
            tracing::warn!(
                ?error,
                cutoff_seen_at = cutoff.as_str(),
                "presence stale-device expiration failed"
            );
        }
    }
}
