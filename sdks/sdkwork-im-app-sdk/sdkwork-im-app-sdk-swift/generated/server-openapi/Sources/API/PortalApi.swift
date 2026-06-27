import Foundation

public class PortalApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Read the tenant portal sign-in snapshot
    public func accessRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/access"), responseType: [String: Any].self)
    }

    /// Read the tenant automation snapshot
    public func automationRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/automation"), responseType: [String: Any].self)
    }

    /// Read the tenant conversations snapshot
    public func conversationSnapshotRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/conversations"), responseType: [String: Any].self)
    }

    /// Read the tenant dashboard snapshot
    public func dashboardRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/dashboard"), responseType: [String: Any].self)
    }

    /// Read the tenant governance snapshot
    public func governanceRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/governance"), responseType: [String: Any].self)
    }

    /// Read the tenant portal home snapshot
    public func homeRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/home"), responseType: [String: Any].self)
    }

    /// Read the tenant media snapshot
    public func mediaRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/media"), responseType: [String: Any].self)
    }

    /// Read the tenant realtime snapshot
    public func realtimeRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/portal/realtime"), responseType: [String: Any].self)
    }

    /// Read the current tenant workspace snapshot
    public func workspaceRetrieve() async throws -> PortalWorkspaceView? {
        return try await client.get(ApiPaths.appPath("/portal/workspace"), responseType: PortalWorkspaceView.self)
    }



}
