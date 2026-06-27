use axum::extract::{Extension, Path, Query, State};
use axum::http::HeaderMap;
use axum::response::Html;
use axum::Json;
use im_app_context::AppContext;
use im_platform_contracts::ProviderPolicyPreview;
use session_gateway::{RealtimeNodeLifecycleView, RealtimeRouteMigrationResult};
use sdkwork_im_openapi::render_docs_html;
use serde_json::Value as JsonValue;

use crate::dto::{
    MigrateRoutesRequest, ProtocolGovernanceResponse, ProtocolRegistryResponse,
    ProviderBindingCommitResponse, ProviderBindingsQuery, ProviderBindingsResponse,
    ProviderPolicyDiffQuery, ProviderPolicyDiffResponse, ProviderPolicyHistoryResponse,
    ProviderPolicyReadStatus, ProviderPolicyRollbackRequest, ProviderRegistrySnapshotResponse,
    UpsertProviderBindingPolicyRequest,
};
use crate::error::ControlPlaneError;
use crate::helpers::{
    compatibility_response, ensure_control_read_access, ensure_control_write_access,
    governance_response, mirror_all_provider_bindings_into_ops_runtime,
    mirror_node_into_ops_runtime, mirror_provider_bindings_into_ops_runtime,
    provider_binding_commit_response, provider_bindings_response,
    provider_policy_diff_response, provider_policy_history_response,
    provider_registry_snapshot_response, record_control_plane_audit, resolve_request_app_context,
    schema_response, validate_optional_tenant_id,
};
use crate::openapi::{control_plane_openapi_document, control_plane_openapi_spec};
use crate::state::AppState;

pub(crate) async fn openapi_document() -> Json<JsonValue> {
    Json(control_plane_openapi_document())
}

pub(crate) async fn docs() -> Html<String> {
    Html(render_docs_html(&control_plane_openapi_spec()))
}

pub(crate) async fn protocol_registry_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProtocolRegistryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    Ok(Json(ProtocolRegistryResponse {
        protocol_version: state.protocol_registry.protocol_version().to_owned(),
        bindings: state.protocol_registry.bindings().iter().cloned().collect(),
        codecs: state.protocol_registry.codecs().iter().cloned().collect(),
        schemas: state
            .protocol_registry
            .schemas()
            .values()
            .map(schema_response)
            .collect(),
        compatibility_matrix: state
            .protocol_registry
            .compatibility_matrix()
            .values()
            .map(compatibility_response)
            .collect(),
    }))
}

pub(crate) async fn protocol_governance_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProtocolGovernanceResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let governance = state
        .protocol_registry
        .governance_snapshot()
        .ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "protocol_governance_unavailable",
                "control plane governance snapshot is not initialized",
            )
        })?;

    Ok(Json(governance_response(
        governance,
        state.protocol_registry.as_ref(),
    )))
}

pub(crate) async fn provider_registry_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderRegistrySnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    Ok(Json(provider_registry_snapshot_response(
        state.provider_registry.snapshot(),
    )))
}

pub(crate) async fn provider_bindings_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ProviderBindingsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(query.tenant_id)?;

    let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
    mirror_provider_bindings_into_ops_runtime(&state, &response);

    Ok(Json(response))
}

pub(crate) async fn upsert_provider_binding_policy(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderBindingCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_write_unavailable",
            "control plane provider policy write is not enabled for this registry",
        )
    })?;

    let (action, aggregate_id, selection_source, commit) =
        if let Some(tenant_id) = tenant_id.as_deref() {
            let commit = provider_registry.commit_upsert(
                Some(tenant_id),
                request.domain,
                request.plugin_id.as_str(),
                request.expected_base_version,
            )?;
            (
                "control.provider_tenant_override_updated",
                format!(
                    "tenant:{tenant_id}:{}",
                    crate::helpers::provider_domain_name(request.domain)
                ),
                "tenant_override",
                commit,
            )
        } else {
            let commit = provider_registry.commit_upsert(
                None,
                request.domain,
                request.plugin_id.as_str(),
                request.expected_base_version,
            )?;
            (
                "control.provider_deployment_profile_updated",
                format!("deployment:{}", crate::helpers::provider_domain_name(request.domain)),
                "deployment_profile",
                commit,
            )
        };

    if commit.applied {
        mirror_all_provider_bindings_into_ops_runtime(&state, provider_registry.as_ref());
    }
    let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
    if commit.applied {
        record_control_plane_audit(
            &state,
            &auth,
            action,
            "provider_policy",
            aggregate_id,
            serde_json::json!({
                "tenantId": response.tenant_id,
                "domain": crate::helpers::provider_domain_name(request.domain),
                "pluginId": request.plugin_id,
                "expectedBaseVersion": request.expected_base_version,
                "currentVersion": commit.current_version,
                "selectionSource": selection_source
            }),
        );
    }

    Ok(Json(provider_binding_commit_response(response, commit)))
}

pub(crate) async fn provider_policy_history(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_history_unavailable",
            "control plane provider policy history is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_policy_history_response(
        ProviderPolicyReadStatus::History,
        provider_registry.policy_history(),
    )))
}

pub(crate) async fn provider_policy_diff(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ProviderPolicyDiffQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyDiffResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_diff_unavailable",
            "control plane provider policy diff is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_policy_diff_response(
        ProviderPolicyReadStatus::Diff,
        provider_registry.diff_versions(query.from_version, query.to_version)?,
    )))
}

pub(crate) async fn provider_policy_preview(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderPolicyPreview>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_preview_unavailable",
            "control plane provider policy preview is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_registry.preview_upsert(
        tenant_id.as_deref(),
        request.domain,
        request.plugin_id.as_str(),
    )?))
}

pub(crate) async fn rollback_provider_policy(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<ProviderPolicyRollbackRequest>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_rollback_unavailable",
            "control plane provider policy rollback is not enabled for this registry",
        )
    })?;

    let rollback_snapshot = provider_registry.rollback_to(request.target_version)?;
    mirror_all_provider_bindings_into_ops_runtime(&state, provider_registry.as_ref());
    let history = provider_registry.policy_history();
    record_control_plane_audit(
        &state,
        &auth,
        "control.provider_policy_rolled_back",
        "provider_policy",
        format!("version:{}", rollback_snapshot.version),
        serde_json::json!({
            "targetVersion": request.target_version,
            "currentVersion": history.current_version,
            "rollbackFromVersion": rollback_snapshot.rollback_from_version
        }),
    );

    Ok(Json(provider_policy_history_response(
        ProviderPolicyReadStatus::RolledBack,
        history,
    )))
}

pub(crate) async fn drain_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let lifecycle = state
        .realtime_cluster
        .mark_node_draining(node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_draining_marked",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "nodeId": node_id,
            "drainStatus": lifecycle.drain_status,
            "rebalanceState": lifecycle.rebalance_state,
            "ownedRouteCount": lifecycle.owned_route_count
        }),
    );
    Ok(Json(lifecycle))
}

pub(crate) async fn activate_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let lifecycle = state.realtime_cluster.activate_node(node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_activated",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "nodeId": node_id,
            "drainStatus": lifecycle.drain_status,
            "rebalanceState": lifecycle.rebalance_state,
            "ownedRouteCount": lifecycle.owned_route_count
        }),
    );
    Ok(Json(lifecycle))
}

pub(crate) async fn migrate_node_routes(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<MigrateRoutesRequest>,
) -> Result<Json<RealtimeRouteMigrationResult>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let migration = state
        .realtime_cluster
        .migrate_node_routes(node_id.as_str(), request.target_node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    mirror_node_into_ops_runtime(&state, request.target_node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_routes_migrated",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "sourceNodeId": migration.source_node_id,
            "targetNodeId": migration.target_node_id,
            "migratedRouteCount": migration.migrated_route_count,
            "sourceDrainStatus": migration.source_drain_status,
            "sourceRebalanceState": migration.source_rebalance_state,
            "targetDrainStatus": migration.target_drain_status,
            "targetRebalanceState": migration.target_rebalance_state
        }),
    );
    Ok(Json(migration))
}
