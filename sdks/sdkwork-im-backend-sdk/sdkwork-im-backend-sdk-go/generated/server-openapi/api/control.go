package api

import (
    "encoding/json"
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-backend-api-generated/types"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type ControlApi struct {
    client *sdkhttp.Client
}

func NewControlApi(client *sdkhttp.Client) *ControlApi {
    return &ControlApi{client: client}
}

// Activate a realtime node and clear drain state.
func (a *ControlApi) NodesActivate(nodeId string) (sdktypes.RouteNodeLifecycle, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/nodes/%s/activate", SerializePathParameter(nodeId, PathParameterSpec{Name: "nodeId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.RouteNodeLifecycle
        return zero, err
    }
    return decodeResult[sdktypes.RouteNodeLifecycle](raw)
}

// Mark a realtime node as draining.
func (a *ControlApi) NodesDrain(nodeId string) (sdktypes.RouteNodeLifecycle, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/nodes/%s/drain", SerializePathParameter(nodeId, PathParameterSpec{Name: "nodeId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.RouteNodeLifecycle
        return zero, err
    }
    return decodeResult[sdktypes.RouteNodeLifecycle](raw)
}

// Migrate owned routes from the source node to the target node.
func (a *ControlApi) NodesRoutesMigrate(nodeId string, body sdktypes.MigrateRoutesRequest) (sdktypes.RouteMigrationResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/nodes/%s/routes/migrate", SerializePathParameter(nodeId, PathParameterSpec{Name: "nodeId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RouteMigrationResult
        return zero, err
    }
    return decodeResult[sdktypes.RouteMigrationResult](raw)
}

// Read the control-plane protocol governance snapshot.
func (a *ControlApi) ProtocolGovernanceRetrieve() (sdktypes.ProtocolGovernanceResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/protocol_governance"), nil, nil)
    if err != nil {
        var zero sdktypes.ProtocolGovernanceResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolGovernanceResponse](raw)
}

// Read the control-plane protocol registry snapshot.
func (a *ControlApi) ProtocolRegistryRetrieve() (sdktypes.ProtocolRegistryResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/protocol_registry"), nil, nil)
    if err != nil {
        var zero sdktypes.ProtocolRegistryResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolRegistryResponse](raw)
}

// Read provider policy history.
func (a *ControlApi) ProviderPoliciesList() (sdktypes.ProviderPolicyHistoryResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/provider_policies"), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderPolicyHistoryResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderPolicyHistoryResponse](raw)
}

// Read provider policy diff between two versions.
func (a *ControlApi) ProviderPoliciesDiffList(fromVersion int, toVersion int) (sdktypes.ProviderPolicyDiffResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "fromVersion", Value: fromVersion, Style: "form", Explode: true, AllowReserved: false},
        {Name: "toVersion", Value: toVersion, Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/provider_policies/diff"), query), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderPolicyDiffResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderPolicyDiffResponse](raw)
}

// Preview the effective provider policy result before commit.
func (a *ControlApi) ProviderPoliciesPreview(body sdktypes.UpsertProviderBindingPolicyRequest) (sdktypes.ProviderBindingCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/provider_policies/preview"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ProviderBindingCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderBindingCommitResponse](raw)
}

// Rollback provider policy history to a target version.
func (a *ControlApi) ProviderPoliciesRollback(body sdktypes.ProviderPolicyRollbackRequest) (sdktypes.ProviderBindingCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/provider_policies/rollback"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ProviderBindingCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderBindingCommitResponse](raw)
}

// Read the provider registry snapshot.
func (a *ControlApi) ProviderRegistryRetrieve() (sdktypes.ProviderRegistrySnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/provider_registry"), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderRegistrySnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderRegistrySnapshotResponse](raw)
}

// Read effective provider bindings.
func (a *ControlApi) ProviderBindingsList(tenantId *string) (sdktypes.ProviderBindingsResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "tenantId", Value: func() interface{} { if tenantId == nil { return nil }; return *tenantId }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/provider_bindings"), query), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderBindingsResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderBindingsResponse](raw)
}

// Upsert a provider binding policy.
func (a *ControlApi) ProviderBindingsCreate(body sdktypes.UpsertProviderBindingPolicyRequest) (sdktypes.ProviderBindingCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/provider_bindings"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ProviderBindingCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderBindingCommitResponse](raw)
}

// Bind a direct chat to a conversation.
func (a *ControlApi) SocialDirectChatsBindingsCreate(body sdktypes.BindDirectChatRequest) (sdktypes.SocialDirectChatCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/direct_chats/bindings"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialDirectChatCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialDirectChatCommitResponse](raw)
}

// Read a direct chat snapshot.
func (a *ControlApi) SocialDirectChatsRetrieve(directChatId string) (sdktypes.SocialDirectChatSnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/direct_chats/%s", SerializePathParameter(directChatId, PathParameterSpec{Name: "directChatId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialDirectChatSnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialDirectChatSnapshotResponse](raw)
}

// Establish an external collaboration connection.
func (a *ControlApi) SocialExternalConnectionsCreate(body sdktypes.EstablishExternalConnectionRequest) (sdktypes.SocialExternalConnectionCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/external_connections"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialExternalConnectionCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalConnectionCommitResponse](raw)
}

// Read an external connection snapshot.
func (a *ControlApi) SocialExternalConnectionsRetrieve(connectionId string) (sdktypes.SocialExternalConnectionSnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/external_connections/%s", SerializePathParameter(connectionId, PathParameterSpec{Name: "connectionId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialExternalConnectionSnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalConnectionSnapshotResponse](raw)
}

// Bind an external member link.
func (a *ControlApi) SocialExternalMemberLinksCreate(body sdktypes.BindExternalMemberLinkRequest) (sdktypes.SocialExternalMemberLinkCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/external_member_links"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialExternalMemberLinkCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalMemberLinkCommitResponse](raw)
}

// Read an external member link snapshot.
func (a *ControlApi) SocialExternalMemberLinksRetrieve(linkId string) (sdktypes.SocialExternalMemberLinkSnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/external_member_links/%s", SerializePathParameter(linkId, PathParameterSpec{Name: "linkId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialExternalMemberLinkSnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalMemberLinkSnapshotResponse](raw)
}

// Submit a friend request event.
func (a *ControlApi) SocialFriendRequestsCreate(body sdktypes.SubmitFriendRequestRequest) (sdktypes.SocialFriendRequestCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/friend_requests"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestCommitResponse](raw)
}

// Read a friend request snapshot.
func (a *ControlApi) SocialFriendRequestsRetrieve(requestId string) (sdktypes.SocialFriendRequestSnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialFriendRequestSnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestSnapshotResponse](raw)
}

// Accept a friend request.
func (a *ControlApi) SocialFriendRequestsAccept(requestId string, body sdktypes.AcceptFriendRequestRequest) (sdktypes.SocialFriendRequestCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s/accept", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestCommitResponse](raw)
}

// Decline a friend request.
func (a *ControlApi) SocialFriendRequestsDecline(requestId string, body sdktypes.DeclineFriendRequestRequest) (sdktypes.SocialFriendRequestCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s/decline", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestCommitResponse](raw)
}

// Cancel a friend request.
func (a *ControlApi) SocialFriendRequestsCancel(requestId string, body sdktypes.CancelFriendRequestRequest) (sdktypes.SocialFriendRequestCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s/cancel", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestCommitResponse](raw)
}

// Activate a friendship event.
func (a *ControlApi) SocialFriendshipsCreate(body sdktypes.ActivateFriendshipRequest) (sdktypes.SocialFriendshipCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/friendships"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendshipCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipCommitResponse](raw)
}

// Read a friendship snapshot.
func (a *ControlApi) SocialFriendshipsRetrieve(friendshipId string) (sdktypes.SocialFriendshipSnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/friendships/%s", SerializePathParameter(friendshipId, PathParameterSpec{Name: "friendshipId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialFriendshipSnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipSnapshotResponse](raw)
}

// Remove a friendship.
func (a *ControlApi) SocialFriendshipsRemove(friendshipId string, body sdktypes.RemoveFriendshipRequest) (sdktypes.SocialFriendshipCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friendships/%s/remove", SerializePathParameter(friendshipId, PathParameterSpec{Name: "friendshipId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendshipCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipCommitResponse](raw)
}

// Claim selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeClaimPendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncPendingTargetedClaimRequest) (sdktypes.SocialSharedChannelSyncPendingClaimResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncPendingClaimResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncPendingClaimResponse](raw)
}

// Read the dead-letter shared-channel sync queue.
func (a *ControlApi) SocialRuntimeDeadLetterSharedChannelSyncList() (sdktypes.SocialSharedChannelSyncDeadLetterInventoryResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/social/runtime/dead_letter_shared_channel_sync"), nil, nil)
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncDeadLetterInventoryResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncDeadLetterInventoryResponse](raw)
}

// Read the delivered shared-channel sync ledger.
func (a *ControlApi) SocialRuntimeDeliveredSharedChannelSyncList() (sdktypes.SocialSharedChannelSyncDeliveredInventoryResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/social/runtime/delivered_shared_channel_sync"), nil, nil)
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncDeliveredInventoryResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncDeliveredInventoryResponse](raw)
}

// Read merged shared-channel sync delivery state.
func (a *ControlApi) SocialRuntimeDeliveryStateSharedChannelSyncList() (sdktypes.SocialSharedChannelSyncDeliveryStateInventoryResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/social/runtime/delivery_state_shared_channel_sync"), nil, nil)
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncDeliveryStateInventoryResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncDeliveryStateInventoryResponse](raw)
}

// Read the pending shared-channel sync queue.
func (a *ControlApi) SocialRuntimePendingSharedChannelSyncList() (sdktypes.SocialSharedChannelSyncPendingInventoryResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/social/runtime/pending_shared_channel_sync"), nil, nil)
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncPendingInventoryResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncPendingInventoryResponse](raw)
}

// Reclaim stale shared-channel sync pending ownership.
func (a *ControlApi) SocialRuntimeReclaimStalePendingSharedChannelSyncCreate() (sdktypes.SocialSharedChannelSyncPendingStaleReclaimResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncPendingStaleReclaimResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncPendingStaleReclaimResponse](raw)
}

// Release selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeReleasePendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncPendingTargetedReleaseRequest) (sdktypes.SocialSharedChannelSyncPendingReleaseResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncPendingReleaseResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncPendingReleaseResponse](raw)
}

// Repair the persisted social runtime derived snapshot.
func (a *ControlApi) SocialRuntimeRepairDerivedSnapshotCreate() (sdktypes.SocialRuntimeRepairResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/repair_derived_snapshot"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialRuntimeRepairResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeRepairResponse](raw)
}

// Repair shared-channel sync backlog state.
func (a *ControlApi) SocialRuntimeRepairSharedChannelSyncCreate() (sdktypes.SocialSharedChannelSyncRepairResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/repair_shared_channel_sync"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncRepairResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncRepairResponse](raw)
}

// Republish selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncTargetedRepublishRequest) (sdktypes.SocialSharedChannelSyncTargetedRepublishResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncTargetedRepublishResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncTargetedRepublishResponse](raw)
}

// Requeue all dead-letter shared-channel sync entries.
func (a *ControlApi) SocialRuntimeRequeueDeadLetterSharedChannelSyncCreate() (sdktypes.SocialSharedChannelSyncDeadLetterRequeueResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncDeadLetterRequeueResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncDeadLetterRequeueResponse](raw)
}

// Requeue selected dead-letter shared-channel sync entries.
func (a *ControlApi) SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncDeadLetterTargetedRequeueRequest) (sdktypes.SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncDeadLetterTargetedRequeueResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncDeadLetterTargetedRequeueResponse](raw)
}

// Take over selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncPendingTargetedTakeoverRequest) (sdktypes.SocialSharedChannelSyncPendingTakeoverResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelSyncPendingTakeoverResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelSyncPendingTakeoverResponse](raw)
}

// Apply a shared-channel policy.
func (a *ControlApi) SocialSharedChannelPoliciesCreate(body sdktypes.ApplySharedChannelPolicyRequest) (sdktypes.SocialSharedChannelPolicyCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/shared_channel_policies"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelPolicyCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelPolicyCommitResponse](raw)
}

// Read a shared-channel policy snapshot.
func (a *ControlApi) SocialSharedChannelPoliciesRetrieve(policyId string) (sdktypes.SocialSharedChannelPolicySnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/shared_channel_policies/%s", SerializePathParameter(policyId, PathParameterSpec{Name: "policyId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialSharedChannelPolicySnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelPolicySnapshotResponse](raw)
}

// Block a user in the social graph.
func (a *ControlApi) SocialUserBlocksCreate(body sdktypes.BlockUserRequest) (sdktypes.SocialUserBlockCommitResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/user_blocks"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialUserBlockCommitResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialUserBlockCommitResponse](raw)
}

// Read a user block snapshot.
func (a *ControlApi) SocialUserBlocksRetrieve(blockId string) (sdktypes.SocialUserBlockSnapshotResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/user_blocks/%s", SerializePathParameter(blockId, PathParameterSpec{Name: "blockId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialUserBlockSnapshotResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialUserBlockSnapshotResponse](raw)
}

type PathParameterSpec struct {
    Name    string
    Style   string
    Explode bool
}

func SerializePathParameter(value interface{}, spec PathParameterSpec) string {
    if value == nil {
        return ""
    }
    style := spec.Style
    if style == "" {
        style = "simple"
    }

    switch typed := value.(type) {
    case []string:
        return SerializePathArray(spec.Name, stringSliceToInterface(typed), style, spec.Explode)
    case []int:
        return SerializePathArray(spec.Name, intSliceToInterface(typed), style, spec.Explode)
    case []interface{}:
        return SerializePathArray(spec.Name, typed, style, spec.Explode)
    case map[string]string:
        return SerializePathObject(spec.Name, stringMapToInterface(typed), style, spec.Explode)
    case map[string]int:
        return SerializePathObject(spec.Name, intMapToInterface(typed), style, spec.Explode)
    case map[string]interface{}:
        return SerializePathObject(spec.Name, typed, style, spec.Explode)
    default:
        return PathPrefix(spec.Name, style) + url.PathEscape(fmt.Sprint(value))
    }
}

func SerializePathArray(name string, values []interface{}, style string, explode bool) string {
    serialized := make([]string, 0, len(values))
    for _, item := range values {
        if item != nil {
            serialized = append(serialized, url.PathEscape(fmt.Sprint(item)))
        }
    }
    if len(serialized) == 0 {
        return PathPrefix(name, style)
    }
    if style == "matrix" {
        if explode {
            parts := make([]string, 0, len(serialized))
            for _, item := range serialized {
                parts = append(parts, ";"+name+"="+item)
            }
            return strings.Join(parts, "")
        }
        return ";" + name + "=" + strings.Join(serialized, ",")
    }
    separator := ","
    if explode {
        separator = "."
    }
    return PathPrefix(name, style) + strings.Join(serialized, separator)
}

func SerializePathObject(name string, values map[string]interface{}, style string, explode bool) string {
    entries := make([]string, 0, len(values)*2)
    exploded := make([]string, 0, len(values))
    for key, value := range values {
        if value == nil {
            continue
        }
        escapedKey := url.PathEscape(key)
        escapedValue := url.PathEscape(fmt.Sprint(value))
        if explode {
            if style == "matrix" {
                exploded = append(exploded, ";"+escapedKey+"="+escapedValue)
            } else {
                exploded = append(exploded, escapedKey+"="+escapedValue)
            }
        } else {
            entries = append(entries, escapedKey, escapedValue)
        }
    }
    if style == "matrix" {
        if explode {
            return strings.Join(exploded, "")
        }
        return ";" + name + "=" + strings.Join(entries, ",")
    }
    if explode {
        separator := ","
        if style == "label" {
            separator = "."
        }
        return PathPrefix(name, style) + strings.Join(exploded, separator)
    }
    return PathPrefix(name, style) + strings.Join(entries, ",")
}

func PathPrefix(name string, style string) string {
    if style == "label" {
        return "."
    }
    if style == "matrix" {
        return ";" + name
    }
    return ""
}
type QueryParameterSpec struct {
    Name          string
    Value         interface{}
    Style         string
    Explode       bool
    AllowReserved bool
    ContentType   string
}

func BuildQueryString(parameters []QueryParameterSpec) string {
    pairs := make([]string, 0)
    for _, parameter := range parameters {
        AppendSerializedParameter(&pairs, parameter)
    }
    return strings.Join(pairs, "&")
}

func AppendSerializedParameter(pairs *[]string, parameter QueryParameterSpec) {
    if parameter.Value == nil {
        return
    }

    if parameter.ContentType != "" {
        encoded, _ := json.Marshal(parameter.Value)
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(string(encoded), parameter.AllowReserved))
        return
    }

    style := parameter.Style
    if style == "" {
        style = "form"
    }

    switch value := parameter.Value.(type) {
    case []string:
        AppendArrayParameter(pairs, parameter.Name, stringSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []int:
        AppendArrayParameter(pairs, parameter.Name, intSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []interface{}:
        AppendArrayParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
    case map[string]int:
        AppendObjectParameter(pairs, parameter.Name, intMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]string:
        AppendObjectParameter(pairs, parameter.Name, stringMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]interface{}:
        if style == "deepObject" {
            AppendDeepObjectParameter(pairs, parameter.Name, value, parameter.AllowReserved)
        } else {
            AppendObjectParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
        }
    default:
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(fmt.Sprint(value), parameter.AllowReserved))
    }
}

func AppendArrayParameter(pairs *[]string, name string, value []interface{}, style string, explode bool, allowReserved bool) {
    values := make([]string, 0, len(value))
    for _, item := range value {
        if item != nil {
            values = append(values, fmt.Sprint(item))
        }
    }
    if len(values) == 0 {
        return
    }
    if style == "form" && explode {
        for _, item := range values {
            *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(item, allowReserved))
        }
        return
    }
    *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(values, ","), allowReserved))
}

func AppendObjectParameter(pairs *[]string, name string, value map[string]interface{}, style string, explode bool, allowReserved bool) {
    entries := make([]string, 0, len(value)*2)
    for key, item := range value {
        if item == nil {
            continue
        }
        if style == "form" && explode {
            *pairs = append(*pairs, url.QueryEscape(key)+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
            continue
        }
        entries = append(entries, key, fmt.Sprint(item))
    }
    if len(entries) == 0 {
        return
    }
    if !(style == "form" && explode) {
        *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(entries, ","), allowReserved))
    }
}

func AppendDeepObjectParameter(pairs *[]string, name string, value map[string]interface{}, allowReserved bool) {
    for key, item := range value {
        if item == nil {
            continue
        }
        *pairs = append(*pairs, url.QueryEscape(fmt.Sprintf("%s[%s]", name, key))+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
    }
}

func EncodeQueryValue(value string, allowReserved bool) string {
    encoded := url.QueryEscape(value)
    if !allowReserved {
        return encoded
    }
    replacements := map[string]string{
        "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
        "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
        "%24": "$", "%26": "&", "%27": "'", "%28": "(",
        "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
        "%3B": ";", "%3D": "=",
    }
    for escaped, reserved := range replacements {
        encoded = strings.ReplaceAll(encoded, escaped, reserved)
    }
    return encoded
}



func stringSliceToInterface(values []string) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func intSliceToInterface(values []int) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func stringMapToInterface(values map[string]string) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}

func intMapToInterface(values map[string]int) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}
