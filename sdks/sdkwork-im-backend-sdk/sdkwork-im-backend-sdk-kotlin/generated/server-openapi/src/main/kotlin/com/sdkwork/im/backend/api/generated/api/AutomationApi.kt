package com.sdkwork.im.backend.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.im.backend.api.generated.http.HttpClient

class AutomationApi(private val client: HttpClient) {

    /** Retrieve automation governance */
    suspend fun governanceRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/automation/governance"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }



}
