package com.sdkwork.im.sdk.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.im.sdk.generated.http.HttpClient

class PresenceApi(private val client: HttpClient) {

    /** Publish current client route presence heartbeat */
    suspend fun heartbeat(body: PresenceHeartbeatRequest): PresenceHeartbeatResponse? {
        val raw = client.post(ApiPaths.imPath("/presence/heartbeat"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PresenceHeartbeatResponse>() {})
    }

    /** Retrieve current principal presence */
    suspend fun meRetrieve(): PresenceMeRetrieveResponse? {
        val raw = client.get(ApiPaths.imPath("/presence/me"))
        return client.convertValue(raw, object : TypeReference<PresenceMeRetrieveResponse>() {})
    }



}
