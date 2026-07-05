use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

use im_app_context::is_production_like_im_environment;
use im_domain_core::retention::is_retention_expired;
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_message::TimelineProjectionStore;

use crate::model::{TimelineViewEntry, TimelineWindowView};
use crate::ProjectionError;

/// Unlimited in-memory timeline retention (development / in-memory backends).
pub const PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED: usize = usize::MAX;

/// Default hot-cache size per conversation when Postgres durable timeline is enabled.
pub const PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT: usize = 1000;

const PROJECTION_TIMELINE_MEMORY_CAP_ENV: &str = "SDKWORK_IM_PROJECTION_TIMELINE_MEMORY_CAP";

fn parse_env_memory_cap() -> Option<usize> {
    std::env::var(PROJECTION_TIMELINE_MEMORY_CAP_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
}

/// Resolve per-conversation in-memory timeline cap from process environment.
pub fn resolve_memory_timeline_cap_from_env(durable_timeline_enabled: bool) -> usize {
    let env_cap = parse_env_memory_cap();
    if durable_timeline_enabled {
        return env_cap.unwrap_or(PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT);
    }
    if is_production_like_im_environment() {
        return env_cap.unwrap_or(PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT);
    }
    PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED
}

pub struct TimelineTierConfig {
    durable_timeline: OnceLock<Arc<dyn TimelineProjectionStore + Send + Sync>>,
    memory_timeline_cap: AtomicUsize,
}

impl Default for TimelineTierConfig {
    fn default() -> Self {
        Self {
            durable_timeline: OnceLock::new(),
            memory_timeline_cap: AtomicUsize::new(PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED),
        }
    }
}

impl TimelineTierConfig {
    pub fn configure_durable_timeline(
        &self,
        store: Arc<dyn TimelineProjectionStore + Send + Sync>,
        memory_cap: usize,
    ) {
        let _ = self.durable_timeline.set(store);
        self.set_memory_timeline_cap(memory_cap);
    }

    pub fn set_memory_timeline_cap(&self, memory_cap: usize) {
        self.memory_timeline_cap
            .store(memory_cap.max(1), Ordering::Relaxed);
    }

    pub fn memory_timeline_cap(&self) -> usize {
        self.memory_timeline_cap.load(Ordering::Relaxed)
    }

    pub fn durable_timeline_store(&self) -> Option<Arc<dyn TimelineProjectionStore + Send + Sync>> {
        self.durable_timeline.get().cloned()
    }
}

pub fn trim_timeline_to_cap(timeline: &mut BTreeMap<u64, TimelineViewEntry>, cap: usize) {
    if cap >= PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED {
        return;
    }
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
) -> TimelineWindowView {
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
    TimelineWindowView {
        items: window,
        next_after_seq,
        has_more,
    }
}

pub fn timeline_window_from_durable_store(
    store: &dyn TimelineProjectionStore,
    tenant_id: &str,
    conversation_id: &str,
    after_seq: u64,
    limit: usize,
) -> Result<TimelineWindowView, ProjectionError> {
    let now = utc_now_rfc3339_millis();
    let window = store
        .load_timeline_window(tenant_id, conversation_id, after_seq, limit)
        .map_err(ProjectionError::StoreFailure)?;
    let mut items = Vec::with_capacity(window.items.len());
    for (_, payload) in window.items {
        let entry = serde_json::from_str::<TimelineViewEntry>(&payload)
            .map_err(ProjectionError::InvalidSnapshot)?;
        if is_retention_expired(entry.retention_until.as_deref(), now.as_str()) {
            continue;
        }
        items.push(entry);
    }
    let has_more = window.has_more;
    let next_after_seq = items.last().map(|entry| entry.message_seq);
    Ok(TimelineWindowView {
        items,
        next_after_seq,
        has_more,
    })
}

pub fn load_timeline_tail_for_restore(
    store: &dyn TimelineProjectionStore,
    tenant_id: &str,
    conversation_id: &str,
    message_count: u64,
    cap: usize,
) -> Result<BTreeMap<u64, TimelineViewEntry>, ProjectionError> {
    if cap >= PROJECTION_TIMELINE_MEMORY_CAP_UNLIMITED || message_count == 0 {
        return load_full_timeline_for_restore(store, tenant_id, conversation_id);
    }
    let after_seq = message_count.saturating_sub(cap as u64);
    let window = store
        .load_timeline_window(tenant_id, conversation_id, after_seq, cap)
        .map_err(ProjectionError::StoreFailure)?;
    parse_timeline_restore_entries(window.items)
}

pub fn load_full_timeline_for_restore(
    store: &dyn TimelineProjectionStore,
    tenant_id: &str,
    conversation_id: &str,
) -> Result<BTreeMap<u64, TimelineViewEntry>, ProjectionError> {
    let rows = store
        .load_timeline(tenant_id, conversation_id)
        .map_err(ProjectionError::StoreFailure)?;
    parse_timeline_restore_entries(rows)
}

fn parse_timeline_restore_entries(
    rows: Vec<(u64, String)>,
) -> Result<BTreeMap<u64, TimelineViewEntry>, ProjectionError> {
    rows.into_iter()
        .map(|(message_seq, payload)| {
            serde_json::from_str::<TimelineViewEntry>(&payload)
                .map(|entry| (message_seq, entry))
                .map_err(ProjectionError::InvalidSnapshot)
        })
        .collect()
}

pub fn resolve_timeline_window(
    tier: &TimelineTierConfig,
    memory_timeline: Option<&BTreeMap<u64, TimelineViewEntry>>,
    tenant_id: &str,
    conversation_id: &str,
    after_seq: u64,
    limit: usize,
) -> Result<TimelineWindowView, ProjectionError> {
    let memory_view = memory_timeline
        .map(|timeline| timeline_window_from_memory(timeline, after_seq, limit))
        .unwrap_or_else(|| TimelineWindowView {
            items: Vec::new(),
            next_after_seq: None,
            has_more: false,
        });

    let Some(store) = tier.durable_timeline_store() else {
        return Ok(memory_view);
    };

    let use_durable = match memory_timeline {
        None => true,
        Some(timeline) if timeline.is_empty() => true,
        Some(timeline) => {
            let min_mem_seq = timeline.keys().next().copied().unwrap_or(1);
            after_seq < min_mem_seq.saturating_sub(1)
        }
    };

    if !use_durable {
        return Ok(memory_view);
    }

    let mut durable_view =
        timeline_window_from_durable_store(store.as_ref(), tenant_id, conversation_id, after_seq, limit)?;
    if let Some(timeline) = memory_timeline {
        if let Some(min_mem_seq) = timeline.keys().next().copied() {
            durable_view.items.retain(|entry| entry.message_seq < min_mem_seq);
            durable_view.has_more = durable_view.has_more
                || timeline.keys().any(|seq| *seq > after_seq && *seq >= min_mem_seq);
            durable_view.next_after_seq = durable_view.items.last().map(|entry| entry.message_seq);
        }
    }
    Ok(durable_view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_adapters_local_memory::MemoryTimelineProjectionStore;
    use im_domain_core::message::{MessageBody, MessageType, Sender};
    use sdkwork_im_contract_message::TimelineProjectionRecord;

    fn sample_entry(seq: u64) -> TimelineViewEntry {
        TimelineViewEntry {
            tenant_id: "100001".into(),
            conversation_id: "c_demo".into(),
            message_id: format!("m{seq}"),
            message_seq: seq,
            summary: Some(format!("msg {seq}")),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: Some("cm_1".into()),
                device_id: None,
                session_id: None,
                metadata: BTreeMap::new(),
            },
            body: MessageBody {
                summary: Some(format!("msg {seq}")),
                parts: Vec::new(),
                render_hints: BTreeMap::new(),
                reply_to: None,
            },
            message_type: MessageType::Standard,
            delivery_mode: "discrete".into(),
            client_msg_id: None,
            stream_session_id: None,
            rtc_session_id: None,
            occurred_at: "2026-01-01T00:00:00.000Z".into(),
            committed_at: None,
            retention_until: None,
        }
    }

    #[test]
    fn trim_timeline_to_cap_drops_oldest_entries() {
        let mut timeline = BTreeMap::from([
            (1, sample_entry(1)),
            (2, sample_entry(2)),
            (3, sample_entry(3)),
        ]);
        trim_timeline_to_cap(&mut timeline, 2);
        assert_eq!(timeline.keys().copied().collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn resolve_timeline_window_uses_durable_history_before_hot_cache() {
        let store = Arc::new(MemoryTimelineProjectionStore::default());
        store
            .upsert_timeline_entries(
                "100001",
                "c_demo",
                &[
                    TimelineProjectionRecord {
                        message_seq: 1,
                        payload: serde_json::to_string(&sample_entry(1)).expect("serialize"),
                    },
                    TimelineProjectionRecord {
                        message_seq: 2,
                        payload: serde_json::to_string(&sample_entry(2)).expect("serialize"),
                    },
                ],
            )
            .expect("upsert");

        let tier = TimelineTierConfig::default();
        tier.configure_durable_timeline(store, 1);

        let mut memory = BTreeMap::from([(2, sample_entry(2))]);
        let window = resolve_timeline_window(
            &tier,
            Some(&memory),
            "100001",
            "c_demo",
            0,
            10,
        )
        .expect("window");
        assert_eq!(window.items.len(), 1);
        assert_eq!(window.items[0].message_seq, 1);

        memory.insert(2, sample_entry(2));
        let hot_window = resolve_timeline_window(
            &tier,
            Some(&memory),
            "100001",
            "c_demo",
            1,
            10,
        )
        .expect("hot window");
        assert_eq!(hot_window.items.len(), 1);
        assert_eq!(hot_window.items[0].message_seq, 2);
    }

    #[test]
    fn resolve_memory_timeline_cap_uses_default_when_durable_enabled_without_env_override() {
        if parse_env_memory_cap().is_some() {
            return;
        }
        assert_eq!(
            resolve_memory_timeline_cap_from_env(true),
            PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT
        );
    }

    #[test]
    fn resolve_memory_timeline_cap_enforces_default_in_production_without_durable_store() {
        const SDKWORK_IM_ENVIRONMENT_ENV: &str = "SDKWORK_IM_ENVIRONMENT";
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().expect("env test lock");
        let previous = std::env::var(SDKWORK_IM_ENVIRONMENT_ENV).ok();
        unsafe {
            std::env::set_var(SDKWORK_IM_ENVIRONMENT_ENV, "prod");
        }
        assert_eq!(
            resolve_memory_timeline_cap_from_env(false),
            PROJECTION_TIMELINE_MEMORY_CAP_DEFAULT
        );
        unsafe {
            match previous {
                Some(value) => std::env::set_var(SDKWORK_IM_ENVIRONMENT_ENV, value),
                None => std::env::remove_var(SDKWORK_IM_ENVIRONMENT_ENV),
            }
        }
    }
}
