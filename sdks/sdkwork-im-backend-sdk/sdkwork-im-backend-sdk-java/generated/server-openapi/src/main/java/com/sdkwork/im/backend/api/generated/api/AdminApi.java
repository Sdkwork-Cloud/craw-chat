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
    public Object apiKeyGroupsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/api_key_groups"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** createApiKeyGroup */
    public Object apiKeyGroupsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_key_groups"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** updateApiKeyGroup */
    public Object apiKeyGroupsUpdate(String groupId, Map<String, Object> body) throws Exception {
        Object raw = client.patch(ApiPaths.backendPath("/admin/api_key_groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteApiKeyGroup */
    public Object apiKeyGroupsDelete(String groupId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/api_key_groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** updateApiKeyGroupStatus */
    public Object apiKeyGroupsStatus(String groupId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_key_groups/" + serializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false)) + "/status"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listApiKeys */
    public Object apiKeysList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/api_keys"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** createApiKey */
    public Object apiKeysCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_keys"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** updateApiKey */
    public Object apiKeysUpdate(String hashedKey, Map<String, Object> body) throws Exception {
        Object raw = client.put(ApiPaths.backendPath("/admin/api_keys/" + serializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteApiKey */
    public Object apiKeysDelete(String hashedKey) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/api_keys/" + serializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** updateApiKeyStatus */
    public Object apiKeysStatus(String hashedKey, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/api_keys/" + serializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false)) + "/status"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listBillingEvents */
    public Object billingEventsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/billing/events"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** getBillingEventSummary */
    public Object billingEventsSummaryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/billing/events/summary"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** getBillingSummary */
    public Object billingSummaryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/billing/summary"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listChannelModels */
    public Object channelModelsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/channel_models"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveChannelModel */
    public Object channelModelsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/channel_models"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteChannelModel */
    public Object channelModelsDelete(String channelId, String modelId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/channel_models/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/models/" + serializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listChannels */
    public Object channelsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/channels"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveChannel */
    public Object channelsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/channels"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteChannel */
    public Object channelsDelete(String channelId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/channels/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listCredentials */
    public Object credentialsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/credentials"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveCredential */
    public Object credentialsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/credentials"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteCredential */
    public Object credentialsProvidersKeysDelete(String tenantId, String providerId, String keyReference) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/credentials/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + "/providers/" + serializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false)) + "/keys/" + serializePathParameter(keyReference, new PathParameterSpec("keyReference", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** reloadExtensionRuntimes */
    public Object extensionsRuntimeReloadsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/extensions/runtime_reloads"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listRuntimeStatuses */
    public Object extensionsRuntimeStatusesList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/extensions/runtime_statuses"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listRateLimitPolicies */
    public Object gatewayRateLimitPoliciesList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** createRateLimitPolicy */
    public Object gatewayRateLimitPoliciesCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/gateway/rate_limit_policies"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listRateLimitWindows */
    public Object gatewayRateLimitWindowsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/gateway/rate_limit_windows"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listMarketingCampaigns */
    public Object marketingCampaignsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/marketing/campaigns"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveMarketingCampaign */
    public Object marketingCampaignsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/marketing/campaigns"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** updateMarketingCampaignStatus */
    public Object marketingCampaignsStatus(String marketingCampaignId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/marketing/campaigns/" + serializePathParameter(marketingCampaignId, new PathParameterSpec("marketingCampaignId", "simple", false)) + "/status"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listModelPrices */
    public Object modelPricesList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/model_prices"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveModelPrice */
    public Object modelPricesCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/model_prices"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteModelPrice */
    public Object modelPricesProvidersDelete(String channelId, String modelId, String proxyProviderId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/model_prices/" + serializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false)) + "/models/" + serializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false)) + "/providers/" + serializePathParameter(proxyProviderId, new PathParameterSpec("proxyProviderId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listModels */
    public Object modelsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/models"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveModel */
    public Object modelsCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/models"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteModel */
    public Object modelsProvidersDelete(String externalName, String providerId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/models/" + serializePathParameter(externalName, new PathParameterSpec("externalName", "simple", false)) + "/providers/" + serializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listProviders */
    public Object providersList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/providers"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveProvider */
    public Object providersCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/providers"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteProvider */
    public Object providersDelete(String providerId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/providers/" + serializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listRoutingDecisionLogs */
    public Object routingDecisionLogsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/routing/decision_logs"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listProviderHealthSnapshots */
    public Object routingHealthSnapshotsRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/routing/health_snapshots"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listRoutingProfiles */
    public Object routingProfilesList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/routing/profiles"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** createRoutingProfile */
    public Object routingProfilesCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/routing/profiles"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listCompiledRoutingSnapshots */
    public Object routingSnapshotsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/routing/snapshots"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listStorageAuditTrail */
    public Object storageAuditList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/audit"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** getGlobalStorageConfig */
    public Object storageConfigRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/config"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveGlobalStorageConfig */
    public Object storageConfigCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/config"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** getTenantStorageConfig */
    public Object storageConfigTenantsRetrieve(String tenantId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/config/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** saveTenantStorageConfig */
    public Object storageConfigTenantsCreate(String tenantId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/config/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** deleteTenantStorageConfig */
    public Object storageConfigTenantsDelete(String tenantId) throws Exception {
        Object raw = client.delete(ApiPaths.backendPath("/admin/storage/config/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** getTenantEffectiveStorageConfig */
    public Object storageEffectiveTenantsRetrieve(String tenantId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/effective/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listStorageProviders */
    public Object storageProvidersList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/storage/providers"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** validateGlobalStorageConfig */
    public Object storageValidationCreate(Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/validate"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** validateTenantStorageConfig */
    public Object storageValidationTenantsCreate(String tenantId, Map<String, Object> body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/admin/storage/validate/tenants/" + serializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false)) + ""), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** listUsageRecords */
    public Object usageRecordsList() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/usage/records"));
        return client.convertValue(raw, new TypeReference<Object>() {});
    }

    /** getUsageSummary */
    public Object usageSummaryRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/admin/usage/summary"));
        return client.convertValue(raw, new TypeReference<Object>() {});
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



}
