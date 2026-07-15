package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class ChatApi(private val client: HttpClient) {

    /** List IM contacts */
    suspend fun contactsList(pageSize: Int? = null, cursor: String? = null): ContactsListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/contacts"), query))
        return client.convertValue(raw, object : TypeReference<ContactsListResponse>() {})
    }

    /** List current inbox window */
    suspend fun inboxList(pageSize: Int? = null, cursor: String? = null, conversationType: String? = null): InboxListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("conversation_type", conversationType, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/inbox"), query))
        return client.convertValue(raw, object : TypeReference<InboxListResponse>() {})
    }

    /** Create a conversation */
    suspend fun conversationsCreate(body: CreateConversationRequest): ConversationsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsCreateResponse201>() {})
    }

    /** Create an agent dialog */
    suspend fun conversationsAgentDialogsCreate(body: CreateAgentDialogRequest): ConversationsAgentDialogsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/agent_dialogs"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsAgentDialogsCreateResponse201>() {})
    }

    /** Create an agent handoff */
    suspend fun conversationsAgentHandoffsCreate(body: CreateAgentHandoffRequest): ConversationsAgentHandoffsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/agent_handoffs"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsAgentHandoffsCreateResponse201>() {})
    }

    /** Create a system channel */
    suspend fun conversationsSystemChannelsCreate(body: CreateSystemChannelRequest): ConversationsSystemChannelsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/system_channels"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsSystemChannelsCreateResponse201>() {})
    }

    /** Create a thread conversation */
    suspend fun conversationsThreadsCreate(body: CreateThreadConversationRequest): ConversationsThreadsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/threads"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsThreadsCreateResponse201>() {})
    }

    /** Create a direct chat conversation binding */
    suspend fun conversationsDirectChatsBindingsCreate(body: BindDirectChatRequest): ConversationsDirectChatsBindingsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/direct_chats/bindings"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsDirectChatsBindingsCreateResponse201>() {})
    }

    /** Retrieve agent handoff state */
    suspend fun conversationsAgentHandoffRetrieve(conversationId: String): ConversationsAgentHandoffRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff"))
        return client.convertValue(raw, object : TypeReference<ConversationsAgentHandoffRetrieveResponse>() {})
    }

    /** Accept agent handoff */
    suspend fun conversationsAgentHandoffAccept(conversationId: String): ConversationsAgentHandoffAcceptResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff/accept"), null)
        return client.convertValue(raw, object : TypeReference<ConversationsAgentHandoffAcceptResponse>() {})
    }

    /** Resolve agent handoff */
    suspend fun conversationsAgentHandoffResolve(conversationId: String): ConversationsAgentHandoffResolveResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff/resolve"), null)
        return client.convertValue(raw, object : TypeReference<ConversationsAgentHandoffResolveResponse>() {})
    }

    /** Close agent handoff */
    suspend fun conversationsAgentHandoffClose(conversationId: String): ConversationsAgentHandoffCloseResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agent_handoff/close"), null)
        return client.convertValue(raw, object : TypeReference<ConversationsAgentHandoffCloseResponse>() {})
    }

    /** Retrieve conversation summary */
    suspend fun conversationsRetrieve(conversationId: String): ConversationsRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<ConversationsRetrieveResponse>() {})
    }

    /** List conversation members */
    suspend fun conversationsMembersList(conversationId: String, pageSize: Int? = null, cursor: String? = null): ConversationsMembersListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members"), query))
        return client.convertValue(raw, object : TypeReference<ConversationsMembersListResponse>() {})
    }

    /** Retrieve the current conversation member */
    suspend fun conversationsMembersCurrentRetrieve(conversationId: String): ConversationsMembersCurrentRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/current"))
        return client.convertValue(raw, object : TypeReference<ConversationsMembersCurrentRetrieveResponse>() {})
    }

    /** Retrieve assigned group agents */
    suspend fun conversationsAgentsRetrieve(conversationId: String): ConversationsAgentsRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agents"))
        return client.convertValue(raw, object : TypeReference<ConversationsAgentsRetrieveResponse>() {})
    }

    /** Update assigned group agents */
    suspend fun conversationsAgentsUpdate(conversationId: String, body: UpdateConversationAgentsRequest): ConversationsAgentsUpdateResponse? {
        val raw = client.put(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/agents"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsAgentsUpdateResponse>() {})
    }

    /** Add a conversation member */
    suspend fun conversationsMembersAdd(conversationId: String, body: AddConversationMemberRequest): ConversationsMembersAddResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/add"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsMembersAddResponse>() {})
    }

    /** Remove a conversation member */
    suspend fun conversationsMembersRemove(conversationId: String, body: RemoveConversationMemberRequest): ConversationsMembersRemoveResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/remove"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsMembersRemoveResponse>() {})
    }

    /** Transfer conversation owner */
    suspend fun conversationsMembersTransferOwner(conversationId: String, body: TransferConversationOwnerRequest): ConversationsMembersTransferOwnerResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/transfer_owner"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsMembersTransferOwnerResponse>() {})
    }

    /** Change conversation member role */
    suspend fun conversationsMembersChangeRole(conversationId: String, body: ChangeConversationMemberRoleRequest): ConversationsMembersChangeRoleResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/change_role"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsMembersChangeRoleResponse>() {})
    }

    /** Leave a conversation */
    suspend fun conversationsMembersLeave(conversationId: String): ConversationsMembersLeaveResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/leave"), null)
        return client.convertValue(raw, object : TypeReference<ConversationsMembersLeaveResponse>() {})
    }

    /** Accept a conversation invitation */
    suspend fun conversationsMembersAcceptInvitation(conversationId: String): ConversationsMembersAcceptInvitationResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/members/accept_invitation"), null)
        return client.convertValue(raw, object : TypeReference<ConversationsMembersAcceptInvitationResponse>() {})
    }

    /** Retrieve conversation preferences */
    suspend fun conversationsPreferencesRetrieve(conversationId: String): ConversationsPreferencesRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/preferences"))
        return client.convertValue(raw, object : TypeReference<ConversationsPreferencesRetrieveResponse>() {})
    }

    /** Update conversation preferences */
    suspend fun conversationsPreferencesUpdate(conversationId: String, body: UpdateConversationPreferencesRequest): ConversationsPreferencesUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/preferences"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsPreferencesUpdateResponse>() {})
    }

    /** Retrieve conversation profile */
    suspend fun conversationsProfileRetrieve(conversationId: String): ConversationsProfileRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/profile"))
        return client.convertValue(raw, object : TypeReference<ConversationsProfileRetrieveResponse>() {})
    }

    /** Update conversation profile */
    suspend fun conversationsProfileUpdate(conversationId: String, body: UpdateConversationProfileRequest): ConversationsProfileUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/profile"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsProfileUpdateResponse>() {})
    }

    /** Retrieve read cursor */
    suspend fun conversationsReadCursorRetrieve(conversationId: String): ConversationsReadCursorRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/read_cursor"))
        return client.convertValue(raw, object : TypeReference<ConversationsReadCursorRetrieveResponse>() {})
    }

    /** Update read cursor */
    suspend fun conversationsReadCursorUpdate(conversationId: String, body: UpdateReadCursorRequest): ConversationsReadCursorUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/read_cursor"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsReadCursorUpdateResponse>() {})
    }

    /** List member directory */
    suspend fun conversationsMemberDirectoryList(conversationId: String, cursor: String? = null, pageSize: Int? = null): ConversationsMemberDirectoryListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page_size", pageSize, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/member_directory"), query))
        return client.convertValue(raw, object : TypeReference<ConversationsMemberDirectoryListResponse>() {})
    }

    /** List conversation message history */
    suspend fun conversationsMessagesList(conversationId: String, cursor: String? = null, pageSize: Int? = null): ConversationMessageListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page_size", pageSize, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/messages"), query))
        return client.convertValue(raw, object : TypeReference<ConversationMessageListResponse>() {})
    }

    /** Post a conversation message */
    suspend fun conversationsMessagesCreate(conversationId: String, body: PostMessageRequest): ConversationsMessagesCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/messages"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsMessagesCreateResponse201>() {})
    }

    /** Publish a system channel message */
    suspend fun conversationsSystemChannelPublish(conversationId: String, body: PostMessageRequest): ConversationsSystemChannelPublishResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/system_channel/publish"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ConversationsSystemChannelPublishResponse>() {})
    }

    /** List pinned messages */
    suspend fun conversationsPinsList(conversationId: String, cursor: String? = null, pageSize: Int? = null): ConversationsPinsListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page_size", pageSize, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/pins"), query))
        return client.convertValue(raw, object : TypeReference<ConversationsPinsListResponse>() {})
    }

    /** Retrieve message interaction summary */
    suspend fun conversationsMessagesInteractionSummaryRetrieve(conversationId: String, messageId: String): ConversationsMessagesInteractionSummaryRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/conversations/${serializePathParameter(conversationId, PathParameterSpec("conversationId", "simple", false))}/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/interaction_summary"))
        return client.convertValue(raw, object : TypeReference<ConversationsMessagesInteractionSummaryRetrieveResponse>() {})
    }

    /** Edit a message */
    suspend fun messagesEdit(messageId: String, body: EditMessageRequest): MessagesEditResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/edit"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessagesEditResponse>() {})
    }

    /** Recall a message */
    suspend fun messagesRecall(messageId: String, body: RecallMessageRequest): MessagesRecallResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/recall"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessagesRecallResponse>() {})
    }

    /** List message favorites */
    suspend fun messagesFavoritesList(pageSize: Int? = null, cursor: String? = null, favoriteType: String? = null, q: String? = null): MessagesFavoritesListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("favoriteType", favoriteType, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/messages/favorites"), query))
        return client.convertValue(raw, object : TypeReference<MessagesFavoritesListResponse>() {})
    }

    /** Favorite a message */
    suspend fun messagesFavoritesCreate(messageId: String, body: FavoriteMessageRequest): MessagesFavoritesCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/favorites"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessagesFavoritesCreateResponse201>() {})
    }

    /** Delete a message favorite */
    suspend fun messagesFavoritesDelete(favoriteId: String): Unit {
        client.delete(ApiPaths.imPath("/chat/messages/favorites/${serializePathParameter(favoriteId, PathParameterSpec("favoriteId", "simple", false))}"))
    }

    /** Delete message visibility for the current principal */
    suspend fun messagesVisibilityDelete(messageId: String): Unit {
        client.delete(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/visibility"))
    }

    /** Add a message reaction */
    suspend fun messagesReactionsCreate(messageId: String, body: MessageReactionRequest): MessagesReactionsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/reactions"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessagesReactionsCreateResponse201>() {})
    }

    /** Remove a message reaction */
    suspend fun messagesReactionsRemove(messageId: String, body: MessageReactionRequest): MessagesReactionsRemoveResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/reactions/remove"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MessagesReactionsRemoveResponse>() {})
    }

    /** Pin a message */
    suspend fun messagesPin(messageId: String): MessagesPinResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/pin"), null)
        return client.convertValue(raw, object : TypeReference<MessagesPinResponse>() {})
    }

    /** Unpin a message */
    suspend fun messagesUnpin(messageId: String): MessagesUnpinResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/messages/${serializePathParameter(messageId, PathParameterSpec("messageId", "simple", false))}/unpin"), null)
        return client.convertValue(raw, object : TypeReference<MessagesUnpinResponse>() {})
    }

    /** Create a live, chat, or game room bound to a group conversation */
    suspend fun roomsCreate(body: CreateRoomRequest): RoomsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/chat/rooms"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RoomsCreateResponse201>() {})
    }

    /** Retrieve room metadata and active member count */
    suspend fun roomsRetrieve(roomId: String): RoomsRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/chat/rooms/${serializePathParameter(roomId, PathParameterSpec("roomId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<RoomsRetrieveResponse>() {})
    }

    /** Enter a room as the authenticated principal */
    suspend fun roomsEnter(roomId: String): RoomsEnterResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/rooms/${serializePathParameter(roomId, PathParameterSpec("roomId", "simple", false))}/enter"), null)
        return client.convertValue(raw, object : TypeReference<RoomsEnterResponse>() {})
    }

    /** Leave a room as the authenticated principal */
    suspend fun roomsLeave(roomId: String): RoomsLeaveResponse? {
        val raw = client.post(ApiPaths.imPath("/chat/rooms/${serializePathParameter(roomId, PathParameterSpec("roomId", "simple", false))}/leave"), null)
        return client.convertValue(raw, object : TypeReference<RoomsLeaveResponse>() {})
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
