import Foundation
import SDKworkCommon

public class SdkworkImBackendClient {
    private let httpClient: HttpClient
    public let ops: OpsApi
    public let audit: AuditApi
    public let automation: AutomationApi
    public let control: ControlApi
    public let admin: AdminApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.ops = OpsApi(client: httpClient)
        self.audit = AuditApi(client: httpClient)
        self.automation = AutomationApi(client: httpClient)
        self.control = ControlApi(client: httpClient)
        self.admin = AdminApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.ops = OpsApi(client: httpClient)
        self.audit = AuditApi(client: httpClient)
        self.automation = AutomationApi(client: httpClient)
        self.control = ControlApi(client: httpClient)
        self.admin = AdminApi(client: httpClient)
    }
    public func setAuthToken(_ token: String) -> SdkworkImBackendClient {
        httpClient.setAuthToken(token)
        return self
    }

    public func setAccessToken(_ token: String) -> SdkworkImBackendClient {
        httpClient.setAccessToken(token)
        return self
    }

    public func setHeader(_ key: String, value: String) -> SdkworkImBackendClient {
        httpClient.setHeader(key, value: value)
        return self
    }
}

public typealias SdkworkBackendClient = SdkworkImBackendClient
