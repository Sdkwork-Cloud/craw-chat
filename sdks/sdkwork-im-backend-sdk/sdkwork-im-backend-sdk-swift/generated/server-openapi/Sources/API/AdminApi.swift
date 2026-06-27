import Foundation

public class AdminApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// listApiKeyGroups
    public func apiKeyGroupsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/api_key_groups"))
    }

    /// createApiKeyGroup
    public func apiKeyGroupsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/api_key_groups"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// updateApiKeyGroup
    public func apiKeyGroupsUpdate(groupId: String, body: [String: Any]) async throws -> Any? {
        return try await client.patch(ApiPaths.backendPath("/admin/api_key_groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteApiKeyGroup
    public func apiKeyGroupsDelete(groupId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/api_key_groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"))
    }

    /// updateApiKeyGroupStatus
    public func apiKeyGroupsStatus(groupId: String, body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/api_key_groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/status"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listApiKeys
    public func apiKeysList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/api_keys"))
    }

    /// createApiKey
    public func apiKeysCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/api_keys"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// updateApiKey
    public func apiKeysUpdate(hashedKey: String, body: [String: Any]) async throws -> Any? {
        return try await client.put(ApiPaths.backendPath("/admin/api_keys/\(serializePathParameter(hashedKey, PathParameterSpec(name: "hashedKey", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteApiKey
    public func apiKeysDelete(hashedKey: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/api_keys/\(serializePathParameter(hashedKey, PathParameterSpec(name: "hashedKey", style: "simple", explode: false)))"))
    }

    /// updateApiKeyStatus
    public func apiKeysStatus(hashedKey: String, body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/api_keys/\(serializePathParameter(hashedKey, PathParameterSpec(name: "hashedKey", style: "simple", explode: false)))/status"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listBillingEvents
    public func billingEventsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/billing/events"))
    }

    /// getBillingEventSummary
    public func billingEventsSummaryRetrieve() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/billing/events/summary"))
    }

    /// getBillingSummary
    public func billingSummaryRetrieve() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/billing/summary"))
    }

    /// listChannelModels
    public func channelModelsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/channel_models"))
    }

    /// saveChannelModel
    public func channelModelsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/channel_models"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteChannelModel
    public func channelModelsDelete(channelId: String, modelId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/channel_models/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/models/\(serializePathParameter(modelId, PathParameterSpec(name: "modelId", style: "simple", explode: false)))"))
    }

    /// listChannels
    public func channelsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/channels"))
    }

    /// saveChannel
    public func channelsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/channels"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteChannel
    public func channelsDelete(channelId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"))
    }

    /// listCredentials
    public func credentialsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/credentials"))
    }

    /// saveCredential
    public func credentialsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/credentials"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteCredential
    public func credentialsProvidersKeysDelete(tenantId: String, providerId: String, keyReference: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/credentials/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))/providers/\(serializePathParameter(providerId, PathParameterSpec(name: "providerId", style: "simple", explode: false)))/keys/\(serializePathParameter(keyReference, PathParameterSpec(name: "keyReference", style: "simple", explode: false)))"))
    }

    /// reloadExtensionRuntimes
    public func extensionsRuntimeReloadsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/extensions/runtime_reloads"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listRuntimeStatuses
    public func extensionsRuntimeStatusesList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/extensions/runtime_statuses"))
    }

    /// listRateLimitPolicies
    public func gatewayRateLimitPoliciesList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"))
    }

    /// createRateLimitPolicy
    public func gatewayRateLimitPoliciesCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listRateLimitWindows
    public func gatewayRateLimitWindowsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/gateway/rate_limit_windows"))
    }

    /// listMarketingCampaigns
    public func marketingCampaignsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/marketing/campaigns"))
    }

    /// saveMarketingCampaign
    public func marketingCampaignsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/marketing/campaigns"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// updateMarketingCampaignStatus
    public func marketingCampaignsStatus(marketingCampaignId: String, body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/marketing/campaigns/\(serializePathParameter(marketingCampaignId, PathParameterSpec(name: "marketingCampaignId", style: "simple", explode: false)))/status"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listModelPrices
    public func modelPricesList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/model_prices"))
    }

    /// saveModelPrice
    public func modelPricesCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/model_prices"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteModelPrice
    public func modelPricesProvidersDelete(channelId: String, modelId: String, proxyProviderId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/model_prices/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/models/\(serializePathParameter(modelId, PathParameterSpec(name: "modelId", style: "simple", explode: false)))/providers/\(serializePathParameter(proxyProviderId, PathParameterSpec(name: "proxyProviderId", style: "simple", explode: false)))"))
    }

    /// listModels
    public func modelsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/models"))
    }

    /// saveModel
    public func modelsCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/models"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteModel
    public func modelsProvidersDelete(externalName: String, providerId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/models/\(serializePathParameter(externalName, PathParameterSpec(name: "externalName", style: "simple", explode: false)))/providers/\(serializePathParameter(providerId, PathParameterSpec(name: "providerId", style: "simple", explode: false)))"))
    }

    /// listProviders
    public func providersList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/providers"))
    }

    /// saveProvider
    public func providersCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/providers"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteProvider
    public func providersDelete(providerId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/providers/\(serializePathParameter(providerId, PathParameterSpec(name: "providerId", style: "simple", explode: false)))"))
    }

    /// listRoutingDecisionLogs
    public func routingDecisionLogsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/routing/decision_logs"))
    }

    /// listProviderHealthSnapshots
    public func routingHealthSnapshotsRetrieve() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/routing/health_snapshots"))
    }

    /// listRoutingProfiles
    public func routingProfilesList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/routing/profiles"))
    }

    /// createRoutingProfile
    public func routingProfilesCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/routing/profiles"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listCompiledRoutingSnapshots
    public func routingSnapshotsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/routing/snapshots"))
    }

    /// listStorageAuditTrail
    public func storageAuditList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/audit"))
    }

    /// getGlobalStorageConfig
    public func storageConfigRetrieve() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/config"))
    }

    /// saveGlobalStorageConfig
    public func storageConfigCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/config"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// getTenantStorageConfig
    public func storageConfigTenantsRetrieve(tenantId: String) async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/config/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"))
    }

    /// saveTenantStorageConfig
    public func storageConfigTenantsCreate(tenantId: String, body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/config/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// deleteTenantStorageConfig
    public func storageConfigTenantsDelete(tenantId: String) async throws -> Any? {
        return try await client.delete(ApiPaths.backendPath("/admin/storage/config/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"))
    }

    /// getTenantEffectiveStorageConfig
    public func storageEffectiveTenantsRetrieve(tenantId: String) async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/effective/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"))
    }

    /// listStorageProviders
    public func storageProvidersList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/providers"))
    }

    /// validateGlobalStorageConfig
    public func storageValidationCreate(body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/validate"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// validateTenantStorageConfig
    public func storageValidationTenantsCreate(tenantId: String, body: [String: Any]) async throws -> Any? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/validate/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json")
    }

    /// listUsageRecords
    public func usageRecordsList() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/usage/records"))
    }

    /// getUsageSummary
    public func usageSummaryRetrieve() async throws -> Any? {
        return try await client.get(ApiPaths.backendPath("/admin/usage/summary"))
    }

    private struct PathParameterSpec {
        let name: String
        let style: String
        let explode: Bool
    }

    private func serializePathParameter(_ value: Any?, _ spec: PathParameterSpec) -> String {
        guard let value else { return "" }
        let style = spec.style.isEmpty ? "simple" : spec.style
        if let array = value as? [Any] {
            return serializePathArray(spec.name, array, style, spec.explode)
        }
        if let object = value as? [String: Any] {
            return serializePathObject(spec.name, object, style, spec.explode)
        }
        return pathPrimitivePrefix(spec.name, style) + pathEncode(String(describing: value))
    }

    private func serializePathArray(_ name: String, _ values: [Any], _ style: String, _ explode: Bool) -> String {
        let serialized = values.map { pathEncode(String(describing: $0)) }
        if serialized.isEmpty { return pathPrefix(name, style) }
        if style == "matrix" {
            if explode {
                return serialized.map { ";\(name)=\($0)" }.joined()
            }
            return ";\(name)=" + serialized.joined(separator: ",")
        }
        let separator = explode ? "." : ","
        return pathPrefix(name, style) + serialized.joined(separator: separator)
    }

    private func serializePathObject(_ name: String, _ values: [String: Any], _ style: String, _ explode: Bool) -> String {
        var entries: [String] = []
        var exploded: [String] = []
        for (key, value) in values {
            let escapedKey = pathEncode(key)
            let escapedValue = pathEncode(String(describing: value))
            if explode {
                if style == "matrix" {
                    exploded.append(";\(escapedKey)=\(escapedValue)")
                } else {
                    exploded.append("\(escapedKey)=\(escapedValue)")
                }
            } else {
                entries.append(escapedKey)
                entries.append(escapedValue)
            }
        }
        if style == "matrix" {
            if explode {
                return exploded.joined()
            }
            return ";\(name)=" + entries.joined(separator: ",")
        }
        if explode {
            let separator = style == "label" ? "." : ","
            return pathPrefix(name, style) + exploded.joined(separator: separator)
        }
        return pathPrefix(name, style) + entries.joined(separator: ",")
    }

    private func pathPrefix(_ name: String, _ style: String) -> String {
        if style == "label" { return "." }
        if style == "matrix" { return ";\(name)" }
        return ""
    }

    private func pathPrimitivePrefix(_ name: String, _ style: String) -> String {
        style == "matrix" ? ";\(name)=" : pathPrefix(name, style)
    }

    private func pathEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? value
    }


}
