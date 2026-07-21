use std::sync::atomic::{AtomicU64, Ordering};

static CAPACITY_EXECUTIONS: AtomicU64 = AtomicU64::new(0);
static CAPACITY_RESPONSES: AtomicU64 = AtomicU64::new(0);
static CAPACITY_FRAMES: AtomicU64 = AtomicU64::new(0);
static CAPACITY_TOOL_CALLS: AtomicU64 = AtomicU64::new(0);
static EVICTIONS_EXECUTIONS_TTL: AtomicU64 = AtomicU64::new(0);
static EVICTIONS_EXECUTIONS_CAPACITY: AtomicU64 = AtomicU64::new(0);
static EVICTIONS_RESPONSES_TTL: AtomicU64 = AtomicU64::new(0);
static EVICTIONS_RESPONSES_CAPACITY: AtomicU64 = AtomicU64::new(0);
static EVICTIONS_TOOL_CALLS_TTL: AtomicU64 = AtomicU64::new(0);
static EVICTIONS_TOOL_CALLS_CAPACITY: AtomicU64 = AtomicU64::new(0);
static JOURNAL_APPEND_FAILURES: AtomicU64 = AtomicU64::new(0);

pub(crate) fn record_capacity_rejection(resource: &str) {
    if let Some(counter) = capacity_counter(resource) {
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

pub(crate) fn record_terminal_evictions(resource: &str, reason: &str, count: usize) {
    if count == 0 {
        return;
    }
    if let Some(counter) = eviction_counter(resource, reason) {
        counter.fetch_add(u64::try_from(count).unwrap_or(u64::MAX), Ordering::Relaxed);
    }
}

pub(crate) fn record_journal_append_failure() {
    JOURNAL_APPEND_FAILURES.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn render_prometheus(
    execution_entries: usize,
    execution_bytes: usize,
    response_entries: usize,
    response_bytes: usize,
    tool_call_entries: usize,
    tool_call_bytes: usize,
) -> String {
    format!(
        "# HELP im_automation_capacity_rejections_total Automation work rejected by bounded in-process capacity.\n\
         # TYPE im_automation_capacity_rejections_total counter\n\
         im_automation_capacity_rejections_total{{resource=\"executions\"}} {}\n\
         im_automation_capacity_rejections_total{{resource=\"agent_responses\"}} {}\n\
         im_automation_capacity_rejections_total{{resource=\"agent_response_frames\"}} {}\n\
         im_automation_capacity_rejections_total{{resource=\"agent_tool_calls\"}} {}\n\
         # HELP im_automation_terminal_evictions_total Terminal in-process entries evicted by resource and reason.\n\
         # TYPE im_automation_terminal_evictions_total counter\n\
         im_automation_terminal_evictions_total{{resource=\"executions\",reason=\"ttl\"}} {}\n\
         im_automation_terminal_evictions_total{{resource=\"executions\",reason=\"capacity\"}} {}\n\
         im_automation_terminal_evictions_total{{resource=\"agent_responses\",reason=\"ttl\"}} {}\n\
         im_automation_terminal_evictions_total{{resource=\"agent_responses\",reason=\"capacity\"}} {}\n\
         im_automation_terminal_evictions_total{{resource=\"agent_tool_calls\",reason=\"ttl\"}} {}\n\
         im_automation_terminal_evictions_total{{resource=\"agent_tool_calls\",reason=\"capacity\"}} {}\n\
         # HELP im_automation_runtime_entries Current bounded in-process entries by resource.\n\
         # TYPE im_automation_runtime_entries gauge\n\
         im_automation_runtime_entries{{resource=\"executions\"}} {}\n\
         im_automation_runtime_entries{{resource=\"agent_responses\"}} {}\n\
         im_automation_runtime_entries{{resource=\"agent_tool_calls\"}} {}\n\
         # HELP im_automation_runtime_estimated_bytes Current estimated in-process resident bytes by resource.\n\
         # TYPE im_automation_runtime_estimated_bytes gauge\n\
         im_automation_runtime_estimated_bytes{{resource=\"executions\"}} {}\n\
         im_automation_runtime_estimated_bytes{{resource=\"agent_responses\"}} {}\n\
         im_automation_runtime_estimated_bytes{{resource=\"agent_tool_calls\"}} {}\n\
         # HELP im_automation_journal_append_failures_total Journal appends that failed before process-local state commit.\n\
         # TYPE im_automation_journal_append_failures_total counter\n\
         im_automation_journal_append_failures_total {}\n",
        CAPACITY_EXECUTIONS.load(Ordering::Relaxed),
        CAPACITY_RESPONSES.load(Ordering::Relaxed),
        CAPACITY_FRAMES.load(Ordering::Relaxed),
        CAPACITY_TOOL_CALLS.load(Ordering::Relaxed),
        EVICTIONS_EXECUTIONS_TTL.load(Ordering::Relaxed),
        EVICTIONS_EXECUTIONS_CAPACITY.load(Ordering::Relaxed),
        EVICTIONS_RESPONSES_TTL.load(Ordering::Relaxed),
        EVICTIONS_RESPONSES_CAPACITY.load(Ordering::Relaxed),
        EVICTIONS_TOOL_CALLS_TTL.load(Ordering::Relaxed),
        EVICTIONS_TOOL_CALLS_CAPACITY.load(Ordering::Relaxed),
        execution_entries,
        response_entries,
        tool_call_entries,
        execution_bytes,
        response_bytes,
        tool_call_bytes,
        JOURNAL_APPEND_FAILURES.load(Ordering::Relaxed),
    )
}

fn capacity_counter(resource: &str) -> Option<&'static AtomicU64> {
    match resource {
        "executions" => Some(&CAPACITY_EXECUTIONS),
        "agent response streams" => Some(&CAPACITY_RESPONSES),
        "agent response frames" => Some(&CAPACITY_FRAMES),
        "agent tool calls" => Some(&CAPACITY_TOOL_CALLS),
        _ => None,
    }
}

fn eviction_counter(resource: &str, reason: &str) -> Option<&'static AtomicU64> {
    match (resource, reason) {
        ("executions", "ttl") => Some(&EVICTIONS_EXECUTIONS_TTL),
        ("executions", "capacity") => Some(&EVICTIONS_EXECUTIONS_CAPACITY),
        ("agent_responses", "ttl") => Some(&EVICTIONS_RESPONSES_TTL),
        ("agent_responses", "capacity") => Some(&EVICTIONS_RESPONSES_CAPACITY),
        ("agent_tool_calls", "ttl") => Some(&EVICTIONS_TOOL_CALLS_TTL),
        ("agent_tool_calls", "capacity") => Some(&EVICTIONS_TOOL_CALLS_CAPACITY),
        _ => None,
    }
}
