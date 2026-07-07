package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class SocialApi(private val client: HttpClient) {

    /** Search social users */
    suspend fun usersList(q: String? = null, pageSize: Int? = null, cursor: String? = null): SocialUsersListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("q", q, "form", true, false, null),
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/social/users"), query))
        return client.convertValue(raw, object : TypeReference<SocialUsersListResponse>() {})
    }

    /** List friend requests */
    suspend fun friendRequestsList(direction: String? = null, status: String? = null, pageSize: Int? = null, cursor: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("direction", direction, "form", true, false, null),
            QueryParameterSpec("status", status, "form", true, false, null),
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/social/friend_requests"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** Create a friend request */
    suspend fun friendRequestsCreate(body: SubmitFriendRequestRequest): SocialFriendRequestsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/social/friend_requests"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestsCreateResponse201>() {})
    }

    /** Retrieve pending incoming friend request count */
    suspend fun friendRequestsPendingCountRetrieve(): SocialFriendRequestsPendingCountRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/social/friend_requests/pending/count"))
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestsPendingCountRetrieveResponse>() {})
    }

    /** Accept a friend request */
    suspend fun friendRequestsAccept(friendRequestId: String): SocialFriendRequestsAcceptResponse? {
        val raw = client.post(ApiPaths.imPath("/social/friend_requests/${serializePathParameter(friendRequestId, PathParameterSpec("friendRequestId", "simple", false))}/accept"), null)
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestsAcceptResponse>() {})
    }

    /** Decline a friend request */
    suspend fun friendRequestsDecline(friendRequestId: String): SocialFriendRequestsDeclineResponse? {
        val raw = client.post(ApiPaths.imPath("/social/friend_requests/${serializePathParameter(friendRequestId, PathParameterSpec("friendRequestId", "simple", false))}/decline"), null)
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestsDeclineResponse>() {})
    }

    /** Cancel a friend request */
    suspend fun friendRequestsCancel(friendRequestId: String): SocialFriendRequestsCancelResponse? {
        val raw = client.post(ApiPaths.imPath("/social/friend_requests/${serializePathParameter(friendRequestId, PathParameterSpec("friendRequestId", "simple", false))}/cancel"), null)
        return client.convertValue(raw, object : TypeReference<SocialFriendRequestsCancelResponse>() {})
    }

    /** Remove a friendship */
    suspend fun friendshipsRemove(friendshipId: String): SocialFriendshipsRemoveResponse? {
        val raw = client.post(ApiPaths.imPath("/social/friendships/${serializePathParameter(friendshipId, PathParameterSpec("friendshipId", "simple", false))}/remove"), null)
        return client.convertValue(raw, object : TypeReference<SocialFriendshipsRemoveResponse>() {})
    }

    /** Block a social user */
    suspend fun userBlocksCreate(body: BlockUserRequest): SocialUserBlocksCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/social/user_blocks"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialUserBlocksCreateResponse201>() {})
    }

    /** Release a social user block */
    suspend fun userBlocksDelete(blockId: String): Unit {
        client.delete(ApiPaths.imPath("/social/user_blocks/${serializePathParameter(blockId, PathParameterSpec("blockId", "simple", false))}"))
    }

    /** List contact tags */
    suspend fun contactsTagsList(pageSize: Int? = null, cursor: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/social/contacts/tags"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** Create a contact tag */
    suspend fun contactsTagsCreate(body: CreateContactTagRequest): SocialContactsTagsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/social/contacts/tags"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialContactsTagsCreateResponse201>() {})
    }

    /** Update a contact tag */
    suspend fun contactsTagsUpdate(tagId: String, body: UpdateContactTagRequest): SocialContactsTagsUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/social/contacts/tags/${serializePathParameter(tagId, PathParameterSpec("tagId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialContactsTagsUpdateResponse>() {})
    }

    /** Delete a contact tag */
    suspend fun contactsTagsDelete(tagId: String): Unit {
        client.delete(ApiPaths.imPath("/social/contacts/tags/${serializePathParameter(tagId, PathParameterSpec("tagId", "simple", false))}"))
    }

    /** Create a contact recommendation */
    suspend fun contactsRecommendationsCreate(targetUserId: String, body: CreateContactRecommendationRequest): SocialContactsRecommendationsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/social/contacts/${serializePathParameter(targetUserId, PathParameterSpec("targetUserId", "simple", false))}/recommendations"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialContactsRecommendationsCreateResponse201>() {})
    }

    /** Retrieve contact preferences */
    suspend fun contactsPreferencesRetrieve(targetUserId: String): SocialContactsPreferencesRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/social/contacts/${serializePathParameter(targetUserId, PathParameterSpec("targetUserId", "simple", false))}/preferences"))
        return client.convertValue(raw, object : TypeReference<SocialContactsPreferencesRetrieveResponse>() {})
    }

    /** Update contact preferences */
    suspend fun contactsPreferencesUpdate(targetUserId: String, body: UpdateContactPreferencesRequest): SocialContactsPreferencesUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/social/contacts/${serializePathParameter(targetUserId, PathParameterSpec("targetUserId", "simple", false))}/preferences"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SocialContactsPreferencesUpdateResponse>() {})
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
