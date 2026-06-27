package com.sdkwork.im.sdk.generated.api

import com.sdkwork.im.sdk.generated.http.HttpClient

/**
 * API modules for sdkwork-im-sdk
 */
class Api(private val client: HttpClient) {
    val presence: PresenceApi = PresenceApi(client)
    val realtime: RealtimeApi = RealtimeApi(client)
    val calls: CallsApi = CallsApi(client)
    val social: SocialApi = SocialApi(client)
    val chat: ChatApi = ChatApi(client)
    val streams: StreamsApi = StreamsApi(client)
    val spaces: SpacesApi = SpacesApi(client)
}
