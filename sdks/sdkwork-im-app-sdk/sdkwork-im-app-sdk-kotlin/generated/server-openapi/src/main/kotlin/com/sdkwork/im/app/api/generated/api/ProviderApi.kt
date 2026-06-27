package com.sdkwork.im.app.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.im.app.api.generated.http.HttpClient

class ProviderApi(private val client: HttpClient) {

    /** Retrieve media provider health */
    suspend fun mediaHealthRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/media/provider_health"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve principal-profile provider health */
    suspend fun principalProfileHealthRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/principal/profiles/provider_health"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }



}
