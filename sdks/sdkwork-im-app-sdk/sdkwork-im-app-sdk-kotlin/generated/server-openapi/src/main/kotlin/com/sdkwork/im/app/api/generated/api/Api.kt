package com.sdkwork.im.app.api.generated.api

import com.sdkwork.im.app.api.generated.http.HttpClient

/**
 * API modules for sdkwork-im-app-sdk
 */
class Api(private val client: HttpClient) {
    val automation: AutomationApi = AutomationApi(client)
    val notification: NotificationApi = NotificationApi(client)
    val portal: PortalApi = PortalApi(client)
    val provider: ProviderApi = ProviderApi(client)
}
