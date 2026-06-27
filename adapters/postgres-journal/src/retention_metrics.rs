use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use crate::RetentionCleanupReport;

static GLOBAL_RETENTION_METRICS: OnceLock<Arc<RetentionPurgeMetrics>> = OnceLock::new();

/// Shared retention purge metrics for Prometheus scrape endpoints.
#[derive(Debug, Default)]
pub struct RetentionPurgeMetrics {
    batches_total: AtomicU64,
    skipped_lock_total: AtomicU64,
    failures_total: AtomicU64,
    commit_journal_deleted_total: AtomicU64,
    conversation_messages_deleted_total: AtomicU64,
    message_media_refs_deleted_total: AtomicU64,
    outbox_events_deleted_total: AtomicU64,
    inbox_events_deleted_total: AtomicU64,
    projection_timeline_entries_deleted_total: AtomicU64,
    realtime_device_events_deleted_total: AtomicU64,
    last_duration_micros: AtomicU64,
}

impl RetentionPurgeMetrics {
    pub fn global() -> Arc<Self> {
        GLOBAL_RETENTION_METRICS
            .get_or_init(|| Arc::new(Self::default()))
            .clone()
    }

    pub fn record_batch(&self, report: &RetentionCleanupReport, duration_micros: u64) {
        self.batches_total.fetch_add(1, Ordering::Relaxed);
        self.commit_journal_deleted_total
            .fetch_add(report.commit_journal_deleted, Ordering::Relaxed);
        self.conversation_messages_deleted_total
            .fetch_add(report.conversation_messages_deleted, Ordering::Relaxed);
        self.message_media_refs_deleted_total
            .fetch_add(report.message_media_refs_deleted, Ordering::Relaxed);
        self.outbox_events_deleted_total
            .fetch_add(report.outbox_events_deleted, Ordering::Relaxed);
        self.inbox_events_deleted_total
            .fetch_add(report.inbox_events_deleted, Ordering::Relaxed);
        self.projection_timeline_entries_deleted_total
            .fetch_add(report.projection_timeline_entries_deleted, Ordering::Relaxed);
        self.realtime_device_events_deleted_total
            .fetch_add(report.realtime_device_events_deleted, Ordering::Relaxed);
        self.last_duration_micros
            .store(duration_micros, Ordering::Relaxed);
    }

    pub fn record_skipped_lock(&self) {
        self.skipped_lock_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failures_total.fetch_add(1, Ordering::Relaxed);
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
        let last_duration_seconds = self.last_duration_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        format!(
            "# HELP im_retention_purge_batches_total Retention purge batch executions.\n\
             # TYPE im_retention_purge_batches_total counter\n\
             im_retention_purge_batches_total{{{base_labels}}} {}\n\
             # HELP im_retention_purge_skipped_lock_total Retention purge ticks skipped because another worker holds the advisory lock.\n\
             # TYPE im_retention_purge_skipped_lock_total counter\n\
             im_retention_purge_skipped_lock_total{{{base_labels}}} {}\n\
             # HELP im_retention_purge_failures_total Retention purge batch failures.\n\
             # TYPE im_retention_purge_failures_total counter\n\
             im_retention_purge_failures_total{{{base_labels}}} {}\n\
             # HELP im_retention_purge_rows_deleted_total Rows deleted by retention purge grouped by store.\n\
             # TYPE im_retention_purge_rows_deleted_total counter\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"commit_journal\"}} {}\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"conversation_messages\"}} {}\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"message_media_refs\"}} {}\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"outbox_events\"}} {}\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"inbox_events\"}} {}\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"projection_timeline\"}} {}\n\
             im_retention_purge_rows_deleted_total{{{base_labels},store=\"realtime_device_events\"}} {}\n\
             # HELP im_retention_purge_last_duration_seconds Duration of the most recent retention purge batch in seconds.\n\
             # TYPE im_retention_purge_last_duration_seconds gauge\n\
             im_retention_purge_last_duration_seconds{{{base_labels}}} {last_duration_seconds}\n\
             # HELP im_health_status Service health status (1 = serving).\n\
             # TYPE im_health_status gauge\n\
             im_health_status{{{base_labels}}} 1\n",
            self.batches_total.load(Ordering::Relaxed),
            self.skipped_lock_total.load(Ordering::Relaxed),
            self.failures_total.load(Ordering::Relaxed),
            self.commit_journal_deleted_total.load(Ordering::Relaxed),
            self.conversation_messages_deleted_total.load(Ordering::Relaxed),
            self.message_media_refs_deleted_total.load(Ordering::Relaxed),
            self.outbox_events_deleted_total.load(Ordering::Relaxed),
            self.inbox_events_deleted_total.load(Ordering::Relaxed),
            self.projection_timeline_entries_deleted_total.load(Ordering::Relaxed),
            self.realtime_device_events_deleted_total.load(Ordering::Relaxed),
        )
    }
}

pub fn retention_purge_metrics() -> Arc<RetentionPurgeMetrics> {
    RetentionPurgeMetrics::global()
}

fn escape_label(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_prometheus_includes_core_counters() {
        let metrics = RetentionPurgeMetrics::default();
        metrics.record_batch(
            &RetentionCleanupReport {
                commit_journal_deleted: 2,
                conversation_messages_deleted: 3,
                message_media_refs_deleted: 0,
                outbox_events_deleted: 0,
                inbox_events_deleted: 0,
                projection_timeline_entries_deleted: 1,
                realtime_device_events_deleted: 0,
            },
            1_500_000,
        );
        let body = metrics.render_prometheus(
            "ops-service",
            "test",
            "standalone",
            "server",
        );
        assert!(body.contains("im_retention_purge_batches_total"));
        assert!(body.contains("store=\"commit_journal\""));
        assert!(body.contains("im_retention_purge_last_duration_seconds"));
        assert!(body.contains("im_health_status"));
    }
}
