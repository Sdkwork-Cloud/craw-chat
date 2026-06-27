package com.sdkwork.im.backend.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.im.backend.api.generated.http.HttpClient

class AuditApi(private val client: HttpClient) {

    /** List audit records */
    suspend fun recordsList(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/audit/records"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Record audit anchor */
    suspend fun recordsCreate(): Map<String, Any>? {
        val raw = client.post(ApiPaths.backendPath("/audit/records"), null)
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Export audit bundle */
    suspend fun exportRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/audit/export"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }



}
