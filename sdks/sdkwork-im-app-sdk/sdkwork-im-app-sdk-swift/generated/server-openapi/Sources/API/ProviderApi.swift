import Foundation

public class ProviderApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Retrieve media provider health
    public func mediaHealthRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/media/provider_health"), responseType: [String: Any].self)
    }

    /// Retrieve principal-profile provider health
    public func principalProfileHealthRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.appPath("/principal/profiles/provider_health"), responseType: [String: Any].self)
    }



}
