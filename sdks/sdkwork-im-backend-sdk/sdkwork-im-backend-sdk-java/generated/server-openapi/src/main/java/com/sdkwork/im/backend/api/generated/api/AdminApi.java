package com.sdkwork.im.backend.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class AdminApi {
    private final HttpClient client;

    public AdminApi(HttpClient client) {
        this.client = client;
    }

    /** listApiKeyGroups */
    public SdkWorkListResponse apiKeyGroupsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/api_key_groups"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** createApiKeyGroup */
    public ApiKeyGroupsCreateResponse201 apiKeyGroupsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_key_groups"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ApiKeyGroupsCreateResponse201>() {});
    }

    /** updateApiKeyGroup */
    public ApiKeyGroupsUpdateResponse apiKeyGroupsUpdate(String groupId, Map<String, Object> body) throws Exception {
        Object raw = client.patch(ApiPaths.backendPath("/admin/api_key_groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ApiKeyGroupsUpdateResponse>() {});
    }

    /** deleteApiKeyGroup */
    public Void apiKeyGroupsDelete(String groupId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/api_key_groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""));
        return null;
    }

    /** updateApiKeyGroupStatus */
    public ApiKeyGroupsStatusResponse apiKeyGroupsStatus(String groupId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_key_groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/status"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ApiKeyGroupsStatusResponse>() {});
    }

    /** listApiKeys */
    public SdkWorkListResponse apiKeysList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/api_keys"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** createApiKey */
    public ApiKeysCreateResponse201 apiKeysCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_keys"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ApiKeysCreateResponse201>() {});
    }

    /** updateApiKey */
    public ApiKeysUpdateResponse apiKeysUpdate(String hashedKey, Map<String, Object> body) throws Exception {
        Object raw = client.put(ApiPaths.backendPath("/admin/api_keys/" + serializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ApiKeysUpdateResponse>() {});
    }

    /** deleteApiKey */
    public Void apiKeysDelete(String hashedKey) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/api_keys/" + serializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false)) + ""));
        return null;
    }

    /** updateApiKeyStatus */
    public ApiKeysStatusResponse apiKeysStatus(String hashedKey, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_keys/" + serializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false)) + "/status"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ApiKeysStatusResponse>() {});
    }

    /** listBillingEvents */
    public SdkWorkListResponse billingEventsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/billing/events"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** getBillingEventSummary */
    public BillingEventsSummaryRetrieveResponse billingEventsSummaryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/billing/events/summary"));
        return client.convertValue(raw, new TypeReference<BillingEventsSummaryRetrieveResponse>() {});
    }

    /** getBillingSummary */
    public BillingSummaryRetrieveResponse billingSummaryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/billing/summary"));
        return client.convertValue(raw, new TypeReference<BillingSummaryRetrieveResponse>() {});
    }

    /** listChannelModels */
    public SdkWorkListResponse channelModelsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/channel_models"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveChannelModel */
    public ChannelModelsCreateResponse201 channelModelsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/channel_models"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ChannelModelsCreateResponse201>() {});
    }

    /** deleteChannelModel */
    public Void channelModelsDelete(String channelId, String modelId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/channel_models/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/models/" + serializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false)) + ""));
        return null;
    }

    /** listChannels */
    public SdkWorkListResponse channelsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/channels"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveChannel */
    public ChannelsCreateResponse201 channelsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ChannelsCreateResponse201>() {});
    }

    /** deleteChannel */
    public Void channelsDelete(String channelId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""));
        return null;
    }

    /** listCredentials */
    public SdkWorkListResponse credentialsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/credentials"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveCredential */
    public CredentialsCreateResponse201 credentialsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/credentials"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<CredentialsCreateResponse201>() {});
    }

    /** deleteCredential */
    public Void credentialsProvidersKeysDelete(String tenantId, String providerId, String keyReference) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/credentials/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + "/providers/" + serializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false)) + "/keys/" + serializePathParameter(keyReference, new PathParameterSpec("keyReference", "simple", false)) + ""));
        return null;
    }

    /** reloadExtensionRuntimes */
    public ExtensionsRuntimeReloadsCreateResponse201 extensionsRuntimeReloadsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/extensions/runtime_reloads"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ExtensionsRuntimeReloadsCreateResponse201>() {});
    }

    /** listRuntimeStatuses */
    public SdkWorkListResponse extensionsRuntimeStatusesList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/extensions/runtime_statuses"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** listRateLimitPolicies */
    public SdkWorkListResponse gatewayRateLimitPoliciesList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** createRateLimitPolicy */
    public GatewayRateLimitPoliciesCreateResponse201 gatewayRateLimitPoliciesCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<GatewayRateLimitPoliciesCreateResponse201>() {});
    }

    /** listRateLimitWindows */
    public SdkWorkListResponse gatewayRateLimitWindowsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/gateway/rate_limit_windows"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** listMarketingCampaigns */
    public SdkWorkListResponse marketingCampaignsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/marketing/campaigns"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveMarketingCampaign */
    public MarketingCampaignsCreateResponse201 marketingCampaignsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/marketing/campaigns"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MarketingCampaignsCreateResponse201>() {});
    }

    /** updateMarketingCampaignStatus */
    public MarketingCampaignsStatusResponse marketingCampaignsStatus(String marketingCampaignId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/marketing/campaigns/" + serializePathParameter(marketingCampaignId, new PathParameterSpec("marketingCampaignId", "simple", false)) + "/status"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MarketingCampaignsStatusResponse>() {});
    }

    /** listModelPrices */
    public SdkWorkListResponse modelPricesList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/model_prices"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveModelPrice */
    public ModelPricesCreateResponse201 modelPricesCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/model_prices"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ModelPricesCreateResponse201>() {});
    }

    /** deleteModelPrice */
    public Void modelPricesProvidersDelete(String channelId, String modelId, String proxyProviderId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/model_prices/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/models/" + serializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false)) + "/providers/" + serializePathParameter(proxyProviderId, new PathParameterSpec("proxyProviderId", "simple", false)) + ""));
        return null;
    }

    /** listModels */
    public SdkWorkListResponse modelsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/models"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveModel */
    public ModelsCreateResponse201 modelsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/models"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ModelsCreateResponse201>() {});
    }

    /** deleteModel */
    public Void modelsProvidersDelete(String externalName, String providerId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/models/" + serializePathParameter(externalName, new PathParameterSpec("externalName", "simple", false)) + "/providers/" + serializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false)) + ""));
        return null;
    }

    /** listProviders */
    public SdkWorkListResponse providersList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/providers"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** saveProvider */
    public ProvidersCreateResponse201 providersCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/providers"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<ProvidersCreateResponse201>() {});
    }

    /** deleteProvider */
    public Void providersDelete(String providerId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/providers/" + serializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false)) + ""));
        return null;
    }

    /** listRoutingDecisionLogs */
    public SdkWorkListResponse routingDecisionLogsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/decision_logs"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** listProviderHealthSnapshots */
    public RoutingHealthSnapshotsRetrieveResponse routingHealthSnapshotsRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/routing/health_snapshots"));
        return client.convertValue(raw, new TypeReference<RoutingHealthSnapshotsRetrieveResponse>() {});
    }

    /** listRoutingProfiles */
    public SdkWorkListResponse routingProfilesList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/profiles"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** createRoutingProfile */
    public RoutingProfilesCreateResponse201 routingProfilesCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/routing/profiles"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RoutingProfilesCreateResponse201>() {});
    }

    /** listCompiledRoutingSnapshots */
    public SdkWorkListResponse routingSnapshotsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/routing/snapshots"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** listStorageAuditTrail */
    public SdkWorkListResponse storageAuditList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/storage/audit"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** getGlobalStorageConfig */
    public StorageConfigRetrieveResponse storageConfigRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/config"));
        return client.convertValue(raw, new TypeReference<StorageConfigRetrieveResponse>() {});
    }

    /** saveGlobalStorageConfig */
    public StorageConfigCreateResponse201 storageConfigCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/config"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StorageConfigCreateResponse201>() {});
    }

    /** getTenantStorageConfig */
    public StorageConfigTenantsRetrieveResponse storageConfigTenantsRetrieve(String tenantId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/config/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<StorageConfigTenantsRetrieveResponse>() {});
    }

    /** saveTenantStorageConfig */
    public StorageConfigTenantsCreateResponse201 storageConfigTenantsCreate(String tenantId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/config/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StorageConfigTenantsCreateResponse201>() {});
    }

    /** deleteTenantStorageConfig */
    public Void storageConfigTenantsDelete(String tenantId) throws Exception {
        client.delete(ApiPaths.backendPath("/admin/storage/config/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""));
        return null;
    }

    /** getTenantEffectiveStorageConfig */
    public StorageEffectiveTenantsRetrieveResponse storageEffectiveTenantsRetrieve(String tenantId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/effective/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<StorageEffectiveTenantsRetrieveResponse>() {});
    }

    /** listStorageProviders */
    public SdkWorkListResponse storageProvidersList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/storage/providers"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** validateGlobalStorageConfig */
    public StorageValidationCreateResponse201 storageValidationCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/validate"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StorageValidationCreateResponse201>() {});
    }

    /** validateTenantStorageConfig */
    public StorageValidationTenantsCreateResponse201 storageValidationTenantsCreate(String tenantId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/validate/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<StorageValidationTenantsCreateResponse201>() {});
    }

    /** listUsageRecords */
    public SdkWorkListResponse usageRecordsList(Integer pageSize, String cursor, Integer page, String q) throws Exception {
        String query = buildQueryString(List.of(
            new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            new QueryParameterSpec("page", page, "form", true, false, null),
            new QueryParameterSpec("q", q, "form", true, false, null)
        ));
        Object raw = client.get(ApiPaths.appendQueryString(ApiPaths.backendPath("/admin/usage/records"), query));
        return client.convertValue(raw, new TypeReference<SdkWorkListResponse>() {});
    }

    /** getUsageSummary */
    public UsageSummaryRetrieveResponse usageSummaryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/usage/summary"));
        return client.convertValue(raw, new TypeReference<UsageSummaryRetrieveResponse>() {});
    }

    private record PathParameterSpec(String name, String style, boolean explode) {}

    private static String serializePathParameter(Object value, PathParameterSpec spec) {
        if (value == null) {
            return "";
        }
        String style = spec.style() == null || spec.style().isBlank() ? "simple" : spec.style();
        if (value instanceof Iterable<?> iterable) {
            return serializePathArray(spec.name(), iterable, style, spec.explode());
        }
        if (value instanceof Map<?, ?> map) {
            return serializePathObject(spec.name(), map, style, spec.explode());
        }
        return pathPrimitivePrefix(spec.name(), style) + pathEncode(String.valueOf(value));
    }

    private static String serializePathArray(String name, Iterable<?> values, String style, boolean explode) {
        List<String> serialized = new java.util.ArrayList<>();
        for (Object item : values) {
            if (item != null) {
                serialized.add(pathEncode(String.valueOf(item)));
            }
        }
        if (serialized.isEmpty()) {
            return pathPrefix(name, style);
        }
        if ("matrix".equals(style)) {
            if (explode) {
                List<String> parts = new java.util.ArrayList<>();
                for (String item : serialized) {
                    parts.add(";" + name + "=" + item);
                }
                return String.join("", parts);
            }
            return ";" + name + "=" + String.join(",", serialized);
        }
        String separator = explode ? "." : ",";
        return pathPrefix(name, style) + String.join(separator, serialized);
    }

    private static String serializePathObject(String name, Map<?, ?> values, String style, boolean explode) {
        List<String> entries = new java.util.ArrayList<>();
        List<String> exploded = new java.util.ArrayList<>();
        values.forEach((key, value) -> {
            if (value == null) {
                return;
            }
            String escapedKey = pathEncode(String.valueOf(key));
            String escapedValue = pathEncode(String.valueOf(value));
            if (explode) {
                if ("matrix".equals(style)) {
                    exploded.add(";" + escapedKey + "=" + escapedValue);
                } else {
                    exploded.add(escapedKey + "=" + escapedValue);
                }
            } else {
                entries.add(escapedKey);
                entries.add(escapedValue);
            }
        });
        if ("matrix".equals(style)) {
            if (explode) {
                return String.join("", exploded);
            }
            return ";" + name + "=" + String.join(",", entries);
        }
        if (explode) {
            String separator = "label".equals(style) ? "." : ",";
            return pathPrefix(name, style) + String.join(separator, exploded);
        }
        return pathPrefix(name, style) + String.join(",", entries);
    }

    private static String pathPrefix(String name, String style) {
        if ("label".equals(style)) {
            return ".";
        }
        if ("matrix".equals(style)) {
            return ";" + name;
        }
        return "";
    }

    private static String pathPrimitivePrefix(String name, String style) {
        if ("matrix".equals(style)) {
            return ";" + name + "=";
        }
        return pathPrefix(name, style);
    }

    private static String pathEncode(String value) {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8).replace("+", "%20");
    }

    private record QueryParameterSpec(String name, Object value, String style, boolean explode, boolean allowReserved, String contentType) {}

    private static String buildQueryString(List<QueryParameterSpec> parameters) throws Exception {
        List<String> pairs = new java.util.ArrayList<>();
        for (QueryParameterSpec parameter : parameters) {
            appendSerializedParameter(pairs, parameter);
        }
        return String.join("&", pairs);
    }

    private static void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) throws Exception {
        if (parameter.value() == null) {
            return;
        }
        if (parameter.contentType() != null && !parameter.contentType().isBlank()) {
            String json = clientObjectMapper().writeValueAsString(parameter.value());
            pairs.add(urlEncode(parameter.name()) + "=" + encodeQueryValue(json, parameter.allowReserved()));
            return;
        }

        String style = parameter.style() == null || parameter.style().isBlank() ? "form" : parameter.style();
        Object value = parameter.value();
        if ("deepObject".equals(style) && value instanceof Map<?, ?> map) {
            appendDeepObjectParameter(pairs, parameter.name(), map, parameter.allowReserved());
        } else if (value instanceof Iterable<?> iterable) {
            appendArrayParameter(pairs, parameter.name(), iterable, style, parameter.explode(), parameter.allowReserved());
        } else if (value instanceof Map<?, ?> map) {
            appendObjectParameter(pairs, parameter.name(), map, style, parameter.explode(), parameter.allowReserved());
        } else {
            pairs.add(urlEncode(parameter.name()) + "=" + encodeQueryValue(String.valueOf(value), parameter.allowReserved()));
        }
    }

    private static void appendArrayParameter(List<String> pairs, String name, Iterable<?> values, String style, boolean explode, boolean allowReserved) {
        List<String> serialized = new java.util.ArrayList<>();
        for (Object item : values) {
            if (item != null) {
                serialized.add(String.valueOf(item));
            }
        }
        if (serialized.isEmpty()) {
            return;
        }
        if ("form".equals(style) && explode) {
            for (String item : serialized) {
                pairs.add(urlEncode(name) + "=" + encodeQueryValue(item, allowReserved));
            }
            return;
        }
        pairs.add(urlEncode(name) + "=" + encodeQueryValue(String.join(",", serialized), allowReserved));
    }

    private static void appendObjectParameter(List<String> pairs, String name, Map<?, ?> values, String style, boolean explode, boolean allowReserved) {
        List<String> serialized = new java.util.ArrayList<>();
        values.forEach((key, value) -> {
            if (value == null) {
                return;
            }
            if ("form".equals(style) && explode) {
                pairs.add(urlEncode(String.valueOf(key)) + "=" + encodeQueryValue(String.valueOf(value), allowReserved));
            } else {
                serialized.add(String.valueOf(key));
                serialized.add(String.valueOf(value));
            }
        });
        if (!serialized.isEmpty()) {
            pairs.add(urlEncode(name) + "=" + encodeQueryValue(String.join(",", serialized), allowReserved));
        }
    }

    private static void appendDeepObjectParameter(List<String> pairs, String name, Map<?, ?> values, boolean allowReserved) {
        values.forEach((key, value) -> {
            if (value != null) {
                pairs.add(urlEncode(name + "[" + key + "]") + "=" + encodeQueryValue(String.valueOf(value), allowReserved));
            }
        });
    }

    private static String encodeQueryValue(String value, boolean allowReserved) {
        String encoded = urlEncode(value);
        if (!allowReserved) {
            return encoded;
        }
        return encoded
            .replace("%3A", ":").replace("%2F", "/").replace("%3F", "?").replace("%23", "#")
            .replace("%5B", "[").replace("%5D", "]").replace("%40", "@").replace("%21", "!")
            .replace("%24", "$").replace("%26", "&").replace("%27", "'").replace("%28", "(")
            .replace("%29", ")").replace("%2A", "*").replace("%2B", "+").replace("%2C", ",")
            .replace("%3B", ";").replace("%3D", "=");
    }

    private static com.fasterxml.jackson.databind.ObjectMapper clientObjectMapper() {
        return new com.fasterxml.jackson.databind.ObjectMapper();
    }


    private static String urlEncode(String value) {
        return java.net.URLEncoder.encode(value, java.nio.charset.StandardCharsets.UTF_8);
    }
}
