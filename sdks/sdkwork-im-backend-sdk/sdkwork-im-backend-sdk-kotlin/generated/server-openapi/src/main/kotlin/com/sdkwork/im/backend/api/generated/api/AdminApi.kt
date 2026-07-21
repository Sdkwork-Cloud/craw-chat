package com.sdkwork.im.backend.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.backend.api.generated.*
import com.sdkwork.im.backend.api.generated.http.HttpClient

class AdminApi(private val client: HttpClient) {

    /** listApiKeyGroups */
    suspend fun apiKeyGroupsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/api_key_groups"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** createApiKeyGroup */
    suspend fun apiKeyGroupsCreate(body: Map<String, Any>): ApiKeyGroupsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/api_key_groups"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ApiKeyGroupsCreateResponse201>() {})
    }

    /** updateApiKeyGroup */
    suspend fun apiKeyGroupsUpdate(groupId: String, body: Map<String, Any>): ApiKeyGroupsUpdateResponse? {
        val raw = client.patch(ApiPaths.backendPath("/admin/api_key_groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ApiKeyGroupsUpdateResponse>() {})
    }

    /** deleteApiKeyGroup */
    suspend fun apiKeyGroupsDelete(groupId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/api_key_groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}"))
    }

    /** updateApiKeyGroupStatus */
    suspend fun apiKeyGroupsStatus(groupId: String, body: Map<String, Any>): ApiKeyGroupsStatusResponse? {
        val raw = client.post(ApiPaths.backendPath("/admin/api_key_groups/${serializePathParameter(groupId, PathParameterSpec("groupId", "simple", false))}/status"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ApiKeyGroupsStatusResponse>() {})
    }

    /** listApiKeys */
    suspend fun apiKeysList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/api_keys"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** createApiKey */
    suspend fun apiKeysCreate(body: Map<String, Any>): ApiKeysCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/api_keys"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ApiKeysCreateResponse201>() {})
    }

    /** updateApiKey */
    suspend fun apiKeysUpdate(hashedKey: String, body: Map<String, Any>): ApiKeysUpdateResponse? {
        val raw = client.put(ApiPaths.backendPath("/admin/api_keys/${serializePathParameter(hashedKey, PathParameterSpec("hashedKey", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ApiKeysUpdateResponse>() {})
    }

    /** deleteApiKey */
    suspend fun apiKeysDelete(hashedKey: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/api_keys/${serializePathParameter(hashedKey, PathParameterSpec("hashedKey", "simple", false))}"))
    }

    /** updateApiKeyStatus */
    suspend fun apiKeysStatus(hashedKey: String, body: Map<String, Any>): ApiKeysStatusResponse? {
        val raw = client.post(ApiPaths.backendPath("/admin/api_keys/${serializePathParameter(hashedKey, PathParameterSpec("hashedKey", "simple", false))}/status"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ApiKeysStatusResponse>() {})
    }

    /** listBillingEvents */
    suspend fun billingEventsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/billing/events"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** getBillingEventSummary */
    suspend fun billingEventsSummaryRetrieve(): BillingEventsSummaryRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/billing/events/summary"))
        return client.convertValue(raw, object : TypeReference<BillingEventsSummaryRetrieveResponse>() {})
    }

    /** getBillingSummary */
    suspend fun billingSummaryRetrieve(): BillingSummaryRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/billing/summary"))
        return client.convertValue(raw, object : TypeReference<BillingSummaryRetrieveResponse>() {})
    }

    /** listChannelModels */
    suspend fun channelModelsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/channel_models"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveChannelModel */
    suspend fun channelModelsCreate(body: Map<String, Any>): ChannelModelsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/channel_models"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ChannelModelsCreateResponse201>() {})
    }

    /** deleteChannelModel */
    suspend fun channelModelsDelete(channelId: String, modelId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/channel_models/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/models/${serializePathParameter(modelId, PathParameterSpec("modelId", "simple", false))}"))
    }

    /** listChannels */
    suspend fun channelsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/channels"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveChannel */
    suspend fun channelsCreate(body: Map<String, Any>): ChannelsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/channels"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ChannelsCreateResponse201>() {})
    }

    /** deleteChannel */
    suspend fun channelsDelete(channelId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/channels/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}"))
    }

    /** listCredentials */
    suspend fun credentialsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/credentials"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveCredential */
    suspend fun credentialsCreate(body: Map<String, Any>): CredentialsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/credentials"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<CredentialsCreateResponse201>() {})
    }

    /** deleteCredential */
    suspend fun credentialsProvidersKeysDelete(tenantId: String, providerId: String, keyReference: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/credentials/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}/providers/${serializePathParameter(providerId, PathParameterSpec("providerId", "simple", false))}/keys/${serializePathParameter(keyReference, PathParameterSpec("keyReference", "simple", false))}"))
    }

    /** reloadExtensionRuntimes */
    suspend fun extensionsRuntimeReloadsCreate(body: Map<String, Any>): ExtensionsRuntimeReloadsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/extensions/runtime_reloads"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ExtensionsRuntimeReloadsCreateResponse201>() {})
    }

    /** listRuntimeStatuses */
    suspend fun extensionsRuntimeStatusesList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/extensions/runtime_statuses"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** listRateLimitPolicies */
    suspend fun gatewayRateLimitPoliciesList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** createRateLimitPolicy */
    suspend fun gatewayRateLimitPoliciesCreate(body: Map<String, Any>): GatewayRateLimitPoliciesCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<GatewayRateLimitPoliciesCreateResponse201>() {})
    }

    /** listRateLimitWindows */
    suspend fun gatewayRateLimitWindowsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/gateway/rate_limit_windows"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** listMarketingCampaigns */
    suspend fun marketingCampaignsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/marketing/campaigns"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveMarketingCampaign */
    suspend fun marketingCampaignsCreate(body: Map<String, Any>): MarketingCampaignsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/marketing/campaigns"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MarketingCampaignsCreateResponse201>() {})
    }

    /** updateMarketingCampaignStatus */
    suspend fun marketingCampaignsStatus(marketingCampaignId: String, body: Map<String, Any>): MarketingCampaignsStatusResponse? {
        val raw = client.post(ApiPaths.backendPath("/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, PathParameterSpec("marketingCampaignId", "simple", false))}/status"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<MarketingCampaignsStatusResponse>() {})
    }

    /** listModelPrices */
    suspend fun modelPricesList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/model_prices"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveModelPrice */
    suspend fun modelPricesCreate(body: Map<String, Any>): ModelPricesCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/model_prices"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ModelPricesCreateResponse201>() {})
    }

    /** deleteModelPrice */
    suspend fun modelPricesProvidersDelete(channelId: String, modelId: String, proxyProviderId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/model_prices/${serializePathParameter(channelId, PathParameterSpec("channelId", "simple", false))}/models/${serializePathParameter(modelId, PathParameterSpec("modelId", "simple", false))}/providers/${serializePathParameter(proxyProviderId, PathParameterSpec("proxyProviderId", "simple", false))}"))
    }

    /** listModels */
    suspend fun modelsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/models"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveModel */
    suspend fun modelsCreate(body: Map<String, Any>): ModelsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/models"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ModelsCreateResponse201>() {})
    }

    /** deleteModel */
    suspend fun modelsProvidersDelete(externalName: String, providerId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/models/${serializePathParameter(externalName, PathParameterSpec("externalName", "simple", false))}/providers/${serializePathParameter(providerId, PathParameterSpec("providerId", "simple", false))}"))
    }

    /** listProviders */
    suspend fun providersList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/providers"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** saveProvider */
    suspend fun providersCreate(body: Map<String, Any>): ProvidersCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/providers"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<ProvidersCreateResponse201>() {})
    }

    /** deleteProvider */
    suspend fun providersDelete(providerId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/providers/${serializePathParameter(providerId, PathParameterSpec("providerId", "simple", false))}"))
    }

    /** listRoutingDecisionLogs */
    suspend fun routingDecisionLogsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/decision_logs"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** listProviderHealthSnapshots */
    suspend fun routingHealthSnapshotsRetrieve(): RoutingHealthSnapshotsRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/routing/health_snapshots"))
        return client.convertValue(raw, object : TypeReference<RoutingHealthSnapshotsRetrieveResponse>() {})
    }

    /** listRoutingProfiles */
    suspend fun routingProfilesList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/profiles"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** createRoutingProfile */
    suspend fun routingProfilesCreate(body: Map<String, Any>): RoutingProfilesCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/routing/profiles"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<RoutingProfilesCreateResponse201>() {})
    }

    /** listCompiledRoutingSnapshots */
    suspend fun routingSnapshotsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/snapshots"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** listStorageAuditTrail */
    suspend fun storageAuditList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/storage/audit"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** getGlobalStorageConfig */
    suspend fun storageConfigRetrieve(): StorageConfigRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/storage/config"))
        return client.convertValue(raw, object : TypeReference<StorageConfigRetrieveResponse>() {})
    }

    /** saveGlobalStorageConfig */
    suspend fun storageConfigCreate(body: Map<String, Any>): StorageConfigCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/storage/config"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StorageConfigCreateResponse201>() {})
    }

    /** getTenantStorageConfig */
    suspend fun storageConfigTenantsRetrieve(tenantId: String): StorageConfigTenantsRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/storage/config/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<StorageConfigTenantsRetrieveResponse>() {})
    }

    /** saveTenantStorageConfig */
    suspend fun storageConfigTenantsCreate(tenantId: String, body: Map<String, Any>): StorageConfigTenantsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/storage/config/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StorageConfigTenantsCreateResponse201>() {})
    }

    /** deleteTenantStorageConfig */
    suspend fun storageConfigTenantsDelete(tenantId: String): Unit {
        client.delete(ApiPaths.backendPath("/admin/storage/config/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"))
    }

    /** getTenantEffectiveStorageConfig */
    suspend fun storageEffectiveTenantsRetrieve(tenantId: String): StorageEffectiveTenantsRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/storage/effective/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"))
        return client.convertValue(raw, object : TypeReference<StorageEffectiveTenantsRetrieveResponse>() {})
    }

    /** listStorageProviders */
    suspend fun storageProvidersList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/storage/providers"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** validateGlobalStorageConfig */
    suspend fun storageValidationCreate(body: Map<String, Any>): StorageValidationCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/storage/validate"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StorageValidationCreateResponse201>() {})
    }

    /** validateTenantStorageConfig */
    suspend fun storageValidationTenantsCreate(tenantId: String, body: Map<String, Any>): StorageValidationTenantsCreateResponse201? {
        val raw = client.post(ApiPaths.backendPath("/admin/storage/validate/tenants/${serializePathParameter(tenantId, PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StorageValidationTenantsCreateResponse201>() {})
    }

    /** listUsageRecords */
    suspend fun usageRecordsList(pageSize: Int? = null, cursor: String? = null, page: Int? = null, q: String? = null): SdkWorkListResponse? {
        val query = buildQueryString(listOf(
            QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            QueryParameterSpec("cursor", cursor, "form", true, false, null),
            QueryParameterSpec("page", page, "form", true, false, null),
            QueryParameterSpec("q", q, "form", true, false, null)
        ))
        val raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/usage/records"), query))
        return client.convertValue(raw, object : TypeReference<SdkWorkListResponse>() {})
    }

    /** getUsageSummary */
    suspend fun usageSummaryRetrieve(): UsageSummaryRetrieveResponse? {
        val raw = client.get(ApiPaths.backendPath("/admin/usage/summary"))
        return client.convertValue(raw, object : TypeReference<UsageSummaryRetrieveResponse>() {})
    }

    private data class PathParameterSpec(val name: String, val style: String, val explode: Boolean)

    private fun serializePathParameter(value: Any?, spec: PathParameterSpec): String {
        if (value == null) return ""
        val style = spec.style.ifBlank { "simple" }
        return when (value) {
            is Iterable<*> -> serializePathArray(spec.name, value, style, spec.explode)
            is Map<*, *> -> serializePathObject(spec.name, value, style, spec.explode)
            else -> pathPrimitivePrefix(spec.name, style) + pathEncode(value.toString())
        }
    }

    private fun serializePathArray(name: String, values: Iterable<*>, style: String, explode: Boolean): String {
        val serialized = values.mapNotNull { it?.toString()?.let(::pathEncode) }
        if (serialized.isEmpty()) return pathPrefix(name, style)
        if (style == "matrix") {
            if (explode) {
                return serialized.joinToString("") { ";$name=$it" }
            }
            return ";$name=" + serialized.joinToString(",")
        }
        val separator = if (explode) "." else ","
        return pathPrefix(name, style) + serialized.joinToString(separator)
    }

    private fun serializePathObject(name: String, values: Map<*, *>, style: String, explode: Boolean): String {
        val entries = mutableListOf<String>()
        val exploded = mutableListOf<String>()
        values.forEach { (key, value) ->
            if (value == null) return@forEach
            val escapedKey = pathEncode(key.toString())
            val escapedValue = pathEncode(value.toString())
            if (explode) {
                if (style == "matrix") {
                    exploded += ";$escapedKey=$escapedValue"
                } else {
                    exploded += "$escapedKey=$escapedValue"
                }
            } else {
                entries += escapedKey
                entries += escapedValue
            }
        }
        if (style == "matrix") {
            if (explode) return exploded.joinToString("")
            return ";$name=" + entries.joinToString(",")
        }
        if (explode) {
            val separator = if (style == "label") "." else ","
            return pathPrefix(name, style) + exploded.joinToString(separator)
        }
        return pathPrefix(name, style) + entries.joinToString(",")
    }

    private fun pathPrefix(name: String, style: String): String {
        return when (style) {
            "label" -> "."
            "matrix" -> ";$name"
            else -> ""
        }
    }

    private fun pathPrimitivePrefix(name: String, style: String): String {
        return if (style == "matrix") ";$name=" else pathPrefix(name, style)
    }

    private fun pathEncode(value: String): String {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8).replace("+", "%20")
    }

    private data class QueryParameterSpec(
        val name: String,
        val value: Any?,
        val style: String,
        val explode: Boolean,
        val allowReserved: Boolean,
        val contentType: String?,
    )

    private val queryObjectMapper = ObjectMapper().registerKotlinModule()

    private fun buildQueryString(parameters: List<QueryParameterSpec>): String {
        val pairs = mutableListOf<String>()
        parameters.forEach { appendSerializedParameter(pairs, it) }
        return pairs.joinToString("&")
    }

    private fun appendSerializedParameter(pairs: MutableList<String>, parameter: QueryParameterSpec) {
        val value = parameter.value ?: return
        if (!parameter.contentType.isNullOrBlank()) {
            val json = queryObjectMapper.writeValueAsString(value)
            pairs += urlEncode(parameter.name) + "=" + encodeQueryValue(json, parameter.allowReserved)
            return
        }

        val style = parameter.style.ifBlank { "form" }
        when (value) {
            is Iterable<*> -> appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved)
            is Map<*, *> -> if (style == "deepObject") {
                appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved)
            } else {
                appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved)
            }
            else -> pairs += urlEncode(parameter.name) + "=" + encodeQueryValue(value.toString(), parameter.allowReserved)
        }
    }

    private fun appendArrayParameter(
        pairs: MutableList<String>,
        name: String,
        values: Iterable<*>,
        style: String,
        explode: Boolean,
        allowReserved: Boolean,
    ) {
        val serialized = values.mapNotNull { it?.toString() }
        if (serialized.isEmpty()) return
        if (style == "form" && explode) {
            serialized.forEach { pairs += urlEncode(name) + "=" + encodeQueryValue(it, allowReserved) }
            return
        }
        pairs += urlEncode(name) + "=" + encodeQueryValue(serialized.joinToString(","), allowReserved)
    }

    private fun appendObjectParameter(
        pairs: MutableList<String>,
        name: String,
        values: Map<*, *>,
        style: String,
        explode: Boolean,
        allowReserved: Boolean,
    ) {
        val serialized = mutableListOf<String>()
        values.forEach { (key, value) ->
            if (value == null) return@forEach
            if (style == "form" && explode) {
                pairs += urlEncode(key.toString()) + "=" + encodeQueryValue(value.toString(), allowReserved)
            } else {
                serialized += key.toString()
                serialized += value.toString()
            }
        }
        if (serialized.isNotEmpty()) {
            pairs += urlEncode(name) + "=" + encodeQueryValue(serialized.joinToString(","), allowReserved)
        }
    }

    private fun appendDeepObjectParameter(pairs: MutableList<String>, name: String, values: Map<*, *>, allowReserved: Boolean) {
        values.forEach { (key, value) ->
            if (value != null) {
                pairs += urlEncode("$name[$key]") + "=" + encodeQueryValue(value.toString(), allowReserved)
            }
        }
    }

    private fun encodeQueryValue(value: String, allowReserved: Boolean): String {
        var encoded = urlEncode(value)
        if (!allowReserved) return encoded
        mapOf(
            "%3A" to ":", "%2F" to "/", "%3F" to "?", "%23" to "#",
            "%5B" to "[", "%5D" to "]", "%40" to "@", "%21" to "!",
            "%24" to "$", "%26" to "&", "%27" to "'", "%28" to "(",
            "%29" to ")", "%2A" to "*", "%2B" to "+", "%2C" to ",",
            "%3B" to ";", "%3D" to "=",
        ).forEach { (escaped, reserved) -> encoded = encoded.replace(escaped, reserved) }
        return encoded
    }

    private fun urlEncode(value: String): String {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8)
    }

}
