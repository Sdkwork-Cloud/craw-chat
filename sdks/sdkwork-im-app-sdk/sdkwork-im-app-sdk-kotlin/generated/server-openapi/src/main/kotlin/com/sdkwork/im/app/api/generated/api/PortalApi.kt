package com.sdkwork.im.app.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.im.app.api.generated.http.HttpClient

class PortalApi(private val client: HttpClient) {

    /** Read the tenant portal sign-in snapshot */
    suspend fun accessRetrieve(): AccessRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/access"))
        return client.convertValue(raw, object : TypeReference<AccessRetrieveResponse>() {})
    }

    /** Read the tenant automation snapshot */
    suspend fun automationRetrieve(): AutomationRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/automation"))
        return client.convertValue(raw, object : TypeReference<AutomationRetrieveResponse>() {})
    }

    /** Read the tenant conversations snapshot */
    suspend fun conversationSnapshotRetrieve(): ConversationSnapshotRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/conversations"))
        return client.convertValue(raw, object : TypeReference<ConversationSnapshotRetrieveResponse>() {})
    }

    /** Read the tenant dashboard snapshot */
    suspend fun dashboardRetrieve(): DashboardRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/dashboard"))
        return client.convertValue(raw, object : TypeReference<DashboardRetrieveResponse>() {})
    }

    /** Read the tenant governance snapshot */
    suspend fun governanceRetrieve(): GovernanceRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/governance"))
        return client.convertValue(raw, object : TypeReference<GovernanceRetrieveResponse>() {})
    }

    /** Read the tenant portal home snapshot */
    suspend fun homeRetrieve(): HomeRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/home"))
        return client.convertValue(raw, object : TypeReference<HomeRetrieveResponse>() {})
    }

    /** Read the tenant media snapshot */
    suspend fun mediaRetrieve(): MediaRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/media"))
        return client.convertValue(raw, object : TypeReference<MediaRetrieveResponse>() {})
    }

    /** Read the tenant realtime snapshot */
    suspend fun realtimeRetrieve(): RealtimeRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/realtime"))
        return client.convertValue(raw, object : TypeReference<RealtimeRetrieveResponse>() {})
    }

    /** Read the current tenant workspace snapshot */
    suspend fun workspaceRetrieve(): WorkspaceRetrieveResponse? {
        val raw = client.get(ApiPaths.appPath("/portal/workspace"))
        return client.convertValue(raw, object : TypeReference<WorkspaceRetrieveResponse>() {})
    }



}
