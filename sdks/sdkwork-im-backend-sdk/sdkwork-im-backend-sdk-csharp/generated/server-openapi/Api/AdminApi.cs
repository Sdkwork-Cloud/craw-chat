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
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ApiKeyGroupsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/api_key_groups"), queryString));
        }

        /// <summary>
        /// createApiKeyGroup
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ApiKeyGroupsCreateResponse201?> ApiKeyGroupsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ApiKeyGroupsCreateResponse201>(ApiPaths.BackendPath("/admin/api_key_groups"), body, null, null, "application/json");
        }

        /// <summary>
        /// updateApiKeyGroup
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ApiKeyGroupsUpdateResponse?> ApiKeyGroupsUpdateAsync(string groupId, Dictionary<string, object> body)
        {
            return await _client.PatchAsync<Sdkwork.Im.BackendApi.Generated.Models.ApiKeyGroupsUpdateResponse>(ApiPaths.BackendPath($"/admin/api_key_groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteApiKeyGroup
        /// </summary>
        public async Task ApiKeyGroupsDeleteAsync(string groupId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/api_key_groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}"));
        }

        /// <summary>
        /// updateApiKeyGroupStatus
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ApiKeyGroupsStatusResponse?> ApiKeyGroupsStatusAsync(string groupId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ApiKeyGroupsStatusResponse>(ApiPaths.BackendPath($"/admin/api_key_groups/{SerializePathParameter(groupId, new PathParameterSpec("groupId", "simple", false))}/status"), body, null, null, "application/json");
        }

        /// <summary>
        /// listApiKeys
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ApiKeysListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/api_keys"), queryString));
        }

        /// <summary>
        /// createApiKey
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ApiKeysCreateResponse201?> ApiKeysCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ApiKeysCreateResponse201>(ApiPaths.BackendPath("/admin/api_keys"), body, null, null, "application/json");
        }

        /// <summary>
        /// updateApiKey
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ApiKeysUpdateResponse?> ApiKeysUpdateAsync(string hashedKey, Dictionary<string, object> body)
        {
            return await _client.PutAsync<Sdkwork.Im.BackendApi.Generated.Models.ApiKeysUpdateResponse>(ApiPaths.BackendPath($"/admin/api_keys/{SerializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteApiKey
        /// </summary>
        public async Task ApiKeysDeleteAsync(string hashedKey)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/api_keys/{SerializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false))}"));
        }

        /// <summary>
        /// updateApiKeyStatus
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ApiKeysStatusResponse?> ApiKeysStatusAsync(string hashedKey, Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ApiKeysStatusResponse>(ApiPaths.BackendPath($"/admin/api_keys/{SerializePathParameter(hashedKey, new PathParameterSpec("hashedKey", "simple", false))}/status"), body, null, null, "application/json");
        }

        /// <summary>
        /// listBillingEvents
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> BillingEventsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/billing/events"), queryString));
        }

        /// <summary>
        /// getBillingEventSummary
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.BillingEventsSummaryRetrieveResponse?> BillingEventsSummaryRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.BillingEventsSummaryRetrieveResponse>(ApiPaths.BackendPath("/admin/billing/events/summary"));
        }

        /// <summary>
        /// getBillingSummary
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.BillingSummaryRetrieveResponse?> BillingSummaryRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.BillingSummaryRetrieveResponse>(ApiPaths.BackendPath("/admin/billing/summary"));
        }

        /// <summary>
        /// listChannelModels
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ChannelModelsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/channel_models"), queryString));
        }

        /// <summary>
        /// saveChannelModel
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ChannelModelsCreateResponse201?> ChannelModelsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ChannelModelsCreateResponse201>(ApiPaths.BackendPath("/admin/channel_models"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteChannelModel
        /// </summary>
        public async Task ChannelModelsDeleteAsync(string channelId, string modelId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/channel_models/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/models/{SerializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false))}"));
        }

        /// <summary>
        /// listChannels
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ChannelsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/channels"), queryString));
        }

        /// <summary>
        /// saveChannel
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ChannelsCreateResponse201?> ChannelsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ChannelsCreateResponse201>(ApiPaths.BackendPath("/admin/channels"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteChannel
        /// </summary>
        public async Task ChannelsDeleteAsync(string channelId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/channels/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}"));
        }

        /// <summary>
        /// listCredentials
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> CredentialsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/credentials"), queryString));
        }

        /// <summary>
        /// saveCredential
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.CredentialsCreateResponse201?> CredentialsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.CredentialsCreateResponse201>(ApiPaths.BackendPath("/admin/credentials"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteCredential
        /// </summary>
        public async Task CredentialsProvidersKeysDeleteAsync(string tenantId, string providerId, string keyReference)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/credentials/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}/providers/{SerializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false))}/keys/{SerializePathParameter(keyReference, new PathParameterSpec("keyReference", "simple", false))}"));
        }

        /// <summary>
        /// reloadExtensionRuntimes
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ExtensionsRuntimeReloadsCreateResponse201?> ExtensionsRuntimeReloadsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ExtensionsRuntimeReloadsCreateResponse201>(ApiPaths.BackendPath("/admin/extensions/runtime_reloads"), body, null, null, "application/json");
        }

        /// <summary>
        /// listRuntimeStatuses
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ExtensionsRuntimeStatusesListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/extensions/runtime_statuses"), queryString));
        }

        /// <summary>
        /// listRateLimitPolicies
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> GatewayRateLimitPoliciesListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/gateway/rate_limit_policies"), queryString));
        }

        /// <summary>
        /// createRateLimitPolicy
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.GatewayRateLimitPoliciesCreateResponse201?> GatewayRateLimitPoliciesCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.GatewayRateLimitPoliciesCreateResponse201>(ApiPaths.BackendPath("/admin/gateway/rate_limit_policies"), body, null, null, "application/json");
        }

        /// <summary>
        /// listRateLimitWindows
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> GatewayRateLimitWindowsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/gateway/rate_limit_windows"), queryString));
        }

        /// <summary>
        /// listMarketingCampaigns
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> MarketingCampaignsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/marketing/campaigns"), queryString));
        }

        /// <summary>
        /// saveMarketingCampaign
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.MarketingCampaignsCreateResponse201?> MarketingCampaignsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.MarketingCampaignsCreateResponse201>(ApiPaths.BackendPath("/admin/marketing/campaigns"), body, null, null, "application/json");
        }

        /// <summary>
        /// updateMarketingCampaignStatus
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.MarketingCampaignsStatusResponse?> MarketingCampaignsStatusAsync(string marketingCampaignId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.MarketingCampaignsStatusResponse>(ApiPaths.BackendPath($"/admin/marketing/campaigns/{SerializePathParameter(marketingCampaignId, new PathParameterSpec("marketingCampaignId", "simple", false))}/status"), body, null, null, "application/json");
        }

        /// <summary>
        /// listModelPrices
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ModelPricesListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/model_prices"), queryString));
        }

        /// <summary>
        /// saveModelPrice
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ModelPricesCreateResponse201?> ModelPricesCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ModelPricesCreateResponse201>(ApiPaths.BackendPath("/admin/model_prices"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteModelPrice
        /// </summary>
        public async Task ModelPricesProvidersDeleteAsync(string channelId, string modelId, string proxyProviderId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/model_prices/{SerializePathParameter(channelId, new PathParameterSpec("channelId", "simple", false))}/models/{SerializePathParameter(modelId, new PathParameterSpec("modelId", "simple", false))}/providers/{SerializePathParameter(proxyProviderId, new PathParameterSpec("proxyProviderId", "simple", false))}"));
        }

        /// <summary>
        /// listModels
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ModelsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/models"), queryString));
        }

        /// <summary>
        /// saveModel
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ModelsCreateResponse201?> ModelsCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ModelsCreateResponse201>(ApiPaths.BackendPath("/admin/models"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteModel
        /// </summary>
        public async Task ModelsProvidersDeleteAsync(string externalName, string providerId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/models/{SerializePathParameter(externalName, new PathParameterSpec("externalName", "simple", false))}/providers/{SerializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false))}"));
        }

        /// <summary>
        /// listProviders
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> ProvidersListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/providers"), queryString));
        }

        /// <summary>
        /// saveProvider
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.ProvidersCreateResponse201?> ProvidersCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.ProvidersCreateResponse201>(ApiPaths.BackendPath("/admin/providers"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteProvider
        /// </summary>
        public async Task ProvidersDeleteAsync(string providerId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/providers/{SerializePathParameter(providerId, new PathParameterSpec("providerId", "simple", false))}"));
        }

        /// <summary>
        /// listRoutingDecisionLogs
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> RoutingDecisionLogsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/routing/decision_logs"), queryString));
        }

        /// <summary>
        /// listProviderHealthSnapshots
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.RoutingHealthSnapshotsRetrieveResponse?> RoutingHealthSnapshotsRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.RoutingHealthSnapshotsRetrieveResponse>(ApiPaths.BackendPath("/admin/routing/health_snapshots"));
        }

        /// <summary>
        /// listRoutingProfiles
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> RoutingProfilesListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/routing/profiles"), queryString));
        }

        /// <summary>
        /// createRoutingProfile
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.RoutingProfilesCreateResponse201?> RoutingProfilesCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.RoutingProfilesCreateResponse201>(ApiPaths.BackendPath("/admin/routing/profiles"), body, null, null, "application/json");
        }

        /// <summary>
        /// listCompiledRoutingSnapshots
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> RoutingSnapshotsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/routing/snapshots"), queryString));
        }

        /// <summary>
        /// listStorageAuditTrail
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> StorageAuditListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/storage/audit"), queryString));
        }

        /// <summary>
        /// getGlobalStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigRetrieveResponse?> StorageConfigRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigRetrieveResponse>(ApiPaths.BackendPath("/admin/storage/config"));
        }

        /// <summary>
        /// saveGlobalStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigCreateResponse201?> StorageConfigCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigCreateResponse201>(ApiPaths.BackendPath("/admin/storage/config"), body, null, null, "application/json");
        }

        /// <summary>
        /// getTenantStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigTenantsRetrieveResponse?> StorageConfigTenantsRetrieveAsync(string tenantId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigTenantsRetrieveResponse>(ApiPaths.BackendPath($"/admin/storage/config/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"));
        }

        /// <summary>
        /// saveTenantStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigTenantsCreateResponse201?> StorageConfigTenantsCreateAsync(string tenantId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageConfigTenantsCreateResponse201>(ApiPaths.BackendPath($"/admin/storage/config/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// deleteTenantStorageConfig
        /// </summary>
        public async Task StorageConfigTenantsDeleteAsync(string tenantId)
        {
            await _client.DeleteAsync<object>(ApiPaths.BackendPath($"/admin/storage/config/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"));
        }

        /// <summary>
        /// getTenantEffectiveStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageEffectiveTenantsRetrieveResponse?> StorageEffectiveTenantsRetrieveAsync(string tenantId)
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageEffectiveTenantsRetrieveResponse>(ApiPaths.BackendPath($"/admin/storage/effective/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"));
        }

        /// <summary>
        /// listStorageProviders
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> StorageProvidersListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/storage/providers"), queryString));
        }

        /// <summary>
        /// validateGlobalStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageValidationCreateResponse201?> StorageValidationCreateAsync(Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageValidationCreateResponse201>(ApiPaths.BackendPath("/admin/storage/validate"), body, null, null, "application/json");
        }

        /// <summary>
        /// validateTenantStorageConfig
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.StorageValidationTenantsCreateResponse201?> StorageValidationTenantsCreateAsync(string tenantId, Dictionary<string, object> body)
        {
            return await _client.PostAsync<Sdkwork.Im.BackendApi.Generated.Models.StorageValidationTenantsCreateResponse201>(ApiPaths.BackendPath($"/admin/storage/validate/tenants/{SerializePathParameter(tenantId, new PathParameterSpec("tenantId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// listUsageRecords
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse?> UsageRecordsListAsync(int? pageSize = null, string? cursor = null, int? page = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page", page, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.SdkWorkListResponse>(ApiPaths.AppendQueryString(ApiPaths.BackendPath("/admin/usage/records"), queryString));
        }

        /// <summary>
        /// getUsageSummary
        /// </summary>
        public async Task<Sdkwork.Im.BackendApi.Generated.Models.UsageSummaryRetrieveResponse?> UsageSummaryRetrieveAsync()
        {
            return await _client.GetAsync<Sdkwork.Im.BackendApi.Generated.Models.UsageSummaryRetrieveResponse>(ApiPaths.BackendPath("/admin/usage/summary"));
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

        private sealed record QueryParameterSpec(
            string Name,
            object? Value,
            string Style,
            bool Explode,
            bool AllowReserved,
            string? ContentType);

        private static string BuildQueryString(IEnumerable<QueryParameterSpec> parameters)
        {
            var pairs = new List<string>();
            foreach (var parameter in parameters)
            {
                AppendSerializedParameter(pairs, parameter);
            }
            return string.Join("&", pairs);
        }

        private static void AppendSerializedParameter(List<string> pairs, QueryParameterSpec parameter)
        {
            if (parameter.Value is null)
            {
                return;
            }

            if (!string.IsNullOrWhiteSpace(parameter.ContentType))
            {
                var json = System.Text.Json.JsonSerializer.Serialize(parameter.Value);
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(json, parameter.AllowReserved));
                return;
            }

            var style = string.IsNullOrWhiteSpace(parameter.Style) ? "form" : parameter.Style;
            if (style == "deepObject" && parameter.Value is System.Collections.IDictionary deepObject)
            {
                AppendDeepObjectParameter(pairs, parameter.Name, deepObject, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IEnumerable enumerable && parameter.Value is not string && parameter.Value is not System.Collections.IDictionary)
            {
                AppendArrayParameter(pairs, parameter.Name, enumerable, style, parameter.Explode, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IDictionary dictionary)
            {
                AppendObjectParameter(pairs, parameter.Name, dictionary, style, parameter.Explode, parameter.AllowReserved);
            }
            else
            {
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(parameter.Value.ToString() ?? string.Empty, parameter.AllowReserved));
            }
        }

        private static void AppendArrayParameter(List<string> pairs, string name, System.Collections.IEnumerable values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(item.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count == 0)
            {
                return;
            }
            if (style == "form" && explode)
            {
                foreach (var item in serialized)
                {
                    pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(item, allowReserved));
                }
                return;
            }
            pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
        }

        private static void AppendObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                if (style == "form" && explode)
                {
                    pairs.Add(Uri.EscapeDataString(item.Key.ToString() ?? string.Empty) + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
                else
                {
                    serialized.Add(item.Key.ToString() ?? string.Empty);
                    serialized.Add(item.Value.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count > 0)
            {
                pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
            }
        }

        private static void AppendDeepObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, bool allowReserved)
        {
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is not null)
                {
                    pairs.Add(Uri.EscapeDataString(name + "[" + item.Key + "]") + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
            }
        }

        private static string EncodeQueryValue(string value, bool allowReserved)
        {
            var encoded = Uri.EscapeDataString(value);
            if (!allowReserved)
            {
                return encoded;
            }
            return encoded
                .Replace("%3A", ":").Replace("%2F", "/").Replace("%3F", "?").Replace("%23", "#")
                .Replace("%5B", "[").Replace("%5D", "]").Replace("%40", "@").Replace("%21", "!")
                .Replace("%24", "$").Replace("%26", "&").Replace("%27", "'").Replace("%28", "(")
                .Replace("%29", ")").Replace("%2A", "*").Replace("%2B", "+").Replace("%2C", ",")
                .Replace("%3B", ";").Replace("%3D", "=");
        }

    }
}
