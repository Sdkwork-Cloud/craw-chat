package com.sdkwork.im.backend.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.im.backend.api.generated.http.HttpClient

class ControlApi(private val client: HttpClient) {

    /** Activate a realtime node and clear drain state. */
    suspend fun nodesActivate(nodeId: String): RouteNodeLifecycle? {
        val raw = client.post(ApiPaths.backendPath("/control/nodes/${serializePathParameter(nodeId, PathParameterSpec("nodeId", "simple", false))}/activate"), null)
        return client.convertValue(raw, object : TypeReference<RouteNodeLifecycle>() {})
    }

    /** Mark a realtime node as draining. */
    suspend fun nodesDrain(nodeId: String): RouteNodeLifecycle? {
        val raw = client.post(ApiPaths.backendPath("/control/nodes/${serializePathParameter(nodeId, PathParameterSpec("nodeId", "simple", false))}/drain"), null)
        return client.convertValue(raw, object : TypeReference<RouteNodeLifecycle>() {})
    }

    /** Migrate owned routes from the source node to the target node. */
    suspend fun nodesRoutesMigrate(nodeId: String, body: MigrateRoutesRequest): RouteMigrationResult? {
        val raw = client.post(ApiPaths.backendPath("/control/nodes/${serializePathParameter(nodeId, PathParameterSpec("nodeId", "simple", false))}/routes/migrate"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RouteMigrationResult>() {})
    }

    /** Read the control-plane protocol governance snapshot. */
    suspend fun protocolGovernanceRetrieve(): ProtocolGovernanceResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/protocol_governance"))
        return client.convertValue(raw, object : TypeReference<ProtocolGovernanceResponse>() {})
    }

    /** Read the control-plane protocol registry snapshot. */
    suspend fun protocolRegistryRetrieve(): ProtocolRegistryResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/protocol_registry"))
        return client.convertValue(raw, object : TypeReference<ProtocolRegistryResponse>() {})
    }

    /** Read provider policy history. */
    suspend fun providerPoliciesList(): ProviderPolicyHistoryResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/provider_policies"))
        return client.convertValue(raw, object : TypeReference<ProviderPolicyHistoryResponse>() {})
    }

    /** Read provider policy diff between two versions. */
    suspend fun providerPoliciesDiffList(fromVersion: Int, toVersion: Int): ProviderPolicyDiffResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("fromVersion", fromVersion, "form", true, false, null),
            QueryParameterSpec("toVersion", toVersion, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_policies/diff"), query))
        return client.convertValue(raw, object : TypeReference<ProviderPolicyDiffResponse>() {})
    }

    /** Preview the effective provider policy result before commit. */
    suspend fun providerPoliciesPreview(body: UpsertProviderBindingPolicyRequest): ProviderBindingCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/provider_policies/preview"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ProviderBindingCommitResponse>() {})
    }

    /** Rollback provider policy history to a target version. */
    suspend fun providerPoliciesRollback(body: ProviderPolicyRollbackRequest): ProviderBindingCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/provider_policies/rollback"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ProviderBindingCommitResponse>() {})
    }

    /** Read the provider registry snapshot. */
    suspend fun providerRegistryRetrieve(): ProviderRegistrySnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/provider_registry"))
        return client.convertValue(raw, object : TypeReference<ProviderRegistrySnapshotResponse>() {})
    }

    /** Read effective provider bindings. */
    suspend fun providerBindingsList(tenantId: String? = null): ProviderBindingsResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("tenantId", tenantId, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/control/provider_bindings"), query))
        return client.convertValue(raw, object : TypeReference<ProviderBindingsResponse>() {})
    }

    /** Upsert a provider binding policy. */
    suspend fun providerBindingsCreate(body: UpsertProviderBindingPolicyRequest): ProviderBindingCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/provider_bindings"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ProviderBindingCommitResponse>() {})
    }

    /** Bind a direct chat to a conversation. */
    suspend fun socialDirectChatsBindingsCreate(body: BindDirectChatRequest): SocialDirectChatCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/direct_chats/bindings"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialDirectChatCommitResponse>() {})
    }

    /** Read a direct chat snapshot. */
    suspend fun socialDirectChatsRetrieve(directChatId: String): SocialDirectChatSnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/direct_chats/${serializePathParameter(directChatId, PathParameterSpec("directChatId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialDirectChatSnapshotResponse>() {})
    }

    /** Establish an external collaboration connection. */
    suspend fun socialExternalConnectionsCreate(body: EstablishExternalConnectionRequest): SocialExternalConnectionCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/external_connections"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialExternalConnectionCommitResponse>() {})
    }

    /** Read an external connection snapshot. */
    suspend fun socialExternalConnectionsRetrieve(connectionId: String): SocialExternalConnectionSnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/external_connections/${serializePathParameter(connectionId, PathParameterSpec("connectionId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialExternalConnectionSnapshotResponse>() {})
    }

    /** Bind an external member link. */
    suspend fun socialExternalMemberLinksCreate(body: BindExternalMemberLinkRequest): SocialExternalMemberLinkCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/external_member_links"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialExternalMemberLinkCommitResponse>() {})
    }

    /** Read an external member link snapshot. */
    suspend fun socialExternalMemberLinksRetrieve(linkId: String): SocialExternalMemberLinkSnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/external_member_links/${serializePathParameter(linkId, PathParameterSpec("linkId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialExternalMemberLinkSnapshotResponse>() {})
    }

    /** Submit a friend request event. */
    suspend fun socialFriendRequestsCreate(body: SubmitFriendRequestRequest): SocialFriendRequestCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/friend_requests"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestCommitResponse>() {})
    }

    /** Read a friend request snapshot. */
    suspend fun socialFriendRequestsRetrieve(requestId: String): SocialFriendRequestSnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/friend_requests/${serializePathParameter(requestId, PathParameterSpec("requestId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestSnapshotResponse>() {})
    }

    /** Accept a friend request. */
    suspend fun socialFriendRequestsAccept(requestId: String, body: AcceptFriendRequestRequest): SocialFriendRequestCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/friend_requests/${serializePathParameter(requestId, PathParameterSpec("requestId", "simple", false))}/accept"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestCommitResponse>() {})
    }

    /** Decline a friend request. */
    suspend fun socialFriendRequestsDecline(requestId: String, body: DeclineFriendRequestRequest): SocialFriendRequestCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/friend_requests/${serializePathParameter(requestId, PathParameterSpec("requestId", "simple", false))}/decline"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestCommitResponse>() {})
    }

    /** Cancel a friend request. */
    suspend fun socialFriendRequestsCancel(requestId: String, body: CancelFriendRequestRequest): SocialFriendRequestCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/friend_requests/${serializePathParameter(requestId, PathParameterSpec("requestId", "simple", false))}/cancel"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestCommitResponse>() {})
    }

    /** Activate a friendship event. */
    suspend fun socialFriendshipsCreate(body: ActivateFriendshipRequest): SocialFriendshipCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/friendships"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendshipCommitResponse>() {})
    }

    /** Read a friendship snapshot. */
    suspend fun socialFriendshipsRetrieve(friendshipId: String): SocialFriendshipSnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/friendships/${serializePathParameter(friendshipId, PathParameterSpec("friendshipId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialFriendshipSnapshotResponse>() {})
    }

    /** Remove a friendship. */
    suspend fun socialFriendshipsRemove(friendshipId: String, body: RemoveFriendshipRequest): SocialFriendshipCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/friendships/${serializePathParameter(friendshipId, PathParameterSpec("friendshipId", "simple", false))}/remove"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendshipCommitResponse>() {})
    }

    /** Claim selected pending shared-channel sync entries. */
    suspend fun socialRuntimeClaimPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedClaimRequest): SocialSharedChannelSyncPendingClaimResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/claim_pending_shared_channel_sync_targeted"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncPendingClaimResponse>() {})
    }

    /** Read the dead-letter shared-channel sync queue. */
    suspend fun socialRuntimeDeadLetterSharedChannelSyncList(): SocialSharedChannelSyncDeadLetterInventoryResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/runtime/dead_letter_shared_channel_sync"))
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncDeadLetterInventoryResponse>() {})
    }

    /** Read the delivered shared-channel sync ledger. */
    suspend fun socialRuntimeDeliveredSharedChannelSyncList(): SocialSharedChannelSyncDeliveredInventoryResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/runtime/delivered_shared_channel_sync"))
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncDeliveredInventoryResponse>() {})
    }

    /** Read merged shared-channel sync delivery state. */
    suspend fun socialRuntimeDeliveryStateSharedChannelSyncList(): SocialSharedChannelSyncDeliveryStateInventoryResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/runtime/delivery_state_shared_channel_sync"))
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncDeliveryStateInventoryResponse>() {})
    }

    /** Read the pending shared-channel sync queue. */
    suspend fun socialRuntimePendingSharedChannelSyncList(): SocialSharedChannelSyncPendingInventoryResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/runtime/pending_shared_channel_sync"))
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncPendingInventoryResponse>() {})
    }

    /** Reclaim stale shared-channel sync pending ownership. */
    suspend fun socialRuntimeReclaimStalePendingSharedChannelSyncCreate(): SocialSharedChannelSyncPendingStaleReclaimResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/reclaim_stale_pending_shared_channel_sync"), null)
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncPendingStaleReclaimResponse>() {})
    }

    /** Release selected pending shared-channel sync entries. */
    suspend fun socialRuntimeReleasePendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedReleaseRequest): SocialSharedChannelSyncPendingReleaseResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/release_pending_shared_channel_sync_targeted"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncPendingReleaseResponse>() {})
    }

    /** Repair the persisted social runtime derived snapshot. */
    suspend fun socialRuntimeRepairDerivedSnapshotCreate(): SocialRuntimeRepairResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/repair_derived_snapshot"), null)
        return client.convertValue(raw, object : TypeReference<SocialRuntimeRepairResponse>() {})
    }

    /** Repair shared-channel sync backlog state. */
    suspend fun socialRuntimeRepairSharedChannelSyncCreate(): SocialSharedChannelSyncRepairResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/repair_shared_channel_sync"), null)
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncRepairResponse>() {})
    }

    /** Republish selected pending shared-channel sync entries. */
    suspend fun socialRuntimeRepublishPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncTargetedRepublishRequest): SocialSharedChannelSyncTargetedRepublishResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/republish_pending_shared_channel_sync_targeted"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncTargetedRepublishResponse>() {})
    }

    /** Requeue all dead-letter shared-channel sync entries. */
    suspend fun socialRuntimeRequeueDeadLetterSharedChannelSyncCreate(): SocialSharedChannelSyncDeadLetterRequeueResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync"), null)
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncDeadLetterRequeueResponse>() {})
    }

    /** Requeue selected dead-letter shared-channel sync entries. */
    suspend fun socialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest): SocialSharedChannelSyncDeadLetterTargetedRequeueResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>() {})
    }

    /** Take over selected pending shared-channel sync entries. */
    suspend fun socialRuntimeTakeoverPendingSharedChannelSyncTargetedCreate(body: SocialSharedChannelSyncPendingTargetedTakeoverRequest): SocialSharedChannelSyncPendingTakeoverResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/runtime/takeover_pending_shared_channel_sync_targeted"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelSyncPendingTakeoverResponse>() {})
    }

    /** Apply a shared-channel policy. */
    suspend fun socialSharedChannelPoliciesCreate(body: ApplySharedChannelPolicyRequest): SocialSharedChannelPolicyCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/shared_channel_policies"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelPolicyCommitResponse>() {})
    }

    /** Read a shared-channel policy snapshot. */
    suspend fun socialSharedChannelPoliciesRetrieve(policyId: String): SocialSharedChannelPolicySnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/shared_channel_policies/${serializePathParameter(policyId, PathParameterSpec("policyId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialSharedChannelPolicySnapshotResponse>() {})
    }

    /** Block a user in the social graph. */
    suspend fun socialUserBlocksCreate(body: BlockUserRequest): SocialUserBlockCommitResponse? {
        val raw = client.post(ApiPaths.backendPath("/control/social/user_blocks"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialUserBlockCommitResponse>() {})
    }

    /** Read a user block snapshot. */
    suspend fun socialUserBlocksRetrieve(blockId: String): SocialUserBlockSnapshotResponse? {
        val raw = client.get(ApiPaths.backendPath("/control/social/user_blocks/${serializePathParameter(blockId, PathParameterSpec("blockId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SocialUserBlockSnapshotResponse>() {})
    }

    private data class PathParameterSpec(val name: String, val style: String, val explode: Boolean)

    private fun serializePathParameter(value: Any?, spec: PathParameterSpec): String {
        if (value == null) return ""
        val style = spec.style.ifBlank { "simple" }
        return when (value) {
            is Iterable<*> -> serializePathArray(spec.name, value, style, spec.explode)
            is Map<*, *> -> serializePathObject(spec.name, value, style, spec.explode)
            else -> pathPrimitivePrefix(spec.name, style) + pathEncode(value.toString())
        }
    }

    private fun serializePathArray(name: String, values: Iterable<*>, style: String, explode: Boolean): String {
        val serialized = values.mapNotNull { it?.toString()?.let(::pathEncode) }
        if (serialized.isEmpty()) return pathPrefix(name, style)
        if (style == "matrix") {
            if (explode) {
                return serialized.joinToString("") { ";$name=$it" }
            }
            return ";$name=" + serialized.joinToString(",")
        }
        val separator = if (explode) "." else ","
        return pathPrefix(name, style) + serialized.joinToString(separator)
    }

    private fun serializePathObject(name: String, values: Map<*, *>, style: String, explode: Boolean): String {
        val entries = mutableListOf<String>()
        val exploded = mutableListOf<String>()
        values.forEach { (key, value) ->
            if (value == null) return@forEach
            val escapedKey = pathEncode(key.toString())
            val escapedValue = pathEncode(value.toString())
            if (explode) {
                if (style == "matrix") {
                    exploded += ";$escapedKey=$escapedValue"
                } else {
                    exploded += "$escapedKey=$escapedValue"
                }
            } else {
                entries += escapedKey
                entries += escapedValue
            }
        }
        if (style == "matrix") {
            if (explode) return exploded.joinToString("")
            return ";$name=" + entries.joinToString(",")
        }
        if (explode) {
            val separator = if (style == "label") "." else ","
            return pathPrefix(name, style) + exploded.joinToString(separator)
        }
        return pathPrefix(name, style) + entries.joinToString(",")
    }

    private fun pathPrefix(name: String, style: String): String {
        return when (style) {
            "label" -> "."
            "matrix" -> ";$name"
            else -> ""
        }
    }

    private fun pathPrimitivePrefix(name: String, style: String): String {
        return if (style == "matrix") ";$name=" else pathPrefix(name, style)
    }

    private fun pathEncode(value: String): String {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8).replace("+", "%20")
    }

    private data class QueryParameterSpec(
        val name: String,
        val value: Any?,
        val style: String,
        val explode: Boolean,
        val allowReserved: Boolean,
        val contentType: String?,
    )

    private val queryObjectMapper = ObjectMapper().registerKotlinModule()

    private fun buildQueryString(parameters: List<QueryParameterSpec>): String {
        val pairs = mutableListOf<String>()
        parameters.forEach { appendSerializedParameter(pairs, it) }
        return pairs.joinToString("&")
    }

    private fun appendSerializedParameter(pairs: MutableList<String>, parameter: QueryParameterSpec) {
        val value = parameter.value ?: return
        if (!parameter.contentType.isNullOrBlank()) {
            val json = queryObjectMapper.writeValueAsString(value)
            pairs += urlEncode(parameter.name) + "=" + encodeQueryValue(json, parameter.allowReserved)
            return
        }

        val style = parameter.style.ifBlank { "form" }
        when (value) {
            is Iterable<*> -> appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved)
            is Map<*, *> -> if (style == "deepObject") {
                appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved)
            } else {
                appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved)
            }
            else -> pairs += urlEncode(parameter.name) + "=" + encodeQueryValue(value.toString(), parameter.allowReserved)
        }
    }

    private fun appendArrayParameter(
        pairs: MutableList<String>,
        name: String,
        values: Iterable<*>,
        style: String,
        explode: Boolean,
        allowReserved: Boolean,
    ) {
        val serialized = values.mapNotNull { it?.toString() }
        if (serialized.isEmpty()) return
        if (style == "form" && explode) {
            serialized.forEach { pairs += urlEncode(name) + "=" + encodeQueryValue(it, allowReserved) }
            return
        }
        pairs += urlEncode(name) + "=" + encodeQueryValue(serialized.joinToString(","), allowReserved)
    }

    private fun appendObjectParameter(
        pairs: MutableList<String>,
        name: String,
        values: Map<*, *>,
        style: String,
        explode: Boolean,
        allowReserved: Boolean,
    ) {
        val serialized = mutableListOf<String>()
        values.forEach { (key, value) ->
            if (value == null) return@forEach
            if (style == "form" && explode) {
                pairs += urlEncode(key.toString()) + "=" + encodeQueryValue(value.toString(), allowReserved)
            } else {
                serialized += key.toString()
                serialized += value.toString()
            }
        }
        if (serialized.isNotEmpty()) {
            pairs += urlEncode(name) + "=" + encodeQueryValue(serialized.joinToString(","), allowReserved)
        }
    }

    private fun appendDeepObjectParameter(pairs: MutableList<String>, name: String, values: Map<*, *>, allowReserved: Boolean) {
        values.forEach { (key, value) ->
            if (value != null) {
                pairs += urlEncode("$name[$key]") + "=" + encodeQueryValue(value.toString(), allowReserved)
            }
        }
    }

    private fun encodeQueryValue(value: String, allowReserved: Boolean): String {
        var encoded = urlEncode(value)
        if (!allowReserved) return encoded
        mapOf(
            "%3A" to ":", "%2F" to "/", "%3F" to "?", "%23" to "#",
            "%5B" to "[", "%5D" to "]", "%40" to "@", "%21" to "!",
            "%24" to "$", "%26" to "&", "%27" to "'", "%28" to "(",
            "%29" to ")", "%2A" to "*", "%2B" to "+", "%2C" to ",",
            "%3B" to ";", "%3D" to "=",
        ).forEach { (escaped, reserved) -> encoded = encoded.replace(escaped, reserved) }
        return encoded
    }

    private fun urlEncode(value: String): String {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8)
    }

}
