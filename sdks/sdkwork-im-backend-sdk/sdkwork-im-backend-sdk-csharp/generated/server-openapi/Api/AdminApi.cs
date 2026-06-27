using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.BackendApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.BackendApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.BackendApi.Generated.Api
{
    public class AdminApi
    {
        private readonly SdkHttpClient _client;

        public AdminApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// listApiKeyGroups
        /// </summary>
        public async Task<object?> ApiKeyGroupsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/api_key_groups"));
        }

        /// <summary>
        /// createApiKeyGroup
        /// </summary>
        public async Task<object?> ApiKeyGroupsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/api_key_groups"), body, null, null, "application/json");
        }

        /// <summary>
        /// updateApiKeyGroup
        /// </summary>
        public async Task<object?> ApiKeyGroupsUpdateAsync(string groupId, Dictionary<string, object> body)
        {
            return await _client.PatchAsync<object>(ApiPaths.BackendPath($"/admin/api_key_groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteApiKeyGroup
        /// </summary>
        public async Task<object?> ApiKeyGroupsDeleteAsync(string groupId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/api_key_groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"));
        }

        /// <summary>
        /// updateApiKeyGroupStatus
        /// </summary>
        public async Task<object?> ApiKeyGroupsStatusAsync(string groupId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath($"/admin/api_key_groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/status"), body, null, null, "application/json");
        }

        /// <summary>
        /// listApiKeys
        /// </summary>
        public async Task<object?> ApiKeysListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/api_keys"));
        }

        /// <summary>
        /// createApiKey
        /// </summary>
        public async Task<object?> ApiKeysCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/api_keys"), body, null, null, "application/json");
        }

        /// <summary>
        /// updateApiKey
        /// </summary>
        public async Task<object?> ApiKeysUpdateAsync(string hashedKey, Dictionary<string, object> body)
        {
            return await _client.PutAsync<object>(ApiPaths.BackendPath($"/admin/api_keys/{SerializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteApiKey
        /// </summary>
        public async Task<object?> ApiKeysDeleteAsync(string hashedKey)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/api_keys/{SerializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false))}"));
        }

        /// <summary>
        /// updateApiKeyStatus
        /// </summary>
        public async Task<object?> ApiKeysStatusAsync(string hashedKey, Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath($"/admin/api_keys/{SerializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false))}/status"), body, null, null, "application/json");
        }

        /// <summary>
        /// listBillingEvents
        /// </summary>
        public async Task<object?> BillingEventsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/billing/events"));
        }

        /// <summary>
        /// getBillingEventSummary
        /// </summary>
        public async Task<object?> BillingEventsSummaryRetrieveAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/billing/events/summary"));
        }

        /// <summary>
        /// getBillingSummary
        /// </summary>
        public async Task<object?> BillingSummaryRetrieveAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/billing/summary"));
        }

        /// <summary>
        /// listChannelModels
        /// </summary>
        public async Task<object?> ChannelModelsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/channel_models"));
        }

        /// <summary>
        /// saveChannelModel
        /// </summary>
        public async Task<object?> ChannelModelsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/channel_models"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteChannelModel
        /// </summary>
        public async Task<object?> ChannelModelsDeleteAsync(string channelId, string modelId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/channel_models/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/models/{SerializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false))}"));
        }

        /// <summary>
        /// listChannels
        /// </summary>
        public async Task<object?> ChannelsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/channels"));
        }

        /// <summary>
        /// saveChannel
        /// </summary>
        public async Task<object?> ChannelsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/channels"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteChannel
        /// </summary>
        public async Task<object?> ChannelsDeleteAsync(string channelId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"));
        }

        /// <summary>
        /// listCredentials
        /// </summary>
        public async Task<object?> CredentialsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/credentials"));
        }

        /// <summary>
        /// saveCredential
        /// </summary>
        public async Task<object?> CredentialsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/credentials"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteCredential
        /// </summary>
        public async Task<object?> CredentialsProvidersKeysDeleteAsync(string tenantId, string providerId, string keyReference)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/credentials/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}/providers/{SerializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false))}/keys/{SerializePathParameter(keyReference, new PathParameterSpec("keyReference", "simple", false))}"));
        }

        /// <summary>
        /// reloadExtensionRuntimes
        /// </summary>
        public async Task<object?> ExtensionsRuntimeReloadsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/extensions/runtime_reloads"), body, null, null, "application/json");
        }

        /// <summary>
        /// listRuntimeStatuses
        /// </summary>
        public async Task<object?> ExtensionsRuntimeStatusesListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/extensions/runtime_statuses"));
        }

        /// <summary>
        /// listRateLimitPolicies
        /// </summary>
        public async Task<object?> GatewayRateLimitPoliciesListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/gateway/rate_limit_policies"));
        }

        /// <summary>
        /// createRateLimitPolicy
        /// </summary>
        public async Task<object?> GatewayRateLimitPoliciesCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/gateway/rate_limit_policies"), body, null, null, "application/json");
        }

        /// <summary>
        /// listRateLimitWindows
        /// </summary>
        public async Task<object?> GatewayRateLimitWindowsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/gateway/rate_limit_windows"));
        }

        /// <summary>
        /// listMarketingCampaigns
        /// </summary>
        public async Task<object?> MarketingCampaignsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/marketing/campaigns"));
        }

        /// <summary>
        /// saveMarketingCampaign
        /// </summary>
        public async Task<object?> MarketingCampaignsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/marketing/campaigns"), body, null, null, "application/json");
        }

        /// <summary>
        /// updateMarketingCampaignStatus
        /// </summary>
        public async Task<object?> MarketingCampaignsStatusAsync(string marketingCampaignId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath($"/admin/marketing/campaigns/{SerializePathParameter(marketingCampaignId, new PathParameterSpec("marketingCampaignId", "simple", false))}/status"), body, null, null, "application/json");
        }

        /// <summary>
        /// listModelPrices
        /// </summary>
        public async Task<object?> ModelPricesListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/model_prices"));
        }

        /// <summary>
        /// saveModelPrice
        /// </summary>
        public async Task<object?> ModelPricesCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/model_prices"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteModelPrice
        /// </summary>
        public async Task<object?> ModelPricesProvidersDeleteAsync(string channelId, string modelId, string proxyProviderId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/model_prices/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/models/{SerializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false))}/providers/{SerializePathParameter(proxyProviderId, new PathParameterSpec("proxyProviderId", "simple", false))}"));
        }

        /// <summary>
        /// listModels
        /// </summary>
        public async Task<object?> ModelsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/models"));
        }

        /// <summary>
        /// saveModel
        /// </summary>
        public async Task<object?> ModelsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/models"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteModel
        /// </summary>
        public async Task<object?> ModelsProvidersDeleteAsync(string externalName, string providerId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/models/{SerializePathParameter(externalName, new PathParameterSpec("externalName", "simple", false))}/providers/{SerializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false))}"));
        }

        /// <summary>
        /// listProviders
        /// </summary>
        public async Task<object?> ProvidersListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/providers"));
        }

        /// <summary>
        /// saveProvider
        /// </summary>
        public async Task<object?> ProvidersCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/providers"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteProvider
        /// </summary>
        public async Task<object?> ProvidersDeleteAsync(string providerId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/providers/{SerializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false))}"));
        }

        /// <summary>
        /// listRoutingDecisionLogs
        /// </summary>
        public async Task<object?> RoutingDecisionLogsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/routing/decision_logs"));
        }

        /// <summary>
        /// listProviderHealthSnapshots
        /// </summary>
        public async Task<object?> RoutingHealthSnapshotsRetrieveAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/routing/health_snapshots"));
        }

        /// <summary>
        /// listRoutingProfiles
        /// </summary>
        public async Task<object?> RoutingProfilesListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/routing/profiles"));
        }

        /// <summary>
        /// createRoutingProfile
        /// </summary>
        public async Task<object?> RoutingProfilesCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/routing/profiles"), body, null, null, "application/json");
        }

        /// <summary>
        /// listCompiledRoutingSnapshots
        /// </summary>
        public async Task<object?> RoutingSnapshotsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/routing/snapshots"));
        }

        /// <summary>
        /// listStorageAuditTrail
        /// </summary>
        public async Task<object?> StorageAuditListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/storage/audit"));
        }

        /// <summary>
        /// getGlobalStorageConfig
        /// </summary>
        public async Task<object?> StorageConfigRetrieveAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/storage/config"));
        }

        /// <summary>
        /// saveGlobalStorageConfig
        /// </summary>
        public async Task<object?> StorageConfigCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/storage/config"), body, null, null, "application/json");
        }

        /// <summary>
        /// getTenantStorageConfig
        /// </summary>
        public async Task<object?> StorageConfigTenantsRetrieveAsync(string tenantId)
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath($"/admin/storage/config/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"));
        }

        /// <summary>
        /// saveTenantStorageConfig
        /// </summary>
        public async Task<object?> StorageConfigTenantsCreateAsync(string tenantId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath($"/admin/storage/config/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteTenantStorageConfig
        /// </summary>
        public async Task<object?> StorageConfigTenantsDeleteAsync(string tenantId)
        {
            return await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/storage/config/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"));
        }

        /// <summary>
        /// getTenantEffectiveStorageConfig
        /// </summary>
        public async Task<object?> StorageEffectiveTenantsRetrieveAsync(string tenantId)
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath($"/admin/storage/effective/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"));
        }

        /// <summary>
        /// listStorageProviders
        /// </summary>
        public async Task<object?> StorageProvidersListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/storage/providers"));
        }

        /// <summary>
        /// validateGlobalStorageConfig
        /// </summary>
        public async Task<object?> StorageValidationCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath("/admin/storage/validate"), body, null, null, "application/json");
        }

        /// <summary>
        /// validateTenantStorageConfig
        /// </summary>
        public async Task<object?> StorageValidationTenantsCreateAsync(string tenantId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<object>(ApiPaths.BackendPath($"/admin/storage/validate/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// listUsageRecords
        /// </summary>
        public async Task<object?> UsageRecordsListAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/usage/records"));
        }

        /// <summary>
        /// getUsageSummary
        /// </summary>
        public async Task<object?> UsageSummaryRetrieveAsync()
        {
            return await _client.GetAsync<object>(ApiPaths.BackendPath("/admin/usage/summary"));
        }

        private sealed record PathParameterSpec(string Name, string Style, bool Explode);

        private static string SerializePathParameter(object? value, PathParameterSpec spec)
        {
            if (value is null)
            {
                return string.Empty;
            }
            var style = string.IsNullOrWhiteSpace(spec.Style) ? "simple" : spec.Style;
            if (value is System.Collections.IDictionary dictionary)
            {
                return SerializePathObject(spec.Name, dictionary, style, spec.Explode);
            }
            if (value is System.Collections.IEnumerable enumerable && value is not string)
            {
                return SerializePathArray(spec.Name, enumerable, style, spec.Explode);
            }
            return PathPrimitivePrefix(spec.Name, style) + Uri.EscapeDataString(value.ToString() ?? string.Empty);
        }

        private static string SerializePathArray(string name, System.Collections.IEnumerable values, string style, bool explode)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(Uri.EscapeDataString(item.ToString() ?? string.Empty));
                }
            }
            if (serialized.Count == 0)
            {
                return PathPrefix(name, style);
            }
            if (style == "matrix")
            {
                if (explode)
                {
                    var parts = new List<string>();
                    foreach (var item in serialized)
                    {
                        parts.Add(";" + name + "=" + item);
                    }
                    return string.Join(string.Empty, parts);
                }
                return ";" + name + "=" + string.Join(",", serialized);
            }
            var separator = explode ? "." : ",";
            return PathPrefix(name, style) + string.Join(separator, serialized);
        }

        private static string SerializePathObject(string name, System.Collections.IDictionary values, string style, bool explode)
        {
            var entries = new List<string>();
            var exploded = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                var escapedKey = Uri.EscapeDataString(item.Key.ToString() ?? string.Empty);
                var escapedValue = Uri.EscapeDataString(item.Value.ToString() ?? string.Empty);
                if (explode)
                {
                    exploded.Add(style == "matrix" ? ";" + escapedKey + "=" + escapedValue : escapedKey + "=" + escapedValue);
                }
                else
                {
                    entries.Add(escapedKey);
                    entries.Add(escapedValue);
                }
            }
            if (style == "matrix")
            {
                return explode ? string.Join(string.Empty, exploded) : ";" + name + "=" + string.Join(",", entries);
            }
            if (explode)
            {
                var separator = style == "label" ? "." : ",";
                return PathPrefix(name, style) + string.Join(separator, exploded);
            }
            return PathPrefix(name, style) + string.Join(",", entries);
        }

        private static string PathPrefix(string name, string style)
        {
            return style switch
            {
                "label" => ".",
                "matrix" => ";" + name,
                _ => string.Empty,
            };
        }

        private static string PathPrimitivePrefix(string name, string style)
        {
            return style == "matrix" ? ";" + name + "=" : PathPrefix(name, style);
        }


    }
}
