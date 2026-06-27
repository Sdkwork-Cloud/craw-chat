//! Prometheus metrics for shared-channel sync stale reclaim and delivery proofs.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

static GLOBAL_SHARED_CHANNEL_SYNC_METRICS: OnceLock<Arc<SharedChannelSyncMetrics>> = OnceLock::new();

/// Shared shared-channel sync metrics for Prometheus scrape endpoints.
#[derive(Debug, Default)]
pub struct SharedChannelSyncMetrics {
    stale_reclaim_ticks_total: AtomicU64,
    stale_reclaim_failures_total: AtomicU64,
    stale_reclaim_claims_reclaimed_total: AtomicU64,
    delivery_proofs_recorded_total: AtomicU64,
    delivery_deduplicated_total: AtomicU64,
    last_stale_reclaim_duration_micros: AtomicU64,
}

impl SharedChannelSyncMetrics {
    pub fn global() -> Arc<Self> {
        GLOBAL_SHARED_CHANNEL_SYNC_METRICS
            .get_or_init(|| Arc::new(Self::default()))
            .clone()
    }

    pub fn record_stale_reclaim_tick(&self, reclaimed: usize, started: Instant) {
        self.stale_reclaim_ticks_total.fetch_add(1, Ordering::Relaxed);
        if reclaimed > 0 {
            self.stale_reclaim_claims_reclaimed_total
                .fetch_add(reclaimed as u64, Ordering::Relaxed);
        }
        self.last_stale_reclaim_duration_micros.store(
            started.elapsed().as_micros().min(u64::MAX as u128) as u64,
            Ordering::Relaxed,
        );
    }

    pub fn record_stale_reclaim_failure(&self) {
        self.stale_reclaim_failures_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_delivery_proof_recorded(&self) {
        self.delivery_proofs_recorded_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_delivery_deduplicated(&self) {
        self.delivery_deduplicated_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn render_prometheus(
        &self,
        service: &str,
        environment: &str,
        deployment_profile: &str,
        runtime_target: &str,
    ) -> String {
        let base_labels = format!(
            "service=\"{}\",environment=\"{}\",deployment_profile=\"{}\",runtime_target=\"{}\"",
            escape_label(service),
            escape_label(environment),
            escape_label(deployment_profile),
            escape_label(runtime_target),
        );
        let last_duration_seconds =
            self.last_stale_reclaim_duration_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        format!(
            "# HELP im_shared_channel_sync_stale_reclaim_ticks_total Shared-channel sync stale reclaim scheduler ticks.\n\
             # TYPE im_shared_channel_sync_stale_reclaim_ticks_total counter\n\
             im_shared_channel_sync_stale_reclaim_ticks_total{{{base_labels}}} {}\n\
             # HELP im_shared_channel_sync_stale_reclaim_failures_total Shared-channel sync stale reclaim tick failures.\n\
             # TYPE im_shared_channel_sync_stale_reclaim_failures_total counter\n\
             im_shared_channel_sync_stale_reclaim_failures_total{{{base_labels}}} {}\n\
             # HELP im_shared_channel_sync_stale_reclaim_claims_reclaimed_total Stale shared-channel sync pending claims reclaimed.\n\
             # TYPE im_shared_channel_sync_stale_reclaim_claims_reclaimed_total counter\n\
             im_shared_channel_sync_stale_reclaim_claims_reclaimed_total{{{base_labels}}} {}\n\
             # HELP im_shared_channel_sync_delivery_proofs_recorded_total New shared-channel sync delivery proofs recorded.\n\
             # TYPE im_shared_channel_sync_delivery_proofs_recorded_total counter\n\
             im_shared_channel_sync_delivery_proofs_recorded_total{{{base_labels}}} {}\n\
             # HELP im_shared_channel_sync_delivery_deduplicated_total Shared-channel sync deliveries skipped because proof already exists.\n\
             # TYPE im_shared_channel_sync_delivery_deduplicated_total counter\n\
             im_shared_channel_sync_delivery_deduplicated_total{{{base_labels}}} {}\n\
             # HELP im_shared_channel_sync_stale_reclaim_last_duration_seconds Duration of the most recent stale reclaim tick in seconds.\n\
             # TYPE im_shared_channel_sync_stale_reclaim_last_duration_seconds gauge\n\
             im_shared_channel_sync_stale_reclaim_last_duration_seconds{{{base_labels}}} {last_duration_seconds}\n\
             # HELP im_health_status Service health status (1 = serving).\n\
             # TYPE im_health_status gauge\n\
             im_health_status{{{base_labels}}} 1\n",
            self.stale_reclaim_ticks_total.load(Ordering::Relaxed),
            self.stale_reclaim_failures_total.load(Ordering::Relaxed),
            self.stale_reclaim_claims_reclaimed_total.load(Ordering::Relaxed),
            self.delivery_proofs_recorded_total.load(Ordering::Relaxed),
            self.delivery_deduplicated_total.load(Ordering::Relaxed),
        )
    }
}

pub fn shared_channel_sync_metrics() -> Arc<SharedChannelSyncMetrics> {
    SharedChannelSyncMetrics::global()
}

fn escape_label(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn environment_metric_label(environment: &str) -> &'static str {
    match environment.trim().to_ascii_lowercase().as_str() {
        "production" | "prod" => "production",
        "staging" => "staging",
        "test" => "test",
        _ => "development",
    }
}

pub fn render_shared_channel_sync_prometheus_from_env() -> String {
    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_owned());
    let deployment_profile = std::env::var("SDKWORK_IM_DEPLOYMENT_PROFILE")
        .unwrap_or_else(|_| "standalone".to_owned());
    let service = std::env::var("SDKWORK_IM_SERVICE_NAME")
        .unwrap_or_else(|_| "comms-social-service".to_owned());
    shared_channel_sync_metrics().render_prometheus(
        service.as_str(),
        environment_metric_label(environment.as_str()),
        deployment_profile.as_str(),
        "server",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_prometheus_includes_core_counters() {
        let metrics = SharedChannelSyncMetrics::default();
        metrics.record_stale_reclaim_tick(2, Instant::now());
        metrics.record_delivery_proof_recorded();
        metrics.record_delivery_deduplicated();
        let body = metrics.render_prometheus(
            "comms-social-service",
            "test",
            "standalone",
            "server",
        );
        assert!(body.contains("im_shared_channel_sync_stale_reclaim_ticks_total"));
        assert!(body.contains("im_shared_channel_sync_delivery_proofs_recorded_total"));
        assert!(body.contains("im_shared_channel_sync_delivery_deduplicated_total"));
        assert!(body.contains("im_health_status"));
    }
}
