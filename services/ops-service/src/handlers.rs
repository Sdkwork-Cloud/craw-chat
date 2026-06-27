use axum::Json;
use axum::extract::{Extension, State};
use axum::http::HeaderMap;
use im_adapters_postgres_journal::{
    PostgresJournalConfig, RetentionCleanupReport, purge_expired_retention_batch,
};
use im_app_context::AppContext;
use im_time::utc_now_rfc3339_millis;
use serde::Deserialize;

use crate::dto::{
    ClusterView, DiagnosticBundle, LagView, OpsHealthResponse, ProjectionReplayStatusView,
    ProviderBindingDriftView, ProviderBindingsView, RetentionPurgeResponse,
    RuntimeDirInspectionView,
};
use crate::error::OpsError;
use crate::helpers::{
    ensure_ops_read_access, ensure_ops_write_access, resolve_request_app_context,
};
use crate::state::AppState;

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const RETENTION_PURGE_DEFAULT_BATCH_SIZE: i64 = 500;
const RETENTION_PURGE_MAX_BATCH_SIZE: i64 = 5_000;

pub(crate) async fn get_ops_health(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpsHealthResponse>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.health_view()))
}

pub(crate) async fn get_cluster(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ClusterView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.cluster_view()))
}

pub(crate) async fn get_lag(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<LagView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.lag_view()))
}

pub(crate) async fn get_runtime_dir(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RuntimeDirInspectionView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.runtime_dir_view()))
}

pub(crate) async fn get_provider_bindings(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.provider_bindings_view()))
}

pub(crate) async fn get_provider_binding_drift(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingDriftView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.provider_binding_drift_view()))
}

pub(crate) async fn get_replay_status(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProjectionReplayStatusView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.replay_status_view()))
}

pub(crate) async fn get_diagnostics(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DiagnosticBundle>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.diagnostic_bundle()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetentionPurgeQuery {
    pub(crate) batch_size: Option<i64>,
}

pub(crate) async fn post_retention_purge(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<RetentionPurgeQuery>,
) -> Result<Json<RetentionPurgeResponse>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_write_access(&auth)?;

    let database_url = std::env::var(IM_DATABASE_URL_ENV).map_err(|_| {
        OpsError::service_unavailable(
            "database_unconfigured",
            format!("{IM_DATABASE_URL_ENV} is required for retention purge"),
        )
    })?;
    let batch_size = query
        .batch_size
        .unwrap_or(RETENTION_PURGE_DEFAULT_BATCH_SIZE)
        .clamp(1, RETENTION_PURGE_MAX_BATCH_SIZE);
    let config = PostgresJournalConfig::new(database_url);
    let pool = config.connect_pool().map_err(|error| {
        OpsError::service_unavailable("database_unavailable", format!("{error:?}"))
    })?;

    let report =
        tokio::task::spawn_blocking(move || purge_expired_retention_batch(&pool, Some(batch_size)))
            .await
            .map_err(|_| {
                OpsError::internal("retention_purge_failed", "retention purge worker panicked")
            })?
            .map_err(|error| OpsError::internal("retention_purge_failed", format!("{error:?}")))?;

    Ok(Json(retention_purge_response(batch_size, report)))
}

fn retention_purge_response(
    batch_size: i64,
    report: RetentionCleanupReport,
) -> RetentionPurgeResponse {
    RetentionPurgeResponse {
        generated_at: utc_now_rfc3339_millis(),
        batch_size,
        commit_journal_deleted: report.commit_journal_deleted,
        conversation_messages_deleted: report.conversation_messages_deleted,
        message_media_refs_deleted: report.message_media_refs_deleted,
        outbox_events_deleted: report.outbox_events_deleted,
        inbox_events_deleted: report.inbox_events_deleted,
        projection_timeline_entries_deleted: report.projection_timeline_entries_deleted,
        realtime_device_events_deleted: report.realtime_device_events_deleted,
    }
}
