import Foundation

public class PortalApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Read the tenant portal access snapshot
    public func accessRetrieve() async throws -> AccessRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/access"), responseType: AccessRetrieveResponse.self)
    }

    /// Read the tenant automation snapshot
    public func automationRetrieve() async throws -> AutomationRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/automation"), responseType: AutomationRetrieveResponse.self)
    }

    /// Read the tenant conversations snapshot
    public func conversationSnapshotRetrieve() async throws -> ConversationSnapshotRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/conversations"), responseType: ConversationSnapshotRetrieveResponse.self)
    }

    /// Read the tenant dashboard snapshot
    public func dashboardRetrieve() async throws -> DashboardRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/dashboard"), responseType: DashboardRetrieveResponse.self)
    }

    /// Read the tenant governance snapshot
    public func governanceRetrieve() async throws -> GovernanceRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/governance"), responseType: GovernanceRetrieveResponse.self)
    }

    /// Read the tenant portal home snapshot
    public func homeRetrieve() async throws -> HomeRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/home"), responseType: HomeRetrieveResponse.self)
    }

    /// Read the tenant media snapshot
    public func mediaRetrieve() async throws -> MediaRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/media"), responseType: MediaRetrieveResponse.self)
    }

    /// Read the tenant realtime snapshot
    public func realtimeRetrieve() async throws -> RealtimeRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/realtime"), responseType: RealtimeRetrieveResponse.self)
    }

    /// Read the current tenant workspace snapshot
    public func workspaceRetrieve() async throws -> WorkspaceRetrieveResponse? {
        return try await client.get(ApiPaths.appPath("/portal/workspace"), responseType: WorkspaceRetrieveResponse.self)
    }



}
