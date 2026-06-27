package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class CallsApi(private val client: HttpClient) {

    /** Create an IM call signaling session */
    suspend fun sessionsCreate(body: CreateRtcSessionRequest): RtcSessionMutationResponse? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcSessionMutationResponse>() {})
    }

    /** Retrieve IM call signaling session state */
    suspend fun sessionsRetrieve(rtcSessionId: String): RtcSession? {
        val raw = client.get(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<RtcSession>() {})
    }

    /** Invite participants into an IM call signaling session */
    suspend fun sessionsInvite(rtcSessionId: String, body: InviteRtcSessionRequest): RtcSessionMutationResponse? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}/invite"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcSessionMutationResponse>() {})
    }

    /** Accept an IM call signaling session */
    suspend fun sessionsAccept(rtcSessionId: String, body: UpdateRtcSessionRequest): RtcSessionMutationResponse? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}/accept"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcSessionMutationResponse>() {})
    }

    /** Reject an IM call signaling session */
    suspend fun sessionsReject(rtcSessionId: String, body: UpdateRtcSessionRequest): RtcSessionMutationResponse? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}/reject"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcSessionMutationResponse>() {})
    }

    /** End an IM call signaling session */
    suspend fun sessionsEnd(rtcSessionId: String, body: UpdateRtcSessionRequest): RtcSessionMutationResponse? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}/end"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcSessionMutationResponse>() {})
    }

    /** Post an IM call signaling event */
    suspend fun sessionsSignalsCreate(rtcSessionId: String, body: PostRtcSignalRequest): RtcSignalEvent? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}/signals"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcSignalEvent>() {})
    }

    /** Issue an RTC media participant credential for an IM call */
    suspend fun sessionsCredentialsCreate(rtcSessionId: String, body: IssueRtcParticipantCredentialRequest): RtcParticipantCredential? {
        val raw = client.post(ApiPaths.imPath("/calls/sessions/${serializePathParameter(rtcSessionId, PathParameterSpec("rtcSessionId", "simple", false))}/credentials"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RtcParticipantCredential>() {})
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
