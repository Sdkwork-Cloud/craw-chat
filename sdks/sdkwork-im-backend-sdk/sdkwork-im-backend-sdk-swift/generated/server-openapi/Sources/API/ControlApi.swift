import Foundation

public class ControlApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Activate a realtime node and clear drain state.
    public func nodesActivate(nodeId: String) async throws -> NodesActivateResponse? {
        return try await client.post(ApiPaths.backendPath("/control/nodes/\(serializePathParameter(nodeId, PathParameterSpec(name: "nodeId", style: "simple", explode: false)))/activate"), body: nil, responseType: NodesActivateResponse.self)
    }

    /// Mark a realtime node as draining.
    public func nodesDrain(nodeId: String) async throws -> NodesDrainResponse? {
        return try await client.post(ApiPaths.backendPath("/control/nodes/\(serializePathParameter(nodeId, PathParameterSpec(name: "nodeId", style: "simple", explode: false)))/drain"), body: nil, responseType: NodesDrainResponse.self)
    }

    /// Migrate owned routes from the source node to the target node.
    public func nodesRoutesMigrate(nodeId: String, body: MigrateRoutesRequest) async throws -> NodesRoutesMigrateResponse? {
        return try await client.post(ApiPaths.backendPath("/control/nodes/\(serializePathParameter(nodeId, PathParameterSpec(name: "nodeId", style: "simple", explode: false)))/routes/migrate"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: NodesRoutesMigrateResponse.self)
    }

    /// Read the control-plane protocol governance snapshot.
    public func protocolGovernanceRetrieve() async throws -> ProtocolGovernanceRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/protocol_governance"), responseType: ProtocolGovernanceRetrieveResponse.self)
    }

    /// Read the control-plane protocol registry snapshot.
    public func protocolRegistryRetrieve() async throws -> ProtocolRegistryRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/protocol_registry"), responseType: ProtocolRegistryRetrieveResponse.self)
    }

    /// Read provider policy history.
    public func providerPoliciesList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_policies"), query), responseType: SdkWorkListResponse.self)
    }

    /// Read provider policy diff between two versions.
    public func providerPoliciesDiffList(fromVersion: String, toVersion: String, pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "fromVersion", value: fromVersion, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "toVersion", value: toVersion, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_policies/diff"), query), responseType: SdkWorkListResponse.self)
    }

    /// Preview the effective provider policy result before commit.
    public func providerPoliciesPreview(body: UpsertProviderBindingPolicyRequest) async throws -> ProviderPoliciesPreviewResponse? {
        return try await client.post(ApiPaths.backendPath("/control/provider_policies/preview"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ProviderPoliciesPreviewResponse.self)
    }

    /// Rollback provider policy history to a target version.
    public func providerPoliciesRollback(body: ProviderPolicyRollbackRequest) async throws -> ProviderPoliciesRollbackResponse? {
        return try await client.post(ApiPaths.backendPath("/control/provider_policies/rollback"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ProviderPoliciesRollbackResponse.self)
    }

    /// Read the provider registry snapshot.
    public func providerRegistryRetrieve() async throws -> ProviderRegistryRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/provider_registry"), responseType: ProviderRegistryRetrieveResponse.self)
    }

    /// Read effective provider bindings.
    public func providerBindingsList(tenantId: String? = nil, pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "tenantId", value: tenantId, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_bindings"), query), responseType: SdkWorkListResponse.self)
    }

    /// Upsert a provider binding policy.
    public func providerBindingsCreate(body: UpsertProviderBindingPolicyRequest) async throws -> ControlProviderBindingsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/provider_bindings"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ControlProviderBindingsCreateResponse201.self)
    }

    /// Bind a direct chat to a conversation.
    public func socialDirectChatsBindingsCreate(body: BindDirectChatRequest) async throws -> SocialDirectChatsBindingsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/direct_chats/bindings"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialDirectChatsBindingsCreateResponse201.self)
    }

    /// Read a direct chat snapshot.
    public func socialDirectChatsRetrieve(directChatId: String) async throws -> SocialDirectChatsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/direct_chats/\(serializePathParameter(directChatId, PathParameterSpec(name: "directChatId", style: "simple", explode: false)))"), responseType: SocialDirectChatsRetrieveResponse.self)
    }

    /// Establish an external collaboration connection.
    public func socialExternalConnectionsCreate(body: EstablishExternalConnectionRequest) async throws -> SocialExternalConnectionsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/external_connections"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialExternalConnectionsCreateResponse201.self)
    }

    /// Read an external connection snapshot.
    public func socialExternalConnectionsRetrieve(connectionId: String) async throws -> SocialExternalConnectionsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/external_connections/\(serializePathParameter(connectionId, PathParameterSpec(name: "connectionId", style: "simple", explode: false)))"), responseType: SocialExternalConnectionsRetrieveResponse.self)
    }

    /// Bind an external member link.
    public func socialExternalMemberLinksCreate(body: BindExternalMemberLinkRequest) async throws -> SocialExternalMemberLinksCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/external_member_links"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialExternalMemberLinksCreateResponse201.self)
    }

    /// Read an external member link snapshot.
    public func socialExternalMemberLinksRetrieve(linkId: String) async throws -> SocialExternalMemberLinksRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/external_member_links/\(serializePathParameter(linkId, PathParameterSpec(name: "linkId", style: "simple", explode: false)))"), responseType: SocialExternalMemberLinksRetrieveResponse.self)
    }

    /// Submit a friend request event.
    public func socialFriendRequestsCreate(body: SubmitFriendRequestRequest) async throws -> SocialFriendRequestsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestsCreateResponse201.self)
    }

    /// Read a friend request snapshot.
    public func socialFriendRequestsRetrieve(requestId: String) async throws -> SocialFriendRequestsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))"), responseType: SocialFriendRequestsRetrieveResponse.self)
    }

    /// Accept a friend request.
    public func socialFriendRequestsAccept(requestId: String, body: AcceptFriendRequestRequest) async throws -> SocialFriendRequestsAcceptResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))/accept"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestsAcceptResponse.self)
    }

    /// Decline a friend request.
    public func socialFriendRequestsDecline(requestId: String, body: DeclineFriendRequestRequest) async throws -> SocialFriendRequestsDeclineResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))/decline"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestsDeclineResponse.self)
    }

    /// Cancel a friend request.
    public func socialFriendRequestsCancel(requestId: String, body: CancelFriendRequestRequest) async throws -> SocialFriendRequestsCancelResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friend_requests/\(serializePathParameter(requestId, PathParameterSpec(name: "requestId", style: "simple", explode: false)))/cancel"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendRequestsCancelResponse.self)
    }

    /// Activate a friendship event.
    public func socialFriendshipsCreate(body: ActivateFriendshipRequest) async throws -> SocialFriendshipsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/friendships"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendshipsCreateResponse201.self)
    }

    /// Read a friendship snapshot.
    public func socialFriendshipsRetrieve(friendshipId: String) async throws -> SocialFriendshipsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/friendships/\(serializePathParameter(friendshipId, PathParameterSpec(name: "friendshipId", style: "simple", explode: false)))"), responseType: SocialFriendshipsRetrieveResponse.self)
    }

    /// Remove a friendship.
    public func socialFriendshipsRemove(friendshipId: String, body: RemoveFriendshipRequest) async throws -> SocialFriendshipsRemoveResponse? {
        return try await client.post(ApiPaths.backendPath("/control/social/friendships/\(serializePathParameter(friendshipId, PathParameterSpec(name: "friendshipId", style: "simple", explode: false)))/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialFriendshipsRemoveResponse.self)
    }

    /// Claim selected pending shared-channel sync entries.
    public func socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedClaimRequest) async throws -> SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201.self)
    }

    /// Read the dead-letter shared-channel sync queue.
    public func socialRuntimeDeadLetterSharedChannelSyncList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/dead_letter_shared_channel_sync"), query), responseType: SdkWorkListResponse.self)
    }

    /// Read the delivered shared-channel sync ledger.
    public func socialRuntimeDeliveredSharedChannelSyncList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/delivered_shared_channel_sync"), query), responseType: SdkWorkListResponse.self)
    }

    /// Read merged shared-channel sync delivery state.
    public func socialRuntimeDeliveryStateSharedChannelSyncList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/delivery_state_shared_channel_sync"), query), responseType: SdkWorkListResponse.self)
    }

    /// Read the pending shared-channel sync queue.
    public func socialRuntimePendingSharedChannelSyncList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/social/runtime/pending_shared_channel_sync"), query), responseType: SdkWorkListResponse.self)
    }

    /// Reclaim stale shared-channel sync pending ownership.
    public func socialRuntimeReclaimStalePendingSharedChannelSyncCreate() async throws -> SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), body: nil, responseType: SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201.self)
    }

    /// Release selected pending shared-channel sync entries.
    public func socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedReleaseRequest) async throws -> SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialRuntimeReleasePendingSharedChannelSyncTargetedCreateResponse201.self)
    }

    /// Repair the persisted social runtime derived snapshot.
    public func socialRuntimeRepairDerivedSnapshotCreate() async throws -> SocialRuntimeRepairDerivedSnapshotCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/repair_derived_snapshot"), body: nil, responseType: SocialRuntimeRepairDerivedSnapshotCreateResponse201.self)
    }

    /// Repair shared-channel sync backlog state.
    public func socialRuntimeRepairSharedChannelSyncCreate() async throws -> SocialRuntimeRepairSharedChannelSyncCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/repair_shared_channel_sync"), body: nil, responseType: SocialRuntimeRepairSharedChannelSyncCreateResponse201.self)
    }

    /// Republish selected pending shared-channel sync entries.
    public func socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncTargetedRepublishRequest) async throws -> SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201.self)
    }

    /// Requeue all dead-letter shared-channel sync entries.
    public func socialRuntimeRequeueDeadLetterSharedChannelSyncCreate() async throws -> SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), body: nil, responseType: SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201.self)
    }

    /// Requeue selected dead-letter shared-channel sync entries.
    public func socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest) async throws -> SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201.self)
    }

    /// Take over selected pending shared-channel sync entries.
    public func socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedTakeoverRequest) async throws -> SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201.self)
    }

    /// Apply a shared-channel policy.
    public func socialSharedChannelPoliciesCreate(body: ApplySharedChannelPolicyRequest) async throws -> SocialSharedChannelPoliciesCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/shared_channel_policies"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialSharedChannelPoliciesCreateResponse201.self)
    }

    /// Read a shared-channel policy snapshot.
    public func socialSharedChannelPoliciesRetrieve(policyId: String) async throws -> SocialSharedChannelPoliciesRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/shared_channel_policies/\(serializePathParameter(policyId, PathParameterSpec(name: "policyId", style: "simple", explode: false)))"), responseType: SocialSharedChannelPoliciesRetrieveResponse.self)
    }

    /// Block a user in the social graph.
    public func socialUserBlocksCreate(body: BlockUserRequest) async throws -> SocialUserBlocksCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/control/social/user_blocks"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SocialUserBlocksCreateResponse201.self)
    }

    /// Read a user block snapshot.
    public func socialUserBlocksRetrieve(blockId: String) async throws -> SocialUserBlocksRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/control/social/user_blocks/\(serializePathParameter(blockId, PathParameterSpec(name: "blockId", style: "simple", explode: false)))"), responseType: SocialUserBlocksRetrieveResponse.self)
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
