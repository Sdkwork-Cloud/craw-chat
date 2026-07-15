import Foundation
import SDKworkCommon

public class SdkworkImAppClient {
    private let httpClient: HttpClient
    public let automation: AutomationApi
    public let notifications: NotificationsApi
    public let portal: PortalApi
    public let provider: ProviderApi
    public let chat: ChatApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.automation = AutomationApi(client: httpClient)
        self.notifications = NotificationsApi(client: httpClient)
        self.portal = PortalApi(client: httpClient)
        self.provider = ProviderApi(client: httpClient)
        self.chat = ChatApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.automation = AutomationApi(client: httpClient)
        self.notifications = NotificationsApi(client: httpClient)
        self.portal = PortalApi(client: httpClient)
        self.provider = ProviderApi(client: httpClient)
        self.chat = ChatApi(client: httpClient)
    }
    public func setAuthToken(_ token: String) -> SdkworkImAppClient {
        httpClient.setAuthToken(token)
        return self
    }

    public func setAccessToken(_ token: String) -> SdkworkImAppClient {
        httpClient.setAccessToken(token)
        return self
    }

    public func setHeader(_ key: String, value: String) -> SdkworkImAppClient {
        httpClient.setHeader(key, value: value)
        return self
    }
}

public typealias SdkworkAppClient = SdkworkImAppClient
