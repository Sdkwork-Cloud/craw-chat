use axum::extract::rejection::QueryRejection;
use axum::extract::{Extension, Query, State};
use axum::response::Response;
use im_adapters_postgres_journal::{
    PostgresJournalConfig, RetentionCleanupReport, purge_expired_retention_batch,
};
use im_app_context::AppContext;
use im_time::utc_now_rfc3339_millis;
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_utils_rust::{SdkWorkCursorListQuery, SdkWorkPageData, SdkWorkResourceData};
use sdkwork_web_core::WebRequestContext;
use serde::Deserialize;

use crate::dto::{
    ClusterView, DiagnosticBundle, LagItem, OpsHealthResponse, ProjectionReplayStatusView,
    ProviderBindingDriftItemView, ProviderBindingSnapshotView, RetentionPurgeResponse,
    RuntimeDirInspectionView,
};
use crate::error::OpsError;
use crate::helpers::{ensure_ops_read_access, ensure_ops_write_access};
use crate::state::AppState;

const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const RETENTION_PURGE_DEFAULT_BATCH_SIZE: i64 = 500;
const RETENTION_PURGE_MAX_BATCH_SIZE: i64 = 5_000;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OpsCursorListQuery {
    #[serde(rename = "page_size")]
    page_size: Option<i32>,
    cursor: Option<String>,
}

impl From<OpsCursorListQuery> for SdkWorkCursorListQuery {
    fn from(value: OpsCursorListQuery) -> Self {
        Self {
            page_size: value.page_size,
            cursor: value.cursor,
        }
    }
}

fn resolve_ops_query(
    query: Result<Query<OpsCursorListQuery>, QueryRejection>,
) -> Result<SdkWorkCursorListQuery, OpsError> {
    query
        .map(|Query(query)| query.into())
        .map_err(|_| OpsError::invalid_parameter("invalid ops list query parameters"))
}

pub(crate) async fn get_ops_health(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<OpsHealthResponse>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(SdkWorkResourceData {
            item: state.runtime.health_view(),
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_cluster(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<ClusterView>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(SdkWorkResourceData {
            item: state.runtime.cluster_view(),
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_lag(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    query: Result<Query<OpsCursorListQuery>, QueryRejection>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<LagItem>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(state.runtime.lag_page(resolve_ops_query(query)?)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_runtime_dir(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<RuntimeDirInspectionView>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(SdkWorkResourceData {
            item: state.runtime.runtime_dir_view(),
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_provider_bindings(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    query: Result<Query<OpsCursorListQuery>, QueryRejection>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<ProviderBindingSnapshotView>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(state
            .runtime
            .provider_bindings_page(resolve_ops_query(query)?)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_provider_binding_drift(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    query: Result<Query<OpsCursorListQuery>, QueryRejection>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkPageData<ProviderBindingDriftItemView>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(state
            .runtime
            .provider_binding_drift_page(resolve_ops_query(query)?)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_replay_status(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<ProjectionReplayStatusView>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(SdkWorkResourceData {
            item: state.runtime.replay_status_view(),
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn get_diagnostics(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<DiagnosticBundle>> = (|| {
        ensure_ops_read_access(&auth)?;
        Ok(SdkWorkResourceData {
            item: state.runtime.diagnostic_bundle(),
        })
    })();
    finish_api_json(&ctx, result)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetentionPurgeQuery {
    pub(crate) batch_size: Option<i64>,
}

pub(crate) async fn post_retention_purge(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    axum::extract::Query(query): axum::extract::Query<RetentionPurgeQuery>,
) -> Response {
    let result: ApiResult<SdkWorkResourceData<RetentionPurgeResponse>> = async {
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
        let report = tokio::task::spawn_blocking(move || {
            purge_expired_retention_batch(&pool, Some(batch_size))
        })
        .await
        .map_err(|_| {
            OpsError::internal("retention_purge_failed", "retention purge worker panicked")
        })?
        .map_err(|error| OpsError::internal("retention_purge_failed", format!("{error:?}")))?;
        Ok(SdkWorkResourceData {
            item: retention_purge_response(batch_size, report),
        })
    }
    .await;
    finish_api_json(&ctx, result)
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
        rtc_sessions_deleted: report.rtc_sessions_deleted,
        rtc_signals_deleted: report.rtc_signals_deleted,
    }
}
