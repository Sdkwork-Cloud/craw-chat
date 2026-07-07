use std::sync::Arc;

use audit_service::AuditRecord;
use im_app_context::AppContext;
use im_time::utc_now_rfc3339_millis;
use ops_service::dto::OpsHealthResponse;
use ops_service::state::OpsRuntime;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::modules::COMMERCIAL_RUNTIME_MODULES;

pub type PortalSnapshot = Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalWorkspaceView {
    pub name: String,
    pub slug: String,
    pub tier: String,
    pub region: String,
    pub support_plan: String,
    pub seats: i32,
    pub active_brands: i32,
    pub uptime: String,
}

pub fn build_portal_workspace_view() -> PortalWorkspaceView {
    let tier = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "development".into());
    let name = std::env::var("SDKWORK_IM_APPLICATION_NAME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Sdkwork IM".into());
    PortalWorkspaceView {
        name,
        slug: "sdkwork-im".into(),
        tier: tier.clone(),
        region: std::env::var("SDKWORK_IM_DEPLOYMENT_REGION")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "local".into()),
        support_plan: tier,
        seats: 1,
        active_brands: i32::try_from(COMMERCIAL_RUNTIME_MODULES.len()).unwrap_or(0),
        uptime: utc_now_rfc3339_millis(),
    }
}

pub fn build_portal_home_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    base_module_snapshot("home", &health)
}

pub fn build_portal_access_snapshot(ops: &OpsRuntime, auth: Option<&AppContext>) -> PortalSnapshot {
    let health = ops.health_view();
    let mut snapshot = base_module_snapshot("access", &health);
    if let Some(auth) = auth
        && let Some(map) = snapshot.as_object_mut()
    {
        map.insert("tenantId".into(), json!(auth.tenant_id));
        map.insert("principalId".into(), json!(auth.user_id));
    }
    snapshot
}

pub fn build_portal_dashboard_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    let metrics = dashboard_metrics(&health);
    let mut snapshot = base_module_snapshot("dashboard", &health);
    if let Some(map) = snapshot.as_object_mut() {
        map.insert("metrics".into(), metrics);
        map.insert(
            "activityTrends".into(),
            activity_trends_from_health(&health),
        );
        map.insert(
            "dataAvailability".into(),
            json!(ops_metrics_data_available(ops)),
        );
    }
    snapshot
}

pub fn build_portal_conversations_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    let message_count = health
        .projection_plane
        .metrics
        .conversation_snapshot_persist
        .success_count;
    let active_groups = health
        .projection_plane
        .replay
        .backlog_size
        .min(u64::from(u32::MAX)) as u32;
    let mut snapshot = base_module_snapshot("conversations", &health);
    if let Some(map) = snapshot.as_object_mut() {
        map.insert("dailyMessages".into(), json!(message_count));
        map.insert("messageCount".into(), json!(message_count));
        map.insert("activeGroups".into(), json!(active_groups));
        map.insert("groupCount".into(), json!(active_groups));
        map.insert(
            "activityTrends".into(),
            activity_trends_from_health(&health),
        );
        map.insert(
            "dataAvailability".into(),
            json!(ops_metrics_data_available(ops)),
        );
    }
    snapshot
}

pub fn build_portal_governance_snapshot(
    ops: &OpsRuntime,
    audit_records: &[AuditRecord],
) -> PortalSnapshot {
    let health = ops.health_view();
    let health_score = governance_health_score(&health, audit_records);
    let mut snapshot = base_module_snapshot("governance", &health);
    if let Some(map) = snapshot.as_object_mut() {
        map.insert("healthScore".into(), json!(health_score));
        map.insert("securityScore".into(), json!(health_score));
        map.insert("intercepts".into(), governance_intercepts(audit_records));
        map.insert("dataAvailability".into(), json!(!audit_records.is_empty()));
    }
    snapshot
}

pub fn build_portal_access_records_snapshot(
    ops: &OpsRuntime,
    audit_records: &[AuditRecord],
) -> PortalSnapshot {
    let health = ops.health_view();
    let mut snapshot = base_module_snapshot("access", &health);
    if let Some(map) = snapshot.as_object_mut() {
        map.insert(
            "items".into(),
            json!(
                audit_records
                    .iter()
                    .take(20)
                    .map(audit_record_to_json)
                    .collect::<Vec<_>>()
            ),
        );
        map.insert(
            "records".into(),
            map.get("items").cloned().unwrap_or(json!([])),
        );
        map.insert("dataAvailability".into(), json!(!audit_records.is_empty()));
    }
    snapshot
}

pub fn build_portal_automation_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    base_module_snapshot("automation", &ops.health_view())
}

pub fn build_portal_media_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    base_module_snapshot("media", &ops.health_view())
}

pub fn build_portal_realtime_snapshot(ops: &OpsRuntime) -> PortalSnapshot {
    let health = ops.health_view();
    let mut snapshot = base_module_snapshot("realtime", &health);
    if let Some(map) = snapshot.as_object_mut() {
        map.insert(
            "realtimeInbox".into(),
            serde_json::to_value(&health.realtime_inbox).unwrap_or(json!({})),
        );
        map.insert(
            "dataAvailability".into(),
            json!(ops_metrics_data_available(ops)),
        );
    }
    snapshot
}

pub fn build_portal_snapshot_for_section(
    section: &str,
    ops: Arc<OpsRuntime>,
    auth: Option<&AppContext>,
    audit_runtime: Option<Arc<audit_service::AuditRuntime>>,
) -> Option<PortalSnapshot> {
    let audit_records = match (auth, audit_runtime.as_ref()) {
        (Some(auth), Some(runtime)) => runtime.list_records(auth).unwrap_or_default(),
        _ => Vec::new(),
    };
    match section {
        "access" if auth.is_some() => Some(build_portal_access_records_snapshot(
            ops.as_ref(),
            audit_records.as_slice(),
        )),
        "access" => Some(build_portal_access_snapshot(ops.as_ref(), auth)),
        "automation" => Some(build_portal_automation_snapshot(ops.as_ref())),
        "conversations" => Some(build_portal_conversations_snapshot(ops.as_ref())),
        "dashboard" => Some(build_portal_dashboard_snapshot(ops.as_ref())),
        "governance" => Some(build_portal_governance_snapshot(
            ops.as_ref(),
            audit_records.as_slice(),
        )),
        "home" => Some(build_portal_home_snapshot(ops.as_ref())),
        "media" => Some(build_portal_media_snapshot(ops.as_ref())),
        "realtime" => Some(build_portal_realtime_snapshot(ops.as_ref())),
        _ => None,
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
        .success_count
        > 0
        || health.projection_plane.replay.replayed_event_count > 0
        || lag_wired
}

fn base_module_snapshot(section: &str, health: &OpsHealthResponse) -> PortalSnapshot {
    json!({
        "section": section,
        "enabledModules": COMMERCIAL_RUNTIME_MODULES,
        "sidebarModules": COMMERCIAL_RUNTIME_MODULES,
        "modules": { "items": COMMERCIAL_RUNTIME_MODULES },
        "features": {
            "chat": true,
            "contacts": true,
            "workspace": true,
        },
        "opsStatus": health.status,
        "generatedAt": utc_now_rfc3339_millis(),
    })
}

fn dashboard_metrics(health: &OpsHealthResponse) -> Value {
    let active_connections = health.realtime_inbox.pending_event_count;
    let message_count = health
        .projection_plane
        .metrics
        .conversation_snapshot_persist
        .success_count;
    let group_count = health.projection_plane.replay.backlog_size;
    let storage_ops = health
        .projection_plane
        .metrics
        .conversation_snapshot_persist
        .success_count
        + health
            .projection_plane
            .metrics
            .client_route_sync_snapshot_persist
            .success_count;
    json!({
        "users": { "totalUsers": { "value": active_connections, "count": active_connections, "total": active_connections } },
        "messages": { "dailyMessages": { "value": message_count, "count": message_count, "daily": message_count } },
        "groups": { "activeGroups": { "value": group_count, "count": group_count, "active": group_count } },
        "storage": {
            "storageUsage": {
                "value": storage_ops,
                "used": storage_ops,
                "usedGb": storage_ops,
                "displayValue": format!("{storage_ops} ops"),
            }
        },
    })
}

fn activity_trends_from_health(health: &OpsHealthResponse) -> Value {
    let daily = health
        .projection_plane
        .metrics
        .conversation_snapshot_persist
        .success_count;
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    json!(
        days.iter()
            .map(|day| json!({ "day": day, "value": daily }))
            .collect::<Vec<_>>()
    )
}

fn governance_health_score(health: &OpsHealthResponse, audit_records: &[AuditRecord]) -> i64 {
    if health.status != "ok" {
        return 40;
    }
    if audit_records.is_empty() {
        return -1;
    }
    let critical = audit_records
        .iter()
        .filter(|record| record.action.contains("critical"))
        .count();
    let high = audit_records
        .iter()
        .filter(|record| record.action.contains("failed") || record.action.contains("denied"))
        .count();
    i64::max(
        0,
        100 - i64::try_from(critical * 15 + high * 8).unwrap_or(100),
    )
}

fn governance_intercepts(audit_records: &[AuditRecord]) -> Value {
    let mut critical = 0_u64;
    let mut high = 0_u64;
    let mut warning = 0_u64;
    let mut info = 0_u64;
    for record in audit_records {
        let action = record.action.to_ascii_lowercase();
        if action.contains("critical") {
            critical += 1;
        } else if action.contains("failed") || action.contains("denied") {
            high += 1;
        } else if action.contains("warning") {
            warning += 1;
        } else {
            info += 1;
        }
    }
    json!([
        { "id": "critical", "title": "Critical security events", "count": critical, "level": "critical" },
        { "id": "high", "title": "High risk events", "count": high, "level": "high" },
        { "id": "warning", "title": "Policy warnings", "count": warning, "level": "warning" },
        { "id": "info", "title": "Informational audit events", "count": info, "level": "info" },
    ])
}

fn audit_record_to_json(record: &AuditRecord) -> Value {
    json!({
        "recordId": record.record_id,
        "id": record.record_id,
        "action": record.action,
        "eventType": record.action,
        "actorId": record.actor_id,
        "createdBy": record.actor_id,
        "userId": record.actor_id,
        "tenantId": record.tenant_id,
        "recordedAt": record.recorded_at,
        "createdAt": record.recorded_at,
        "severity": audit_severity(&record.action),
        "level": audit_severity(&record.action),
        "status": audit_severity(&record.action),
    })
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
        "info"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ops_service::state::OpsRuntime;

    #[test]
    fn governance_snapshot_fail_closed_without_audit_records() {
        let snapshot = build_portal_governance_snapshot(&OpsRuntime::default(), &[]);
        assert_eq!(snapshot["healthScore"], -1);
        assert_eq!(snapshot["dataAvailability"], false);
        assert_eq!(snapshot["section"], "governance");
    }

    #[test]
    fn dashboard_snapshot_exposes_ops_metrics() {
        let snapshot = build_portal_dashboard_snapshot(&OpsRuntime::default());
        assert_eq!(snapshot["section"], "dashboard");
        assert_eq!(snapshot["dataAvailability"], false);
        assert!(snapshot["metrics"]["messages"]["dailyMessages"]["count"].is_number());
    }

    #[test]
    fn unknown_portal_section_returns_none() {
        let ops = Arc::new(OpsRuntime::default());
        assert!(build_portal_snapshot_for_section("unknown", ops, None, None).is_none());
    }
}
