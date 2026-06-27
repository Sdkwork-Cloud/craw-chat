import Foundation

public class ControlApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Activate a realtime node and clear drain state.
    public func nodesActivate(nodeId: String) async throws -> RouteNodeLifecycle? {
        return try await client.post(ApiPaths.backendPath("/control/nodes/\(serializePathParameter(nodeId, PathParameterSpec(name: "nodeId", style: "simple", explode: false)))/activate"), body: nil, responseType: RouteNodeLifecycle.self)
    }

    /// Mark a realtime node as draining.
    public func nodesDrain(nodeId: String) async throws -> RouteNodeLifecycle? {
        return try await client.post(ApiPaths.backendPath("/control/nodes/\(serializePathParameter(nodeId, PathParameterSpec(name: "nodeId", style: "simple", explode: false)))/drain"), body: nil, responseType: RouteNodeLifecycle.self)
    }

    /// Migrate owned routes from the source node to the target node.
    public func nodesRoutesMigrate(nodeId: String, body: MigrateRoutesRequest) async throws -> RouteMigrationResult? {
        return try await client.post(ApiPaths.backendPath("/control/nodes/\(serializePathParameter(nodeId, PathParameterSpec(name: "nodeId", style: "simple", explode: false)))/routes/migrate"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RouteMigrationResult.self)
    }

    /// Read the control-plane protocol governance snapshot.
    public func protocolGovernanceRetrieve() async throws -> ProtocolGovernanceResponse? {
        return try await client.get(ApiPaths.backendPath("/control/protocol_governance"), responseType: ProtocolGovernanceResponse.self)
    }

    /// Read the control-plane protocol registry snapshot.
    public func protocolRegistryRetrieve() async throws -> ProtocolRegistryResponse? {
        return try await client.get(ApiPaths.backendPath("/control/protocol_registry"), responseType: ProtocolRegistryResponse.self)
    }

    /// Read provider policy history.
    public func providerPoliciesList() async throws -> ProviderPolicyHistoryResponse? {
        return try await client.get(ApiPaths.backendPath("/control/provider_policies"), responseType: ProviderPolicyHistoryResponse.self)
    }

    /// Read provider policy diff between two versions.
    public func providerPoliciesDiffList(fromVersion: Int, toVersion: Int) async throws -> ProviderPolicyDiffResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "fromVersion", value: fromVersion, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "toVersion", value: toVersion, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_policies/diff"), query), responseType: ProviderPolicyDiffResponse.self)
    }

    /// Preview the effective provider policy result before commit.
    public func providerPoliciesPreview(body: UpsertProviderBindingPolicyRequest) async throws -> ProviderBindingCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/provider_policies/preview"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ProviderBindingCommitResponse.self)
    }

    /// Rollback provider policy history to a target version.
    public func providerPoliciesRollback(body: ProviderPolicyRollbackRequest) async throws -> ProviderBindingCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/provider_policies/rollback"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ProviderBindingCommitResponse.self)
    }

    /// Read the provider registry snapshot.
    public func providerRegistryRetrieve() async throws -> ProviderRegistrySnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/provider_registry"), responseType: ProviderRegistrySnapshotResponse.self)
    }

    /// Read effective provider bindings.
    public func providerBindingsList(tenantId: String? = nil) async throws -> ProviderBindingsResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "tenantId", value: tenantId, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_bindings"), query), responseType: ProviderBindingsResponse.self)
    }

    /// Upsert a provider binding policy.
    public func providerBindingsCreate(body: UpsertProviderBindingPolicyRequest) async throws -> ProviderBindingCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/provider_bindings"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ProviderBindingCommitResponse.self)
    }

    /// Bind a direct chat to a conversation.
    public func socialDirectChatsBindingsCreate(body: BindDirectChatRequest) async throws -> SocialDirectChatCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/direct_chats/bindings"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialDirectChatCommitResponse.self)
    }

    /// Read a direct chat snapshot.
    public func socialDirectChatsRetrieve(directChatId: String) async throws -> SocialDirectChatSnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/direct_chats/\(serializePathParameter(directChatId, PathParameterSpec(name: "directChatId", style: "simple", explode: false)))"), responseType: SocialDirectChatSnapshotResponse.self)
    }

    /// Establish an external collaboration connection.
    public func socialExternalConnectionsCreate(body: EstablishExternalConnectionRequest) async throws -> SocialExternalConnectionCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/external_connections"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialExternalConnectionCommitResponse.self)
    }

    /// Read an external connection snapshot.
    public func socialExternalConnectionsRetrieve(connectionId: String) async throws -> SocialExternalConnectionSnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/external_connections/\(serializePathParameter(connectionId, PathParameterSpec(name: "connectionId", style: "simple", explode: false)))"), responseType: SocialExternalConnectionSnapshotResponse.self)
    }

    /// Bind an external member link.
    public func socialExternalMemberLinksCreate(body: BindExternalMemberLinkRequest) async throws -> SocialExternalMemberLinkCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/external_member_links"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialExternalMemberLinkCommitResponse.self)
    }

    /// Read an external member link snapshot.
    public func socialExternalMemberLinksRetrieve(linkId: String) async throws -> SocialExternalMemberLinkSnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/external_member_links/\(serializePathParameter(linkId, PathParameterSpec(name: "linkId", style: "simple", explode: false)))"), responseType: SocialExternalMemberLinkSnapshotResponse.self)
    }

    /// Submit a friend request event.
    public func socialFriendRequestsCreate(body: SubmitFriendRequestRequest) async throws -> SocialFriendRequestCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestCommitResponse.self)
    }

    /// Read a friend request snapshot.
    public func socialFriendRequestsRetrieve(requestId: String) async throws -> SocialFriendRequestSnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))"), responseType: SocialFriendRequestSnapshotResponse.self)
    }

    /// Accept a friend request.
    public func socialFriendRequestsAccept(requestId: String, body: AcceptFriendRequestRequest) async throws -> SocialFriendRequestCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))/accept"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestCommitResponse.self)
    }

    /// Decline a friend request.
    public func socialFriendRequestsDecline(requestId: String, body: DeclineFriendRequestRequest) async throws -> SocialFriendRequestCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))/decline"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestCommitResponse.self)
    }

    /// Cancel a friend request.
    public func socialFriendRequestsCancel(requestId: String, body: CancelFriendRequestRequest) async throws -> SocialFriendRequestCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))/cancel"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestCommitResponse.self)
    }

    /// Activate a friendship event.
    public func socialFriendshipsCreate(body: ActivateFriendshipRequest) async throws -> SocialFriendshipCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friendships"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendshipCommitResponse.self)
    }

    /// Read a friendship snapshot.
    public func socialFriendshipsRetrieve(friendshipId: String) async throws -> SocialFriendshipSnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/friendships/\(serializePathParameter(friendshipId, PathParameterSpec(name: "friendshipId", style: "simple", explode: false)))"), responseType: SocialFriendshipSnapshotResponse.self)
    }

    /// Remove a friendship.
    public func socialFriendshipsRemove(friendshipId: String, body: RemoveFriendshipRequest) async throws -> SocialFriendshipCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friendships/\(serializePathParameter(friendshipId, PathParameterSpec(name: "friendshipId", style: "simple", explode: false)))/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendshipCommitResponse.self)
    }

    /// Claim selected pending shared-channel sync entries.
    public func socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedClaimRequest) async throws -> SocialSharedChannelSyncPendingClaimResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelSyncPendingClaimResponse.self)
    }

    /// Read the dead-letter shared-channel sync queue.
    public func socialRuntimeDeadLetterSharedChannelSyncList() async throws -> SocialSharedChannelSyncDeadLetterInventoryResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/runtime/dead_letter_shared_channel_sync"), responseType: SocialSharedChannelSyncDeadLetterInventoryResponse.self)
    }

    /// Read the delivered shared-channel sync ledger.
    public func socialRuntimeDeliveredSharedChannelSyncList() async throws -> SocialSharedChannelSyncDeliveredInventoryResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/runtime/delivered_shared_channel_sync"), responseType: SocialSharedChannelSyncDeliveredInventoryResponse.self)
    }

    /// Read merged shared-channel sync delivery state.
    public func socialRuntimeDeliveryStateSharedChannelSyncList() async throws -> SocialSharedChannelSyncDeliveryStateInventoryResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/runtime/delivery_state_shared_channel_sync"), responseType: SocialSharedChannelSyncDeliveryStateInventoryResponse.self)
    }

    /// Read the pending shared-channel sync queue.
    public func socialRuntimePendingSharedChannelSyncList() async throws -> SocialSharedChannelSyncPendingInventoryResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/runtime/pending_shared_channel_sync"), responseType: SocialSharedChannelSyncPendingInventoryResponse.self)
    }

    /// Reclaim stale shared-channel sync pending ownership.
    public func socialRuntimeReclaimStalePendingSharedChannelSyncCreate() async throws -> SocialSharedChannelSyncPendingStaleReclaimResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), body: nil, responseType: SocialSharedChannelSyncPendingStaleReclaimResponse.self)
    }

    /// Release selected pending shared-channel sync entries.
    public func socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedReleaseRequest) async throws -> SocialSharedChannelSyncPendingReleaseResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelSyncPendingReleaseResponse.self)
    }

    /// Repair the persisted social runtime derived snapshot.
    public func socialRuntimeRepairDerivedSnapshotCreate() async throws -> SocialRuntimeRepairResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/repair_derived_snapshot"), body: nil, responseType: SocialRuntimeRepairResponse.self)
    }

    /// Repair shared-channel sync backlog state.
    public func socialRuntimeRepairSharedChannelSyncCreate() async throws -> SocialSharedChannelSyncRepairResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/repair_shared_channel_sync"), body: nil, responseType: SocialSharedChannelSyncRepairResponse.self)
    }

    /// Republish selected pending shared-channel sync entries.
    public func socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncTargetedRepublishRequest) async throws -> SocialSharedChannelSyncTargetedRepublishResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelSyncTargetedRepublishResponse.self)
    }

    /// Requeue all dead-letter shared-channel sync entries.
    public func socialRuntimeRequeueDeadLetterSharedChannelSyncCreate() async throws -> SocialSharedChannelSyncDeadLetterRequeueResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), body: nil, responseType: SocialSharedChannelSyncDeadLetterRequeueResponse.self)
    }

    /// Requeue selected dead-letter shared-channel sync entries.
    public func socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest) async throws -> SocialSharedChannelSyncDeadLetterTargetedRequeueResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelSyncDeadLetterTargetedRequeueResponse.self)
    }

    /// Take over selected pending shared-channel sync entries.
    public func socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedTakeoverRequest) async throws -> SocialSharedChannelSyncPendingTakeoverResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelSyncPendingTakeoverResponse.self)
    }

    /// Apply a shared-channel policy.
    public func socialSharedChannelPoliciesCreate(body: ApplySharedChannelPolicyRequest) async throws -> SocialSharedChannelPolicyCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/shared_channel_policies"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelPolicyCommitResponse.self)
    }

    /// Read a shared-channel policy snapshot.
    public func socialSharedChannelPoliciesRetrieve(policyId: String) async throws -> SocialSharedChannelPolicySnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/shared_channel_policies/\(serializePathParameter(policyId, PathParameterSpec(name: "policyId", style: "simple", explode: false)))"), responseType: SocialSharedChannelPolicySnapshotResponse.self)
    }

    /// Block a user in the social graph.
    public func socialUserBlocksCreate(body: BlockUserRequest) async throws -> SocialUserBlockCommitResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/user_blocks"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialUserBlockCommitResponse.self)
    }

    /// Read a user block snapshot.
    public func socialUserBlocksRetrieve(blockId: String) async throws -> SocialUserBlockSnapshotResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/user_blocks/\(serializePathParameter(blockId, PathParameterSpec(name: "blockId", style: "simple", explode: false)))"), responseType: SocialUserBlockSnapshotResponse.self)
    }

    private struct PathParameterSpec {
        let name: String
        let style: String
        let explode: Bool
    }

    private func serializePathParameter(_ value: Any?, _ spec: PathParameterSpec) -> String {
        guard let value else { return "" }
        let style = spec.style.isEmpty ? "simple" : spec.style
        if let array = value as? [Any] {
            return serializePathArray(spec.name, array, style, spec.explode)
        }
        if let object = value as? [String: Any] {
            return serializePathObject(spec.name, object, style, spec.explode)
        }
        return pathPrimitivePrefix(spec.name, style) + pathEncode(String(describing: value))
    }

    private func serializePathArray(_ name: String, _ values: [Any], _ style: String, _ explode: Bool) -> String {
        let serialized = values.map { pathEncode(String(describing: $0)) }
        if serialized.isEmpty { return pathPrefix(name, style) }
        if style == "matrix" {
            if explode {
                return serialized.map { ";\(name)=\($0)" }.joined()
            }
            return ";\(name)=" + serialized.joined(separator: ",")
        }
        let separator = explode ? "." : ","
        return pathPrefix(name, style) + serialized.joined(separator: separator)
    }

    private func serializePathObject(_ name: String, _ values: [String: Any], _ style: String, _ explode: Bool) -> String {
        var entries: [String] = []
        var exploded: [String] = []
        for (key, value) in values {
            let escapedKey = pathEncode(key)
            let escapedValue = pathEncode(String(describing: value))
            if explode {
                if style == "matrix" {
                    exploded.append(";\(escapedKey)=\(escapedValue)")
                } else {
                    exploded.append("\(escapedKey)=\(escapedValue)")
                }
            } else {
                entries.append(escapedKey)
                entries.append(escapedValue)
            }
        }
        if style == "matrix" {
            if explode {
                return exploded.joined()
            }
            return ";\(name)=" + entries.joined(separator: ",")
        }
        if explode {
            let separator = style == "label" ? "." : ","
            return pathPrefix(name, style) + exploded.joined(separator: separator)
        }
        return pathPrefix(name, style) + entries.joined(separator: ",")
    }

    private func pathPrefix(_ name: String, _ style: String) -> String {
        if style == "label" { return "." }
        if style == "matrix" { return ";\(name)" }
        return ""
    }

    private func pathPrimitivePrefix(_ name: String, _ style: String) -> String {
        style == "matrix" ? ";\(name)=" : pathPrefix(name, style)
    }

    private func pathEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? value
    }

    private struct QueryParameterSpec {
        let name: String
        let value: Any?
        let style: String
        let explode: Bool
        let allowReserved: Bool
        let contentType: String?
    }

    private func buildQueryString(_ parameters: [QueryParameterSpec]) -> String {
        var pairs: [String] = []
        for parameter in parameters {
            appendSerializedParameter(&pairs, parameter)
        }
        return pairs.joined(separator: "&")
    }

    private func appendSerializedParameter(_ pairs: inout [String], _ parameter: QueryParameterSpec) {
        guard let value = parameter.value else { return }
        if let contentType = parameter.contentType, !contentType.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            let data = (try? JSONSerialization.data(withJSONObject: value, options: [])) ?? Data(String(describing: value).utf8)
            let json = String(data: data, encoding: .utf8) ?? String(describing: value)
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(json, allowReserved: parameter.allowReserved))")
            return
        }

        let style = parameter.style.isEmpty ? "form" : parameter.style
        if style == "deepObject", let object = value as? [String: Any] {
            appendDeepObjectParameter(&pairs, name: parameter.name, values: object, allowReserved: parameter.allowReserved)
        } else if let array = value as? [Any] {
            appendArrayParameter(&pairs, name: parameter.name, values: array, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else if let object = value as? [String: Any] {
            appendObjectParameter(&pairs, name: parameter.name, values: object, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else {
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(String(describing: value), allowReserved: parameter.allowReserved))")
        }
    }

    private func appendArrayParameter(
        _ pairs: inout [String],
        name: String,
        values: [Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        let serialized = values.map { String(describing: $0) }
        guard !serialized.isEmpty else { return }
        if style == "form" && explode {
            for item in serialized {
                pairs.append("\(urlEncode(name))=\(encodeQueryValue(item, allowReserved: allowReserved))")
            }
            return
        }
        pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
    }

    private func appendObjectParameter(
        _ pairs: inout [String],
        name: String,
        values: [String: Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        var serialized: [String] = []
        for (key, value) in values {
            if style == "form" && explode {
                pairs.append("\(urlEncode(key))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
            } else {
                serialized.append(key)
                serialized.append(String(describing: value))
            }
        }
        if !serialized.isEmpty {
            pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
        }
    }

    private func appendDeepObjectParameter(_ pairs: inout [String], name: String, values: [String: Any], allowReserved: Bool) {
        for (key, value) in values {
            pairs.append("\(urlEncode("\(name)[\(key)]"))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
        }
    }

    private func encodeQueryValue(_ value: String, allowReserved: Bool) -> String {
        var encoded = urlEncode(value)
        if !allowReserved { return encoded }
        [
            "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
            "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
            "%24": "$", "%26": "&", "%27": "'", "%28": "(",
            "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
            "%3B": ";", "%3D": "=",
        ].forEach { encoded = encoded.replacingOccurrences(of: $0.key, with: $0.value) }
        return encoded
    }

    private func urlEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? value
    }

}
