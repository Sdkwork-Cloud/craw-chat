package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class ChatApi(private val client: HttpClient) {

    /** List IM contacts */
    suspend fun contactsList(limit: Int? = null, cursor: String? = null): ContactsResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("limit", limit, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/contacts"), query))
        return client.convertValue(raw, object : TypeReference<ContactsResponse>() {})
    }

    /** Retrieve current inbox window */
    suspend fun inboxRetrieve(limit: Int? = null, cursor: String? = null): InboxResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("limit", limit, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/inbox"), query))
        return client.convertValue(raw, object : TypeReference<InboxResponse>() {})
    }

    /** Create a conversation */
    suspend fun conversationsCreate(body: CreateConversationRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Create an agent dialog */
    suspend fun conversationsAgentDialogsCreate(body: CreateAgentDialogRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/agent_dialogs"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Create an agent handoff */
    suspend fun conversationsAgentHandoffsCreate(body: CreateAgentDialogRequest): AckResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/agent_handoffs"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Create a system channel */
    suspend fun conversationsSystemChannelsCreate(body: CreateConversationRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/system_channels"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Create a thread conversation */
    suspend fun conversationsThreadsCreate(body: CreateConversationRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/threads"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Bind a direct chat conversation */
    suspend fun conversationsDirectChatsBind(body: BindDirectChatRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/direct_chats/bindings"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Retrieve agent handoff state */
    suspend fun conversationsAgentHandoffRetrieve(conversationId: String): AckResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff"))
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Accept agent handoff */
    suspend fun conversationsAgentHandoffAccept(conversationId: String): AckResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff/accept"), null)
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Resolve agent handoff */
    suspend fun conversationsAgentHandoffResolve(conversationId: String): AckResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff/resolve"), null)
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Close agent handoff */
    suspend fun conversationsAgentHandoffClose(conversationId: String): AckResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff/close"), null)
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Retrieve conversation summary */
    suspend fun conversationsRetrieve(conversationId: String): ConversationSummaryView? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<ConversationSummaryView>() {})
    }

    /** List conversation members */
    suspend fun conversationsMembersList(conversationId: String, limit: Int? = null, cursor: String? = null): ListMembersResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("limit", limit, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members"), query))
        return client.convertValue(raw, object : TypeReference<ListMembersResponse>() {})
    }

    /** Add a conversation member */
    suspend fun conversationsMembersAdd(conversationId: String, body: AddConversationMemberRequest): ConversationMember? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/add"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationMember>() {})
    }

    /** Remove a conversation member */
    suspend fun conversationsMembersRemove(conversationId: String, body: RemoveConversationMemberRequest): AckResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/remove"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Transfer conversation owner */
    suspend fun conversationsMembersTransferOwner(conversationId: String, body: TransferConversationOwnerRequest): ConversationMember? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/transfer_owner"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationMember>() {})
    }

    /** Change conversation member role */
    suspend fun conversationsMembersChangeRole(conversationId: String, body: ChangeConversationMemberRoleRequest): ConversationMember? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/change_role"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationMember>() {})
    }

    /** Leave a conversation */
    suspend fun conversationsMembersLeave(conversationId: String): AckResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/leave"), null)
        return client.convertValue(raw, object : TypeReference<AckResponse>() {})
    }

    /** Retrieve conversation preferences */
    suspend fun conversationsPreferencesRetrieve(conversationId: String): ConversationPreferencesView? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/preferences"))
        return client.convertValue(raw, object : TypeReference<ConversationPreferencesView>() {})
    }

    /** Update conversation preferences */
    suspend fun conversationsPreferencesUpdate(conversationId: String, body: UpdateConversationPreferencesRequest): ConversationPreferencesView? {
        val raw = client.patch(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/preferences"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationPreferencesView>() {})
    }

    /** Retrieve conversation profile */
    suspend fun conversationsProfileRetrieve(conversationId: String): ConversationProfileView? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/profile"))
        return client.convertValue(raw, object : TypeReference<ConversationProfileView>() {})
    }

    /** Update conversation profile */
    suspend fun conversationsProfileUpdate(conversationId: String, body: UpdateConversationProfileRequest): ConversationProfileView? {
        val raw = client.patch(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/profile"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationProfileView>() {})
    }

    /** Retrieve read cursor */
    suspend fun conversationsReadCursorRetrieve(conversationId: String): ReadCursorView? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/read_cursor"))
        return client.convertValue(raw, object : TypeReference<ReadCursorView>() {})
    }

    /** Update read cursor */
    suspend fun conversationsReadCursorUpdate(conversationId: String, body: UpdateReadCursorRequest): ReadCursorView? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/read_cursor"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ReadCursorView>() {})
    }

    /** List member directory */
    suspend fun conversationsMemberDirectoryList(conversationId: String): MemberDirectoryResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/member_directory"))
        return client.convertValue(raw, object : TypeReference<MemberDirectoryResponse>() {})
    }

    /** List conversation message timeline */
    suspend fun conversationsMessagesList(conversationId: String, afterSeq: Int? = null, limit: Int? = null): TimelineResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("afterSeq", afterSeq, "form", true, false, null),
            QueryParameterSpec("limit", limit, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/messages"), query))
        return client.convertValue(raw, object : TypeReference<TimelineResponse>() {})
    }

    /** Post a conversation message */
    suspend fun conversationsMessagesCreate(conversationId: String, body: PostMessageRequest): PostedMessageResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/messages"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PostedMessageResponse>() {})
    }

    /** Publish a system channel message */
    suspend fun conversationsSystemChannelPublish(conversationId: String, body: PostMessageRequest): PostedMessageResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/system_channel/publish"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PostedMessageResponse>() {})
    }

    /** List pinned messages */
    suspend fun conversationsPinsList(conversationId: String): PinnedMessagesResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/pins"))
        return client.convertValue(raw, object : TypeReference<PinnedMessagesResponse>() {})
    }

    /** Retrieve message interaction summary */
    suspend fun conversationsMessagesInteractionSummaryRetrieve(conversationId: String, messageId: String): MessageInteractionSummaryView? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/interaction_summary"))
        return client.convertValue(raw, object : TypeReference<MessageInteractionSummaryView>() {})
    }

    /** Edit a message */
    suspend fun messagesEdit(messageId: String, body: EditMessageRequest): PostedMessageResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/edit"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PostedMessageResponse>() {})
    }

    /** Recall a message */
    suspend fun messagesRecall(messageId: String): PostedMessageResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/recall"), null)
        return client.convertValue(raw, object : TypeReference<PostedMessageResponse>() {})
    }

    /** List message favorites */
    suspend fun messagesFavoritesList(limit: Int? = null, cursor: String? = null, favoriteType: String? = null, q: String? = null): FavoriteMessagesResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("limit", limit, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("favoriteType", favoriteType, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/messages/favorites"), query))
        return client.convertValue(raw, object : TypeReference<FavoriteMessagesResponse>() {})
    }

    /** Favorite a message */
    suspend fun messagesFavoritesCreate(messageId: String, body: FavoriteMessageRequest): MessageFavoriteView? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/favorites"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessageFavoriteView>() {})
    }

    /** Delete a message favorite */
    suspend fun messagesFavoritesDelete(favoriteId: String): DeleteMessageFavoriteResponse? {
        val raw = client.delete(ApiPaths.imPath("/chat/messages/favorites/${serializePathParameter(favoriteId, PathParameterSpec("favoriteId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<DeleteMessageFavoriteResponse>() {})
    }

    /** Delete message visibility for the current principal */
    suspend fun messagesVisibilityDelete(messageId: String): MessageVisibilityMutationResult? {
        val raw = client.delete(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/visibility"))
        return client.convertValue(raw, object : TypeReference<MessageVisibilityMutationResult>() {})
    }

    /** Add a message reaction */
    suspend fun messagesReactionsCreate(messageId: String, body: MessageReactionRequest): MessageReactionMutationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/reactions"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessageReactionMutationResult>() {})
    }

    /** Remove a message reaction */
    suspend fun messagesReactionsDelete(messageId: String, body: MessageReactionRequest): MessageReactionMutationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/reactions/remove"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessageReactionMutationResult>() {})
    }

    /** Pin a message */
    suspend fun messagesPinCreate(messageId: String): MessagePinMutationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/pin"), null)
        return client.convertValue(raw, object : TypeReference<MessagePinMutationResult>() {})
    }

    /** Unpin a message */
    suspend fun messagesPinDelete(messageId: String): MessagePinMutationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/unpin"), null)
        return client.convertValue(raw, object : TypeReference<MessagePinMutationResult>() {})
    }

    /** Create a live, chat, or game room bound to a group conversation */
    suspend fun roomsCreate(body: CreateRoomRequest): CreateConversationResult? {
        val raw = client.post(ApiPaths.imPath("/chat/rooms"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CreateConversationResult>() {})
    }

    /** Get room metadata and active member count */
    suspend fun roomsGet(roomId: String): RoomView? {
        val raw = client.get(ApiPaths.imPath("/chat/rooms/${serializePathParameter(roomId, PathParameterSpec("roomId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<RoomView>() {})
    }

    /** Enter a room as the authenticated principal */
    suspend fun roomsEnter(roomId: String): EnterRoomResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/rooms/${serializePathParameter(roomId, PathParameterSpec("roomId", "simple", false))}/enter"), null)
        return client.convertValue(raw, object : TypeReference<EnterRoomResponse>() {})
    }

    /** Leave a room as the authenticated principal */
    suspend fun roomsLeave(roomId: String): EnterRoomResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/rooms/${serializePathParameter(roomId, PathParameterSpec("roomId", "simple", false))}/leave"), null)
        return client.convertValue(raw, object : TypeReference<EnterRoomResponse>() {})
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
