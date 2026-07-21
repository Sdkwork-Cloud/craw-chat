import Foundation

public class AutomationApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Retrieve automation governance
    public func governanceRetrieve() async throws -> GovernanceRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/automation/governance"), responseType: GovernanceRetrieveResponse.self)
    }



}
