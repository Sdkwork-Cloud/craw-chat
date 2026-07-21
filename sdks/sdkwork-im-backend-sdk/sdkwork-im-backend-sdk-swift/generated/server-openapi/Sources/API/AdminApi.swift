import Foundation

public class AdminApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// listApiKeyGroups
    public func apiKeyGroupsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/api_key_groups"), query), responseType: SdkWorkListResponse.self)
    }

    /// createApiKeyGroup
    public func apiKeyGroupsCreate(body: [String: Any]) async throws -> ApiKeyGroupsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/api_key_groups"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ApiKeyGroupsCreateResponse201.self)
    }

    /// updateApiKeyGroup
    public func apiKeyGroupsUpdate(groupId: String, body: [String: Any]) async throws -> ApiKeyGroupsUpdateResponse? {
        return try await client.patch(ApiPaths.backendPath("/admin/api_key_groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ApiKeyGroupsUpdateResponse.self)
    }

    /// deleteApiKeyGroup
    public func apiKeyGroupsDelete(groupId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/api_key_groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"))
    }

    /// updateApiKeyGroupStatus
    public func apiKeyGroupsStatus(groupId: String, body: [String: Any]) async throws -> ApiKeyGroupsStatusResponse? {
        return try await client.post(ApiPaths.backendPath("/admin/api_key_groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/status"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ApiKeyGroupsStatusResponse.self)
    }

    /// listApiKeys
    public func apiKeysList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/api_keys"), query), responseType: SdkWorkListResponse.self)
    }

    /// createApiKey
    public func apiKeysCreate(body: [String: Any]) async throws -> ApiKeysCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/api_keys"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ApiKeysCreateResponse201.self)
    }

    /// updateApiKey
    public func apiKeysUpdate(hashedKey: String, body: [String: Any]) async throws -> ApiKeysUpdateResponse? {
        return try await client.put(ApiPaths.backendPath("/admin/api_keys/\(serializePathParameter(hashedKey, PathParameterSpec(name: "hashedKey", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ApiKeysUpdateResponse.self)
    }

    /// deleteApiKey
    public func apiKeysDelete(hashedKey: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/api_keys/\(serializePathParameter(hashedKey, PathParameterSpec(name: "hashedKey", style: "simple", explode: false)))"))
    }

    /// updateApiKeyStatus
    public func apiKeysStatus(hashedKey: String, body: [String: Any]) async throws -> ApiKeysStatusResponse? {
        return try await client.post(ApiPaths.backendPath("/admin/api_keys/\(serializePathParameter(hashedKey, PathParameterSpec(name: "hashedKey", style: "simple", explode: false)))/status"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ApiKeysStatusResponse.self)
    }

    /// listBillingEvents
    public func billingEventsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/billing/events"), query), responseType: SdkWorkListResponse.self)
    }

    /// getBillingEventSummary
    public func billingEventsSummaryRetrieve() async throws -> BillingEventsSummaryRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/billing/events/summary"), responseType: BillingEventsSummaryRetrieveResponse.self)
    }

    /// getBillingSummary
    public func billingSummaryRetrieve() async throws -> BillingSummaryRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/billing/summary"), responseType: BillingSummaryRetrieveResponse.self)
    }

    /// listChannelModels
    public func channelModelsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/channel_models"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveChannelModel
    public func channelModelsCreate(body: [String: Any]) async throws -> ChannelModelsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/channel_models"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ChannelModelsCreateResponse201.self)
    }

    /// deleteChannelModel
    public func channelModelsDelete(channelId: String, modelId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/channel_models/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/models/\(serializePathParameter(modelId, PathParameterSpec(name: "modelId", style: "simple", explode: false)))"))
    }

    /// listChannels
    public func channelsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/channels"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveChannel
    public func channelsCreate(body: [String: Any]) async throws -> ChannelsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/channels"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ChannelsCreateResponse201.self)
    }

    /// deleteChannel
    public func channelsDelete(channelId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"))
    }

    /// listCredentials
    public func credentialsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/credentials"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveCredential
    public func credentialsCreate(body: [String: Any]) async throws -> CredentialsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/credentials"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CredentialsCreateResponse201.self)
    }

    /// deleteCredential
    public func credentialsProvidersKeysDelete(tenantId: String, providerId: String, keyReference: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/credentials/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))/providers/\(serializePathParameter(providerId, PathParameterSpec(name: "providerId", style: "simple", explode: false)))/keys/\(serializePathParameter(keyReference, PathParameterSpec(name: "keyReference", style: "simple", explode: false)))"))
    }

    /// reloadExtensionRuntimes
    public func extensionsRuntimeReloadsCreate(body: [String: Any]) async throws -> ExtensionsRuntimeReloadsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/extensions/runtime_reloads"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ExtensionsRuntimeReloadsCreateResponse201.self)
    }

    /// listRuntimeStatuses
    public func extensionsRuntimeStatusesList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/extensions/runtime_statuses"), query), responseType: SdkWorkListResponse.self)
    }

    /// listRateLimitPolicies
    public func gatewayRateLimitPoliciesList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), query), responseType: SdkWorkListResponse.self)
    }

    /// createRateLimitPolicy
    public func gatewayRateLimitPoliciesCreate(body: [String: Any]) async throws -> GatewayRateLimitPoliciesCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: GatewayRateLimitPoliciesCreateResponse201.self)
    }

    /// listRateLimitWindows
    public func gatewayRateLimitWindowsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/gateway/rate_limit_windows"), query), responseType: SdkWorkListResponse.self)
    }

    /// listMarketingCampaigns
    public func marketingCampaignsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/marketing/campaigns"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveMarketingCampaign
    public func marketingCampaignsCreate(body: [String: Any]) async throws -> MarketingCampaignsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/marketing/campaigns"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MarketingCampaignsCreateResponse201.self)
    }

    /// updateMarketingCampaignStatus
    public func marketingCampaignsStatus(marketingCampaignId: String, body: [String: Any]) async throws -> MarketingCampaignsStatusResponse? {
        return try await client.post(ApiPaths.backendPath("/admin/marketing/campaigns/\(serializePathParameter(marketingCampaignId, PathParameterSpec(name: "marketingCampaignId", style: "simple", explode: false)))/status"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MarketingCampaignsStatusResponse.self)
    }

    /// listModelPrices
    public func modelPricesList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/model_prices"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveModelPrice
    public func modelPricesCreate(body: [String: Any]) async throws -> ModelPricesCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/model_prices"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ModelPricesCreateResponse201.self)
    }

    /// deleteModelPrice
    public func modelPricesProvidersDelete(channelId: String, modelId: String, proxyProviderId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/model_prices/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/models/\(serializePathParameter(modelId, PathParameterSpec(name: "modelId", style: "simple", explode: false)))/providers/\(serializePathParameter(proxyProviderId, PathParameterSpec(name: "proxyProviderId", style: "simple", explode: false)))"))
    }

    /// listModels
    public func modelsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/models"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveModel
    public func modelsCreate(body: [String: Any]) async throws -> ModelsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/models"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ModelsCreateResponse201.self)
    }

    /// deleteModel
    public func modelsProvidersDelete(externalName: String, providerId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/models/\(serializePathParameter(externalName, PathParameterSpec(name: "externalName", style: "simple", explode: false)))/providers/\(serializePathParameter(providerId, PathParameterSpec(name: "providerId", style: "simple", explode: false)))"))
    }

    /// listProviders
    public func providersList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/providers"), query), responseType: SdkWorkListResponse.self)
    }

    /// saveProvider
    public func providersCreate(body: [String: Any]) async throws -> ProvidersCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/providers"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ProvidersCreateResponse201.self)
    }

    /// deleteProvider
    public func providersDelete(providerId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/providers/\(serializePathParameter(providerId, PathParameterSpec(name: "providerId", style: "simple", explode: false)))"))
    }

    /// listRoutingDecisionLogs
    public func routingDecisionLogsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/decision_logs"), query), responseType: SdkWorkListResponse.self)
    }

    /// listProviderHealthSnapshots
    public func routingHealthSnapshotsRetrieve() async throws -> RoutingHealthSnapshotsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/routing/health_snapshots"), responseType: RoutingHealthSnapshotsRetrieveResponse.self)
    }

    /// listRoutingProfiles
    public func routingProfilesList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/profiles"), query), responseType: SdkWorkListResponse.self)
    }

    /// createRoutingProfile
    public func routingProfilesCreate(body: [String: Any]) async throws -> RoutingProfilesCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/routing/profiles"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RoutingProfilesCreateResponse201.self)
    }

    /// listCompiledRoutingSnapshots
    public func routingSnapshotsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/snapshots"), query), responseType: SdkWorkListResponse.self)
    }

    /// listStorageAuditTrail
    public func storageAuditList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/storage/audit"), query), responseType: SdkWorkListResponse.self)
    }

    /// getGlobalStorageConfig
    public func storageConfigRetrieve() async throws -> StorageConfigRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/config"), responseType: StorageConfigRetrieveResponse.self)
    }

    /// saveGlobalStorageConfig
    public func storageConfigCreate(body: [String: Any]) async throws -> StorageConfigCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/config"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StorageConfigCreateResponse201.self)
    }

    /// getTenantStorageConfig
    public func storageConfigTenantsRetrieve(tenantId: String) async throws -> StorageConfigTenantsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/config/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"), responseType: StorageConfigTenantsRetrieveResponse.self)
    }

    /// saveTenantStorageConfig
    public func storageConfigTenantsCreate(tenantId: String, body: [String: Any]) async throws -> StorageConfigTenantsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/config/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StorageConfigTenantsCreateResponse201.self)
    }

    /// deleteTenantStorageConfig
    public func storageConfigTenantsDelete(tenantId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.backendPath("/admin/storage/config/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"))
    }

    /// getTenantEffectiveStorageConfig
    public func storageEffectiveTenantsRetrieve(tenantId: String) async throws -> StorageEffectiveTenantsRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/storage/effective/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"), responseType: StorageEffectiveTenantsRetrieveResponse.self)
    }

    /// listStorageProviders
    public func storageProvidersList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/storage/providers"), query), responseType: SdkWorkListResponse.self)
    }

    /// validateGlobalStorageConfig
    public func storageValidationCreate(body: [String: Any]) async throws -> StorageValidationCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/validate"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StorageValidationCreateResponse201.self)
    }

    /// validateTenantStorageConfig
    public func storageValidationTenantsCreate(tenantId: String, body: [String: Any]) async throws -> StorageValidationTenantsCreateResponse201? {
        return try await client.post(ApiPaths.backendPath("/admin/storage/validate/tenants/\(serializePathParameter(tenantId, PathParameterSpec(name: "tenantId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: StorageValidationTenantsCreateResponse201.self)
    }

    /// listUsageRecords
    public func usageRecordsList(pageSize: Int? = nil, cursor: String? = nil, page: Int? = nil, q: String? = nil) async throws -> SdkWorkListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page", value: page, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/usage/records"), query), responseType: SdkWorkListResponse.self)
    }

    /// getUsageSummary
    public func usageSummaryRetrieve() async throws -> UsageSummaryRetrieveResponse? {
        return try await client.get(ApiPaths.backendPath("/admin/usage/summary"), responseType: UsageSummaryRetrieveResponse.self)
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

    private struct QueryParameterSpec {
        let name: String
        let value: Any?
        let style: String
        let explode: Bool
        let allowReserved: Bool
        let contentType: String?
    }

    private func buildQueryString(_ parameters: [QueryParameterSpec]) -> String {
        var pairs: [String] = []
        for parameter in parameters {
            appendSerializedParameter(&pairs, parameter)
        }
        return pairs.joined(separator: "&")
    }

    private func appendSerializedParameter(_ pairs: inout [String], _ parameter: QueryParameterSpec) {
        guard let value = parameter.value else { return }
        if let contentType = parameter.contentType, !contentType.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            let data = (try? JSONSerialization.data(withJSONObject: value, options: [])) ?? Data(String(describing: value).utf8)
            let json = String(data: data, encoding: .utf8) ?? String(describing: value)
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(json, allowReserved: parameter.allowReserved))")
            return
        }

        let style = parameter.style.isEmpty ? "form" : parameter.style
        if style == "deepObject", let object = value as? [String: Any] {
            appendDeepObjectParameter(&pairs, name: parameter.name, values: object, allowReserved: parameter.allowReserved)
        } else if let array = value as? [Any] {
            appendArrayParameter(&pairs, name: parameter.name, values: array, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else if let object = value as? [String: Any] {
            appendObjectParameter(&pairs, name: parameter.name, values: object, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else {
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(String(describing: value), allowReserved: parameter.allowReserved))")
        }
    }

    private func appendArrayParameter(
        _ pairs: inout [String],
        name: String,
        values: [Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        let serialized = values.map { String(describing: $0) }
        guard !serialized.isEmpty else { return }
        if style == "form" && explode {
            for item in serialized {
                pairs.append("\(urlEncode(name))=\(encodeQueryValue(item, allowReserved: allowReserved))")
            }
            return
        }
        pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
    }

    private func appendObjectParameter(
        _ pairs: inout [String],
        name: String,
        values: [String: Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        var serialized: [String] = []
        for (key, value) in values {
            if style == "form" && explode {
                pairs.append("\(urlEncode(key))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
            } else {
                serialized.append(key)
                serialized.append(String(describing: value))
            }
        }
        if !serialized.isEmpty {
            pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
        }
    }

    private func appendDeepObjectParameter(_ pairs: inout [String], name: String, values: [String: Any], allowReserved: Bool) {
        for (key, value) in values {
            pairs.append("\(urlEncode("\(name)[\(key)]"))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
        }
    }

    private func encodeQueryValue(_ value: String, allowReserved: Bool) -> String {
        var encoded = urlEncode(value)
        if !allowReserved { return encoded }
        [
            "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
            "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
            "%24": "$", "%26": "&", "%27": "'", "%28": "(",
            "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
            "%3B": ";", "%3D": "=",
        ].forEach { encoded = encoded.replacingOccurrences(of: $0.key, with: $0.value) }
        return encoded
    }

    private func urlEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? value
    }

}
