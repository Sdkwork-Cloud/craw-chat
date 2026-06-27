//! Background stale-claim reclaim scheduler for idle shared-channel sync pending pools.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::task::JoinHandle;
use tokio::time::{self, MissedTickBehavior};
use tracing::{info, warn};

use crate::runtime::SocialRuntime;
use crate::shared_channel_sync_metrics::shared_channel_sync_metrics;

const SCHEDULER_ENABLED_ENV: &str = "SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED";
const INTERVAL_SECONDS_ENV: &str = "SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_INTERVAL_SECONDS";
const DEFAULT_INTERVAL_SECONDS: u64 = 60;
const MIN_INTERVAL_SECONDS: u64 = 15;
const MAX_INTERVAL_SECONDS: u64 = 3_600;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SharedChannelSyncStaleReclaimSchedulerConfig {
    pub interval: Duration,
}

impl SharedChannelSyncStaleReclaimSchedulerConfig {
    pub fn from_env() -> Option<Self> {
        if !scheduler_enabled_from_env() {
            return None;
        }
        Some(Self {
            interval: Duration::from_secs(read_u64_env(
                INTERVAL_SECONDS_ENV,
                DEFAULT_INTERVAL_SECONDS,
                MIN_INTERVAL_SECONDS,
                MAX_INTERVAL_SECONDS,
            )),
        })
    }
}

pub fn spawn_shared_channel_sync_stale_reclaim_scheduler_from_env(
    runtime: Arc<SocialRuntime>,
) -> Option<JoinHandle<()>> {
    let config = SharedChannelSyncStaleReclaimSchedulerConfig::from_env()?;
    if runtime
        .shared_channel_sync_stale_reclaim_scheduler_started
        .swap(true, std::sync::atomic::Ordering::AcqRel)
    {
        warn!(
            target: "sdkwork.im",
            event = "im.shared_channel_sync.stale_reclaim.scheduler_duplicate",
            "shared-channel sync stale reclaim scheduler already started"
        );
        return None;
    }
    Some(spawn_shared_channel_sync_stale_reclaim_scheduler(runtime, config))
}

pub fn spawn_shared_channel_sync_stale_reclaim_scheduler(
    runtime: Arc<SocialRuntime>,
    config: SharedChannelSyncStaleReclaimSchedulerConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = time::interval(config.interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            let started = Instant::now();
            match runtime.reclaim_stale_pending_shared_channel_sync_claims_persisted() {
                Ok(response) => {
                    shared_channel_sync_metrics()
                        .record_stale_reclaim_tick(response.reclaimed, started);
                    if response.reclaimed > 0 {
                        info!(
                            target: "sdkwork.im",
                            event = "im.shared_channel_sync.stale_reclaim.scheduler_tick",
                            reclaimed = response.reclaimed,
                            pending_before = response.pending_before,
                            pending_after = response.pending_after,
                            "reclaimed stale shared-channel sync pending claims"
                        );
                    }
                }
                Err(error) => {
                    shared_channel_sync_metrics().record_stale_reclaim_failure();
                    warn!(
                        target: "sdkwork.im",
                        event = "im.shared_channel_sync.stale_reclaim.scheduler_tick_failed",
                        error = %error,
                        "shared-channel sync stale reclaim scheduler tick failed"
                    );
                }
            }
            ticker.tick().await;
        }
    })
}

fn scheduler_enabled_from_env() -> bool {
    match std::env::var(SCHEDULER_ENABLED_ENV)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("0") | Some("false") | Some("off") | Some("no") => false,
        _ => true,
    }
}

fn read_u64_env(name: &str, default: u64, min: u64, max: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(|value| value.clamp(min, max))
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_disabled_by_env() {
        unsafe {
            std::env::set_var(SCHEDULER_ENABLED_ENV, "off");
        }
        assert!(SharedChannelSyncStaleReclaimSchedulerConfig::from_env().is_none());
        unsafe {
            std::env::remove_var(SCHEDULER_ENABLED_ENV);
        }
    }
}
