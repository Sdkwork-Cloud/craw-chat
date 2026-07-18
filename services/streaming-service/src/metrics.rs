use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Default)]
pub(crate) struct StreamRuntimeMetrics {
    append_requests_total: AtomicU64,
    append_duration_micros_total: AtomicU64,
    append_version_conflicts_total: AtomicU64,
    transition_version_conflicts_total: AtomicU64,
    concurrency_exhausted_total: AtomicU64,
    capacity_rejections_total: AtomicU64,
    store_errors_total: AtomicU64,
    readiness_failures_total: AtomicU64,
    frame_page_requests_total: AtomicU64,
    frame_page_items_total: AtomicU64,
}

pub(crate) struct AppendTimer<'a> {
    metrics: &'a StreamRuntimeMetrics,
    started: Instant,
}

impl StreamRuntimeMetrics {
    pub(crate) fn track_append(&self) -> AppendTimer<'_> {
        self.append_requests_total.fetch_add(1, Ordering::Relaxed);
        AppendTimer {
            metrics: self,
            started: Instant::now(),
        }
    }

    pub(crate) fn record_append_version_conflict(&self) {
        self.append_version_conflicts_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_transition_version_conflict(&self) {
        self.transition_version_conflicts_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_concurrency_exhausted(&self) {
        self.concurrency_exhausted_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_capacity_rejection(&self) {
        self.capacity_rejections_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_store_error(&self) {
        self.store_errors_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_readiness_failure(&self) {
        self.readiness_failures_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_frame_page(&self, item_count: usize) {
        self.frame_page_requests_total
            .fetch_add(1, Ordering::Relaxed);
        self.frame_page_items_total.fetch_add(
            u64::try_from(item_count).unwrap_or(u64::MAX),
            Ordering::Relaxed,
        );
    }

    pub(crate) fn render_prometheus(&self) -> String {
        format!(
            "# HELP im_stream_append_requests_total Stream append requests processed by this instance.\n\
             # TYPE im_stream_append_requests_total counter\n\
             im_stream_append_requests_total {}\n\
             # HELP im_stream_append_duration_seconds_total Cumulative stream append processing duration.\n\
             # TYPE im_stream_append_duration_seconds_total counter\n\
             im_stream_append_duration_seconds_total {}\n\
             # HELP im_stream_version_conflicts_total Optimistic concurrency conflicts by operation.\n\
             # TYPE im_stream_version_conflicts_total counter\n\
             im_stream_version_conflicts_total{{operation=\"append\"}} {}\n\
             im_stream_version_conflicts_total{{operation=\"transition\"}} {}\n\
             # HELP im_stream_concurrency_exhausted_total Requests rejected after bounded concurrency retries.\n\
             # TYPE im_stream_concurrency_exhausted_total counter\n\
             im_stream_concurrency_exhausted_total {}\n\
             # HELP im_stream_capacity_rejections_total Stream open requests rejected by tenant-organization capacity.\n\
             # TYPE im_stream_capacity_rejections_total counter\n\
             im_stream_capacity_rejections_total {}\n\
             # HELP im_stream_store_errors_total Authoritative stream store operations that failed.\n\
             # TYPE im_stream_store_errors_total counter\n\
             im_stream_store_errors_total {}\n\
             # HELP im_stream_readiness_failures_total Authoritative stream store readiness probes that failed.\n\
             # TYPE im_stream_readiness_failures_total counter\n\
             im_stream_readiness_failures_total {}\n\
             # HELP im_stream_frame_page_requests_total Bounded stream frame page requests.\n\
             # TYPE im_stream_frame_page_requests_total counter\n\
             im_stream_frame_page_requests_total {}\n\
             # HELP im_stream_frame_page_items_total Stream frames returned in bounded pages.\n\
             # TYPE im_stream_frame_page_items_total counter\n\
             im_stream_frame_page_items_total {}\n",
            self.append_requests_total.load(Ordering::Relaxed),
            self.append_duration_micros_total.load(Ordering::Relaxed) as f64 / 1_000_000.0,
            self.append_version_conflicts_total.load(Ordering::Relaxed),
            self.transition_version_conflicts_total
                .load(Ordering::Relaxed),
            self.concurrency_exhausted_total.load(Ordering::Relaxed),
            self.capacity_rejections_total.load(Ordering::Relaxed),
            self.store_errors_total.load(Ordering::Relaxed),
            self.readiness_failures_total.load(Ordering::Relaxed),
            self.frame_page_requests_total.load(Ordering::Relaxed),
            self.frame_page_items_total.load(Ordering::Relaxed),
        )
    }
}

impl Drop for AppendTimer<'_> {
    fn drop(&mut self) {
        self.metrics.append_duration_micros_total.fetch_add(
            self.started.elapsed().as_micros().min(u64::MAX as u128) as u64,
            Ordering::Relaxed,
        );
    }
}
