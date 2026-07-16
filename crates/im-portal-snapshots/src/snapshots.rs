use std::sync::Arc;

use audit_service::{AuditRecord, AuditRecordSample};
use im_app_context::AppContext;
use im_time::utc_now_rfc3339_millis;
use ops_service::dto::OpsHealthResponse;
use ops_service::state::OpsRuntime;
use serde::{Deserialize, Serialize};

const PORTAL_AUDIT_SAMPLE_LIMIT: usize = 20;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalWorkspaceView {
    pub name: String,
    pub slug: String,
    pub environment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seats: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_brands: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalSnapshotMeta {
    pub section: String,
    pub generated_at: String,
    pub ops_status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalDataState {
    Available,
    Partial,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalDataAvailability {
    pub state: PortalDataState,
    pub source: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalModuleSnapshot {
    pub meta: PortalSnapshotMeta,
    pub availability: PortalDataAvailability,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalOperationalMetrics {
    pub client_route_window_count: String,
    pub pending_realtime_event_count: String,
    pub conversation_snapshot_persist_success_count: String,
    pub conversation_snapshot_persist_failure_count: String,
    pub projection_replay_backlog_size: String,
    pub projection_replayed_event_count: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalDashboardSnapshot {
    pub meta: PortalSnapshotMeta,
    pub availability: PortalDataAvailability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PortalOperationalMetrics>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalConversationSnapshot {
    pub meta: PortalSnapshotMeta,
    pub availability: PortalDataAvailability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<PortalConversationProjectionMetrics>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalConversationProjectionMetrics {
    pub persist_success_count: String,
    pub persist_failure_count: String,
    pub restore_success_count: String,
    pub replay_backlog_size: String,
    pub replayed_event_count: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalAuditRecordView {
    pub record_id: String,
    pub action: String,
    pub actor_id: String,
    pub recorded_at: String,
    pub severity: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalAccessSnapshot {
    pub meta: PortalSnapshotMeta,
    pub availability: PortalDataAvailability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    pub recent_items: Vec<PortalAuditRecordView>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalGovernanceRiskSample {
    pub critical_count: String,
    pub high_count: String,
    pub warning_count: String,
    pub informational_count: String,
}

impl Default for PortalGovernanceRiskSample {
    fn default() -> Self {
        Self {
            critical_count: "0".into(),
            high_count: "0".into(),
            warning_count: "0".into(),
            informational_count: "0".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalGovernanceSnapshot {
    pub meta: PortalSnapshotMeta,
    pub availability: PortalDataAvailability,
    pub sampled_event_count: String,
    pub risk_sample: PortalGovernanceRiskSample,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalRealtimeMetrics {
    pub client_route_window_count: String,
    pub pending_event_count: String,
    pub max_client_route_window_event_count: String,
    pub client_route_window_capacity: String,
    pub max_client_route_window_usage_permille: u32,
    pub capacity_trimmed_event_count: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_pending_occurred_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalRealtimeSnapshot {
    pub meta: PortalSnapshotMeta,
    pub availability: PortalDataAvailability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PortalRealtimeMetrics>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortalSnapshot {
    Access(PortalAccessSnapshot),
    Conversation(PortalConversationSnapshot),
    Dashboard(PortalDashboardSnapshot),
    Governance(PortalGovernanceSnapshot),
    Module(PortalModuleSnapshot),
    Realtime(PortalRealtimeSnapshot),
}

pub fn build_portal_workspace_view() -> PortalWorkspaceView {
    PortalWorkspaceView {
        name: non_empty_env("SDKWORK_IM_APPLICATION_NAME").unwrap_or_else(|| "Sdkwork IM".into()),
        slug: "sdkwork-im".into(),
        environment: non_empty_env("SDKWORK_IM_ENVIRONMENT")
            .unwrap_or_else(|| "development".into()),
        region: non_empty_env("SDKWORK_IM_DEPLOYMENT_REGION"),
        tier: non_empty_env("SDKWORK_IM_COMMERCIAL_TIER"),
        support_plan: non_empty_env("SDKWORK_IM_SUPPORT_PLAN"),
        seats: positive_u64_env("SDKWORK_IM_LICENSED_SEATS"),
        active_brands: positive_u64_env("SDKWORK_IM_ACTIVE_BRANDS"),
    }
}

pub fn build_portal_home_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    PortalSnapshot::Module(unavailable_module_snapshot("home", ops))
}

pub fn build_portal_access_snapshot(ops: &OpsRuntime, auth: Option<&AppContext>) -> PortalSnapshot {
    PortalSnapshot::Access(PortalAccessSnapshot {
        meta: snapshot_meta("access", &ops.health_view(), false),
        availability: unavailable_availability(
            "audit",
            "authenticated audit sample was not supplied",
        ),
        tenant_id: auth.map(|value| value.tenant_id.clone()),
        principal_id: auth.map(|value| value.user_id.clone()),
        recent_items: Vec::new(),
        has_more: false,
    })
}

pub fn build_portal_dashboard_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    let available = ops_metrics_data_available(ops);
    PortalSnapshot::Dashboard(PortalDashboardSnapshot {
        meta: snapshot_meta("dashboard", &health, available),
        availability: metrics_availability(available),
        metrics: available.then(|| operational_metrics(&health)),
    })
}

pub fn build_portal_conversations_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    let available = ops_metrics_data_available(ops);
    let metrics = &health.projection_plane.metrics;
    PortalSnapshot::Conversation(PortalConversationSnapshot {
        meta: snapshot_meta("conversations", &health, available),
        availability: metrics_availability(available),
        projection: available.then(|| PortalConversationProjectionMetrics {
            persist_success_count: metrics
                .conversation_snapshot_persist
                .success_count
                .to_string(),
            persist_failure_count: metrics
                .conversation_snapshot_persist
                .failure_count
                .to_string(),
            restore_success_count: metrics
                .conversation_snapshot_restore
                .success_count
                .to_string(),
            replay_backlog_size: health.projection_plane.replay.backlog_size.to_string(),
            replayed_event_count: health
                .projection_plane
                .replay
                .replayed_event_count
                .to_string(),
        }),
    })
}

pub fn build_portal_governance_snapshot(
    ops: &OpsRuntime,
    audit_sample: &AuditRecordSample,
) -> PortalSnapshot {
    let health = ops.health_view();
    PortalSnapshot::Governance(PortalGovernanceSnapshot {
        meta: snapshot_meta("governance", &health, true),
        availability: audit_sample_availability(audit_sample),
        sampled_event_count: audit_sample.items.len().to_string(),
        risk_sample: governance_risk_sample(audit_sample.items.as_slice()),
    })
}

pub fn build_portal_access_records_snapshot(
    ops: &OpsRuntime,
    auth: Option<&AppContext>,
    audit_sample: &AuditRecordSample,
) -> PortalSnapshot {
    let health = ops.health_view();
    PortalSnapshot::Access(PortalAccessSnapshot {
        meta: snapshot_meta("access", &health, true),
        availability: audit_sample_availability(audit_sample),
        tenant_id: auth.map(|value| value.tenant_id.clone()),
        principal_id: auth.map(|value| value.user_id.clone()),
        recent_items: audit_sample.items.iter().map(audit_record_view).collect(),
        has_more: audit_sample.has_more,
    })
}

pub fn build_portal_automation_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    PortalSnapshot::Module(unavailable_module_snapshot("automation", ops))
}

pub fn build_portal_media_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    PortalSnapshot::Module(unavailable_module_snapshot("media", ops))
}

pub fn build_portal_realtime_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    let available = ops_metrics_data_available(ops);
    let realtime = &health.realtime_inbox;
    PortalSnapshot::Realtime(PortalRealtimeSnapshot {
        meta: snapshot_meta("realtime", &health, available),
        availability: metrics_availability(available),
        metrics: available.then(|| PortalRealtimeMetrics {
            client_route_window_count: realtime.client_route_window_count.to_string(),
            pending_event_count: realtime.pending_event_count.to_string(),
            max_client_route_window_event_count: realtime
                .max_client_route_window_event_count
                .to_string(),
            client_route_window_capacity: realtime.client_route_window_capacity.to_string(),
            max_client_route_window_usage_permille: u32::try_from(
                realtime.max_client_route_window_usage_permille.min(1000),
            )
            .unwrap_or(1000),
            capacity_trimmed_event_count: realtime.capacity_trimmed_event_count.to_string(),
            oldest_pending_occurred_at: realtime.oldest_pending_occurred_at.clone(),
        }),
    })
}

pub fn build_portal_snapshot_for_section(
    section: &str,
    ops: Arc<OpsRuntime>,
    auth: Option<&AppContext>,
    audit_sample: Option<&AuditRecordSample>,
) -> Option<PortalSnapshot> {
    match section {
        "access" if audit_sample.is_some() => Some(build_portal_access_records_snapshot(
            ops.as_ref(),
            auth,
            audit_sample.expect("checked audit sample"),
        )),
        "access" => Some(build_portal_access_snapshot(ops.as_ref(), auth)),
        "automation" => Some(build_portal_automation_snapshot(ops.as_ref())),
        "conversations" => Some(build_portal_conversations_snapshot(ops.as_ref())),
        "dashboard" => Some(build_portal_dashboard_snapshot(ops.as_ref())),
        "governance" if audit_sample.is_some() => Some(build_portal_governance_snapshot(
            ops.as_ref(),
            audit_sample.expect("checked audit sample"),
        )),
        "governance" => Some(PortalSnapshot::Governance(PortalGovernanceSnapshot {
            meta: snapshot_meta("governance", &ops.health_view(), false),
            availability: unavailable_availability("audit", "audit sample was not supplied"),
            sampled_event_count: "0".into(),
            risk_sample: PortalGovernanceRiskSample::default(),
        })),
        "home" => Some(build_portal_home_snapshot(ops.as_ref())),
        "media" => Some(build_portal_media_snapshot(ops.as_ref())),
        "realtime" => Some(build_portal_realtime_snapshot(ops.as_ref())),
        _ => None,
    }
}

fn unavailable_module_snapshot(section: &str, ops: &OpsRuntime) -> PortalModuleSnapshot {
    PortalModuleSnapshot {
        meta: snapshot_meta(section, &ops.health_view(), false),
        availability: unavailable_availability(
            section,
            "authoritative section data source is not wired",
        ),
    }
}

fn snapshot_meta(
    section: &str,
    health: &OpsHealthResponse,
    data_available: bool,
) -> PortalSnapshotMeta {
    let ops_status = if data_available || health.status != "ok" {
        health.status.clone()
    } else {
        "unknown".into()
    };
    PortalSnapshotMeta {
        section: section.into(),
        generated_at: utc_now_rfc3339_millis(),
        ops_status,
    }
}

fn metrics_availability(available: bool) -> PortalDataAvailability {
    if available {
        PortalDataAvailability {
            state: PortalDataState::Available,
            source: "ops".into(),
            complete: true,
            reason: None,
        }
    } else {
        unavailable_availability("ops", "ops metrics have not reported authoritative data")
    }
}

fn audit_sample_availability(sample: &AuditRecordSample) -> PortalDataAvailability {
    PortalDataAvailability {
        state: if sample.has_more {
            PortalDataState::Partial
        } else {
            PortalDataState::Available
        },
        source: "audit".into(),
        complete: !sample.has_more,
        reason: sample
            .has_more
            .then(|| format!("showing the latest {PORTAL_AUDIT_SAMPLE_LIMIT} audit records")),
    }
}

fn unavailable_availability(source: &str, reason: &str) -> PortalDataAvailability {
    PortalDataAvailability {
        state: PortalDataState::Unavailable,
        source: source.into(),
        complete: false,
        reason: Some(reason.into()),
    }
}

fn operational_metrics(health: &OpsHealthResponse) -> PortalOperationalMetrics {
    PortalOperationalMetrics {
        client_route_window_count: health.realtime_inbox.client_route_window_count.to_string(),
        pending_realtime_event_count: health.realtime_inbox.pending_event_count.to_string(),
        conversation_snapshot_persist_success_count: health
            .projection_plane
            .metrics
            .conversation_snapshot_persist
            .success_count
            .to_string(),
        conversation_snapshot_persist_failure_count: health
            .projection_plane
            .metrics
            .conversation_snapshot_persist
            .failure_count
            .to_string(),
        projection_replay_backlog_size: health.projection_plane.replay.backlog_size.to_string(),
        projection_replayed_event_count: health
            .projection_plane
            .replay
            .replayed_event_count
            .to_string(),
    }
}

fn ops_metrics_data_available(ops: &OpsRuntime) -> bool {
    let health = ops.health_view();
    if health.status != "ok" {
        return false;
    }
    let lag_wired = !ops.lag_view().items.is_empty();
    health
        .projection_plane
        .metrics
        .conversation_snapshot_persist
        .attempt_count
        > 0
        || health.projection_plane.replay.replayed_event_count > 0
        || health.realtime_inbox.client_route_window_count > 0
        || lag_wired
}

fn governance_risk_sample(records: &[AuditRecord]) -> PortalGovernanceRiskSample {
    let mut critical_count = 0_u64;
    let mut high_count = 0_u64;
    let mut warning_count = 0_u64;
    let mut informational_count = 0_u64;
    for record in records {
        match audit_severity(record.action.as_str()) {
            "critical" => critical_count += 1,
            "high" => high_count += 1,
            "warning" => warning_count += 1,
            _ => informational_count += 1,
        }
    }
    PortalGovernanceRiskSample {
        critical_count: critical_count.to_string(),
        high_count: high_count.to_string(),
        warning_count: warning_count.to_string(),
        informational_count: informational_count.to_string(),
    }
}

fn audit_record_view(record: &AuditRecord) -> PortalAuditRecordView {
    PortalAuditRecordView {
        record_id: record.record_id.clone(),
        action: record.action.clone(),
        actor_id: record.actor_id.clone(),
        recorded_at: record.recorded_at.clone(),
        severity: audit_severity(record.action.as_str()).into(),
    }
}

fn audit_severity(action: &str) -> &'static str {
    let action = action.to_ascii_lowercase();
    if action.contains("critical") {
        "critical"
    } else if action.contains("failed") || action.contains("denied") {
        "high"
    } else if action.contains("warning") {
        "warning"
    } else {
        "informational"
    }
}

fn non_empty_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn positive_u64_env(name: &str) -> Option<String> {
    non_empty_env(name)
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .map(|value| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_snapshot_does_not_fabricate_metrics() {
        let PortalSnapshot::Dashboard(snapshot) =
            build_portal_dashboard_snapshot(&OpsRuntime::default())
        else {
            panic!("dashboard builder must return a dashboard snapshot");
        };
        assert_eq!(snapshot.meta.section, "dashboard");
        assert_eq!(snapshot.meta.ops_status, "unknown");
        assert_eq!(snapshot.availability.state, PortalDataState::Unavailable);
        assert!(snapshot.metrics.is_none());
    }

    #[test]
    fn governance_snapshot_marks_truncated_audit_data_as_partial() {
        let sample = AuditRecordSample {
            items: Vec::new(),
            has_more: true,
        };
        let PortalSnapshot::Governance(snapshot) =
            build_portal_governance_snapshot(&OpsRuntime::default(), &sample)
        else {
            panic!("governance builder must return a governance snapshot");
        };
        assert_eq!(snapshot.availability.state, PortalDataState::Partial);
        assert!(!snapshot.availability.complete);
    }

    #[test]
    fn unknown_portal_section_returns_none() {
        let ops = Arc::new(OpsRuntime::default());
        assert!(build_portal_snapshot_for_section("unknown", ops, None, None).is_none());
    }
}
