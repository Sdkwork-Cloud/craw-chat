package com.sdkwork.im.sdk.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.sdk.generated.http.HttpClient
import com.sdkwork.im.sdk.generated.api.PresenceApi
import com.sdkwork.im.sdk.generated.api.RealtimeApi
import com.sdkwork.im.sdk.generated.api.CallsApi
import com.sdkwork.im.sdk.generated.api.SocialApi
import com.sdkwork.im.sdk.generated.api.ChatApi
import com.sdkwork.im.sdk.generated.api.StreamsApi
import com.sdkwork.im.sdk.generated.api.SpacesApi

open class SdkworkImClient {
    private val httpClient: HttpClient

    lateinit var presence: PresenceApi
    lateinit var realtime: RealtimeApi
    lateinit var calls: CallsApi
    lateinit var social: SocialApi
    lateinit var chat: ChatApi
    lateinit var streams: StreamsApi
    lateinit var spaces: SpacesApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        calls = CallsApi(httpClient)
        social = SocialApi(httpClient)
        chat = ChatApi(httpClient)
        streams = StreamsApi(httpClient)
        spaces = SpacesApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        calls = CallsApi(httpClient)
        social = SocialApi(httpClient)
        chat = ChatApi(httpClient)
        streams = StreamsApi(httpClient)
        spaces = SpacesApi(httpClient)
    }
    fun setAuthToken(token: String): SdkworkImClient {
        httpClient.setAuthToken(token)
        return this
    }

    fun setAccessToken(token: String): SdkworkImClient {
        httpClient.setAccessToken(token)
        return this
    }

    fun setHeader(key: String, value: String): SdkworkImClient {
        httpClient.setHeader(key, value)
        return this
    }
}
