package com.sdkwork.im.app.api.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.app.api.generated.http.HttpClient
import com.sdkwork.im.app.api.generated.api.AutomationApi
import com.sdkwork.im.app.api.generated.api.NotificationApi
import com.sdkwork.im.app.api.generated.api.PortalApi
import com.sdkwork.im.app.api.generated.api.ProviderApi

open class SdkworkImAppClient {
    private val httpClient: HttpClient

    lateinit var automation: AutomationApi
    lateinit var notification: NotificationApi
    lateinit var portal: PortalApi
    lateinit var provider: ProviderApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        automation = AutomationApi(httpClient)
        notification = NotificationApi(httpClient)
        portal = PortalApi(httpClient)
        provider = ProviderApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        automation = AutomationApi(httpClient)
        notification = NotificationApi(httpClient)
        portal = PortalApi(httpClient)
        provider = ProviderApi(httpClient)
    }
    fun setAuthToken(token: String): SdkworkImAppClient {
        httpClient.setAuthToken(token)
        return this
    }

    fun setAccessToken(token: String): SdkworkImAppClient {
        httpClient.setAccessToken(token)
        return this
    }

    fun setHeader(key: String, value: String): SdkworkImAppClient {
        httpClient.setHeader(key, value)
        return this
    }
}
