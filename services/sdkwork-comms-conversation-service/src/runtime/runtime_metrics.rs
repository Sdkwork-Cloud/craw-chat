use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use super::{
    ConversationRuntime, resolve_conversation_cache_max_bytes, resolve_max_conversations_in_memory,
    write_runtime_state,
};
use sdkwork_im_contract_message::CommitJournal;

const SLOW_METRICS_SCAN_THRESHOLD: Duration = Duration::from_millis(10);

#[derive(Default)]
pub(super) struct ConversationRuntimeMetrics {
    conversation_evictions_count_total: AtomicU64,
    conversation_evictions_bytes_total: AtomicU64,
    conversation_evicted_bytes_count_total: AtomicU64,
    conversation_evicted_bytes_bytes_total: AtomicU64,
    message_evictions_total: AtomicU64,
    eviction_checks_total: AtomicU64,
    eviction_operations_total: AtomicU64,
    eviction_duration_micros_total: AtomicU64,
    metrics_scans_total: AtomicU64,
    metrics_scan_duration_micros_total: AtomicU64,
    slow_metrics_scans_total: AtomicU64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConversationRuntimeMetricsSnapshot {
    pub conversation_entries: usize,
    pub conversation_capacity: usize,
    pub estimated_conversation_bytes: usize,
    pub conversation_byte_budget_bytes: usize,
    pub conversation_budget_utilization_ratio: f64,
    pub dirty_conversation_entries: usize,
    pub message_cache_entries: usize,
    pub message_cache_bytes: usize,
    pub message_locator_entries: usize,
    pub business_binding_entries: usize,
    pub actor_inbox_actor_entries: usize,
    pub actor_inbox_conversation_entries: usize,
    pub replay_cache_entries: usize,
    pub replay_cache_bytes: usize,
    pub conversation_evictions_count_total: u64,
    pub conversation_evictions_bytes_total: u64,
    pub conversation_evicted_bytes_count_total: u64,
    pub conversation_evicted_bytes_total: u64,
    pub message_evictions_total: u64,
    pub eviction_checks_total: u64,
    pub eviction_operations_total: u64,
    pub eviction_duration_micros_total: u64,
    pub metrics_scans_total: u64,
    pub metrics_scan_duration_micros_total: u64,
    pub slow_metrics_scans_total: u64,
}

impl ConversationRuntimeMetrics {
    pub(super) fn record_eviction_check(
        &self,
        over_count: bool,
        over_bytes: bool,
        evicted_count: usize,
        evicted_bytes: usize,
        duration: Duration,
    ) {
        self.eviction_checks_total.fetch_add(1, Ordering::Relaxed);
        self.eviction_duration_micros_total.fetch_add(
            duration.as_micros().min(u64::MAX as u128) as u64,
            Ordering::Relaxed,
        );
        if evicted_count == 0 {
            return;
        }
        self.eviction_operations_total
            .fetch_add(1, Ordering::Relaxed);
        let evicted_count = usize_to_u64(evicted_count);
        let evicted_bytes = usize_to_u64(evicted_bytes);
        if over_bytes {
            self.conversation_evictions_bytes_total
                .fetch_add(evicted_count, Ordering::Relaxed);
            self.conversation_evicted_bytes_bytes_total
                .fetch_add(evicted_bytes, Ordering::Relaxed);
        } else if over_count {
            self.conversation_evictions_count_total
                .fetch_add(evicted_count, Ordering::Relaxed);
            self.conversation_evicted_bytes_count_total
                .fetch_add(evicted_bytes, Ordering::Relaxed);
        }
    }

    pub(super) fn record_message_evictions(&self, count: usize) {
        self.message_evictions_total
            .fetch_add(usize_to_u64(count), Ordering::Relaxed);
    }

    fn record_metrics_scan(&self, duration: Duration) {
        self.metrics_scans_total.fetch_add(1, Ordering::Relaxed);
        self.metrics_scan_duration_micros_total.fetch_add(
            duration.as_micros().min(u64::MAX as u128) as u64,
            Ordering::Relaxed,
        );
        if duration >= SLOW_METRICS_SCAN_THRESHOLD {
            self.slow_metrics_scans_total
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn runtime_metrics_snapshot(&self) -> ConversationRuntimeMetricsSnapshot {
        let started = std::time::Instant::now();
        let conversation_capacity = resolve_max_conversations_in_memory();
        let conversation_byte_budget_bytes = resolve_conversation_cache_max_bytes();
        let mut state = write_runtime_state(&self.state, "runtime.state.metrics_snapshot");
        state.refresh_dirty_conversation_weights();

        let mut message_cache_entries = 0usize;
        let mut message_cache_bytes = 0usize;
        let mut replay_cache_entries = 0usize;
        let mut replay_cache_bytes = 0usize;
        for conversation in state.conversations.values() {
            message_cache_entries = message_cache_entries
                .saturating_add(conversation.message_log.cached_message_count());
            message_cache_bytes =
                message_cache_bytes.saturating_add(conversation.message_log.cached_message_bytes());
            replay_cache_entries = replay_cache_entries
                .saturating_add(conversation.posted_message_requests.len())
                .saturating_add(conversation.message_mutation_requests.len());
            replay_cache_bytes = replay_cache_bytes
                .saturating_add(conversation.posted_message_requests.cached_bytes())
                .saturating_add(conversation.message_mutation_requests.cached_bytes());
        }

        let conversation_entries = state.conversations.len();
        let estimated_conversation_bytes = state.estimated_conversation_bytes;
        let dirty_conversation_entries = state.dirty_conversation_scopes.len();
        let message_locator_entries = state.message_locator.len();
        let business_binding_entries = state.business_index.len();
        let actor_inbox_actor_entries = state.actor_inbox.actor_count();
        let actor_inbox_conversation_entries =
            state.actor_inbox.conversation_association_count();
        drop(state);
        self.metrics.record_metrics_scan(started.elapsed());

        ConversationRuntimeMetricsSnapshot {
            conversation_entries,
            conversation_capacity,
            estimated_conversation_bytes,
            conversation_byte_budget_bytes,
            conversation_budget_utilization_ratio: utilization_ratio(
                estimated_conversation_bytes,
                conversation_byte_budget_bytes,
            ),
            dirty_conversation_entries,
            message_cache_entries,
            message_cache_bytes,
            message_locator_entries,
            business_binding_entries,
            actor_inbox_actor_entries,
            actor_inbox_conversation_entries,
            replay_cache_entries,
            replay_cache_bytes,
            conversation_evictions_count_total: self
                .metrics
                .conversation_evictions_count_total
                .load(Ordering::Relaxed),
            conversation_evictions_bytes_total: self
                .metrics
                .conversation_evictions_bytes_total
                .load(Ordering::Relaxed),
            conversation_evicted_bytes_count_total: self
                .metrics
                .conversation_evicted_bytes_count_total
                .load(Ordering::Relaxed),
            conversation_evicted_bytes_total: self
                .metrics
                .conversation_evicted_bytes_bytes_total
                .load(Ordering::Relaxed),
            message_evictions_total: self.metrics.message_evictions_total.load(Ordering::Relaxed),
            eviction_checks_total: self.metrics.eviction_checks_total.load(Ordering::Relaxed),
            eviction_operations_total: self
                .metrics
                .eviction_operations_total
                .load(Ordering::Relaxed),
            eviction_duration_micros_total: self
                .metrics
                .eviction_duration_micros_total
                .load(Ordering::Relaxed),
            metrics_scans_total: self.metrics.metrics_scans_total.load(Ordering::Relaxed),
            metrics_scan_duration_micros_total: self
                .metrics
                .metrics_scan_duration_micros_total
                .load(Ordering::Relaxed),
            slow_metrics_scans_total: self
                .metrics
                .slow_metrics_scans_total
                .load(Ordering::Relaxed),
        }
    }

    pub fn render_runtime_metrics_prometheus(
        &self,
        service: &str,
        environment: &str,
        deployment_profile: &str,
        runtime_target: &str,
    ) -> String {
        self.runtime_metrics_snapshot().render_prometheus(
            service,
            environment,
            deployment_profile,
            runtime_target,
        )
    }
}

impl ConversationRuntimeMetricsSnapshot {
    pub fn render_prometheus(
        &self,
        service: &str,
        environment: &str,
        deployment_profile: &str,
        runtime_target: &str,
    ) -> String {
        let labels = format!(
            "service=\"{}\",environment=\"{}\",deployment_profile=\"{}\",runtime_target=\"{}\"",
            escape_label(service),
            escape_label(environment),
            escape_label(deployment_profile),
            escape_label(runtime_target),
        );
        let eviction_duration_seconds = self.eviction_duration_micros_total as f64 / 1_000_000.0;
        let scan_duration_seconds = self.metrics_scan_duration_micros_total as f64 / 1_000_000.0;
        format!(
            "# HELP im_conversation_runtime_entries Conversation aggregates held in this process.\n\
             # TYPE im_conversation_runtime_entries gauge\n\
             im_conversation_runtime_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_capacity Configured maximum conversation aggregates in this process.\n\
             # TYPE im_conversation_runtime_capacity gauge\n\
             im_conversation_runtime_capacity{{{labels}}} {}\n\
             # HELP im_conversation_runtime_estimated_bytes Estimated bytes held by conversation aggregate caches.\n\
             # TYPE im_conversation_runtime_estimated_bytes gauge\n\
             im_conversation_runtime_estimated_bytes{{{labels}}} {}\n\
             # HELP im_conversation_runtime_budget_bytes Configured byte budget for conversation aggregate caches.\n\
             # TYPE im_conversation_runtime_budget_bytes gauge\n\
             im_conversation_runtime_budget_bytes{{{labels}}} {}\n\
             # HELP im_conversation_runtime_budget_utilization_ratio Estimated conversation cache utilization divided by its byte budget.\n\
             # TYPE im_conversation_runtime_budget_utilization_ratio gauge\n\
             im_conversation_runtime_budget_utilization_ratio{{{labels}}} {}\n\
             # HELP im_conversation_runtime_dirty_entries Conversation entries awaiting weight refresh.\n\
             # TYPE im_conversation_runtime_dirty_entries gauge\n\
             im_conversation_runtime_dirty_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_message_cache_entries Messages held in conversation caches.\n\
             # TYPE im_conversation_runtime_message_cache_entries gauge\n\
             im_conversation_runtime_message_cache_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_message_cache_bytes Serialized message bytes held in conversation caches.\n\
             # TYPE im_conversation_runtime_message_cache_bytes gauge\n\
             im_conversation_runtime_message_cache_bytes{{{labels}}} {}\n\
             # HELP im_conversation_runtime_message_locator_entries Message locator entries held in this process.\n\
             # TYPE im_conversation_runtime_message_locator_entries gauge\n\
             im_conversation_runtime_message_locator_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_business_binding_entries Business binding index entries held in this process.\n\
             # TYPE im_conversation_runtime_business_binding_entries gauge\n\
             im_conversation_runtime_business_binding_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_actor_inbox_actor_entries Actor inbox keys held in this process.\n\
             # TYPE im_conversation_runtime_actor_inbox_actor_entries gauge\n\
             im_conversation_runtime_actor_inbox_actor_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_actor_inbox_conversation_entries Actor-to-conversation associations held in this process.\n\
             # TYPE im_conversation_runtime_actor_inbox_conversation_entries gauge\n\
             im_conversation_runtime_actor_inbox_conversation_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_replay_cache_entries Idempotency replay entries held in conversation caches.\n\
             # TYPE im_conversation_runtime_replay_cache_entries gauge\n\
             im_conversation_runtime_replay_cache_entries{{{labels}}} {}\n\
             # HELP im_conversation_runtime_replay_cache_bytes Estimated idempotency replay payload bytes held in conversation caches.\n\
             # TYPE im_conversation_runtime_replay_cache_bytes gauge\n\
             im_conversation_runtime_replay_cache_bytes{{{labels}}} {}\n\
             # HELP im_conversation_runtime_evictions_total Conversation entries evicted by bounded pressure reason.\n\
             # TYPE im_conversation_runtime_evictions_total counter\n\
             im_conversation_runtime_evictions_total{{{labels},reason=\"count\"}} {}\n\
             im_conversation_runtime_evictions_total{{{labels},reason=\"bytes\"}} {}\n\
             # HELP im_conversation_runtime_evicted_bytes_total Estimated conversation bytes released by bounded pressure reason.\n\
             # TYPE im_conversation_runtime_evicted_bytes_total counter\n\
             im_conversation_runtime_evicted_bytes_total{{{labels},reason=\"count\"}} {}\n\
             im_conversation_runtime_evicted_bytes_total{{{labels},reason=\"bytes\"}} {}\n\
             # HELP im_conversation_runtime_message_evictions_total Messages evicted from per-conversation caches.\n\
             # TYPE im_conversation_runtime_message_evictions_total counter\n\
             im_conversation_runtime_message_evictions_total{{{labels}}} {}\n\
             # HELP im_conversation_runtime_eviction_checks_total Conversation cache budget checks.\n\
             # TYPE im_conversation_runtime_eviction_checks_total counter\n\
             im_conversation_runtime_eviction_checks_total{{{labels}}} {}\n\
             # HELP im_conversation_runtime_eviction_operations_total Conversation cache checks that evicted at least one entry.\n\
             # TYPE im_conversation_runtime_eviction_operations_total counter\n\
             im_conversation_runtime_eviction_operations_total{{{labels}}} {}\n\
             # HELP im_conversation_runtime_eviction_duration_seconds_total Cumulative time spent checking and evicting conversation cache entries.\n\
             # TYPE im_conversation_runtime_eviction_duration_seconds_total counter\n\
             im_conversation_runtime_eviction_duration_seconds_total{{{labels}}} {eviction_duration_seconds}\n\
             # HELP im_conversation_runtime_metrics_scans_total Runtime metrics snapshot scans.\n\
             # TYPE im_conversation_runtime_metrics_scans_total counter\n\
             im_conversation_runtime_metrics_scans_total{{{labels}}} {}\n\
             # HELP im_conversation_runtime_metrics_scan_duration_seconds_total Cumulative time spent scanning runtime cache metrics.\n\
             # TYPE im_conversation_runtime_metrics_scan_duration_seconds_total counter\n\
             im_conversation_runtime_metrics_scan_duration_seconds_total{{{labels}}} {scan_duration_seconds}\n\
             # HELP im_conversation_runtime_slow_metrics_scans_total Runtime cache metric scans taking at least 10 milliseconds.\n\
             # TYPE im_conversation_runtime_slow_metrics_scans_total counter\n\
             im_conversation_runtime_slow_metrics_scans_total{{{labels}}} {}\n",
            self.conversation_entries,
            self.conversation_capacity,
            self.estimated_conversation_bytes,
            self.conversation_byte_budget_bytes,
            self.conversation_budget_utilization_ratio,
            self.dirty_conversation_entries,
            self.message_cache_entries,
            self.message_cache_bytes,
            self.message_locator_entries,
            self.business_binding_entries,
            self.actor_inbox_actor_entries,
            self.actor_inbox_conversation_entries,
            self.replay_cache_entries,
            self.replay_cache_bytes,
            self.conversation_evictions_count_total,
            self.conversation_evictions_bytes_total,
            self.conversation_evicted_bytes_count_total,
            self.conversation_evicted_bytes_total,
            self.message_evictions_total,
            self.eviction_checks_total,
            self.eviction_operations_total,
            self.metrics_scans_total,
            self.slow_metrics_scans_total,
        )
    }
}

fn utilization_ratio(used: usize, budget: usize) -> f64 {
    if budget == 0 {
        return 0.0;
    }
    used as f64 / budget as f64
}

fn usize_to_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn escape_label(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
