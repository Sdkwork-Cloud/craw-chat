import Foundation

public class OpsApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Retrieve ops health
    public func healthRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/health"), responseType: [String: Any].self)
    }

    /// Retrieve cluster state
    public func clusterRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/cluster"), responseType: [String: Any].self)
    }

    /// Retrieve projection lag
    public func lagRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/lag"), responseType: [String: Any].self)
    }

    /// Retrieve replay status
    public func replayStatusRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/replay_status"), responseType: [String: Any].self)
    }

    /// Retrieve commercial readiness
    public func commercialReadinessRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/commercial_readiness"), responseType: [String: Any].self)
    }

    /// Inspect runtime directory
    public func runtimeDirRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/runtime_dir"), responseType: [String: Any].self)
    }

    /// List provider bindings
    public func providerBindingsList() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/provider_bindings"), responseType: [String: Any].self)
    }

    /// Retrieve provider binding drift
    public func providerBindingsDriftRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/provider_bindings/drift"), responseType: [String: Any].self)
    }

    /// Retrieve diagnostics
    public func diagnosticsRetrieve() async throws -> [String: Any]? {
        return try await client.get(ApiPaths.backendPath("/ops/diagnostics"), responseType: [String: Any].self)
    }



}
