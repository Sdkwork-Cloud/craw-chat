import Foundation

public class PresenceApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Publish current client route presence heartbeat
    public func heartbeat(body: PresenceHeartbeatRequest) async throws -> PresenceHeartbeatResponse? {
        return try await client.post(ApiPaths.imPath("/presence/heartbeat"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PresenceHeartbeatResponse.self)
    }

    /// Retrieve current principal presence
    public func meRetrieve() async throws -> PresenceMeRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/presence/me"), responseType: PresenceMeRetrieveResponse.self)
    }



}
