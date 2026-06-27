package com.sdkwork.im.app.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.im.app.api.generated.http.HttpClient

class AutomationApi(private val client: HttpClient) {

    /** Start an agent response stream */
    suspend fun agentResponsesCreate(body: StartAgentResponseRequest): StreamSession? {
        val raw = client.post(ApiPaths.appPath("/automation/agent_responses"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamSession>() {})
    }

    /** Complete an agent response stream */
    suspend fun agentResponsesComplete(streamId: String, body: CompleteAgentResponseRequest): StreamSession? {
        val raw = client.post(ApiPaths.appPath("/automation/agent_responses/${serializePathParameter(streamId, PathParameterSpec("streamId", "simple", false))}/complete"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamSession>() {})
    }

    /** Append a frame to an agent response stream */
    suspend fun agentResponsesFramesCreate(streamId: String, body: AppendAgentResponseDeltaRequest): StreamFrame? {
        val raw = client.post(ApiPaths.appPath("/automation/agent_responses/${serializePathParameter(streamId, PathParameterSpec("streamId", "simple", false))}/frames"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamFrame>() {})
    }

    /** Request an agent tool call */
    suspend fun agentToolCallsCreate(body: RequestAgentToolCallRequest): AgentToolCall? {
        val raw = client.post(ApiPaths.appPath("/automation/agent_tool_calls"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<AgentToolCall>() {})
    }

    /** Request an automation execution */
    suspend fun executionsCreate(body: RequestAutomationExecution): AutomationExecutionRequestResponse? {
        val raw = client.post(ApiPaths.appPath("/automation/executions"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<AutomationExecutionRequestResponse>() {})
    }

    /** Get an automation execution */
    suspend fun executionsRetrieve(executionId: String): AutomationExecution? {
        val raw = client.get(ApiPaths.appPath("/automation/executions/${serializePathParameter(executionId, PathParameterSpec("executionId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<AutomationExecution>() {})
    }

    /** Complete an agent tool call */
    suspend fun agentToolCallsComplete(executionId: String, toolCallId: String, body: CompleteAgentToolCallRequest): AgentToolCall? {
        val raw = client.post(ApiPaths.appPath("/automation/executions/${serializePathParameter(executionId, PathParameterSpec("executionId", "simple", false))}/agent_tool_calls/${serializePathParameter(toolCallId, PathParameterSpec("toolCallId", "simple", false))}/complete"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<AgentToolCall>() {})
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
