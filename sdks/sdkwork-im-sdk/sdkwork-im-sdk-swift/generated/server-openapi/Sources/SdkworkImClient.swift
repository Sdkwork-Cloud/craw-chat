import Foundation
import SDKworkCommon

public class SdkworkImClient {
    private let httpClient: HttpClient
    public let presence: PresenceApi
    public let realtime: RealtimeApi
    public let calls: CallsApi
    public let social: SocialApi
    public let chat: ChatApi
    public let streams: StreamsApi
    public let spaces: SpacesApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.presence = PresenceApi(client: httpClient)
        self.realtime = RealtimeApi(client: httpClient)
        self.calls = CallsApi(client: httpClient)
        self.social = SocialApi(client: httpClient)
        self.chat = ChatApi(client: httpClient)
        self.streams = StreamsApi(client: httpClient)
        self.spaces = SpacesApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.presence = PresenceApi(client: httpClient)
        self.realtime = RealtimeApi(client: httpClient)
        self.calls = CallsApi(client: httpClient)
        self.social = SocialApi(client: httpClient)
        self.chat = ChatApi(client: httpClient)
        self.streams = StreamsApi(client: httpClient)
        self.spaces = SpacesApi(client: httpClient)
    }
    public func setAuthToken(_ token: String) -> SdkworkImClient {
        httpClient.setAuthToken(token)
        return self
    }

    public func setAccessToken(_ token: String) -> SdkworkImClient {
        httpClient.setAccessToken(token)
        return self
    }

    public func setHeader(_ key: String, value: String) -> SdkworkImClient {
        httpClient.setHeader(key, value: value)
        return self
    }
}
