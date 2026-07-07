package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class SpacesApi(private val client: HttpClient) {

    /** Create a space */
    suspend fun create(body: SpaceCreateRequest): SpacesCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesCreateResponse201>() {})
    }

    /** List spaces */
    suspend fun list(pageSize: Int? = null, cursor: String? = null): SpacesListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces"), query))
        return client.convertValue(raw, object : TypeReference<SpacesListResponse>() {})
    }

    /** Retrieve a space */
    suspend fun retrieve(spaceId: String): SpacesRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesRetrieveResponse>() {})
    }

    /** Update a space */
    suspend fun update(spaceId: String, body: SpaceUpdateRequest): SpacesUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesUpdateResponse>() {})
    }

    /** Delete a space */
    suspend fun delete(spaceId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}"))
    }

    /** List spaces members */
    suspend fun membersList(spaceId: String, pageSize: Int? = null, cursor: String? = null): SpacesMembersListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members"), query))
        return client.convertValue(raw, object : TypeReference<SpacesMembersListResponse>() {})
    }

    /** Create spaces members */
    suspend fun membersCreate(spaceId: String, body: SpaceMemberCreateRequest): SpacesMembersCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesMembersCreateResponse201>() {})
    }

    /** retrieve spaces members */
    suspend fun membersRetrieve(spaceId: String, userId: String): SpacesMembersRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesMembersRetrieveResponse>() {})
    }

    /** Update spaces members */
    suspend fun membersUpdate(spaceId: String, userId: String, body: SpaceMemberUpdateRequest): SpacesMembersUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesMembersUpdateResponse>() {})
    }

    /** Delete spaces members */
    suspend fun membersDelete(spaceId: String, userId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
    }

    /** List spaces groups */
    suspend fun groupsList(spaceId: String, pageSize: Int? = null, cursor: String? = null): SpacesGroupsListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups"), query))
        return client.convertValue(raw, object : TypeReference<SpacesGroupsListResponse>() {})
    }

    /** Create spaces groups */
    suspend fun groupsCreate(spaceId: String, body: SpaceGroupCreateRequest): SpacesGroupsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesGroupsCreateResponse201>() {})
    }

    /** retrieve spaces groups */
    suspend fun groupsRetrieve(spaceId: String, groupId: String): SpacesGroupsRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesGroupsRetrieveResponse>() {})
    }

    /** Update spaces groups */
    suspend fun groupsUpdate(spaceId: String, groupId: String, body: SpaceGroupUpdateRequest): SpacesGroupsUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesGroupsUpdateResponse>() {})
    }

    /** Delete spaces groups */
    suspend fun groupsDelete(spaceId: String, groupId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"))
    }

    /** List spaces groups members */
    suspend fun groupsMembersList(spaceId: String, groupId: String, pageSize: Int? = null, cursor: String? = null): SpacesGroupsMembersListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members"), query))
        return client.convertValue(raw, object : TypeReference<SpacesGroupsMembersListResponse>() {})
    }

    /** Create spaces groups members */
    suspend fun groupsMembersCreate(spaceId: String, groupId: String, body: SpaceGroupMemberCreateRequest): SpacesGroupsMembersCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesGroupsMembersCreateResponse201>() {})
    }

    /** retrieve spaces groups members */
    suspend fun groupsMembersRetrieve(spaceId: String, groupId: String, userId: String): SpacesGroupsMembersRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesGroupsMembersRetrieveResponse>() {})
    }

    /** Update spaces groups members */
    suspend fun groupsMembersUpdate(spaceId: String, groupId: String, userId: String, body: SpaceGroupMemberUpdateRequest): SpacesGroupsMembersUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesGroupsMembersUpdateResponse>() {})
    }

    /** Delete spaces groups members */
    suspend fun groupsMembersDelete(spaceId: String, groupId: String, userId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
    }

    /** List spaces channels */
    suspend fun channelsList(spaceId: String, pageSize: Int? = null, cursor: String? = null): SpacesChannelsListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels"), query))
        return client.convertValue(raw, object : TypeReference<SpacesChannelsListResponse>() {})
    }

    /** Create spaces channels */
    suspend fun channelsCreate(spaceId: String, body: SpaceChannelCreateRequest): SpacesChannelsCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesChannelsCreateResponse201>() {})
    }

    /** retrieve spaces channels */
    suspend fun channelsRetrieve(spaceId: String, channelId: String): SpacesChannelsRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesChannelsRetrieveResponse>() {})
    }

    /** Update spaces channels */
    suspend fun channelsUpdate(spaceId: String, channelId: String, body: SpaceChannelUpdateRequest): SpacesChannelsUpdateResponse? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesChannelsUpdateResponse>() {})
    }

    /** Delete spaces channels */
    suspend fun channelsDelete(spaceId: String, channelId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"))
    }

    /** List spaces channels access Rules */
    suspend fun channelsAccessRulesList(spaceId: String, channelId: String, pageSize: Int? = null, cursor: String? = null): SpacesChannelsAccessRulesListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/access_rules"), query))
        return client.convertValue(raw, object : TypeReference<SpacesChannelsAccessRulesListResponse>() {})
    }

    /** Create spaces channels access Rules */
    suspend fun channelsAccessRulesCreate(spaceId: String, channelId: String, body: SpaceChannelAccessRuleCreateRequest): SpacesChannelsAccessRulesCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/access_rules"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesChannelsAccessRulesCreateResponse201>() {})
    }

    /** Delete spaces channels access Rules */
    suspend fun channelsAccessRulesDelete(spaceId: String, channelId: String, ruleId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/access_rules/${serializePathParameter(ruleId, PathParameterSpec("ruleId", "simple", false))}"))
    }

    /** List spaces invites */
    suspend fun invitesList(spaceId: String, status: String? = null, pageSize: Int? = null, cursor: String? = null): SpacesInvitesListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("status", status, "form", true, false, null),
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites"), query))
        return client.convertValue(raw, object : TypeReference<SpacesInvitesListResponse>() {})
    }

    /** Create spaces invites */
    suspend fun invitesCreate(spaceId: String, body: SpaceInviteCreateRequest): SpacesInvitesCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesInvitesCreateResponse201>() {})
    }

    /** retrieve spaces invites */
    suspend fun invitesRetrieve(spaceId: String, inviteCode: String): SpacesInvitesRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites/${serializePathParameter(inviteCode, PathParameterSpec("inviteCode", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesInvitesRetrieveResponse>() {})
    }

    /** Delete spaces invites */
    suspend fun invitesDelete(spaceId: String, inviteCode: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites/${serializePathParameter(inviteCode, PathParameterSpec("inviteCode", "simple", false))}"))
    }

    /** Accept spaces invites */
    suspend fun invitesAccept(spaceId: String, inviteCode: String): SdkWorkCommandResponse? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites/${serializePathParameter(inviteCode, PathParameterSpec("inviteCode", "simple", false))}/accept"), null)
        return client.convertValue(raw, object : TypeReference<SdkWorkCommandResponse>() {})
    }

    /** List spaces bans */
    suspend fun bansList(spaceId: String, pageSize: Int? = null, cursor: String? = null): SpacesBansListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans"), query))
        return client.convertValue(raw, object : TypeReference<SpacesBansListResponse>() {})
    }

    /** Create spaces bans */
    suspend fun bansCreate(spaceId: String, body: SpaceBanCreateRequest): SpacesBansCreateResponse201? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpacesBansCreateResponse201>() {})
    }

    /** retrieve spaces bans */
    suspend fun bansRetrieve(spaceId: String, userId: String): SpacesBansRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpacesBansRetrieveResponse>() {})
    }

    /** Delete spaces bans */
    suspend fun bansDelete(spaceId: String, userId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
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
