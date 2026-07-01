use axum::extract::{Extension, Path, Query, State};
use axum::response::{Html, Response};
use axum::Json;
use im_app_context::AppContext;
use im_platform_contracts::ProviderPolicyPreview;
use session_gateway::{RealtimeNodeLifecycleView, RealtimeRouteMigrationResult};
use sdkwork_im_openapi::render_docs_html;
use sdkwork_routes_web_framework_backend_api::response::{ApiResult, finish_api_json};
use sdkwork_web_core::WebRequestContext;
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
    provider_registry_snapshot_response, record_control_plane_audit, schema_response,
    validate_optional_tenant_id,
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
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProtocolRegistryResponse> = (|| {
        ensure_control_read_access(&auth)?;
        Ok(ProtocolRegistryResponse {
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
        })
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn protocol_governance_snapshot(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProtocolGovernanceResponse> = (|| {
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
        Ok(governance_response(
            governance,
            state.protocol_registry.as_ref(),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn provider_registry_snapshot(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProviderRegistrySnapshotResponse> = (|| {
        ensure_control_read_access(&auth)?;
        Ok(provider_registry_snapshot_response(
            state.provider_registry.snapshot(),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn provider_bindings_snapshot(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<ProviderBindingsQuery>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProviderBindingsResponse> = (|| {
        ensure_control_read_access(&auth)?;
        let tenant_id = validate_optional_tenant_id(query.tenant_id)?;
        let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
        mirror_provider_bindings_into_ops_runtime(&state, &response);
        Ok(response)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn upsert_provider_binding_policy(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Response {
    let result: ApiResult<ProviderBindingCommitResponse> = (|| {
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
                )
                .map_err(ControlPlaneError::from)?;
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
                )
                .map_err(ControlPlaneError::from)?;
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

        Ok(provider_binding_commit_response(response, commit))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn provider_policy_history(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProviderPolicyHistoryResponse> = (|| {
        ensure_control_read_access(&auth)?;
        let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "provider_policy_history_unavailable",
                "control plane provider policy history is not enabled for this registry",
            )
        })?;
        Ok(provider_policy_history_response(
            ProviderPolicyReadStatus::History,
            provider_registry.policy_history(),
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn provider_policy_diff(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Query(query): Query<ProviderPolicyDiffQuery>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<ProviderPolicyDiffResponse> = (|| {
        ensure_control_read_access(&auth)?;
        let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "provider_policy_diff_unavailable",
                "control plane provider policy diff is not enabled for this registry",
            )
        })?;
        Ok(provider_policy_diff_response(
            ProviderPolicyReadStatus::Diff,
            provider_registry
                .diff_versions(query.from_version, query.to_version)
                .map_err(ControlPlaneError::from)?,
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn provider_policy_preview(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Response {
    let result: ApiResult<ProviderPolicyPreview> = (|| {
        ensure_control_write_access(&auth)?;
        let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
        let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "provider_policy_preview_unavailable",
                "control plane provider policy preview is not enabled for this registry",
            )
        })?;
        Ok(provider_registry
            .preview_upsert(
                tenant_id.as_deref(),
                request.domain,
                request.plugin_id.as_str(),
            )
            .map_err(ControlPlaneError::from)?)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn rollback_provider_policy(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    State(state): State<AppState>,
    Json(request): Json<ProviderPolicyRollbackRequest>,
) -> Response {
    let result: ApiResult<ProviderPolicyHistoryResponse> = (|| {
        ensure_control_write_access(&auth)?;
        let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "provider_policy_rollback_unavailable",
                "control plane provider policy rollback is not enabled for this registry",
            )
        })?;
        let rollback_snapshot = provider_registry
            .rollback_to(request.target_version)
            .map_err(ControlPlaneError::from)?;
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
        Ok(provider_policy_history_response(
            ProviderPolicyReadStatus::RolledBack,
            history,
        ))
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn drain_node(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(node_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<RealtimeNodeLifecycleView> = (|| {
        ensure_control_write_access(&auth)?;
        let lifecycle = state
            .realtime_cluster
            .mark_node_draining(node_id.as_str())
            .map_err(ControlPlaneError::from)?;
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
        Ok(lifecycle)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn activate_node(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(node_id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    let result: ApiResult<RealtimeNodeLifecycleView> = (|| {
        ensure_control_write_access(&auth)?;
        let lifecycle = state
            .realtime_cluster
            .activate_node(node_id.as_str())
            .map_err(ControlPlaneError::from)?;
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
        Ok(lifecycle)
    })();
    finish_api_json(&ctx, result)
}

pub(crate) async fn migrate_node_routes(
    Extension(ctx): Extension<WebRequestContext>,
    Extension(auth): Extension<AppContext>,
    Path(node_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<MigrateRoutesRequest>,
) -> Response {
    let result: ApiResult<RealtimeRouteMigrationResult> = (|| {
        ensure_control_write_access(&auth)?;
        let migration = state
            .realtime_cluster
            .migrate_node_routes(node_id.as_str(), request.target_node_id.as_str())
            .map_err(ControlPlaneError::from)?;
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
        Ok(migration)
    })();
    finish_api_json(&ctx, result)
}
