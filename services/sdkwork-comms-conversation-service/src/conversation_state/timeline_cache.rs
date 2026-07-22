use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::atomic::{AtomicUsize, Ordering};

use im_domain_core::retention::is_retention_expired;
use im_time::utc_now_rfc3339_millis;
use sdkwork_utils_rust::http_api::SdkWorkPageData;

use crate::conversation_state::list_page;
use crate::conversation_state::model::TimelineViewEntry;

pub const CONVERSATION_TIMELINE_CACHE_CAP_DEFAULT: usize = 1_000;
pub const CONVERSATION_TIMELINE_CACHE_CAP_MAX: usize = 10_000;
const CONVERSATION_TIMELINE_CACHE_CAP_ENV: &str = "SDKWORK_IM_CONVERSATION_TIMELINE_CACHE_CAP";

pub struct TimelineCacheConfig {
    memory_cap: AtomicUsize,
}

impl Default for TimelineCacheConfig {
    fn default() -> Self {
        Self {
            memory_cap: AtomicUsize::new(resolve_memory_timeline_cap_from_env()),
        }
    }
}

impl TimelineCacheConfig {
    pub fn set_memory_timeline_cap(&self, memory_cap: usize) {
        self.memory_cap.store(
            memory_cap.clamp(1, CONVERSATION_TIMELINE_CACHE_CAP_MAX),
            Ordering::Relaxed,
        );
    }

    pub fn memory_timeline_cap(&self) -> usize {
        self.memory_cap.load(Ordering::Relaxed)
    }
}

pub fn resolve_memory_timeline_cap_from_env() -> usize {
    std::env::var(CONVERSATION_TIMELINE_CACHE_CAP_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(CONVERSATION_TIMELINE_CACHE_CAP_DEFAULT)
        .min(CONVERSATION_TIMELINE_CACHE_CAP_MAX)
}

pub fn trim_timeline_to_cap(timeline: &mut BTreeMap<u64, TimelineViewEntry>, cap: usize) {
    while timeline.len() > cap {
        let Some(oldest_seq) = timeline.keys().next().copied() else {
            break;
        };
        timeline.remove(&oldest_seq);
    }
}

pub fn timeline_window_from_memory(
    timeline: &BTreeMap<u64, TimelineViewEntry>,
    after_seq: u64,
    limit: usize,
) -> SdkWorkPageData<TimelineViewEntry> {
    let now = utc_now_rfc3339_millis();
    let mut window = timeline
        .range((Excluded(after_seq), Unbounded))
        .map(|(_, entry)| entry)
        .filter(|entry| !is_retention_expired(entry.retention_until.as_deref(), now.as_str()))
        .take(limit.saturating_add(1))
        .cloned()
        .collect::<Vec<_>>();
    let has_more = window.len() > limit;
    if has_more {
        window.truncate(limit);
    }
    let next_after_seq = window.last().map(|entry| entry.message_seq);
    list_page::seq_cursor_page(window, limit, next_after_seq, has_more)
}
