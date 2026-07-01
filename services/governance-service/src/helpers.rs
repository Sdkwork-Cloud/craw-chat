use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use audit_service::RecordAuditAnchor;
use im_app_context::AppContext;
use im_platform_contracts::{
    PROVIDER_REGISTRY_INTERFACE_VERSION, ProviderDomain, ProviderPolicyCommit, ProviderPolicyDiff,
    ProviderPolicyHistory, ProviderRegistry, RuntimeProviderRegistry,
};
use ops_service::{ProviderBindingItemView, ProviderBindingSnapshotView, RouteOwnershipView};
use sdkwork_im_ccp_registry::{
    BusinessPolicyVocabulary, CapabilityProfile, ClientCompatibilityDescriptor,
    EffectiveProtocolSnapshot, KillSwitchRule, ProtocolGovernanceSnapshot, QuotaProfile,
    ReleaseChannel, RolloutPolicy, SchemaDescriptor,
};

use crate::dto::{
    BusinessPolicyVocabularyResponse, CapabilityProfileResponse, ClientCompatibilityResponse,
    EffectiveProtocolSnapshotResponse, KillSwitchResponse, ProtocolGovernanceResponse,
    ProtocolSchemaResponse, ProviderBindingCommitResponse, ProviderBindingsResponse,
    ProviderPolicyDiffResponse, ProviderPolicyHistoryResponse, ProviderPolicyReadStatus,
    ProviderRegistrySnapshotResponse, ProviderSurfaceReadStatus, QuotaProfileResponse,
    RolloutPolicyResponse, SdkCompatibilityBaselineResponse,
};
use crate::error::ControlPlaneError;
use crate::state::AppState;

const CONTROL_PLANE_MAX_ID_BYTES: usize = 256;
static CONTROL_PLANE_AUDIT_RECORD_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(crate) fn mirror_node_into_ops_runtime(state: &AppState, node_id: &str) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    if governance_loop.ops_runtime.node_id() != node_id {
        return;
    }

    let Some(lifecycle) = state.realtime_cluster.node_lifecycle(node_id) else {
        return;
    };
    governance_loop.ops_runtime.set_node_lifecycle(
        lifecycle.drain_status.as_str(),
        lifecycle.rebalance_state.as_str(),
    );
    governance_loop.ops_runtime.update_route_ownership(
        state
            .realtime_cluster
            .routes_for_node(node_id)
            .into_iter()
            .map(|route| RouteOwnershipView {
                tenant_id: route.tenant_id,
                principal_id: route.principal_id,
                device_id: route.device_id,
                owner_node_id: route.owner_node_id,
                connection_kind: route.connection_kind,
                bound_at: route.bound_at,
            })
            .collect(),
    );
}

pub(crate) fn mirror_provider_bindings_into_ops_runtime(
    state: &AppState,
    response: &ProviderBindingsResponse,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };

    governance_loop
        .ops_runtime
        .update_provider_binding_snapshot(provider_binding_snapshot_view(response));
}

pub(crate) fn mirror_all_provider_bindings_into_ops_runtime(
    state: &AppState,
    provider_registry: &RuntimeProviderRegistry,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };

    let mut snapshots = vec![provider_binding_snapshot_view(&provider_bindings_response(
        provider_registry,
        None,
    ))];
    let mut tenant_ids = provider_registry.tenant_ids_with_overrides();
    tenant_ids.sort();
    snapshots.extend(tenant_ids.into_iter().map(|tenant_id| {
        provider_binding_snapshot_view(&provider_bindings_response(
            provider_registry,
            Some(tenant_id),
        ))
    }));
    governance_loop
        .ops_runtime
        .replace_provider_binding_snapshots(snapshots);
}

pub(crate) fn record_control_plane_audit(
    state: &AppState,
    auth: &AppContext,
    action: &str,
    aggregate_type: &str,
    aggregate_id: String,
    payload: serde_json::Value,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    let record_id = control_plane_audit_record_id();
    let payload = match serde_json::to_string(&payload) {
        Ok(serialized) => serialized,
        Err(error) => {
            tracing::warn!(error = ?error, "control plane audit payload serialization failed");
            return;
        }
    };
    if let Err(error) = governance_loop.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id,
            aggregate_type: aggregate_type.into(),
            aggregate_id,
            action: action.into(),
            payload: Some(payload),
        },
    ) {
        tracing::warn!("control-plane audit write failed for {aggregate_type}/{action}: {error:?}");
    }
}

fn control_plane_audit_record_id() -> String {
    let recorded_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let sequence = CONTROL_PLANE_AUDIT_RECORD_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("control-audit-{recorded_nanos:x}-{sequence:x}")
}

pub(crate) fn provider_bindings_response(
    provider_registry: &dyn ProviderRegistry,
    tenant_id: Option<String>,
) -> ProviderBindingsResponse {
    let precedence = provider_registry.snapshot().precedence;
    let effective_bindings = ProviderDomain::ALL
        .into_iter()
        .filter_map(|domain| provider_registry.effective_binding(domain, tenant_id.as_deref()))
        .collect();
    ProviderBindingsResponse {
        status: ProviderSurfaceReadStatus::Bindings,
        interface_version: PROVIDER_REGISTRY_INTERFACE_VERSION.into(),
        tenant_id,
        effective_bindings,
        precedence,
    }
}

pub(crate) fn provider_binding_commit_response(
    response: ProviderBindingsResponse,
    commit: ProviderPolicyCommit,
) -> ProviderBindingCommitResponse {
    ProviderBindingCommitResponse {
        status: commit.status,
        applied: commit.applied,
        interface_version: response.interface_version,
        tenant_id: response.tenant_id,
        current_version: commit.current_version,
        committed_binding: commit.committed_binding,
        diff: commit.diff,
        effective_bindings: response.effective_bindings,
        precedence: response.precedence,
    }
}

pub(crate) fn provider_registry_snapshot_response(
    snapshot: im_platform_contracts::ProviderRegistrySnapshot,
) -> ProviderRegistrySnapshotResponse {
    ProviderRegistrySnapshotResponse {
        status: ProviderSurfaceReadStatus::Registry,
        snapshot,
    }
}

pub(crate) fn provider_policy_history_response(
    status: ProviderPolicyReadStatus,
    history: ProviderPolicyHistory,
) -> ProviderPolicyHistoryResponse {
    ProviderPolicyHistoryResponse { status, history }
}

pub(crate) fn provider_policy_diff_response(
    status: ProviderPolicyReadStatus,
    diff: ProviderPolicyDiff,
) -> ProviderPolicyDiffResponse {
    ProviderPolicyDiffResponse { status, diff }
}

fn provider_binding_snapshot_view(response: &ProviderBindingsResponse) -> ProviderBindingSnapshotView {
    ProviderBindingSnapshotView {
        interface_version: response.interface_version.clone(),
        tenant_id: response.tenant_id.clone(),
        effective_bindings: response
            .effective_bindings
            .iter()
            .map(|binding| ProviderBindingItemView {
                domain: provider_domain_name(binding.domain).into(),
                default_plugin_id: binding.default_plugin_id.clone(),
                selected_plugin_id: binding.selected_plugin_id.clone(),
                selection_source: binding.selection_source.clone(),
                tenant_override_allowed: binding.tenant_override_allowed,
            })
            .collect(),
        precedence: response.precedence.clone(),
    }
}

pub(crate) fn ensure_control_write_access(auth: &AppContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.write"))
}

pub(crate) fn ensure_control_read_access(auth: &AppContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.read") || auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.read"))
}

pub(crate) fn schema_response(schema: &SchemaDescriptor) -> ProtocolSchemaResponse {
    ProtocolSchemaResponse {
        schema: schema.schema.clone(),
        kind: schema.kind.clone(),
        stage: schema.stage.as_str().to_owned(),
        binding_protocols: schema.binding_protocols.iter().cloned().collect(),
        required_capabilities: schema.required_capabilities.iter().cloned().collect(),
        supported_consumers: schema.supported_consumers.iter().cloned().collect(),
    }
}

pub(crate) fn compatibility_response(
    descriptor: &ClientCompatibilityDescriptor,
) -> ClientCompatibilityResponse {
    ClientCompatibilityResponse {
        client_type: descriptor.client_type.clone(),
        minimum_protocol_version: descriptor.minimum_protocol_version.clone(),
        supported_bindings: descriptor.supported_bindings.iter().cloned().collect(),
        supported_codecs: descriptor.supported_codecs.iter().cloned().collect(),
        supported_capabilities: descriptor.supported_capabilities.iter().cloned().collect(),
        blocked_experimental_capabilities: descriptor
            .blocked_experimental_capabilities
            .iter()
            .cloned()
            .collect(),
    }
}

pub(crate) fn governance_response(
    governance: &ProtocolGovernanceSnapshot,
    registry: &sdkwork_im_ccp_registry::CcpRegistry,
) -> ProtocolGovernanceResponse {
    ProtocolGovernanceResponse {
        capability_profile: capability_profile_response(&governance.capability_profile),
        quota_profile: quota_profile_response(&governance.quota_profile),
        rollout_policy: rollout_policy_response(&governance.rollout_policy),
        kill_switch: kill_switch_response(&governance.kill_switch),
        effective_snapshot: effective_snapshot_response(&governance.effective_snapshot),
        business_policy_vocabulary: business_policy_vocabulary_response(
            &governance.business_policy_vocabulary,
        ),
        sdk_compatibility_baseline: sdk_compatibility_baseline_response(registry),
    }
}

fn capability_profile_response(profile: &CapabilityProfile) -> CapabilityProfileResponse {
    CapabilityProfileResponse {
        profile_id: profile.profile_id.clone(),
        release_channel: release_channel(profile.release_channel.clone()).to_owned(),
        enabled_capabilities: profile.enabled_capabilities.iter().cloned().collect(),
        experimental_capabilities: profile.experimental_capabilities.iter().cloned().collect(),
    }
}

fn quota_profile_response(profile: &QuotaProfile) -> QuotaProfileResponse {
    QuotaProfileResponse {
        profile_id: profile.profile_id.clone(),
        max_concurrent_sessions_per_tenant: profile.max_concurrent_sessions_per_tenant,
        max_subscriptions_per_session: profile.max_subscriptions_per_session,
        max_inflight_messages: profile.max_inflight_messages,
        max_payload_bytes: profile.max_payload_bytes,
    }
}

fn rollout_policy_response(policy: &RolloutPolicy) -> RolloutPolicyResponse {
    RolloutPolicyResponse {
        policy_id: policy.policy_id.clone(),
        release_channel: release_channel(policy.release_channel.clone()).to_owned(),
        traffic_percent: policy.traffic_percent,
        cell_selector: policy.cell_selector.clone(),
        region_selector: policy.region_selector.clone(),
        operator_override: policy.operator_override,
        tenant_allowlist: policy.tenant_allowlist.iter().cloned().collect(),
    }
}

fn kill_switch_response(kill_switch: &KillSwitchRule) -> KillSwitchResponse {
    KillSwitchResponse {
        rule_id: kill_switch.rule_id.clone(),
        active: kill_switch.active,
        reason: kill_switch.reason.clone(),
        disabled_capabilities: kill_switch.disabled_capabilities.iter().cloned().collect(),
        disabled_bindings: kill_switch.disabled_bindings.iter().cloned().collect(),
        disabled_codecs: kill_switch.disabled_codecs.iter().cloned().collect(),
    }
}

fn effective_snapshot_response(
    snapshot: &EffectiveProtocolSnapshot,
) -> EffectiveProtocolSnapshotResponse {
    EffectiveProtocolSnapshotResponse {
        protocol_version: snapshot.protocol_version.clone(),
        release_channel: release_channel(snapshot.release_channel.clone()).to_owned(),
        enabled_capabilities: snapshot.enabled_capabilities.iter().cloned().collect(),
        allowed_bindings: snapshot.allowed_bindings.iter().cloned().collect(),
        allowed_codecs: snapshot.allowed_codecs.iter().cloned().collect(),
        quota_profile_id: snapshot.quota_profile_id.clone(),
        kill_switch_active: snapshot.kill_switch_active,
        precedence: snapshot.precedence.clone(),
    }
}

fn business_policy_vocabulary_response(
    vocabulary: &BusinessPolicyVocabulary,
) -> BusinessPolicyVocabularyResponse {
    BusinessPolicyVocabularyResponse {
        policy_version_field: vocabulary.policy_version_field.clone(),
        capability_flags_field: vocabulary.capability_flags_field.clone(),
        history_visibility_field: vocabulary.history_visibility_field.clone(),
        history_visibility_modes: vocabulary.history_visibility_modes.clone(),
        retention_policy_ref_field: vocabulary.retention_policy_ref_field.clone(),
        retention_policy_scopes: vocabulary.retention_policy_scopes.clone(),
        retention_classes: vocabulary.retention_classes.clone(),
    }
}

fn sdk_compatibility_baseline_response(
    registry: &sdkwork_im_ccp_registry::CcpRegistry,
) -> SdkCompatibilityBaselineResponse {
    SdkCompatibilityBaselineResponse {
        im_sdk_family: "sdkwork-im-sdk",
        app_sdk_family: "sdkwork-im-app-sdk",
        backend_sdk_family: "sdkwork-im-backend-sdk",
        rtc_sdk_family: "sdkwork-rtc-sdk",
        matrix_client_types: registry.compatibility_matrix().keys().cloned().collect(),
        protocol_registry_path: "/backend/v3/api/control/protocol_registry",
        protocol_governance_path: "/backend/v3/api/control/protocol_governance",
    }
}

fn release_channel(channel: ReleaseChannel) -> &'static str {
    channel.as_str()
}

pub(crate) fn provider_domain_name(domain: ProviderDomain) -> &'static str {
    domain.as_str()
}

pub(crate) fn validate_optional_tenant_id(
    tenant_id: Option<String>,
) -> Result<Option<String>, ControlPlaneError> {
    if let Some(tenant_id) = tenant_id {
        validate_required_with_code("tenantId", tenant_id.as_str(), "invalid_provider_policy")?;
        validate_payload_size("tenantId", tenant_id.as_str(), CONTROL_PLANE_MAX_ID_BYTES)?;
        return Ok(Some(tenant_id));
    }

    Ok(None)
}

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), ControlPlaneError> {
    let actual_bytes = value.len();
    if actual_bytes > max_bytes {
        return Err(ControlPlaneError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }

    Ok(())
}

fn validate_required_with_code(
    field: &'static str,
    value: &str,
    code: &'static str,
) -> Result<(), ControlPlaneError> {
    if value.trim().is_empty() {
        return Err(ControlPlaneError::invalid(
            code,
            format!("{field} cannot be empty"),
        ));
    }

    Ok(())
}
