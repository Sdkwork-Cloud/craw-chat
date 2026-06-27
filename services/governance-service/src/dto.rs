use im_platform_contracts::{
    EffectiveProviderBinding, ProviderDomain, ProviderPolicyDiff, ProviderPolicyHistory,
    ProviderPolicyResultStatus, ProviderRegistrySnapshot,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MigrateRoutesRequest {
    pub(crate) target_node_id: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderBindingsQuery {
    pub(crate) tenant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpsertProviderBindingPolicyRequest {
    pub(crate) tenant_id: Option<String>,
    pub(crate) domain: ProviderDomain,
    pub(crate) plugin_id: String,
    pub(crate) expected_base_version: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderPolicyRollbackRequest {
    pub(crate) target_version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderPolicyDiffQuery {
    pub(crate) from_version: u64,
    pub(crate) to_version: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProtocolRegistryResponse {
    pub(crate) protocol_version: String,
    pub(crate) bindings: Vec<String>,
    pub(crate) codecs: Vec<String>,
    pub(crate) schemas: Vec<ProtocolSchemaResponse>,
    pub(crate) compatibility_matrix: Vec<ClientCompatibilityResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProtocolGovernanceResponse {
    pub(crate) capability_profile: CapabilityProfileResponse,
    pub(crate) quota_profile: QuotaProfileResponse,
    pub(crate) rollout_policy: RolloutPolicyResponse,
    pub(crate) kill_switch: KillSwitchResponse,
    pub(crate) effective_snapshot: EffectiveProtocolSnapshotResponse,
    pub(crate) business_policy_vocabulary: BusinessPolicyVocabularyResponse,
    pub(crate) sdk_compatibility_baseline: SdkCompatibilityBaselineResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProtocolSchemaResponse {
    pub(crate) schema: String,
    pub(crate) kind: String,
    pub(crate) stage: String,
    pub(crate) binding_protocols: Vec<String>,
    pub(crate) required_capabilities: Vec<String>,
    pub(crate) supported_consumers: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientCompatibilityResponse {
    pub(crate) client_type: String,
    pub(crate) minimum_protocol_version: String,
    pub(crate) supported_bindings: Vec<String>,
    pub(crate) supported_codecs: Vec<String>,
    pub(crate) supported_capabilities: Vec<String>,
    pub(crate) blocked_experimental_capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CapabilityProfileResponse {
    pub(crate) profile_id: String,
    pub(crate) release_channel: String,
    pub(crate) enabled_capabilities: Vec<String>,
    pub(crate) experimental_capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QuotaProfileResponse {
    pub(crate) profile_id: String,
    pub(crate) max_concurrent_sessions_per_tenant: u32,
    pub(crate) max_subscriptions_per_session: u32,
    pub(crate) max_inflight_messages: u32,
    pub(crate) max_payload_bytes: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RolloutPolicyResponse {
    pub(crate) policy_id: String,
    pub(crate) release_channel: String,
    pub(crate) traffic_percent: u8,
    pub(crate) cell_selector: String,
    pub(crate) region_selector: String,
    pub(crate) operator_override: bool,
    pub(crate) tenant_allowlist: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KillSwitchResponse {
    pub(crate) rule_id: String,
    pub(crate) active: bool,
    pub(crate) reason: String,
    pub(crate) disabled_capabilities: Vec<String>,
    pub(crate) disabled_bindings: Vec<String>,
    pub(crate) disabled_codecs: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EffectiveProtocolSnapshotResponse {
    pub(crate) protocol_version: String,
    pub(crate) release_channel: String,
    pub(crate) enabled_capabilities: Vec<String>,
    pub(crate) allowed_bindings: Vec<String>,
    pub(crate) allowed_codecs: Vec<String>,
    pub(crate) quota_profile_id: String,
    pub(crate) kill_switch_active: bool,
    pub(crate) precedence: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BusinessPolicyVocabularyResponse {
    pub(crate) policy_version_field: String,
    pub(crate) capability_flags_field: String,
    pub(crate) history_visibility_field: String,
    pub(crate) history_visibility_modes: Vec<String>,
    pub(crate) retention_policy_ref_field: String,
    pub(crate) retention_policy_scopes: Vec<String>,
    pub(crate) retention_classes: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SdkCompatibilityBaselineResponse {
    pub(crate) im_sdk_family: &'static str,
    pub(crate) app_sdk_family: &'static str,
    pub(crate) backend_sdk_family: &'static str,
    pub(crate) rtc_sdk_family: &'static str,
    pub(crate) matrix_client_types: Vec<String>,
    pub(crate) protocol_registry_path: &'static str,
    pub(crate) protocol_governance_path: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderBindingsResponse {
    pub(crate) status: ProviderSurfaceReadStatus,
    pub(crate) interface_version: String,
    pub(crate) tenant_id: Option<String>,
    pub(crate) effective_bindings: Vec<EffectiveProviderBinding>,
    pub(crate) precedence: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderBindingCommitResponse {
    pub(crate) status: ProviderPolicyResultStatus,
    pub(crate) applied: bool,
    pub(crate) interface_version: String,
    pub(crate) tenant_id: Option<String>,
    pub(crate) current_version: u64,
    pub(crate) committed_binding: EffectiveProviderBinding,
    pub(crate) diff: ProviderPolicyDiff,
    pub(crate) effective_bindings: Vec<EffectiveProviderBinding>,
    pub(crate) precedence: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProviderSurfaceReadStatus {
    Registry,
    Bindings,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderRegistrySnapshotResponse {
    pub(crate) status: ProviderSurfaceReadStatus,
    #[serde(flatten)]
    pub(crate) snapshot: ProviderRegistrySnapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProviderPolicyReadStatus {
    History,
    Diff,
    RolledBack,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderPolicyHistoryResponse {
    pub(crate) status: ProviderPolicyReadStatus,
    #[serde(flatten)]
    pub(crate) history: ProviderPolicyHistory,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderPolicyDiffResponse {
    pub(crate) status: ProviderPolicyReadStatus,
    #[serde(flatten)]
    pub(crate) diff: ProviderPolicyDiff,
}
