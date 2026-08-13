use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceHealthView {
    pub service: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpsHealthResponse {
    pub status: String,
    pub items: Vec<ServiceHealthView>,
    pub realtime_inbox: RealtimeInboxDiagnosticsView,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNodeView {
    pub node_id: String,
    pub profile: String,
    pub bind_addr: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub client_route_count: usize,
    pub owned_scopes: Vec<String>,
    pub services: Vec<ServiceHealthView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterView {
    pub nodes: Vec<ClusterNodeView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LagItem {
    pub component: String,
    pub scope_id: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub current_offset: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub committed_offset: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub lag: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LagView {
    pub items: Vec<LagItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirInspectionItem {
    pub file_name: String,
    pub path: String,
    pub required: bool,
    pub exists: bool,
    pub parseable: bool,
    pub status: String,
    pub size_bytes: Option<u64>,
    pub parse_error: Option<String>,
    pub recommended_action: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirInspectionView {
    pub status: String,
    pub runtime_dir: Option<String>,
    pub state_dir: Option<String>,
    pub healthy_file_count: usize,
    pub missing_file_count: usize,
    pub corrupt_file_count: usize,
    pub files: Vec<RuntimeDirInspectionItem>,
}

impl RuntimeDirInspectionView {
    pub fn unmanaged() -> Self {
        Self {
            status: "unmanaged".into(),
            runtime_dir: None,
            state_dir: None,
            healthy_file_count: 0,
            missing_file_count: 0,
            corrupt_file_count: 0,
            files: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionPurgeResponse {
    pub generated_at: String,
    pub batch_size: i64,
    pub commit_journal_deleted: u64,
    pub conversation_messages_deleted: u64,
    pub message_media_refs_deleted: u64,
    pub outbox_events_deleted: u64,
    pub inbox_events_deleted: u64,
    pub realtime_device_events_deleted: u64,
    pub rtc_sessions_deleted: u64,
    pub invitations_deleted: u64,
    pub audit_records_deleted: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideEffectOutboxDiagnosticsView {
    pub name: String,
    pub status: String,
    pub pending_count: u64,
    pub delivered_count: u64,
    pub failed_attempt_count: u64,
    pub oldest_pending_created_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeInboxDiagnosticsView {
    pub status: String,
    pub client_route_window_count: u64,
    pub pending_event_count: u64,
    pub max_client_route_window_event_count: u64,
    pub client_route_window_capacity: u64,
    pub max_client_route_window_usage_permille: u64,
    pub max_trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub max_capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub oldest_pending_occurred_at: Option<String>,
    pub high_risk_windows: Vec<RealtimeInboxHighRiskWindowView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeInboxHighRiskWindowView {
    pub tenant_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub pending_event_count: u64,
    pub trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub usage_permille: u64,
    pub oldest_pending_occurred_at: Option<String>,
}

impl Default for RealtimeInboxDiagnosticsView {
    fn default() -> Self {
        Self {
            status: "unavailable".into(),
            client_route_window_count: 0,
            pending_event_count: 0,
            max_client_route_window_event_count: 0,
            client_route_window_capacity: 0,
            max_client_route_window_usage_permille: 0,
            max_trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            max_capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            oldest_pending_occurred_at: None,
            high_risk_windows: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticBundle {
    pub generated_at: String,
    pub profile: String,
    pub node_id: String,
    pub bind_addr: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub owned_scopes: Vec<String>,
    pub services: Vec<ServiceHealthView>,
    pub lag: Vec<LagItem>,
    pub client_routes: Vec<RouteOwnershipView>,
    pub provider_bindings: Vec<ProviderBindingSnapshotView>,
    pub provider_binding_drift: ProviderBindingDriftView,
    pub side_effect_outboxes: Vec<SideEffectOutboxDiagnosticsView>,
    pub realtime_inbox: RealtimeInboxDiagnosticsView,
    pub collection_limit: u32,
    pub collection_totals: BTreeMap<String, u64>,
    pub truncated_collections: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteOwnershipView {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub connection_kind: String,
    pub bound_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingItemView {
    pub domain: String,
    pub default_plugin_id: Option<String>,
    pub selected_plugin_id: Option<String>,
    pub selection_source: String,
    pub tenant_override_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingSnapshotView {
    pub interface_version: String,
    pub tenant_id: Option<String>,
    pub effective_bindings: Vec<ProviderBindingItemView>,
    pub precedence: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingsView {
    pub items: Vec<ProviderBindingSnapshotView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingDriftItemView {
    pub tenant_id: String,
    pub domain: String,
    pub baseline_selected_plugin_id: Option<String>,
    pub selected_plugin_id: Option<String>,
    pub baseline_selection_source: String,
    pub selection_source: String,
    pub drift_kind: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingDriftView {
    pub baseline_tenant_id: Option<String>,
    pub items: Vec<ProviderBindingDriftItemView>,
}

/// Commit-journal replay status view (`GET /backend/v3/api/ops/replay_status`).
///
/// `status` is `enabled` when the shared PostgreSQL journal pool is configured
/// and the head query succeeded, `not_configured` when `SDKWORK_DATABASE_URL`
/// is absent, and `unavailable` when the pool or the query failed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalReplayStatusView {
    pub status: String,
    pub mode: String,
    pub database_configured: bool,
    pub journal_ready: bool,
    pub total_commits: Option<u64>,
    pub head_commit_offset: Option<i64>,
    pub latest_occurred_at: Option<String>,
    pub detail: Option<String>,
    pub generated_at: String,
}
