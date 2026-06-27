use std::sync::Arc;

use crate::api::paths::backend_path;
use crate::api::paths::append_query_string;
use crate::http::{SdkworkError, SdkworkHttpClient};
use crate::models::{AcceptFriendRequestRequest, ActivateFriendshipRequest, ApplySharedChannelPolicyRequest, BindDirectChatRequest, BindExternalMemberLinkRequest, BlockUserRequest, CancelFriendRequestRequest, DeclineFriendRequestRequest, EstablishExternalConnectionRequest, MigrateRoutesRequest, ProtocolGovernanceResponse, ProtocolRegistryResponse, ProviderBindingCommitResponse, ProviderBindingsResponse, ProviderPolicyDiffResponse, ProviderPolicyHistoryResponse, ProviderPolicyRollbackRequest, ProviderRegistrySnapshotResponse, RemoveFriendshipRequest, RouteMigrationResult, RouteNodeLifecycle, SocialDirectChatCommitResponse, SocialDirectChatSnapshotResponse, SocialExternalConnectionCommitResponse, SocialExternalConnectionSnapshotResponse, SocialExternalMemberLinkCommitResponse, SocialExternalMemberLinkSnapshotResponse, SocialFriendRequestCommitResponse, SocialFriendRequestSnapshotResponse, SocialFriendshipCommitResponse, SocialFriendshipSnapshotResponse, SocialRuntimeRepairResponse, SocialSharedChannelPolicyCommitResponse, SocialSharedChannelPolicySnapshotResponse, SocialSharedChannelSyncDeadLetterInventoryResponse, SocialSharedChannelSyncDeadLetterRequeueResponse, SocialSharedChannelSyncDeadLetterTargetedRequeueRequest, SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, SocialSharedChannelSyncDeliveredInventoryResponse, SocialSharedChannelSyncDeliveryStateInventoryResponse, SocialSharedChannelSyncPendingClaimResponse, SocialSharedChannelSyncPendingInventoryResponse, SocialSharedChannelSyncPendingReleaseResponse, SocialSharedChannelSyncPendingStaleReclaimResponse, SocialSharedChannelSyncPendingTakeoverResponse, SocialSharedChannelSyncPendingTargetedClaimRequest, SocialSharedChannelSyncPendingTargetedReleaseRequest, SocialSharedChannelSyncPendingTargetedTakeoverRequest, SocialSharedChannelSyncRepairResponse, SocialSharedChannelSyncTargetedRepublishRequest, SocialSharedChannelSyncTargetedRepublishResponse, SocialUserBlockCommitResponse, SocialUserBlockSnapshotResponse, SubmitFriendRequestRequest, UpsertProviderBindingPolicyRequest};

#[derive(Clone)]
pub struct ControlApi {
    client: Arc<SdkworkHttpClient>,
}

impl ControlApi {
    pub fn new(client: Arc<SdkworkHttpClient>) -> Self {
        Self { client }
    }

    /// Activate a realtime node and clear drain state.
    pub async fn nodes_activate(&self, node_id: &str) -> Result<RouteNodeLifecycle, SdkworkError> {
        let path = backend_path(&format!("/control/nodes/{}/activate", serialize_path_parameter(node_id, PathParameterSpec::new("nodeId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Mark a realtime node as draining.
    pub async fn nodes_drain(&self, node_id: &str) -> Result<RouteNodeLifecycle, SdkworkError> {
        let path = backend_path(&format!("/control/nodes/{}/drain", serialize_path_parameter(node_id, PathParameterSpec::new("nodeId", "simple", false))));
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Migrate owned routes from the source node to the target node.
    pub async fn nodes_routes_migrate(&self, node_id: &str, body: &MigrateRoutesRequest) -> Result<RouteMigrationResult, SdkworkError> {
        let path = backend_path(&format!("/control/nodes/{}/routes/migrate", serialize_path_parameter(node_id, PathParameterSpec::new("nodeId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read the control-plane protocol governance snapshot.
    pub async fn protocol_governance_retrieve(&self) -> Result<ProtocolGovernanceResponse, SdkworkError> {
        let path = backend_path(&"/control/protocol_governance".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the control-plane protocol registry snapshot.
    pub async fn protocol_registry_retrieve(&self) -> Result<ProtocolRegistryResponse, SdkworkError> {
        let path = backend_path(&"/control/protocol_registry".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read provider policy history.
    pub async fn provider_policies_list(&self) -> Result<ProviderPolicyHistoryResponse, SdkworkError> {
        let path = backend_path(&"/control/provider_policies".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read provider policy diff between two versions.
    pub async fn provider_policies_diff_list(&self, from_version: i64, to_version: i64) -> Result<ProviderPolicyDiffResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("fromVersion", from_version, "form", true, false, None),
            QueryParameterSpec::new("toVersion", to_version, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/control/provider_policies/diff".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// Preview the effective provider policy result before commit.
    pub async fn provider_policies_preview(&self, body: &UpsertProviderBindingPolicyRequest) -> Result<ProviderBindingCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/provider_policies/preview".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Rollback provider policy history to a target version.
    pub async fn provider_policies_rollback(&self, body: &ProviderPolicyRollbackRequest) -> Result<ProviderBindingCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/provider_policies/rollback".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read the provider registry snapshot.
    pub async fn provider_registry_retrieve(&self) -> Result<ProviderRegistrySnapshotResponse, SdkworkError> {
        let path = backend_path(&"/control/provider_registry".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read effective provider bindings.
    pub async fn provider_bindings_list(&self, tenant_id: Option<&str>) -> Result<ProviderBindingsResponse, SdkworkError> {
        let query = build_query_string(&[
            QueryParameterSpec::new("tenantId", tenant_id, "form", true, false, None),
        ]);
        let path = append_query_string(backend_path(&"/control/provider_bindings".to_string()), &query);
        self.client.get(&path, None, None).await
    }

    /// Upsert a provider binding policy.
    pub async fn provider_bindings_create(&self, body: &UpsertProviderBindingPolicyRequest) -> Result<ProviderBindingCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/provider_bindings".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Bind a direct chat to a conversation.
    pub async fn social_direct_chats_bindings_create(&self, body: &BindDirectChatRequest) -> Result<SocialDirectChatCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/direct_chats/bindings".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read a direct chat snapshot.
    pub async fn social_direct_chats_retrieve(&self, direct_chat_id: &str) -> Result<SocialDirectChatSnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/direct_chats/{}", serialize_path_parameter(direct_chat_id, PathParameterSpec::new("directChatId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Establish an external collaboration connection.
    pub async fn social_external_connections_create(&self, body: &EstablishExternalConnectionRequest) -> Result<SocialExternalConnectionCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/external_connections".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read an external connection snapshot.
    pub async fn social_external_connections_retrieve(&self, connection_id: &str) -> Result<SocialExternalConnectionSnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/external_connections/{}", serialize_path_parameter(connection_id, PathParameterSpec::new("connectionId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Bind an external member link.
    pub async fn social_external_member_links_create(&self, body: &BindExternalMemberLinkRequest) -> Result<SocialExternalMemberLinkCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/external_member_links".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read an external member link snapshot.
    pub async fn social_external_member_links_retrieve(&self, link_id: &str) -> Result<SocialExternalMemberLinkSnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/external_member_links/{}", serialize_path_parameter(link_id, PathParameterSpec::new("linkId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Submit a friend request event.
    pub async fn social_friend_requests_create(&self, body: &SubmitFriendRequestRequest) -> Result<SocialFriendRequestCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/friend_requests".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read a friend request snapshot.
    pub async fn social_friend_requests_retrieve(&self, request_id: &str) -> Result<SocialFriendRequestSnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/friend_requests/{}", serialize_path_parameter(request_id, PathParameterSpec::new("requestId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Accept a friend request.
    pub async fn social_friend_requests_accept(&self, request_id: &str, body: &AcceptFriendRequestRequest) -> Result<SocialFriendRequestCommitResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/friend_requests/{}/accept", serialize_path_parameter(request_id, PathParameterSpec::new("requestId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Decline a friend request.
    pub async fn social_friend_requests_decline(&self, request_id: &str, body: &DeclineFriendRequestRequest) -> Result<SocialFriendRequestCommitResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/friend_requests/{}/decline", serialize_path_parameter(request_id, PathParameterSpec::new("requestId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Cancel a friend request.
    pub async fn social_friend_requests_cancel(&self, request_id: &str, body: &CancelFriendRequestRequest) -> Result<SocialFriendRequestCommitResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/friend_requests/{}/cancel", serialize_path_parameter(request_id, PathParameterSpec::new("requestId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Activate a friendship event.
    pub async fn social_friendships_create(&self, body: &ActivateFriendshipRequest) -> Result<SocialFriendshipCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/friendships".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read a friendship snapshot.
    pub async fn social_friendships_retrieve(&self, friendship_id: &str) -> Result<SocialFriendshipSnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/friendships/{}", serialize_path_parameter(friendship_id, PathParameterSpec::new("friendshipId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Remove a friendship.
    pub async fn social_friendships_remove(&self, friendship_id: &str, body: &RemoveFriendshipRequest) -> Result<SocialFriendshipCommitResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/friendships/{}/remove", serialize_path_parameter(friendship_id, PathParameterSpec::new("friendshipId", "simple", false))));
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Claim selected pending shared-channel sync entries.
    pub async fn social_runtime_claim_pending_shared_channel_sync_targeted_create(&self, body: &SocialSharedChannelSyncPendingTargetedClaimRequest) -> Result<SocialSharedChannelSyncPendingClaimResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/claim_pending_shared_channel_sync_targeted".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read the dead-letter shared-channel sync queue.
    pub async fn social_runtime_dead_letter_shared_channel_sync_list(&self) -> Result<SocialSharedChannelSyncDeadLetterInventoryResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/dead_letter_shared_channel_sync".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the delivered shared-channel sync ledger.
    pub async fn social_runtime_delivered_shared_channel_sync_list(&self) -> Result<SocialSharedChannelSyncDeliveredInventoryResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/delivered_shared_channel_sync".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read merged shared-channel sync delivery state.
    pub async fn social_runtime_delivery_state_shared_channel_sync_list(&self) -> Result<SocialSharedChannelSyncDeliveryStateInventoryResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/delivery_state_shared_channel_sync".to_string());
        self.client.get(&path, None, None).await
    }

    /// Read the pending shared-channel sync queue.
    pub async fn social_runtime_pending_shared_channel_sync_list(&self) -> Result<SocialSharedChannelSyncPendingInventoryResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/pending_shared_channel_sync".to_string());
        self.client.get(&path, None, None).await
    }

    /// Reclaim stale shared-channel sync pending ownership.
    pub async fn social_runtime_reclaim_stale_pending_shared_channel_sync_create(&self) -> Result<SocialSharedChannelSyncPendingStaleReclaimResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/reclaim_stale_pending_shared_channel_sync".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Release selected pending shared-channel sync entries.
    pub async fn social_runtime_release_pending_shared_channel_sync_targeted_create(&self, body: &SocialSharedChannelSyncPendingTargetedReleaseRequest) -> Result<SocialSharedChannelSyncPendingReleaseResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/release_pending_shared_channel_sync_targeted".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Repair the persisted social runtime derived snapshot.
    pub async fn social_runtime_repair_derived_snapshot_create(&self) -> Result<SocialRuntimeRepairResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/repair_derived_snapshot".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Repair shared-channel sync backlog state.
    pub async fn social_runtime_repair_shared_channel_sync_create(&self) -> Result<SocialSharedChannelSyncRepairResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/repair_shared_channel_sync".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Republish selected pending shared-channel sync entries.
    pub async fn social_runtime_republish_pending_shared_channel_sync_targeted_create(&self, body: &SocialSharedChannelSyncTargetedRepublishRequest) -> Result<SocialSharedChannelSyncTargetedRepublishResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/republish_pending_shared_channel_sync_targeted".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Requeue all dead-letter shared-channel sync entries.
    pub async fn social_runtime_requeue_dead_letter_shared_channel_sync_create(&self) -> Result<SocialSharedChannelSyncDeadLetterRequeueResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/requeue_dead_letter_shared_channel_sync".to_string());
        self.client.post(&path, Option::<&serde_json::Value>::None, None, None, None).await
    }

    /// Requeue selected dead-letter shared-channel sync entries.
    pub async fn social_runtime_requeue_dead_letter_shared_channel_sync_targeted_create(&self, body: &SocialSharedChannelSyncDeadLetterTargetedRequeueRequest) -> Result<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Take over selected pending shared-channel sync entries.
    pub async fn social_runtime_takeover_pending_shared_channel_sync_targeted_create(&self, body: &SocialSharedChannelSyncPendingTargetedTakeoverRequest) -> Result<SocialSharedChannelSyncPendingTakeoverResponse, SdkworkError> {
        let path = backend_path(&"/control/social/runtime/takeover_pending_shared_channel_sync_targeted".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Apply a shared-channel policy.
    pub async fn social_shared_channel_policies_create(&self, body: &ApplySharedChannelPolicyRequest) -> Result<SocialSharedChannelPolicyCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/shared_channel_policies".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read a shared-channel policy snapshot.
    pub async fn social_shared_channel_policies_retrieve(&self, policy_id: &str) -> Result<SocialSharedChannelPolicySnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/shared_channel_policies/{}", serialize_path_parameter(policy_id, PathParameterSpec::new("policyId", "simple", false))));
        self.client.get(&path, None, None).await
    }

    /// Block a user in the social graph.
    pub async fn social_user_blocks_create(&self, body: &BlockUserRequest) -> Result<SocialUserBlockCommitResponse, SdkworkError> {
        let path = backend_path(&"/control/social/user_blocks".to_string());
        self.client.post(&path, Some(body), None, None, Some("application/json")).await
    }

    /// Read a user block snapshot.
    pub async fn social_user_blocks_retrieve(&self, block_id: &str) -> Result<SocialUserBlockSnapshotResponse, SdkworkError> {
        let path = backend_path(&format!("/control/social/user_blocks/{}", serialize_path_parameter(block_id, PathParameterSpec::new("blockId", "simple", false))));
        self.client.get(&path, None, None).await
    }

}

struct PathParameterSpec<'a> {
    name: &'a str,
    style: &'a str,
    explode: bool,
}

impl<'a> PathParameterSpec<'a> {
    fn new(name: &'a str, style: &'a str, explode: bool) -> Self {
        Self { name, style, explode }
    }
}

fn serialize_path_parameter<T: serde::Serialize>(value: T, spec: PathParameterSpec<'_>) -> String {
    let value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    if value.is_null() {
        return String::new();
    }
    let style = if spec.style.is_empty() { "simple" } else { spec.style };
    match value {
        serde_json::Value::Array(values) => serialize_path_array(spec.name, &values, style, spec.explode),
        serde_json::Value::Object(values) => serialize_path_object(spec.name, &values, style, spec.explode),
        value => format!("{}{}", path_primitive_prefix(spec.name, style), percent_encode(&primitive_to_string(&value))),
    }
}

fn serialize_path_array(name: &str, values: &[serde_json::Value], style: &str, explode: bool) -> String {
    let serialized = values
        .iter()
        .filter(|value| !value.is_null())
        .map(|value| percent_encode(&primitive_to_string(value)))
        .collect::<Vec<_>>();
    if serialized.is_empty() {
        return path_prefix(name, style);
    }
    if style == "matrix" {
        if explode {
            return serialized.iter().map(|item| format!(";{}={}", name, item)).collect::<Vec<_>>().join("");
        }
        return format!(";{}={}", name, serialized.join(","));
    }
    let separator = if explode { "." } else { "," };
    format!("{}{}", path_prefix(name, style), serialized.join(separator))
}

fn serialize_path_object(
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
) -> String {
    let mut entries = Vec::new();
    let mut exploded = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        let escaped_key = percent_encode(key);
        let escaped_value = percent_encode(&primitive_to_string(value));
        if explode {
            if style == "matrix" {
                exploded.push(format!(";{}={}", escaped_key, escaped_value));
            } else {
                exploded.push(format!("{}={}", escaped_key, escaped_value));
            }
        } else {
            entries.push(escaped_key);
            entries.push(escaped_value);
        }
    }
    if style == "matrix" {
        if explode {
            return exploded.join("");
        }
        return format!(";{}={}", name, entries.join(","));
    }
    if explode {
        let separator = if style == "label" { "." } else { "," };
        return format!("{}{}", path_prefix(name, style), exploded.join(separator));
    }
    format!("{}{}", path_prefix(name, style), entries.join(","))
}

fn path_prefix(name: &str, style: &str) -> String {
    match style {
        "label" => ".".to_string(),
        "matrix" => format!(";{}", name),
        _ => String::new(),
    }
}

fn path_primitive_prefix(name: &str, style: &str) -> String {
    if style == "matrix" {
        format!(";{}=", name)
    } else {
        path_prefix(name, style)
    }
}


struct QueryParameterSpec<'a> {
    name: &'a str,
    value: serde_json::Value,
    style: &'a str,
    explode: bool,
    allow_reserved: bool,
    content_type: Option<&'a str>,
}

impl<'a> QueryParameterSpec<'a> {
    fn new<T: serde::Serialize>(
        name: &'a str,
        value: T,
        style: &'a str,
        explode: bool,
        allow_reserved: bool,
        content_type: Option<&'a str>,
    ) -> Self {
        Self {
            name,
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            style,
            explode,
            allow_reserved,
            content_type,
        }
    }
}

fn build_query_string(parameters: &[QueryParameterSpec<'_>]) -> String {
    let mut pairs = Vec::new();
    for parameter in parameters {
        append_serialized_parameter(&mut pairs, parameter);
    }
    pairs.join("&")
}

fn append_serialized_parameter(pairs: &mut Vec<String>, parameter: &QueryParameterSpec<'_>) {
    if parameter.value.is_null() {
        return;
    }
    if parameter.content_type.is_some() {
        pairs.push(format!(
            "{}={}",
            percent_encode(parameter.name),
            encode_query_value(&parameter.value.to_string(), parameter.allow_reserved)
        ));
        return;
    }

    let style = if parameter.style.is_empty() { "form" } else { parameter.style };
    match &parameter.value {
        serde_json::Value::Array(values) => append_array_parameter(pairs, parameter.name, values, style, parameter.explode, parameter.allow_reserved),
        serde_json::Value::Object(values) if style == "deepObject" => append_deep_object_parameter(pairs, parameter.name, values, parameter.allow_reserved),
        serde_json::Value::Object(values) => append_object_parameter(pairs, parameter.name, values, style, parameter.explode, parameter.allow_reserved),
        value => pairs.push(format!("{}={}", percent_encode(parameter.name), encode_query_value(&primitive_to_string(value), parameter.allow_reserved))),
    }
}

fn append_array_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &[serde_json::Value],
    style: &str,
    explode: bool,
    allow_reserved: bool,
) {
    let serialized = values.iter().filter(|value| !value.is_null()).map(primitive_to_string).collect::<Vec<_>>();
    if serialized.is_empty() {
        return;
    }
    if style == "form" && explode {
        for item in serialized {
            pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&item, allow_reserved)));
        }
        return;
    }
    pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&serialized.join(","), allow_reserved)));
}

fn append_object_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    style: &str,
    explode: bool,
    allow_reserved: bool,
) {
    let mut serialized = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        if style == "form" && explode {
            pairs.push(format!("{}={}", percent_encode(key), encode_query_value(&primitive_to_string(value), allow_reserved)));
        } else {
            serialized.push(key.clone());
            serialized.push(primitive_to_string(value));
        }
    }
    if !serialized.is_empty() {
        pairs.push(format!("{}={}", percent_encode(name), encode_query_value(&serialized.join(","), allow_reserved)));
    }
}

fn append_deep_object_parameter(
    pairs: &mut Vec<String>,
    name: &str,
    values: &serde_json::Map<String, serde_json::Value>,
    allow_reserved: bool,
) {
    for (key, value) in values {
        if !value.is_null() {
            pairs.push(format!("{}={}", percent_encode(&format!("{}[{}]", name, key)), encode_query_value(&primitive_to_string(value), allow_reserved)));
        }
    }
}

fn encode_query_value(value: &str, allow_reserved: bool) -> String {
    let mut encoded = percent_encode(value);
    if !allow_reserved {
        return encoded;
    }
    for (escaped, reserved) in [
        ("%3A", ":"), ("%2F", "/"), ("%3F", "?"), ("%23", "#"),
        ("%5B", "["), ("%5D", "]"), ("%40", "@"), ("%21", "!"),
        ("%24", "$"), ("%26", "&"), ("%27", "'"), ("%28", "("),
        ("%29", ")"), ("%2A", "*"), ("%2B", "+"), ("%2C", ","),
        ("%3B", ";"), ("%3D", "="),
    ] {
        encoded = encoded.replace(escaped, reserved);
    }
    encoded
}

fn primitive_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        other => other.to_string(),
    }
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{:02X}", byte).chars().collect(),
        })
        .collect()
}
