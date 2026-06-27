using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.BackendApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.BackendApi.Generated.Api
{
    public class ControlApi
    {
        private readonly SdkHttpClient _client;

        public ControlApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Activate a realtime node and clear drain state.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.RouteNodeLifecycle?> NodesActivateAsync(string nodeId)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.RouteNodeLifecycle>(ApiPaths.BackendPath($"/control/nodes/{SerializePathParameter(nodeId, new PathParameterSpec("nodeId", "simple", false))}/activate"), null);
        }

        /// <summary>
        /// Mark a realtime node as draining.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.RouteNodeLifecycle?> NodesDrainAsync(string nodeId)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.RouteNodeLifecycle>(ApiPaths.BackendPath($"/control/nodes/{SerializePathParameter(nodeId, new PathParameterSpec("nodeId", "simple", false))}/drain"), null);
        }

        /// <summary>
        /// Migrate owned routes from the source node to the target node.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.RouteMigrationResult?> NodesRoutesMigrateAsync(string nodeId, Sdkwork.Im.BackendApi.Generated.Models.MigrateRoutesRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.RouteMigrationResult>(ApiPaths.BackendPath($"/control/nodes/{SerializePathParameter(nodeId, new PathParameterSpec("nodeId", "simple", false))}/routes/migrate"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read the control-plane protocol governance snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProtocolGovernanceResponse?> ProtocolGovernanceRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProtocolGovernanceResponse>(ApiPaths.BackendPath("/control/protocol_governance"));
        }

        /// <summary>
        /// Read the control-plane protocol registry snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProtocolRegistryResponse?> ProtocolRegistryRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProtocolRegistryResponse>(ApiPaths.BackendPath("/control/protocol_registry"));
        }

        /// <summary>
        /// Read provider policy history.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderPolicyHistoryResponse?> ProviderPoliciesListAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderPolicyHistoryResponse>(ApiPaths.BackendPath("/control/provider_policies"));
        }

        /// <summary>
        /// Read provider policy diff between two versions.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderPolicyDiffResponse?> ProviderPoliciesDiffListAsync(int fromVersion, int toVersion)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("fromVersion", fromVersion, "form", true, false, null),
                new QueryParameterSpec("toVersion", toVersion, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderPolicyDiffResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/control/provider_policies/diff"), queryString));
        }

        /// <summary>
        /// Preview the effective provider policy result before commit.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingCommitResponse?> ProviderPoliciesPreviewAsync(Sdkwork.Im.BackendApi.Generated.Models.UpsertProviderBindingPolicyRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingCommitResponse>(ApiPaths.BackendPath("/control/provider_policies/preview"), body, null, null, "application/json");
        }

        /// <summary>
        /// Rollback provider policy history to a target version.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingCommitResponse?> ProviderPoliciesRollbackAsync(Sdkwork.Im.BackendApi.Generated.Models.ProviderPolicyRollbackRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingCommitResponse>(ApiPaths.BackendPath("/control/provider_policies/rollback"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read the provider registry snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderRegistrySnapshotResponse?> ProviderRegistryRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderRegistrySnapshotResponse>(ApiPaths.BackendPath("/control/provider_registry"));
        }

        /// <summary>
        /// Read effective provider bindings.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingsResponse?> ProviderBindingsListAsync(string? tenantId = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("tenantId", tenantId, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingsResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/control/provider_bindings"), queryString));
        }

        /// <summary>
        /// Upsert a provider binding policy.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingCommitResponse?> ProviderBindingsCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.UpsertProviderBindingPolicyRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ProviderBindingCommitResponse>(ApiPaths.BackendPath("/control/provider_bindings"), body, null, null, "application/json");
        }

        /// <summary>
        /// Bind a direct chat to a conversation.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialDirectChatCommitResponse?> SocialDirectChatsBindingsCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.BindDirectChatRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialDirectChatCommitResponse>(ApiPaths.BackendPath("/control/social/direct_chats/bindings"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read a direct chat snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialDirectChatSnapshotResponse?> SocialDirectChatsRetrieveAsync(string directChatId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialDirectChatSnapshotResponse>(ApiPaths.BackendPath($"/control/social/direct_chats/{SerializePathParameter(directChatId, new PathParameterSpec("directChatId", "simple", false))}"));
        }

        /// <summary>
        /// Establish an external collaboration connection.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalConnectionCommitResponse?> SocialExternalConnectionsCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.EstablishExternalConnectionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalConnectionCommitResponse>(ApiPaths.BackendPath("/control/social/external_connections"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read an external connection snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalConnectionSnapshotResponse?> SocialExternalConnectionsRetrieveAsync(string connectionId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalConnectionSnapshotResponse>(ApiPaths.BackendPath($"/control/social/external_connections/{SerializePathParameter(connectionId, new PathParameterSpec("connectionId", "simple", false))}"));
        }

        /// <summary>
        /// Bind an external member link.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalMemberLinkCommitResponse?> SocialExternalMemberLinksCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.BindExternalMemberLinkRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalMemberLinkCommitResponse>(ApiPaths.BackendPath("/control/social/external_member_links"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read an external member link snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalMemberLinkSnapshotResponse?> SocialExternalMemberLinksRetrieveAsync(string linkId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialExternalMemberLinkSnapshotResponse>(ApiPaths.BackendPath($"/control/social/external_member_links/{SerializePathParameter(linkId, new PathParameterSpec("linkId", "simple", false))}"));
        }

        /// <summary>
        /// Submit a friend request event.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse?> SocialFriendRequestsCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.SubmitFriendRequestRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse>(ApiPaths.BackendPath("/control/social/friend_requests"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read a friend request snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestSnapshotResponse?> SocialFriendRequestsRetrieveAsync(string requestId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestSnapshotResponse>(ApiPaths.BackendPath($"/control/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}"));
        }

        /// <summary>
        /// Accept a friend request.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse?> SocialFriendRequestsAcceptAsync(string requestId, Sdkwork.Im.BackendApi.Generated.Models.AcceptFriendRequestRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse>(ApiPaths.BackendPath($"/control/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}/accept"), body, null, null, "application/json");
        }

        /// <summary>
        /// Decline a friend request.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse?> SocialFriendRequestsDeclineAsync(string requestId, Sdkwork.Im.BackendApi.Generated.Models.DeclineFriendRequestRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse>(ApiPaths.BackendPath($"/control/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}/decline"), body, null, null, "application/json");
        }

        /// <summary>
        /// Cancel a friend request.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse?> SocialFriendRequestsCancelAsync(string requestId, Sdkwork.Im.BackendApi.Generated.Models.CancelFriendRequestRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendRequestCommitResponse>(ApiPaths.BackendPath($"/control/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}/cancel"), body, null, null, "application/json");
        }

        /// <summary>
        /// Activate a friendship event.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendshipCommitResponse?> SocialFriendshipsCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.ActivateFriendshipRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendshipCommitResponse>(ApiPaths.BackendPath("/control/social/friendships"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read a friendship snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendshipSnapshotResponse?> SocialFriendshipsRetrieveAsync(string friendshipId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendshipSnapshotResponse>(ApiPaths.BackendPath($"/control/social/friendships/{SerializePathParameter(friendshipId, new PathParameterSpec("friendshipId", "simple", false))}"));
        }

        /// <summary>
        /// Remove a friendship.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendshipCommitResponse?> SocialFriendshipsRemoveAsync(string friendshipId, Sdkwork.Im.BackendApi.Generated.Models.RemoveFriendshipRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialFriendshipCommitResponse>(ApiPaths.BackendPath($"/control/social/friendships/{SerializePathParameter(friendshipId, new PathParameterSpec("friendshipId", "simple", false))}/remove"), body, null, null, "application/json");
        }

        /// <summary>
        /// Claim selected pending shared-channel sync entries.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingClaimResponse?> SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingTargetedClaimRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingClaimResponse>(ApiPaths.BackendPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read the dead-letter shared-channel sync queue.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterInventoryResponse?> SocialRuntimeDeadLetterSharedChannelSyncListAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterInventoryResponse>(ApiPaths.BackendPath("/control/social/runtime/dead_letter_shared_channel_sync"));
        }

        /// <summary>
        /// Read the delivered shared-channel sync ledger.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeliveredInventoryResponse?> SocialRuntimeDeliveredSharedChannelSyncListAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeliveredInventoryResponse>(ApiPaths.BackendPath("/control/social/runtime/delivered_shared_channel_sync"));
        }

        /// <summary>
        /// Read merged shared-channel sync delivery state.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeliveryStateInventoryResponse?> SocialRuntimeDeliveryStateSharedChannelSyncListAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeliveryStateInventoryResponse>(ApiPaths.BackendPath("/control/social/runtime/delivery_state_shared_channel_sync"));
        }

        /// <summary>
        /// Read the pending shared-channel sync queue.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingInventoryResponse?> SocialRuntimePendingSharedChannelSyncListAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingInventoryResponse>(ApiPaths.BackendPath("/control/social/runtime/pending_shared_channel_sync"));
        }

        /// <summary>
        /// Reclaim stale shared-channel sync pending ownership.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingStaleReclaimResponse?> SocialRuntimeReclaimStalePendingSharedChannelSyncCreateAsync()
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingStaleReclaimResponse>(ApiPaths.BackendPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), null);
        }

        /// <summary>
        /// Release selected pending shared-channel sync entries.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingReleaseResponse?> SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingTargetedReleaseRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingReleaseResponse>(ApiPaths.BackendPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        }

        /// <summary>
        /// Repair the persisted social runtime derived snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialRuntimeRepairResponse?> SocialRuntimeRepairDerivedSnapshotCreateAsync()
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialRuntimeRepairResponse>(ApiPaths.BackendPath("/control/social/runtime/repair_derived_snapshot"), null);
        }

        /// <summary>
        /// Repair shared-channel sync backlog state.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncRepairResponse?> SocialRuntimeRepairSharedChannelSyncCreateAsync()
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncRepairResponse>(ApiPaths.BackendPath("/control/social/runtime/repair_shared_channel_sync"), null);
        }

        /// <summary>
        /// Republish selected pending shared-channel sync entries.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncTargetedRepublishResponse?> SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncTargetedRepublishRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncTargetedRepublishResponse>(ApiPaths.BackendPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        }

        /// <summary>
        /// Requeue all dead-letter shared-channel sync entries.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterRequeueResponse?> SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateAsync()
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterRequeueResponse>(ApiPaths.BackendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), null);
        }

        /// <summary>
        /// Requeue selected dead-letter shared-channel sync entries.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterTargetedRequeueResponse?> SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterTargetedRequeueRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>(ApiPaths.BackendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body, null, null, "application/json");
        }

        /// <summary>
        /// Take over selected pending shared-channel sync entries.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingTakeoverResponse?> SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingTargetedTakeoverRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelSyncPendingTakeoverResponse>(ApiPaths.BackendPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body, null, null, "application/json");
        }

        /// <summary>
        /// Apply a shared-channel policy.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelPolicyCommitResponse?> SocialSharedChannelPoliciesCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.ApplySharedChannelPolicyRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelPolicyCommitResponse>(ApiPaths.BackendPath("/control/social/shared_channel_policies"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read a shared-channel policy snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelPolicySnapshotResponse?> SocialSharedChannelPoliciesRetrieveAsync(string policyId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialSharedChannelPolicySnapshotResponse>(ApiPaths.BackendPath($"/control/social/shared_channel_policies/{SerializePathParameter(policyId, new PathParameterSpec("policyId", "simple", false))}"));
        }

        /// <summary>
        /// Block a user in the social graph.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialUserBlockCommitResponse?> SocialUserBlocksCreateAsync(Sdkwork.Im.BackendApi.Generated.Models.BlockUserRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialUserBlockCommitResponse>(ApiPaths.BackendPath("/control/social/user_blocks"), body, null, null, "application/json");
        }

        /// <summary>
        /// Read a user block snapshot.
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SocialUserBlockSnapshotResponse?> SocialUserBlocksRetrieveAsync(string blockId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SocialUserBlockSnapshotResponse>(ApiPaths.BackendPath($"/control/social/user_blocks/{SerializePathParameter(blockId, new PathParameterSpec("blockId", "simple", false))}"));
        }

        private sealed record PathParameterSpec(string Name, string Style, bool Explode);

        private static string SerializePathParameter(object? value, PathParameterSpec spec)
        {
            if (value is null)
            {
                return string.Empty;
            }
            var style = string.IsNullOrWhiteSpace(spec.Style) ? "simple" : spec.Style;
            if (value is System.Collections.IDictionary dictionary)
            {
                return SerializePathObject(spec.Name, dictionary, style, spec.Explode);
            }
            if (value is System.Collections.IEnumerable enumerable && value is not string)
            {
                return SerializePathArray(spec.Name, enumerable, style, spec.Explode);
            }
            return PathPrimitivePrefix(spec.Name, style) + Uri.EscapeDataString(value.ToString() ?? string.Empty);
        }

        private static string SerializePathArray(string name, System.Collections.IEnumerable values, string style, bool explode)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(Uri.EscapeDataString(item.ToString() ?? string.Empty));
                }
            }
            if (serialized.Count == 0)
            {
                return PathPrefix(name, style);
            }
            if (style == "matrix")
            {
                if (explode)
                {
                    var parts = new List<string>();
                    foreach (var item in serialized)
                    {
                        parts.Add(";" + name + "=" + item);
                    }
                    return string.Join(string.Empty, parts);
                }
                return ";" + name + "=" + string.Join(",", serialized);
            }
            var separator = explode ? "." : ",";
            return PathPrefix(name, style) + string.Join(separator, serialized);
        }

        private static string SerializePathObject(string name, System.Collections.IDictionary values, string style, bool explode)
        {
            var entries = new List<string>();
            var exploded = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                var escapedKey = Uri.EscapeDataString(item.Key.ToString() ?? string.Empty);
                var escapedValue = Uri.EscapeDataString(item.Value.ToString() ?? string.Empty);
                if (explode)
                {
                    exploded.Add(style == "matrix" ? ";" + escapedKey + "=" + escapedValue : escapedKey + "=" + escapedValue);
                }
                else
                {
                    entries.Add(escapedKey);
                    entries.Add(escapedValue);
                }
            }
            if (style == "matrix")
            {
                return explode ? string.Join(string.Empty, exploded) : ";" + name + "=" + string.Join(",", entries);
            }
            if (explode)
            {
                var separator = style == "label" ? "." : ",";
                return PathPrefix(name, style) + string.Join(separator, exploded);
            }
            return PathPrefix(name, style) + string.Join(",", entries);
        }

        private static string PathPrefix(string name, string style)
        {
            return style switch
            {
                "label" => ".",
                "matrix" => ";" + name,
                _ => string.Empty,
            };
        }

        private static string PathPrimitivePrefix(string name, string style)
        {
            return style == "matrix" ? ";" + name + "=" : PathPrefix(name, style);
        }

        private sealed record QueryParameterSpec(
            string Name,
            object? Value,
            string Style,
            bool Explode,
            bool AllowReserved,
            string? ContentType);

        private static string BuildQueryString(IEnumerable<QueryParameterSpec> parameters)
        {
            var pairs = new List<string>();
            foreach (var parameter in parameters)
            {
                AppendSerializedParameter(pairs, parameter);
            }
            return string.Join("&", pairs);
        }

        private static void AppendSerializedParameter(List<string> pairs, QueryParameterSpec parameter)
        {
            if (parameter.Value is null)
            {
                return;
            }

            if (!string.IsNullOrWhiteSpace(parameter.ContentType))
            {
                var json = System.Text.Json.JsonSerializer.Serialize(parameter.Value);
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(json, parameter.AllowReserved));
                return;
            }

            var style = string.IsNullOrWhiteSpace(parameter.Style) ? "form" : parameter.Style;
            if (style == "deepObject" && parameter.Value is System.Collections.IDictionary deepObject)
            {
                AppendDeepObjectParameter(pairs, parameter.Name, deepObject, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IEnumerable enumerable && parameter.Value is not string && parameter.Value is not System.Collections.IDictionary)
            {
                AppendArrayParameter(pairs, parameter.Name, enumerable, style, parameter.Explode, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IDictionary dictionary)
            {
                AppendObjectParameter(pairs, parameter.Name, dictionary, style, parameter.Explode, parameter.AllowReserved);
            }
            else
            {
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(parameter.Value.ToString() ?? string.Empty, parameter.AllowReserved));
            }
        }

        private static void AppendArrayParameter(List<string> pairs, string name, System.Collections.IEnumerable values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(item.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count == 0)
            {
                return;
            }
            if (style == "form" && explode)
            {
                foreach (var item in serialized)
                {
                    pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(item, allowReserved));
                }
                return;
            }
            pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
        }

        private static void AppendObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                if (style == "form" && explode)
                {
                    pairs.Add(Uri.EscapeDataString(item.Key.ToString() ?? string.Empty) + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
                else
                {
                    serialized.Add(item.Key.ToString() ?? string.Empty);
                    serialized.Add(item.Value.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count > 0)
            {
                pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
            }
        }

        private static void AppendDeepObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, bool allowReserved)
        {
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is not null)
                {
                    pairs.Add(Uri.EscapeDataString(name + "[" + item.Key + "]") + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
            }
        }

        private static string EncodeQueryValue(string value, bool allowReserved)
        {
            var encoded = Uri.EscapeDataString(value);
            if (!allowReserved)
            {
                return encoded;
            }
            return encoded
                .Replace("%3A", ":").Replace("%2F", "/").Replace("%3F", "?").Replace("%23", "#")
                .Replace("%5B", "[").Replace("%5D", "]").Replace("%40", "@").Replace("%21", "!")
                .Replace("%24", "$").Replace("%26", "&").Replace("%27", "'").Replace("%28", "(")
                .Replace("%29", ")").Replace("%2A", "*").Replace("%2B", "+").Replace("%2C", ",")
                .Replace("%3B", ";").Replace("%3D", "=");
        }

    }
}
