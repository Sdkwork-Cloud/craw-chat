import Foundation

public class AuditApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// List audit records
    public func recordsList() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/audit/records"), responseType: [String: Any].self)
    }

    /// Record audit anchor
    public func recordsCreate() async throws -> [String: Any]? {
        return try await client.post(ApiPaths.backendPath("/audit/records"), body: nil, responseType: [String: Any].self)
    }

    /// Export audit bundle
    public func exportRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/audit/export"), responseType: [String: Any].self)
    }



}
