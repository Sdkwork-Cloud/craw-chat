import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class AdminApi {
  final HttpClient _client;

  AdminApi(this._client);

  /// listApiKeyGroups
  Future<SdkWorkListResponse?> apiKeyGroupsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/api_key_groups'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// createApiKeyGroup
  Future<ApiKeyGroupsCreateResponse201?> apiKeyGroupsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/api_key_groups'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ApiKeyGroupsCreateResponse201.fromJson(map);
    })();
  }

  /// updateApiKeyGroup
  Future<ApiKeyGroupsUpdateResponse?> apiKeyGroupsUpdate(String groupId, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.patch(ApiPaths.backendPath('/admin/api_key_groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ApiKeyGroupsUpdateResponse.fromJson(map);
    })();
  }

  /// deleteApiKeyGroup
  Future<void> apiKeyGroupsDelete(String groupId) async {
    await _client.delete(ApiPaths.backendPath('/admin/api_key_groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
  }

  /// updateApiKeyGroupStatus
  Future<ApiKeyGroupsStatusResponse?> apiKeyGroupsStatus(String groupId, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/api_key_groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/status'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ApiKeyGroupsStatusResponse.fromJson(map);
    })();
  }

  /// listApiKeys
  Future<SdkWorkListResponse?> apiKeysList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/api_keys'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// createApiKey
  Future<ApiKeysCreateResponse201?> apiKeysCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/api_keys'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ApiKeysCreateResponse201.fromJson(map);
    })();
  }

  /// updateApiKey
  Future<ApiKeysUpdateResponse?> apiKeysUpdate(String hashedKey, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.put(ApiPaths.backendPath('/admin/api_keys/${serializePathParameter(hashedKey, const PathParameterSpec('hashedKey', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ApiKeysUpdateResponse.fromJson(map);
    })();
  }

  /// deleteApiKey
  Future<void> apiKeysDelete(String hashedKey) async {
    await _client.delete(ApiPaths.backendPath('/admin/api_keys/${serializePathParameter(hashedKey, const PathParameterSpec('hashedKey', 'simple', false))}'));
  }

  /// updateApiKeyStatus
  Future<ApiKeysStatusResponse?> apiKeysStatus(String hashedKey, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/api_keys/${serializePathParameter(hashedKey, const PathParameterSpec('hashedKey', 'simple', false))}/status'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ApiKeysStatusResponse.fromJson(map);
    })();
  }

  /// listBillingEvents
  Future<SdkWorkListResponse?> billingEventsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/billing/events'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// getBillingEventSummary
  Future<BillingEventsSummaryRetrieveResponse?> billingEventsSummaryRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/admin/billing/events/summary'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : BillingEventsSummaryRetrieveResponse.fromJson(map);
    })();
  }

  /// getBillingSummary
  Future<BillingSummaryRetrieveResponse?> billingSummaryRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/admin/billing/summary'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : BillingSummaryRetrieveResponse.fromJson(map);
    })();
  }

  /// listChannelModels
  Future<SdkWorkListResponse?> channelModelsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/channel_models'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveChannelModel
  Future<ChannelModelsCreateResponse201?> channelModelsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/channel_models'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ChannelModelsCreateResponse201.fromJson(map);
    })();
  }

  /// deleteChannelModel
  Future<void> channelModelsDelete(String channelId, String modelId) async {
    await _client.delete(ApiPaths.backendPath('/admin/channel_models/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/models/${serializePathParameter(modelId, const PathParameterSpec('modelId', 'simple', false))}'));
  }

  /// listChannels
  Future<SdkWorkListResponse?> channelsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/channels'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveChannel
  Future<ChannelsCreateResponse201?> channelsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/channels'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ChannelsCreateResponse201.fromJson(map);
    })();
  }

  /// deleteChannel
  Future<void> channelsDelete(String channelId) async {
    await _client.delete(ApiPaths.backendPath('/admin/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'));
  }

  /// listCredentials
  Future<SdkWorkListResponse?> credentialsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/credentials'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveCredential
  Future<CredentialsCreateResponse201?> credentialsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/credentials'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : CredentialsCreateResponse201.fromJson(map);
    })();
  }

  /// deleteCredential
  Future<void> credentialsProvidersKeysDelete(String tenantId, String providerId, String keyReference) async {
    await _client.delete(ApiPaths.backendPath('/admin/credentials/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}/providers/${serializePathParameter(providerId, const PathParameterSpec('providerId', 'simple', false))}/keys/${serializePathParameter(keyReference, const PathParameterSpec('keyReference', 'simple', false))}'));
  }

  /// reloadExtensionRuntimes
  Future<ExtensionsRuntimeReloadsCreateResponse201?> extensionsRuntimeReloadsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/extensions/runtime_reloads'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ExtensionsRuntimeReloadsCreateResponse201.fromJson(map);
    })();
  }

  /// listRuntimeStatuses
  Future<SdkWorkListResponse?> extensionsRuntimeStatusesList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/extensions/runtime_statuses'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// listRateLimitPolicies
  Future<SdkWorkListResponse?> gatewayRateLimitPoliciesList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/gateway/rate_limit_policies'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// createRateLimitPolicy
  Future<GatewayRateLimitPoliciesCreateResponse201?> gatewayRateLimitPoliciesCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/gateway/rate_limit_policies'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : GatewayRateLimitPoliciesCreateResponse201.fromJson(map);
    })();
  }

  /// listRateLimitWindows
  Future<SdkWorkListResponse?> gatewayRateLimitWindowsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/gateway/rate_limit_windows'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// listMarketingCampaigns
  Future<SdkWorkListResponse?> marketingCampaignsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/marketing/campaigns'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveMarketingCampaign
  Future<MarketingCampaignsCreateResponse201?> marketingCampaignsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/marketing/campaigns'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MarketingCampaignsCreateResponse201.fromJson(map);
    })();
  }

  /// updateMarketingCampaignStatus
  Future<MarketingCampaignsStatusResponse?> marketingCampaignsStatus(String marketingCampaignId, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, const PathParameterSpec('marketingCampaignId', 'simple', false))}/status'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MarketingCampaignsStatusResponse.fromJson(map);
    })();
  }

  /// listModelPrices
  Future<SdkWorkListResponse?> modelPricesList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/model_prices'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveModelPrice
  Future<ModelPricesCreateResponse201?> modelPricesCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/model_prices'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ModelPricesCreateResponse201.fromJson(map);
    })();
  }

  /// deleteModelPrice
  Future<void> modelPricesProvidersDelete(String channelId, String modelId, String proxyProviderId) async {
    await _client.delete(ApiPaths.backendPath('/admin/model_prices/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/models/${serializePathParameter(modelId, const PathParameterSpec('modelId', 'simple', false))}/providers/${serializePathParameter(proxyProviderId, const PathParameterSpec('proxyProviderId', 'simple', false))}'));
  }

  /// listModels
  Future<SdkWorkListResponse?> modelsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/models'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveModel
  Future<ModelsCreateResponse201?> modelsCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/models'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ModelsCreateResponse201.fromJson(map);
    })();
  }

  /// deleteModel
  Future<void> modelsProvidersDelete(String externalName, String providerId) async {
    await _client.delete(ApiPaths.backendPath('/admin/models/${serializePathParameter(externalName, const PathParameterSpec('externalName', 'simple', false))}/providers/${serializePathParameter(providerId, const PathParameterSpec('providerId', 'simple', false))}'));
  }

  /// listProviders
  Future<SdkWorkListResponse?> providersList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/providers'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// saveProvider
  Future<ProvidersCreateResponse201?> providersCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/providers'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : ProvidersCreateResponse201.fromJson(map);
    })();
  }

  /// deleteProvider
  Future<void> providersDelete(String providerId) async {
    await _client.delete(ApiPaths.backendPath('/admin/providers/${serializePathParameter(providerId, const PathParameterSpec('providerId', 'simple', false))}'));
  }

  /// listRoutingDecisionLogs
  Future<SdkWorkListResponse?> routingDecisionLogsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/routing/decision_logs'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// listProviderHealthSnapshots
  Future<RoutingHealthSnapshotsRetrieveResponse?> routingHealthSnapshotsRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/admin/routing/health_snapshots'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RoutingHealthSnapshotsRetrieveResponse.fromJson(map);
    })();
  }

  /// listRoutingProfiles
  Future<SdkWorkListResponse?> routingProfilesList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/routing/profiles'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// createRoutingProfile
  Future<RoutingProfilesCreateResponse201?> routingProfilesCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/routing/profiles'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RoutingProfilesCreateResponse201.fromJson(map);
    })();
  }

  /// listCompiledRoutingSnapshots
  Future<SdkWorkListResponse?> routingSnapshotsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/routing/snapshots'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// listStorageAuditTrail
  Future<SdkWorkListResponse?> storageAuditList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/storage/audit'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// getGlobalStorageConfig
  Future<StorageConfigRetrieveResponse?> storageConfigRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/admin/storage/config'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageConfigRetrieveResponse.fromJson(map);
    })();
  }

  /// saveGlobalStorageConfig
  Future<StorageConfigCreateResponse201?> storageConfigCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/storage/config'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageConfigCreateResponse201.fromJson(map);
    })();
  }

  /// getTenantStorageConfig
  Future<StorageConfigTenantsRetrieveResponse?> storageConfigTenantsRetrieve(String tenantId) async {
    final response = await _client.get(ApiPaths.backendPath('/admin/storage/config/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageConfigTenantsRetrieveResponse.fromJson(map);
    })();
  }

  /// saveTenantStorageConfig
  Future<StorageConfigTenantsCreateResponse201?> storageConfigTenantsCreate(String tenantId, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/storage/config/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageConfigTenantsCreateResponse201.fromJson(map);
    })();
  }

  /// deleteTenantStorageConfig
  Future<void> storageConfigTenantsDelete(String tenantId) async {
    await _client.delete(ApiPaths.backendPath('/admin/storage/config/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'));
  }

  /// getTenantEffectiveStorageConfig
  Future<StorageEffectiveTenantsRetrieveResponse?> storageEffectiveTenantsRetrieve(String tenantId) async {
    final response = await _client.get(ApiPaths.backendPath('/admin/storage/effective/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageEffectiveTenantsRetrieveResponse.fromJson(map);
    })();
  }

  /// listStorageProviders
  Future<SdkWorkListResponse?> storageProvidersList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/storage/providers'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// validateGlobalStorageConfig
  Future<StorageValidationCreateResponse201?> storageValidationCreate(Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/storage/validate'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageValidationCreateResponse201.fromJson(map);
    })();
  }

  /// validateTenantStorageConfig
  Future<StorageValidationTenantsCreateResponse201?> storageValidationTenantsCreate(String tenantId, Map<String, dynamic> body) async {
    final payload = body;
    final response = await _client.post(ApiPaths.backendPath('/admin/storage/validate/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StorageValidationTenantsCreateResponse201.fromJson(map);
    })();
  }

  /// listUsageRecords
  Future<SdkWorkListResponse?> usageRecordsList([int? pageSize, String? cursor, int? page, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('cursor', cursor, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.backendPath('/admin/usage/records'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// getUsageSummary
  Future<UsageSummaryRetrieveResponse?> usageSummaryRetrieve() async {
    final response = await _client.get(ApiPaths.backendPath('/admin/usage/summary'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : UsageSummaryRetrieveResponse.fromJson(map);
    })();
  }
}

class PathParameterSpec {
  final String name;
  final String style;
  final bool explode;

  const PathParameterSpec(this.name, this.style, this.explode);
}

String serializePathParameter(dynamic value, PathParameterSpec spec) {
  if (value == null) return '';
  final style = spec.style.trim().isEmpty ? 'simple' : spec.style;
  if (value is Iterable) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (value is Map) {
    return serializePathObject(spec.name, value, style, spec.explode);
  }
  return pathPrimitivePrefix(spec.name, style) + Uri.encodeComponent(value.toString());
}

String serializePathArray(String name, Iterable values, String style, bool explode) {
  final serialized = values.where((item) => item != null).map((item) => Uri.encodeComponent(item.toString())).toList();
  if (serialized.isEmpty) return pathPrefix(name, style);
  if (style == 'matrix') {
    if (explode) {
      return serialized.map((item) => ';$name=$item').join();
    }
    return ';$name=${serialized.join(',')}';
  }
  final separator = explode ? '.' : ',';
  return pathPrefix(name, style) + serialized.join(separator);
}

String serializePathObject(String name, Map values, String style, bool explode) {
  final entries = <String>[];
  final exploded = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    final escapedKey = Uri.encodeComponent(key.toString());
    final escapedValue = Uri.encodeComponent(value.toString());
    if (explode) {
      if (style == 'matrix') {
        exploded.add(';$escapedKey=$escapedValue');
      } else {
        exploded.add('$escapedKey=$escapedValue');
      }
    } else {
      entries.add(escapedKey);
      entries.add(escapedValue);
    }
  });
  if (style == 'matrix') {
    if (explode) return exploded.join();
    return ';$name=${entries.join(',')}';
  }
  if (explode) {
    final separator = style == 'label' ? '.' : ',';
    return pathPrefix(name, style) + exploded.join(separator);
  }
  return pathPrefix(name, style) + entries.join(',');
}

String pathPrefix(String name, String style) {
  if (style == 'label') return '.';
  if (style == 'matrix') return ';$name';
  return '';
}

String pathPrimitivePrefix(String name, String style) {
  return style == 'matrix' ? ';$name=' : pathPrefix(name, style);
}
class QueryParameterSpec {
  final String name;
  final dynamic value;
  final String style;
  final bool explode;
  final bool allowReserved;
  final String? contentType;

  const QueryParameterSpec(
    this.name,
    this.value,
    this.style,
    this.explode,
    this.allowReserved,
    this.contentType,
  );
}

String buildQueryString(List<QueryParameterSpec> parameters) {
  final pairs = <String>[];
  for (final parameter in parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) {
  final value = parameter.value;
  if (value == null) return;

  final contentType = parameter.contentType;
  if (contentType != null && contentType.trim().isNotEmpty) {
    pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(jsonEncode(value), parameter.allowReserved)}');
    return;
  }

  final style = parameter.style.trim().isEmpty ? 'form' : parameter.style;
  if (style == 'deepObject' && value is Map) {
    appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved);
    return;
  }
  if (value is Iterable) {
    appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  if (value is Map) {
    appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(value.toString(), parameter.allowReserved)}');
}

void appendArrayParameter(
  List<String> pairs,
  String name,
  Iterable values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = values.where((item) => item != null).map((item) => item.toString()).toList();
  if (serialized.isEmpty) return;
  if (style == 'form' && explode) {
    for (final item in serialized) {
      pairs.add('${urlEncode(name)}=${encodeQueryValue(item, allowReserved)}');
    }
    return;
  }
  pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
}

void appendObjectParameter(
  List<String> pairs,
  String name,
  Map values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    if (style == 'form' && explode) {
      pairs.add('${urlEncode(key.toString())}=${encodeQueryValue(value.toString(), allowReserved)}');
      return;
    }
    serialized.add(key.toString());
    serialized.add(value.toString());
  });
  if (serialized.isNotEmpty) {
    pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
  }
}

void appendDeepObjectParameter(List<String> pairs, String name, Map values, bool allowReserved) {
  values.forEach((key, value) {
    if (value != null) {
      pairs.add('${urlEncode('$name[$key]')}=${encodeQueryValue(value.toString(), allowReserved)}');
    }
  });
}

String encodeQueryValue(String value, bool allowReserved) {
  var encoded = urlEncode(value);
  if (!allowReserved) return encoded;
  const replacements = <String, String>{
    '%3A': ':',
    '%2F': '/',
    '%3F': '?',
    '%23': '#',
    '%5B': '[',
    '%5D': ']',
    '%40': '@',
    '%21': '!',
    '%24': r'$',
    '%26': '&',
    '%27': "'",
    '%28': '(',
    '%29': ')',
    '%2A': '*',
    '%2B': '+',
    '%2C': ',',
    '%3B': ';',
    '%3D': '=',
  };
  replacements.forEach((escaped, reserved) {
    encoded = encoded.replaceAll(escaped, reserved);
  });
  return encoded;
}

String urlEncode(String value) => Uri.encodeQueryComponent(value);
