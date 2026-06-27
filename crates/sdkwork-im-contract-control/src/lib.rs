use im_domain_core::presence::{PresenceClientView, PresenceStatus};
use im_domain_core::realtime::{RealtimeEvent, RealtimeSubscription};
use im_time::{
    compare_optional_rfc3339_asc, max_rfc3339_string, rfc3339_gt, rfc3339_le, rfc3339_lt,
};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};

const REALTIME_EVENT_WINDOW_HIGH_RISK_LIMIT: usize = 5;

fn default_organization_id() -> String {
    "0".to_owned()
}

pub fn normalize_realtime_organization_id(organization_id: &str) -> String {
    let trimmed = organization_id.trim();
    if trimmed.is_empty() || trimmed == "0" || trimmed == "default" {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

pub fn realtime_principal_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    realtime_scope_key_parts(&[
        tenant_id,
        normalize_realtime_organization_id(organization_id).as_str(),
        principal_kind,
        principal_id,
    ])
}

pub fn realtime_client_route_scope_key(
    tenant_id: &str,
    organization_id: &str,
    principal_kind: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    realtime_scope_key_parts(&[
        tenant_id,
        normalize_realtime_organization_id(organization_id).as_str(),
        principal_kind,
        principal_id,
        device_id,
    ])
}

pub fn realtime_scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeCheckpointRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub latest_realtime_seq: u64,
    pub acked_through_seq: u64,
    pub trimmed_through_seq: u64,
    #[serde(default)]
    pub capacity_trimmed_event_count: u64,
    #[serde(default)]
    pub capacity_trimmed_through_seq: u64,
    #[serde(default)]
    pub last_capacity_trimmed_at: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeEventWindowRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub events: Vec<RealtimeEvent>,
    pub trimmed_through_seq: u64,
    #[serde(default)]
    pub capacity_trimmed_event_count: u64,
    #[serde(default)]
    pub capacity_trimmed_through_seq: u64,
    #[serde(default)]
    pub last_capacity_trimmed_at: Option<String>,
    pub updated_at: String,
}

impl RealtimeEventWindowRecord {
    pub fn normalized(mut self) -> Self {
        self.events.sort_by_key(|event| event.realtime_seq);
        self.events
            .retain(|event| event.realtime_seq > self.trimmed_through_seq);
        self.capacity_trimmed_through_seq = self
            .capacity_trimmed_through_seq
            .min(self.trimmed_through_seq);
        if self.capacity_trimmed_event_count == 0 {
            self.capacity_trimmed_through_seq = 0;
            self.last_capacity_trimmed_at = None;
        }
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeEventWindowDiagnosticsSnapshot {
    pub client_route_window_count: u64,
    pub pending_event_count: u64,
    pub max_client_route_window_event_count: u64,
    pub max_trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub max_capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub oldest_pending_occurred_at: Option<String>,
    pub high_risk_windows: Vec<RealtimeEventWindowHighRiskRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeEventWindowHighRiskRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub pending_event_count: u64,
    pub trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub oldest_pending_occurred_at: Option<String>,
}

impl RealtimeEventWindowDiagnosticsSnapshot {
    pub fn from_records(records: impl IntoIterator<Item = RealtimeEventWindowRecord>) -> Self {
        let mut snapshot = Self::default();
        for record in records {
            let record = record.normalized();
            let pending_event_count = record.events.len() as u64;
            let oldest_pending_occurred_at = record
                .events
                .iter()
                .map(|event| event.occurred_at.clone())
                .min();
            snapshot.client_route_window_count =
                snapshot.client_route_window_count.saturating_add(1);
            snapshot.pending_event_count = snapshot
                .pending_event_count
                .saturating_add(pending_event_count);
            snapshot.max_client_route_window_event_count = snapshot
                .max_client_route_window_event_count
                .max(pending_event_count);
            snapshot.max_trimmed_through_seq = snapshot
                .max_trimmed_through_seq
                .max(record.trimmed_through_seq);
            snapshot.capacity_trimmed_event_count = snapshot
                .capacity_trimmed_event_count
                .saturating_add(record.capacity_trimmed_event_count);
            snapshot.max_capacity_trimmed_through_seq = snapshot
                .max_capacity_trimmed_through_seq
                .max(record.capacity_trimmed_through_seq);
            if let Some(last_capacity_trimmed_at) = record.last_capacity_trimmed_at.clone()
                && snapshot
                    .last_capacity_trimmed_at
                    .as_deref()
                    .is_none_or(|latest| rfc3339_gt(last_capacity_trimmed_at.as_str(), latest))
            {
                snapshot.last_capacity_trimmed_at = Some(last_capacity_trimmed_at);
            }
            if pending_event_count > 0 {
                snapshot
                    .high_risk_windows
                    .push(RealtimeEventWindowHighRiskRecord {
                        tenant_id: record.tenant_id,
                        organization_id: record.organization_id.clone(),
                        principal_kind: record.principal_kind,
                        principal_id: record.principal_id,
                        device_id: record.device_id,
                        pending_event_count,
                        trimmed_through_seq: record.trimmed_through_seq,
                        capacity_trimmed_event_count: record.capacity_trimmed_event_count,
                        capacity_trimmed_through_seq: record.capacity_trimmed_through_seq,
                        last_capacity_trimmed_at: record.last_capacity_trimmed_at,
                        oldest_pending_occurred_at: oldest_pending_occurred_at.clone(),
                    });
            }
            for event in record.events {
                if snapshot
                    .oldest_pending_occurred_at
                    .as_deref()
                    .is_none_or(|oldest| rfc3339_lt(event.occurred_at.as_str(), oldest))
                {
                    snapshot.oldest_pending_occurred_at = Some(event.occurred_at);
                }
            }
        }
        snapshot.high_risk_windows.sort_by(|left, right| {
            right
                .pending_event_count
                .cmp(&left.pending_event_count)
                .then_with(|| {
                    compare_optional_rfc3339_asc(
                        left.oldest_pending_occurred_at.as_deref(),
                        right.oldest_pending_occurred_at.as_deref(),
                    )
                })
                .then_with(|| left.tenant_id.cmp(&right.tenant_id))
                .then_with(|| left.principal_kind.cmp(&right.principal_kind))
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        snapshot
            .high_risk_windows
            .truncate(REALTIME_EVENT_WINDOW_HIGH_RISK_LIMIT);
        snapshot
    }
}

impl RealtimeCheckpointRecord {
    pub fn normalized(mut self) -> Self {
        self.acked_through_seq = self.acked_through_seq.min(self.latest_realtime_seq);
        self.trimmed_through_seq = self.trimmed_through_seq.min(self.latest_realtime_seq);
        self.capacity_trimmed_through_seq = self
            .capacity_trimmed_through_seq
            .min(self.trimmed_through_seq);
        if self.capacity_trimmed_event_count == 0 {
            self.capacity_trimmed_through_seq = 0;
            self.last_capacity_trimmed_at = None;
        }
        self
    }

    pub fn merge_monotonic(self, next: Self) -> Self {
        let latest_realtime_seq = self.latest_realtime_seq.max(next.latest_realtime_seq);
        let acked_through_seq = self.acked_through_seq.max(next.acked_through_seq);
        let trimmed_through_seq = self.trimmed_through_seq.max(next.trimmed_through_seq);
        let capacity_trimmed_event_count = self
            .capacity_trimmed_event_count
            .max(next.capacity_trimmed_event_count);
        let capacity_trimmed_through_seq = self
            .capacity_trimmed_through_seq
            .max(next.capacity_trimmed_through_seq);
        let last_capacity_trimmed_at =
            match (self.last_capacity_trimmed_at, next.last_capacity_trimmed_at) {
                (Some(left), Some(right)) => Some(max_rfc3339_string(left, right)),
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (None, None) => None,
            };
        Self {
            tenant_id: next.tenant_id,
            organization_id: next.organization_id,
            principal_kind: next.principal_kind,
            principal_id: next.principal_id,
            device_id: next.device_id,
            latest_realtime_seq,
            acked_through_seq,
            trimmed_through_seq,
            capacity_trimmed_event_count,
            capacity_trimmed_through_seq,
            last_capacity_trimmed_at,
            updated_at: max_rfc3339_string(self.updated_at, next.updated_at),
        }
        .normalized()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeDisconnectFenceRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub session_id: Option<String>,
    pub owner_node_id: String,
    pub disconnected_at: String,
    pub fence_token: String,
}

impl RealtimeDisconnectFenceRecord {
    pub fn merge_latest(self, next: Self) -> Self {
        if rfc3339_gt(next.disconnected_at.as_str(), self.disconnected_at.as_str()) {
            next
        } else {
            self
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeSubscriptionRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub items: Vec<RealtimeSubscription>,
    pub synced_at: String,
}

impl RealtimeSubscriptionRecord {
    pub fn matches_scope_event(&self, scope_type: &str, scope_id: &str, event_type: &str) -> bool {
        self.items.iter().any(|item| {
            item.scope_type == scope_type
                && item.scope_id == scope_id
                && (item.event_types.is_empty()
                    || item
                        .event_types
                        .iter()
                        .any(|candidate| candidate == event_type))
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RealtimeMatchingSubscriptionQuery<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub principal_kind: &'a str,
    pub principal_id: &'a str,
    pub scope_type: &'a str,
    pub scope_id: &'a str,
    pub event_type: &'a str,
    pub candidate_device_ids: &'a [String],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresenceStateRecord {
    pub tenant_id: String,
    #[serde(default = "default_organization_id")]
    pub organization_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub presence: PresenceClientView,
    pub resume_required: bool,
    pub updated_at: String,
}

impl PresenceStateRecord {
    pub fn online_seen_at(&self) -> Option<&str> {
        if !matches!(self.presence.status, PresenceStatus::Online) {
            return None;
        }
        self.presence.last_seen_at.as_deref()
    }

    pub fn is_online_seen_at_or_before(&self, cutoff_seen_at: &str) -> bool {
        self.online_seen_at()
            .map(|last_seen_at| rfc3339_le(last_seen_at, cutoff_seen_at))
            .unwrap_or(false)
    }

    pub fn into_expired_offline(mut self, expired_at: &str) -> Self {
        self.presence.status = PresenceStatus::Offline;
        self.presence.session_id = None;
        self.presence.last_seen_at = Some(expired_at.to_owned());
        self.resume_required = true;
        self.updated_at = expired_at.to_owned();
        self
    }
}

pub trait RealtimeCheckpointStore: Send + Sync {
    fn load_checkpoint(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError>;

    fn save_checkpoints(&self, records: Vec<RealtimeCheckpointRecord>)
    -> Result<(), ContractError>;

    fn save_checkpoint(&self, record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        self.save_checkpoints(vec![record])
    }
}

pub trait RealtimeEventWindowStore: Send + Sync {
    fn load_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeEventWindowRecord>, ContractError>;

    fn save_windows(&self, records: Vec<RealtimeEventWindowRecord>) -> Result<(), ContractError>;

    fn save_window(&self, record: RealtimeEventWindowRecord) -> Result<(), ContractError> {
        self.save_windows(vec![record])
    }

    fn clear_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError>;

    fn diagnostics_snapshot(&self)
    -> Result<RealtimeEventWindowDiagnosticsSnapshot, ContractError>;

    fn trim_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        acked_through_seq: u64,
    ) -> Result<(), ContractError>;
}

pub trait RealtimeDisconnectFenceStore: Send + Sync {
    fn load_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError>;

    fn save_fence(&self, record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError>;

    fn clear_fence(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError>;

    fn clear_fence_disconnected_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_disconnected_at: &str,
    ) -> Result<bool, ContractError>;

    fn clear_fence_if_matches(
        &self,
        expected: &RealtimeDisconnectFenceRecord,
    ) -> Result<bool, ContractError>;
}

pub trait RealtimeSubscriptionStore: Send + Sync {
    fn load_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<RealtimeSubscriptionRecord>, ContractError>;

    fn load_matching_subscriptions(
        &self,
        query: RealtimeMatchingSubscriptionQuery<'_>,
    ) -> Result<Vec<RealtimeSubscriptionRecord>, ContractError>;

    fn save_subscriptions(&self, record: RealtimeSubscriptionRecord) -> Result<(), ContractError>;

    fn clear_subscriptions(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<bool, ContractError>;

    fn clear_subscriptions_synced_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_synced_at: &str,
    ) -> Result<bool, ContractError>;
}

pub trait PresenceStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError>;

    fn save_state(&self, record: PresenceStateRecord) -> Result<(), ContractError>;

    fn list_states_for_principal(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> Result<Vec<PresenceStateRecord>, ContractError>;

    fn list_online_states_seen_at_or_before(
        &self,
        cutoff_seen_at: &str,
        limit: usize,
    ) -> Result<Vec<PresenceStateRecord>, ContractError>;

    fn expire_online_state_if_seen_at_or_before(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        cutoff_seen_at: &str,
        expired_at: &str,
    ) -> Result<Option<PresenceStateRecord>, ContractError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::realtime::RealtimeEvent;

    #[test]
    fn test_disconnect_fence_record_merge_keeps_latest_disconnect() {
        let latest = RealtimeDisconnectFenceRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_new".into()),
            owner_node_id: "node_b".into(),
            disconnected_at: "2026-05-06T00:00:02.000Z".into(),
            fence_token: "100001:user:1:d_pad:2026-05-06T00:00:02.000Z:s_new".into(),
        };
        let stale = RealtimeDisconnectFenceRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_old".into()),
            owner_node_id: "node_a".into(),
            disconnected_at: "2026-05-06T00:00:01.000Z".into(),
            fence_token: "100001:user:1:d_pad:2026-05-06T00:00:01.000Z:s_old".into(),
        };

        let merged = latest.merge_latest(stale);

        assert_eq!(merged.session_id.as_deref(), Some("s_new"));
        assert_eq!(merged.owner_node_id, "node_b");
        assert_eq!(merged.disconnected_at, "2026-05-06T00:00:02.000Z");
        assert_eq!(
            merged.fence_token,
            "100001:user:1:d_pad:2026-05-06T00:00:02.000Z:s_new"
        );
    }

    #[test]
    fn test_disconnect_fence_record_merge_compares_rfc3339_by_instant() {
        let latest = RealtimeDisconnectFenceRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_new".into()),
            owner_node_id: "node_b".into(),
            disconnected_at: "2026-05-06T00:00:00.100Z".into(),
            fence_token: "latest".into(),
        };
        let stale = RealtimeDisconnectFenceRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_old".into()),
            owner_node_id: "node_a".into(),
            disconnected_at: "2026-05-06T00:00:00Z".into(),
            fence_token: "stale".into(),
        };

        let merged = latest.clone().merge_latest(stale);

        assert_eq!(merged, latest);
    }

    #[test]
    fn test_realtime_checkpoint_merge_preserves_capacity_trim_metadata_monotonically() {
        let current = RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 12,
            acked_through_seq: 8,
            trimmed_through_seq: 8,
            capacity_trimmed_event_count: 4,
            capacity_trimmed_through_seq: 6,
            last_capacity_trimmed_at: Some("2026-05-09T10:00:06.000Z".into()),
            updated_at: "2026-05-09T10:00:08.000Z".into(),
        };
        let stale = RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 10,
            acked_through_seq: 7,
            trimmed_through_seq: 7,
            capacity_trimmed_event_count: 3,
            capacity_trimmed_through_seq: 5,
            last_capacity_trimmed_at: Some("2026-05-09T10:00:05.000Z".into()),
            updated_at: "2026-05-09T10:00:07.000Z".into(),
        };
        let advanced = RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 14,
            acked_through_seq: 9,
            trimmed_through_seq: 9,
            capacity_trimmed_event_count: 6,
            capacity_trimmed_through_seq: 9,
            last_capacity_trimmed_at: Some("2026-05-09T10:00:09.000Z".into()),
            updated_at: "2026-05-09T10:00:09.000Z".into(),
        };

        let merged_stale = current.clone().merge_monotonic(stale);

        assert_eq!(merged_stale.latest_realtime_seq, 12);
        assert_eq!(merged_stale.acked_through_seq, 8);
        assert_eq!(merged_stale.trimmed_through_seq, 8);
        assert_eq!(merged_stale.capacity_trimmed_event_count, 4);
        assert_eq!(merged_stale.capacity_trimmed_through_seq, 6);
        assert_eq!(
            merged_stale.last_capacity_trimmed_at.as_deref(),
            Some("2026-05-09T10:00:06.000Z")
        );

        let merged_advanced = current.merge_monotonic(advanced);

        assert_eq!(merged_advanced.latest_realtime_seq, 14);
        assert_eq!(merged_advanced.acked_through_seq, 9);
        assert_eq!(merged_advanced.trimmed_through_seq, 9);
        assert_eq!(merged_advanced.capacity_trimmed_event_count, 6);
        assert_eq!(merged_advanced.capacity_trimmed_through_seq, 9);
        assert_eq!(
            merged_advanced.last_capacity_trimmed_at.as_deref(),
            Some("2026-05-09T10:00:09.000Z")
        );
    }

    #[test]
    fn test_realtime_checkpoint_merge_compares_rfc3339_timestamps_by_instant() {
        let current = RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 12,
            acked_through_seq: 8,
            trimmed_through_seq: 8,
            capacity_trimmed_event_count: 4,
            capacity_trimmed_through_seq: 6,
            last_capacity_trimmed_at: Some("2026-05-09T10:00:00Z".into()),
            updated_at: "2026-05-09T10:00:00Z".into(),
        };
        let later = RealtimeCheckpointRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 12,
            acked_through_seq: 8,
            trimmed_through_seq: 8,
            capacity_trimmed_event_count: 4,
            capacity_trimmed_through_seq: 6,
            last_capacity_trimmed_at: Some("2026-05-09T10:00:00.100Z".into()),
            updated_at: "2026-05-09T10:00:00.100Z".into(),
        };

        let merged = current.merge_monotonic(later);

        assert_eq!(merged.updated_at, "2026-05-09T10:00:00.100Z");
        assert_eq!(
            merged.last_capacity_trimmed_at.as_deref(),
            Some("2026-05-09T10:00:00.100Z")
        );
    }

    #[test]
    fn test_presence_online_seen_at_or_before_compares_rfc3339_by_instant() {
        let presence = PresenceStateRecord {
            tenant_id: "100001".into(),
            organization_id: "0".into(),
            principal_kind: "user".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
            presence: PresenceClientView {
                tenant_id: "100001".into(),
                principal_id: "1".into(),
                device_id: "d_pad".into(),
                platform: None,
                session_id: Some("s_demo".into()),
                status: PresenceStatus::Online,
                last_sync_seq: 42,
                last_resume_at: None,
                last_seen_at: Some("2026-05-06T00:00:00.100Z".into()),
            },
            resume_required: false,
            updated_at: "2026-05-06T00:00:00.100Z".into(),
        };

        assert!(
            !presence.is_online_seen_at_or_before("2026-05-06T00:00:00Z"),
            "later fractional last_seen_at must not be treated as older than a whole-second cutoff"
        );
    }

    #[test]
    fn test_realtime_event_window_diagnostics_compare_rfc3339_by_instant() {
        let snapshot = RealtimeEventWindowDiagnosticsSnapshot::from_records(vec![
            realtime_window_record_with_time(
                "100001",
                "user",
                "1",
                "d_pad",
                "2026-05-09T10:00:00.100Z",
            ),
            realtime_window_record_with_time(
                "100001",
                "user",
                "1",
                "d_phone",
                "2026-05-09T10:00:00Z",
            ),
        ]);

        assert_eq!(
            snapshot.last_capacity_trimmed_at.as_deref(),
            Some("2026-05-09T10:00:00.100Z")
        );
        assert_eq!(
            snapshot.oldest_pending_occurred_at.as_deref(),
            Some("2026-05-09T10:00:00Z")
        );
        assert_eq!(snapshot.high_risk_windows[0].device_id, "d_phone");
    }

    #[test]
    fn test_realtime_event_window_diagnostics_lists_top_high_risk_windows_without_payloads() {
        let snapshot = RealtimeEventWindowDiagnosticsSnapshot::from_records(vec![
            realtime_window_record("100001", "user", "1", "d_one", 3, 0),
            realtime_window_record("100001", "user", "1", "d_two", 7, 7),
            realtime_window_record("100001", "agent", "a_demo", "d_agent", 5, 0),
            realtime_window_record("100001", "user", "1", "d_empty", 0, 9),
            realtime_window_record("100001", "user", "1", "d_three", 6, 0),
            realtime_window_record("100001", "user", "1", "d_four", 4, 0),
            realtime_window_record("100001", "user", "1", "d_five", 2, 0),
        ]);

        assert_eq!(snapshot.client_route_window_count, 7);
        assert_eq!(snapshot.pending_event_count, 27);
        assert_eq!(snapshot.max_client_route_window_event_count, 7);
        assert_eq!(snapshot.capacity_trimmed_event_count, 16);
        assert_eq!(snapshot.max_capacity_trimmed_through_seq, 9);
        assert_eq!(
            snapshot.last_capacity_trimmed_at.as_deref(),
            Some("2026-05-09T10:00:09.000Z")
        );
        assert_eq!(snapshot.high_risk_windows.len(), 5);
        assert_eq!(snapshot.high_risk_windows[0].device_id, "d_two");
        assert_eq!(snapshot.high_risk_windows[0].pending_event_count, 7);
        assert_eq!(
            snapshot.high_risk_windows[0].capacity_trimmed_event_count,
            7
        );
        assert_eq!(
            snapshot.high_risk_windows[0].capacity_trimmed_through_seq,
            7
        );
        assert_eq!(snapshot.high_risk_windows[1].device_id, "d_three");
        assert_eq!(snapshot.high_risk_windows[2].device_id, "d_agent");
        assert_eq!(snapshot.high_risk_windows[3].device_id, "d_four");
        assert_eq!(snapshot.high_risk_windows[4].device_id, "d_one");
        assert!(
            snapshot
                .high_risk_windows
                .iter()
                .all(|window| window.device_id != "d_empty"),
            "empty windows must not be listed as high-risk"
        );
    }

    #[test]
    fn test_realtime_principal_scope_key_extends_client_route_scope_prefix() {
        let principal = realtime_principal_scope_key("100001", "org_a", "user", "1");
        let client_route =
            realtime_client_route_scope_key("100001", "org_a", "user", "1", "d_pad");
        assert!(
            client_route.starts_with(principal.as_str()),
            "client route scope key must extend principal scope key prefix"
        );
        assert_ne!(
            realtime_principal_scope_key("100001", "org_a", "user", "1"),
            realtime_principal_scope_key("100001", "org_b", "user", "1"),
        );
    }

    fn realtime_window_record(
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        event_count: u64,
        trimmed_through_seq: u64,
    ) -> RealtimeEventWindowRecord {
        RealtimeEventWindowRecord {
            tenant_id: tenant_id.into(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            events: (trimmed_through_seq.saturating_add(1)
                ..trimmed_through_seq
                    .saturating_add(1)
                    .saturating_add(event_count))
                .map(|seq| RealtimeEvent {
                    tenant_id: tenant_id.into(),
                    principal_id: principal_id.into(),
                    device_id: device_id.into(),
                    realtime_seq: seq,
                    scope_type: "conversation".into(),
                    scope_id: "c_demo".into(),
                    event_type: "message.posted".into(),
                    delivery_class: "ephemeral".into(),
                    payload: format!(r#"{{"messageId":"msg_{seq}"}}"#),
                    occurred_at: format!("2026-05-09T10:00:{seq:02}.000Z"),
                })
                .collect(),
            trimmed_through_seq,
            capacity_trimmed_event_count: trimmed_through_seq,
            capacity_trimmed_through_seq: trimmed_through_seq,
            last_capacity_trimmed_at: (trimmed_through_seq > 0)
                .then(|| format!("2026-05-09T10:00:{trimmed_through_seq:02}.000Z")),
            updated_at: "2026-05-09T10:00:00.000Z".into(),
        }
    }

    fn realtime_window_record_with_time(
        tenant_id: &str,
        principal_kind: &str,
        principal_id: &str,
        device_id: &str,
        timestamp: &str,
    ) -> RealtimeEventWindowRecord {
        RealtimeEventWindowRecord {
            tenant_id: tenant_id.into(),
            organization_id: "0".into(),
            principal_kind: principal_kind.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            events: vec![RealtimeEvent {
                tenant_id: tenant_id.into(),
                principal_id: principal_id.into(),
                device_id: device_id.into(),
                realtime_seq: 2,
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_type: "message.posted".into(),
                delivery_class: "ephemeral".into(),
                payload: r#"{"messageId":"msg_2"}"#.into(),
                occurred_at: timestamp.into(),
            }],
            trimmed_through_seq: 1,
            capacity_trimmed_event_count: 1,
            capacity_trimmed_through_seq: 1,
            last_capacity_trimmed_at: Some(timestamp.into()),
            updated_at: timestamp.into(),
        }
    }
}
