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
func (a *ControlApi) NodesActivate(nodeId string) (sdktypes.NodesActivateResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/nodes/%s/activate", SerializePathParameter(nodeId, PathParameterSpec{Name: "nodeId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.NodesActivateResponse
        return zero, err
    }
    return decodeResult[sdktypes.NodesActivateResponse](raw)
}

// Mark a realtime node as draining.
func (a *ControlApi) NodesDrain(nodeId string) (sdktypes.NodesDrainResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/nodes/%s/drain", SerializePathParameter(nodeId, PathParameterSpec{Name: "nodeId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.NodesDrainResponse
        return zero, err
    }
    return decodeResult[sdktypes.NodesDrainResponse](raw)
}

// Migrate owned routes from the source node to the target node.
func (a *ControlApi) NodesRoutesMigrate(nodeId string, body sdktypes.MigrateRoutesRequest) (sdktypes.NodesRoutesMigrateResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/nodes/%s/routes/migrate", SerializePathParameter(nodeId, PathParameterSpec{Name: "nodeId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.NodesRoutesMigrateResponse
        return zero, err
    }
    return decodeResult[sdktypes.NodesRoutesMigrateResponse](raw)
}

// Read the control-plane protocol governance snapshot.
func (a *ControlApi) ProtocolGovernanceRetrieve() (sdktypes.ProtocolGovernanceRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/protocol_governance"), nil, nil)
    if err != nil {
        var zero sdktypes.ProtocolGovernanceRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolGovernanceRetrieveResponse](raw)
}

// Read the control-plane protocol registry snapshot.
func (a *ControlApi) ProtocolRegistryRetrieve() (sdktypes.ProtocolRegistryRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/protocol_registry"), nil, nil)
    if err != nil {
        var zero sdktypes.ProtocolRegistryRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolRegistryRetrieveResponse](raw)
}

// Read provider policy history.
func (a *ControlApi) ProviderPoliciesList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/provider_policies"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Read provider policy diff between two versions.
func (a *ControlApi) ProviderPoliciesDiffList(fromVersion string, toVersion string, pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "fromVersion", Value: fromVersion, Style: "form", Explode: true, AllowReserved: false},
        {Name: "toVersion", Value: toVersion, Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/provider_policies/diff"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Preview the effective provider policy result before commit.
func (a *ControlApi) ProviderPoliciesPreview(body sdktypes.UpsertProviderBindingPolicyRequest) (sdktypes.ProviderPoliciesPreviewResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/provider_policies/preview"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ProviderPoliciesPreviewResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderPoliciesPreviewResponse](raw)
}

// Rollback provider policy history to a target version.
func (a *ControlApi) ProviderPoliciesRollback(body sdktypes.ProviderPolicyRollbackRequest) (sdktypes.ProviderPoliciesRollbackResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/control/provider_policies/rollback"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ProviderPoliciesRollbackResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderPoliciesRollbackResponse](raw)
}

// Read the provider registry snapshot.
func (a *ControlApi) ProviderRegistryRetrieve() (sdktypes.ProviderRegistryRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/control/provider_registry"), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderRegistryRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderRegistryRetrieveResponse](raw)
}

// Read effective provider bindings.
func (a *ControlApi) ProviderBindingsList(tenantId *string, pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "tenantId", Value: func() interface{} { if tenantId == nil { return nil }; return *tenantId }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/provider_bindings"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Upsert a provider binding policy.
func (a *ControlApi) ProviderBindingsCreate(body sdktypes.UpsertProviderBindingPolicyRequest) (sdktypes.ControlProviderBindingsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/provider_bindings"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ControlProviderBindingsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ControlProviderBindingsCreateResponse201](raw)
}

// Bind a direct chat to a conversation.
func (a *ControlApi) SocialDirectChatsBindingsCreate(body sdktypes.BindDirectChatRequest) (sdktypes.SocialDirectChatsBindingsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/direct_chats/bindings"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialDirectChatsBindingsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialDirectChatsBindingsCreateResponse201](raw)
}

// Read a direct chat snapshot.
func (a *ControlApi) SocialDirectChatsRetrieve(directChatId string) (sdktypes.SocialDirectChatsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/direct_chats/%s", SerializePathParameter(directChatId, PathParameterSpec{Name: "directChatId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialDirectChatsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialDirectChatsRetrieveResponse](raw)
}

// Establish an external collaboration connection.
func (a *ControlApi) SocialExternalConnectionsCreate(body sdktypes.EstablishExternalConnectionRequest) (sdktypes.SocialExternalConnectionsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/external_connections"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialExternalConnectionsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalConnectionsCreateResponse201](raw)
}

// Read an external connection snapshot.
func (a *ControlApi) SocialExternalConnectionsRetrieve(connectionId string) (sdktypes.SocialExternalConnectionsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/external_connections/%s", SerializePathParameter(connectionId, PathParameterSpec{Name: "connectionId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialExternalConnectionsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalConnectionsRetrieveResponse](raw)
}

// Bind an external member link.
func (a *ControlApi) SocialExternalMemberLinksCreate(body sdktypes.BindExternalMemberLinkRequest) (sdktypes.SocialExternalMemberLinksCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/external_member_links"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialExternalMemberLinksCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalMemberLinksCreateResponse201](raw)
}

// Read an external member link snapshot.
func (a *ControlApi) SocialExternalMemberLinksRetrieve(linkId string) (sdktypes.SocialExternalMemberLinksRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/external_member_links/%s", SerializePathParameter(linkId, PathParameterSpec{Name: "linkId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialExternalMemberLinksRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialExternalMemberLinksRetrieveResponse](raw)
}

// Submit a friend request event.
func (a *ControlApi) SocialFriendRequestsCreate(body sdktypes.SubmitFriendRequestRequest) (sdktypes.SocialFriendRequestsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/friend_requests"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsCreateResponse201](raw)
}

// Read a friend request snapshot.
func (a *ControlApi) SocialFriendRequestsRetrieve(requestId string) (sdktypes.SocialFriendRequestsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialFriendRequestsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsRetrieveResponse](raw)
}

// Accept a friend request.
func (a *ControlApi) SocialFriendRequestsAccept(requestId string, body sdktypes.AcceptFriendRequestRequest) (sdktypes.SocialFriendRequestsAcceptResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s/accept", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsAcceptResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsAcceptResponse](raw)
}

// Decline a friend request.
func (a *ControlApi) SocialFriendRequestsDecline(requestId string, body sdktypes.DeclineFriendRequestRequest) (sdktypes.SocialFriendRequestsDeclineResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s/decline", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsDeclineResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsDeclineResponse](raw)
}

// Cancel a friend request.
func (a *ControlApi) SocialFriendRequestsCancel(requestId string, body sdktypes.CancelFriendRequestRequest) (sdktypes.SocialFriendRequestsCancelResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friend_requests/%s/cancel", SerializePathParameter(requestId, PathParameterSpec{Name: "requestId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsCancelResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsCancelResponse](raw)
}

// Activate a friendship event.
func (a *ControlApi) SocialFriendshipsCreate(body sdktypes.ActivateFriendshipRequest) (sdktypes.SocialFriendshipsCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/friendships"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendshipsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipsCreateResponse201](raw)
}

// Read a friendship snapshot.
func (a *ControlApi) SocialFriendshipsRetrieve(friendshipId string) (sdktypes.SocialFriendshipsRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/friendships/%s", SerializePathParameter(friendshipId, PathParameterSpec{Name: "friendshipId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialFriendshipsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipsRetrieveResponse](raw)
}

// Remove a friendship.
func (a *ControlApi) SocialFriendshipsRemove(friendshipId string, body sdktypes.RemoveFriendshipRequest) (sdktypes.SocialFriendshipsRemoveResponse, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/control/social/friendships/%s/remove", SerializePathParameter(friendshipId, PathParameterSpec{Name: "friendshipId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendshipsRemoveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipsRemoveResponse](raw)
}

// Claim selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeClaimPendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncPendingTargetedClaimRequest) (sdktypes.SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201](raw)
}

// Read the dead-letter shared-channel sync queue.
func (a *ControlApi) SocialRuntimeDeadLetterSharedChannelSyncList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/social/runtime/dead_letter_shared_channel_sync"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Read the delivered shared-channel sync ledger.
func (a *ControlApi) SocialRuntimeDeliveredSharedChannelSyncList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/social/runtime/delivered_shared_channel_sync"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Read merged shared-channel sync delivery state.
func (a *ControlApi) SocialRuntimeDeliveryStateSharedChannelSyncList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/social/runtime/delivery_state_shared_channel_sync"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Read the pending shared-channel sync queue.
func (a *ControlApi) SocialRuntimePendingSharedChannelSyncList(pageSize *int, cursor *string, page *int, q *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page", Value: func() interface{} { if page == nil { return nil }; return *page }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(BackendApiPath("/control/social/runtime/pending_shared_channel_sync"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Reclaim stale shared-channel sync pending ownership.
func (a *ControlApi) SocialRuntimeReclaimStalePendingSharedChannelSyncCreate() (sdktypes.SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201](raw)
}

// Release selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeReleasePendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncPendingTargetedReleaseRequest) (sdktypes.SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201](raw)
}

// Repair the persisted social runtime derived snapshot.
func (a *ControlApi) SocialRuntimeRepairDerivedSnapshotCreate() (sdktypes.SocialRuntimeRepairDerivedSnapshotCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/repair_derived_snapshot"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialRuntimeRepairDerivedSnapshotCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeRepairDerivedSnapshotCreateResponse201](raw)
}

// Repair shared-channel sync backlog state.
func (a *ControlApi) SocialRuntimeRepairSharedChannelSyncCreate() (sdktypes.SocialRuntimeRepairSharedChannelSyncCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/repair_shared_channel_sync"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialRuntimeRepairSharedChannelSyncCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeRepairSharedChannelSyncCreateResponse201](raw)
}

// Republish selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncTargetedRepublishRequest) (sdktypes.SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201](raw)
}

// Requeue all dead-letter shared-channel sync entries.
func (a *ControlApi) SocialRuntimeRequeueDeadLetterSharedChannelSyncCreate() (sdktypes.SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201](raw)
}

// Requeue selected dead-letter shared-channel sync entries.
func (a *ControlApi) SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncDeadLetterTargetedRequeueRequest) (sdktypes.SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201](raw)
}

// Take over selected pending shared-channel sync entries.
func (a *ControlApi) SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(body sdktypes.SocialSharedChannelSyncPendingTargetedTakeoverRequest) (sdktypes.SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201](raw)
}

// Apply a shared-channel policy.
func (a *ControlApi) SocialSharedChannelPoliciesCreate(body sdktypes.ApplySharedChannelPolicyRequest) (sdktypes.SocialSharedChannelPoliciesCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/shared_channel_policies"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialSharedChannelPoliciesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelPoliciesCreateResponse201](raw)
}

// Read a shared-channel policy snapshot.
func (a *ControlApi) SocialSharedChannelPoliciesRetrieve(policyId string) (sdktypes.SocialSharedChannelPoliciesRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/shared_channel_policies/%s", SerializePathParameter(policyId, PathParameterSpec{Name: "policyId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialSharedChannelPoliciesRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialSharedChannelPoliciesRetrieveResponse](raw)
}

// Block a user in the social graph.
func (a *ControlApi) SocialUserBlocksCreate(body sdktypes.BlockUserRequest) (sdktypes.SocialUserBlocksCreateResponse201, error) {
    raw, err := a.client.Post(BackendApiPath("/control/social/user_blocks"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialUserBlocksCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialUserBlocksCreateResponse201](raw)
}

// Read a user block snapshot.
func (a *ControlApi) SocialUserBlocksRetrieve(blockId string) (sdktypes.SocialUserBlocksRetrieveResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/control/social/user_blocks/%s", SerializePathParameter(blockId, PathParameterSpec{Name: "blockId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialUserBlocksRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialUserBlocksRetrieveResponse](raw)
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
