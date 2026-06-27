use std::collections::BTreeMap;

use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

use crate::TimelineProjectionService;
use crate::projection::ProjectionError;

const MAX_RECENT_PROJECTION_EVENTS: usize = 20;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionOperationMetricView {
    pub attempt_count: u64,
    pub success_count: u64,
    pub failure_count: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneMetricsView {
    pub conversation_snapshot_persist: ProjectionOperationMetricView,
    pub conversation_snapshot_restore: ProjectionOperationMetricView,
    pub client_route_sync_snapshot_persist: ProjectionOperationMetricView,
    pub client_route_sync_snapshot_restore: ProjectionOperationMetricView,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionReplayMetricsView {
    pub backlog_size: u64,
    pub replayed_event_count: u64,
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionUpdateDelayView {
    pub timeline_ms: u64,
    pub inbox_ms: u64,
    pub source_event_type: Option<String>,
    pub scope_id: Option<String>,
    pub recorded_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionLagItemView {
    pub component: String,
    pub scope_id: String,
    pub current_offset: u64,
    pub committed_offset: u64,
    pub lag: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionTraceView {
    pub trace_id: String,
    pub operation: String,
    pub scope_type: String,
    pub scope_id: String,
    pub outcome: String,
    pub recorded_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionLogView {
    pub level: String,
    pub code: String,
    pub operation: String,
    pub scope_type: String,
    pub scope_id: String,
    pub message: String,
    pub recorded_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneObservabilityView {
    pub status: String,
    pub metrics: ProjectionPlaneMetricsView,
    pub replay: ProjectionReplayMetricsView,
    pub rebuild_duration_ms: u64,
    pub update_delay: ProjectionUpdateDelayView,
    pub last_failure_code: Option<String>,
    pub last_failure_message: Option<String>,
    pub traces: Vec<ProjectionTraceView>,
    pub logs: Vec<ProjectionLogView>,
}

#[derive(Clone, Debug)]
pub(super) enum ProjectionSnapshotOperation {
    ConversationSnapshotPersist,
    ConversationSnapshotRestore,
    ClientRouteSyncSnapshotPersist,
    ClientRouteSyncSnapshotRestore,
}

#[derive(Default)]
pub(super) struct ProjectionObservabilityState {
    status: Option<String>,
    metrics: ProjectionPlaneMetricsView,
    replay: ProjectionReplayMetricsView,
    rebuild_duration_ms: u64,
    update_delay: ProjectionUpdateDelayView,
    live_lag: BTreeMap<String, ProjectionLagItemView>,
    last_failure_code: Option<String>,
    last_failure_message: Option<String>,
    traces: Vec<ProjectionTraceView>,
    logs: Vec<ProjectionLogView>,
    trace_seq: u64,
}

impl TimelineProjectionService {
    pub fn projection_plane_observability(&self) -> ProjectionPlaneObservabilityView {
        super::lock_projection_mutex(&self.observability, "projection observability").snapshot()
    }

    pub fn projection_live_lag_items(&self) -> Vec<ProjectionLagItemView> {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .live_lag_items()
    }

    pub(super) fn record_projection_snapshot_success(
        &self,
        operation: ProjectionSnapshotOperation,
        scope_type: &str,
        scope_id: &str,
        message: impl Into<String>,
    ) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_success(operation, scope_type, scope_id, message.into());
    }

    pub(super) fn record_projection_snapshot_failure(
        &self,
        operation: ProjectionSnapshotOperation,
        scope_type: &str,
        scope_id: &str,
        error: &ProjectionError,
    ) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_failure(operation, scope_type, scope_id, error);
    }

    pub fn record_projection_replay_metrics(
        &self,
        backlog_size: u64,
        replayed_event_count: u64,
        duration_ms: u64,
    ) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_replay_metrics(backlog_size, replayed_event_count, duration_ms);
    }

    pub fn record_projection_rebuild_duration(&self, duration_ms: u64) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_rebuild_duration(duration_ms);
    }

    pub(super) fn record_projection_live_lag_observed(&self, scope_id: &str, current_offset: u64) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_live_lag_observed(scope_id, current_offset);
    }

    pub(super) fn record_projection_live_lag_committed(
        &self,
        scope_id: &str,
        committed_offset: u64,
    ) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_live_lag_committed(scope_id, committed_offset);
    }

    pub fn record_projection_update_delay(
        &self,
        source_event_type: &str,
        scope_id: &str,
        timeline_ms: u64,
        inbox_ms: u64,
    ) {
        super::lock_projection_mutex(&self.observability, "projection observability")
            .record_update_delay(source_event_type, scope_id, timeline_ms, inbox_ms);
    }
}

impl ProjectionObservabilityState {
    fn snapshot(&self) -> ProjectionPlaneObservabilityView {
        ProjectionPlaneObservabilityView {
            status: self.status.clone().unwrap_or_else(|| "idle".into()),
            metrics: self.metrics.clone(),
            replay: self.replay.clone(),
            rebuild_duration_ms: self.rebuild_duration_ms,
            update_delay: self.update_delay.clone(),
            last_failure_code: self.last_failure_code.clone(),
            last_failure_message: self.last_failure_message.clone(),
            traces: self.traces.clone(),
            logs: self.logs.clone(),
        }
    }

    fn record_success(
        &mut self,
        operation: ProjectionSnapshotOperation,
        scope_type: &str,
        scope_id: &str,
        message: String,
    ) {
        let counter = operation.metric_mut(&mut self.metrics);
        counter.attempt_count += 1;
        counter.success_count += 1;
        self.status = Some("ok".into());

        self.trace_seq += 1;
        let trace_id = format!("projection-trace-{}", self.trace_seq);
        let recorded_at = utc_now_rfc3339_millis();
        push_bounded(
            &mut self.traces,
            ProjectionTraceView {
                trace_id,
                operation: operation.label().into(),
                scope_type: scope_type.into(),
                scope_id: scope_id.into(),
                outcome: "success".into(),
                recorded_at: recorded_at.clone(),
            },
        );
        push_bounded(
            &mut self.logs,
            ProjectionLogView {
                level: "info".into(),
                code: operation.success_code().into(),
                operation: operation.label().into(),
                scope_type: scope_type.into(),
                scope_id: scope_id.into(),
                message,
                recorded_at,
            },
        );
    }

    fn record_failure(
        &mut self,
        operation: ProjectionSnapshotOperation,
        scope_type: &str,
        scope_id: &str,
        error: &ProjectionError,
    ) {
        let counter = operation.metric_mut(&mut self.metrics);
        counter.attempt_count += 1;
        counter.failure_count += 1;
        self.status = Some("degraded".into());

        let failure_code = format!(
            "{}:{}",
            operation.failure_code(),
            projection_error_code(error)
        );
        let failure_message = format!("{error:?}");
        self.last_failure_code = Some(failure_code.clone());
        self.last_failure_message = Some(failure_message.clone());

        self.trace_seq += 1;
        let trace_id = format!("projection-trace-{}", self.trace_seq);
        let recorded_at = utc_now_rfc3339_millis();
        push_bounded(
            &mut self.traces,
            ProjectionTraceView {
                trace_id,
                operation: operation.label().into(),
                scope_type: scope_type.into(),
                scope_id: scope_id.into(),
                outcome: "failure".into(),
                recorded_at: recorded_at.clone(),
            },
        );
        push_bounded(
            &mut self.logs,
            ProjectionLogView {
                level: "error".into(),
                code: failure_code,
                operation: operation.label().into(),
                scope_type: scope_type.into(),
                scope_id: scope_id.into(),
                message: failure_message,
                recorded_at,
            },
        );
    }

    fn record_replay_metrics(
        &mut self,
        backlog_size: u64,
        replayed_event_count: u64,
        duration_ms: u64,
    ) {
        self.replay = ProjectionReplayMetricsView {
            backlog_size,
            replayed_event_count,
            duration_ms,
        };
    }

    fn record_rebuild_duration(&mut self, duration_ms: u64) {
        self.rebuild_duration_ms = duration_ms;
    }

    fn live_lag_items(&self) -> Vec<ProjectionLagItemView> {
        self.live_lag.values().cloned().collect()
    }

    fn record_live_lag_observed(&mut self, scope_id: &str, current_offset: u64) {
        let lag = self
            .live_lag
            .entry(scope_id.into())
            .or_insert_with(|| ProjectionLagItemView {
                component: "projection_live".into(),
                scope_id: scope_id.into(),
                current_offset: 0,
                committed_offset: 0,
                lag: 0,
            });
        lag.current_offset = lag.current_offset.max(current_offset);
        lag.lag = lag.current_offset.saturating_sub(lag.committed_offset);
    }

    fn record_live_lag_committed(&mut self, scope_id: &str, committed_offset: u64) {
        self.live_lag.insert(
            scope_id.into(),
            ProjectionLagItemView {
                component: "projection_live".into(),
                scope_id: scope_id.into(),
                current_offset: committed_offset,
                committed_offset,
                lag: 0,
            },
        );
    }

    fn record_update_delay(
        &mut self,
        source_event_type: &str,
        scope_id: &str,
        timeline_ms: u64,
        inbox_ms: u64,
    ) {
        if self.status.is_none() {
            self.status = Some("ok".into());
        }
        self.update_delay = ProjectionUpdateDelayView {
            timeline_ms,
            inbox_ms,
            source_event_type: Some(source_event_type.into()),
            scope_id: Some(scope_id.into()),
            recorded_at: Some(utc_now_rfc3339_millis()),
        };
    }
}

impl ProjectionSnapshotOperation {
    fn label(&self) -> &'static str {
        match self {
            Self::ConversationSnapshotPersist => "conversation_snapshot.persist",
            Self::ConversationSnapshotRestore => "conversation_snapshot.restore",
            Self::ClientRouteSyncSnapshotPersist => "client_route_sync_snapshot.persist",
            Self::ClientRouteSyncSnapshotRestore => "client_route_sync_snapshot.restore",
        }
    }

    fn success_code(&self) -> &'static str {
        match self {
            Self::ConversationSnapshotPersist => "projection_snapshot_persist_succeeded",
            Self::ConversationSnapshotRestore => "projection_snapshot_restore_succeeded",
            Self::ClientRouteSyncSnapshotPersist => "client_route_sync_snapshot_persist_succeeded",
            Self::ClientRouteSyncSnapshotRestore => "client_route_sync_snapshot_restore_succeeded",
        }
    }

    fn failure_code(&self) -> &'static str {
        match self {
            Self::ConversationSnapshotPersist => "projection_snapshot_persist_failed",
            Self::ConversationSnapshotRestore => "projection_snapshot_restore_failed",
            Self::ClientRouteSyncSnapshotPersist => "client_route_sync_snapshot_persist_failed",
            Self::ClientRouteSyncSnapshotRestore => "client_route_sync_snapshot_restore_failed",
        }
    }

    fn metric_mut<'a>(
        &self,
        metrics: &'a mut ProjectionPlaneMetricsView,
    ) -> &'a mut ProjectionOperationMetricView {
        match self {
            Self::ConversationSnapshotPersist => &mut metrics.conversation_snapshot_persist,
            Self::ConversationSnapshotRestore => &mut metrics.conversation_snapshot_restore,
            Self::ClientRouteSyncSnapshotPersist => &mut metrics.client_route_sync_snapshot_persist,
            Self::ClientRouteSyncSnapshotRestore => &mut metrics.client_route_sync_snapshot_restore,
        }
    }
}

fn push_bounded<T>(items: &mut Vec<T>, item: T) {
    if items.len() >= MAX_RECENT_PROJECTION_EVENTS {
        items.remove(0);
    }
    items.push(item);
}

fn projection_error_code(error: &ProjectionError) -> &'static str {
    match error {
        ProjectionError::InvalidPayload(_) => "invalid_payload",
        ProjectionError::InvalidSnapshot(_) => "invalid_snapshot",
        ProjectionError::InvalidEvent(_) => "invalid_event",
        ProjectionError::StoreFailure(_) => "store_failure",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn poison_mutex<T>(mutex: &std::sync::Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_projection_plane_observability_recovers_from_poisoned_lock() {
        let projection = TimelineProjectionService::default();
        poison_mutex(&projection.observability);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.projection_plane_observability()
        }));
        assert!(
            result.is_ok(),
            "projection_plane_observability should not panic when observability lock is poisoned"
        );
    }

    #[test]
    fn test_record_projection_update_delay_recovers_from_poisoned_lock() {
        let projection = TimelineProjectionService::default();
        poison_mutex(&projection.observability);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            projection.record_projection_update_delay("message.posted", "100001:c_demo", 10, 20)
        }));
        assert!(
            result.is_ok(),
            "record_projection_update_delay should not panic when observability lock is poisoned"
        );
    }
}
