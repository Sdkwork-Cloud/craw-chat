package com.sdkwork.im.app.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.im.app.api.generated.http.HttpClient

class PortalApi(private val client: HttpClient) {

    /** Read the tenant portal sign-in snapshot */
    suspend fun accessRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/access"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant automation snapshot */
    suspend fun automationRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/automation"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant conversations snapshot */
    suspend fun conversationSnapshotRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/conversations"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant dashboard snapshot */
    suspend fun dashboardRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/dashboard"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant governance snapshot */
    suspend fun governanceRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/governance"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant portal home snapshot */
    suspend fun homeRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/home"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant media snapshot */
    suspend fun mediaRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/media"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant realtime snapshot */
    suspend fun realtimeRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/portal/realtime"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the current tenant workspace snapshot */
    suspend fun workspaceRetrieve(): PortalWorkspaceView? {
        val raw = client.get(ApiPaths.appPath("/portal/workspace"))
        return client.convertValue(raw, object : TypeReference<PortalWorkspaceView>() {})
    }



}
