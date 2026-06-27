package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class SpacesApi(private val client: HttpClient) {

    /** Create a space */
    suspend fun create(body: SpaceCreateRequest): SpaceView? {
        val raw = client.post(ApiPaths.imPath("/spaces"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceView>() {})
    }

    /** List spaces */
    suspend fun list(): SpaceListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces"))
        return client.convertValue(raw, object : TypeReference<SpaceListResponse>() {})
    }

    /** Get a space */
    suspend fun get_(spaceId: String): SpaceView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceView>() {})
    }

    /** Update a space */
    suspend fun update(spaceId: String, body: SpaceUpdateRequest): SpaceView? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceView>() {})
    }

    /** Delete a space */
    suspend fun delete(spaceId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}"))
    }

    /** List spaces members */
    suspend fun membersList(spaceId: String): SpaceMemberListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members"))
        return client.convertValue(raw, object : TypeReference<SpaceMemberListResponse>() {})
    }

    /** Create spaces members */
    suspend fun membersCreate(spaceId: String, body: SpaceMemberCreateRequest): SpaceMemberView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceMemberView>() {})
    }

    /** Get spaces members */
    suspend fun membersGet(spaceId: String, userId: String): SpaceMemberView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceMemberView>() {})
    }

    /** Update spaces members */
    suspend fun membersUpdate(spaceId: String, userId: String, body: SpaceMemberUpdateRequest): SpaceMemberView? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceMemberView>() {})
    }

    /** Delete spaces members */
    suspend fun membersDelete(spaceId: String, userId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
    }

    /** List spaces groups */
    suspend fun groupsList(spaceId: String): SpaceGroupListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups"))
        return client.convertValue(raw, object : TypeReference<SpaceGroupListResponse>() {})
    }

    /** Create spaces groups */
    suspend fun groupsCreate(spaceId: String, body: SpaceGroupCreateRequest): SpaceGroupView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceGroupView>() {})
    }

    /** Get spaces groups */
    suspend fun groupsGet(spaceId: String, groupId: String): SpaceGroupView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceGroupView>() {})
    }

    /** Update spaces groups */
    suspend fun groupsUpdate(spaceId: String, groupId: String, body: SpaceGroupUpdateRequest): SpaceGroupView? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceGroupView>() {})
    }

    /** Delete spaces groups */
    suspend fun groupsDelete(spaceId: String, groupId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"))
    }

    /** List spaces groups members */
    suspend fun groupsMembersList(spaceId: String, groupId: String): SpaceGroupMemberListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members"))
        return client.convertValue(raw, object : TypeReference<SpaceGroupMemberListResponse>() {})
    }

    /** Create spaces groups members */
    suspend fun groupsMembersCreate(spaceId: String, groupId: String, body: SpaceGroupMemberCreateRequest): SpaceGroupMemberView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceGroupMemberView>() {})
    }

    /** Get spaces groups members */
    suspend fun groupsMembersGet(spaceId: String, groupId: String, userId: String): SpaceGroupMemberView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceGroupMemberView>() {})
    }

    /** Update spaces groups members */
    suspend fun groupsMembersUpdate(spaceId: String, groupId: String, userId: String, body: SpaceGroupMemberUpdateRequest): Unit {
        client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"), body, null, null, "application/json")
    }

    /** Delete spaces groups members */
    suspend fun groupsMembersDelete(spaceId: String, groupId: String, userId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/members/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
    }

    /** List spaces channels */
    suspend fun channelsList(spaceId: String): SpaceChannelListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels"))
        return client.convertValue(raw, object : TypeReference<SpaceChannelListResponse>() {})
    }

    /** Create spaces channels */
    suspend fun channelsCreate(spaceId: String, body: SpaceChannelCreateRequest): SpaceChannelView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceChannelView>() {})
    }

    /** Get spaces channels */
    suspend fun channelsGet(spaceId: String, channelId: String): SpaceChannelView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceChannelView>() {})
    }

    /** Update spaces channels */
    suspend fun channelsUpdate(spaceId: String, channelId: String, body: SpaceChannelUpdateRequest): SpaceChannelView? {
        val raw = client.patch(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceChannelView>() {})
    }

    /** Delete spaces channels */
    suspend fun channelsDelete(spaceId: String, channelId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"))
    }

    /** List spaces channels access Rules */
    suspend fun channelsAccessRulesList(spaceId: String, channelId: String): SpaceChannelAccessRuleListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/access_rules"))
        return client.convertValue(raw, object : TypeReference<SpaceChannelAccessRuleListResponse>() {})
    }

    /** Create spaces channels access Rules */
    suspend fun channelsAccessRulesCreate(spaceId: String, channelId: String, body: SpaceChannelAccessRuleCreateRequest): SpaceChannelAccessRuleView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/access_rules"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceChannelAccessRuleView>() {})
    }

    /** Delete spaces channels access Rules */
    suspend fun channelsAccessRulesDelete(spaceId: String, channelId: String, ruleId: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/access_rules/${serializePathParameter(ruleId, PathParameterSpec("ruleId", "simple", false))}"))
    }

    /** List spaces invites */
    suspend fun invitesList(spaceId: String): SpaceInviteListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites"))
        return client.convertValue(raw, object : TypeReference<SpaceInviteListResponse>() {})
    }

    /** Create spaces invites */
    suspend fun invitesCreate(spaceId: String, body: SpaceInviteCreateRequest): SpaceInviteView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceInviteView>() {})
    }

    /** Get spaces invites */
    suspend fun invitesGet(spaceId: String, inviteCode: String): SpaceInviteView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites/${serializePathParameter(inviteCode, PathParameterSpec("inviteCode", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceInviteView>() {})
    }

    /** Revoke spaces invites */
    suspend fun invitesRevoke(spaceId: String, inviteCode: String): Unit {
        client.delete(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites/${serializePathParameter(inviteCode, PathParameterSpec("inviteCode", "simple", false))}"))
    }

    /** Accept spaces invites */
    suspend fun invitesAccept(spaceId: String, inviteCode: String): Unit {
        client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/invites/${serializePathParameter(inviteCode, PathParameterSpec("inviteCode", "simple", false))}/accept"), null)
    }

    /** List spaces bans */
    suspend fun bansList(spaceId: String): SpaceBanListResponse? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans"))
        return client.convertValue(raw, object : TypeReference<SpaceBanListResponse>() {})
    }

    /** Create spaces bans */
    suspend fun bansCreate(spaceId: String, body: SpaceBanCreateRequest): SpaceBanView? {
        val raw = client.post(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SpaceBanView>() {})
    }

    /** Get spaces bans */
    suspend fun bansGet(spaceId: String, userId: String): SpaceBanView? {
        val raw = client.get(ApiPaths.imPath("/spaces/${serializePathParameter(spaceId, PathParameterSpec("spaceId", "simple", false))}/bans/${serializePathParameter(userId, PathParameterSpec("userId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<SpaceBanView>() {})
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


}
