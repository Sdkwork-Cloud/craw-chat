package com.sdkwork.im.backend.api.generated.api

import com.sdkwork.im.backend.api.generated.http.HttpClient

/**
 * API modules for sdkwork-im-backend-sdk
 */
class Api(private val client: HttpClient) {
    val ops: OpsApi = OpsApi(client)
    val audit: AuditApi = AuditApi(client)
    val automation: AutomationApi = AutomationApi(client)
    val control: ControlApi = ControlApi(client)
    val admin: AdminApi = AdminApi(client)
}
