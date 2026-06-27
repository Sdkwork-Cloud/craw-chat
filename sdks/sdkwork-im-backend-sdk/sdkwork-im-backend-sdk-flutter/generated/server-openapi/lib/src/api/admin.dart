import '../http/client.dart';

import 'paths.dart';


class AdminApi {
  final HttpClient _client;

  AdminApi(this._client);

  /// listApiKeyGroups
  Future<dynamic> apiKeyGroupsList() async {
    return _client.get(ApiPaths.backendPath('/admin/api_key_groups'));
  }

  /// createApiKeyGroup
  Future<dynamic> apiKeyGroupsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/api_key_groups'), body: payload, contentType: 'application/json');
  }

  /// updateApiKeyGroup
  Future<dynamic> apiKeyGroupsUpdate(String groupId, Map<String, dynamic> body) async {
    final payload = body;
    return _client.patch(ApiPaths.backendPath('/admin/api_key_groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'), body: payload, contentType: 'application/json');
  }

  /// deleteApiKeyGroup
  Future<dynamic> apiKeyGroupsDelete(String groupId) async {
    return _client.delete(ApiPaths.backendPath('/admin/api_key_groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
  }

  /// updateApiKeyGroupStatus
  Future<dynamic> apiKeyGroupsStatus(String groupId, Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/api_key_groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}/status'), body: payload, contentType: 'application/json');
  }

  /// listApiKeys
  Future<dynamic> apiKeysList() async {
    return _client.get(ApiPaths.backendPath('/admin/api_keys'));
  }

  /// createApiKey
  Future<dynamic> apiKeysCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/api_keys'), body: payload, contentType: 'application/json');
  }

  /// updateApiKey
  Future<dynamic> apiKeysUpdate(String hashedKey, Map<String, dynamic> body) async {
    final payload = body;
    return _client.put(ApiPaths.backendPath('/admin/api_keys/${serializePathParameter(hashedKey, const PathParameterSpec('hashedKey', 'simple', false))}'), body: payload, contentType: 'application/json');
  }

  /// deleteApiKey
  Future<dynamic> apiKeysDelete(String hashedKey) async {
    return _client.delete(ApiPaths.backendPath('/admin/api_keys/${serializePathParameter(hashedKey, const PathParameterSpec('hashedKey', 'simple', false))}'));
  }

  /// updateApiKeyStatus
  Future<dynamic> apiKeysStatus(String hashedKey, Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/api_keys/${serializePathParameter(hashedKey, const PathParameterSpec('hashedKey', 'simple', false))}/status'), body: payload, contentType: 'application/json');
  }

  /// listBillingEvents
  Future<dynamic> billingEventsList() async {
    return _client.get(ApiPaths.backendPath('/admin/billing/events'));
  }

  /// getBillingEventSummary
  Future<dynamic> billingEventsSummaryRetrieve() async {
    return _client.get(ApiPaths.backendPath('/admin/billing/events/summary'));
  }

  /// getBillingSummary
  Future<dynamic> billingSummaryRetrieve() async {
    return _client.get(ApiPaths.backendPath('/admin/billing/summary'));
  }

  /// listChannelModels
  Future<dynamic> channelModelsList() async {
    return _client.get(ApiPaths.backendPath('/admin/channel_models'));
  }

  /// saveChannelModel
  Future<dynamic> channelModelsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/channel_models'), body: payload, contentType: 'application/json');
  }

  /// deleteChannelModel
  Future<dynamic> channelModelsDelete(String channelId, String modelId) async {
    return _client.delete(ApiPaths.backendPath('/admin/channel_models/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/models/${serializePathParameter(modelId, const PathParameterSpec('modelId', 'simple', false))}'));
  }

  /// listChannels
  Future<dynamic> channelsList() async {
    return _client.get(ApiPaths.backendPath('/admin/channels'));
  }

  /// saveChannel
  Future<dynamic> channelsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/channels'), body: payload, contentType: 'application/json');
  }

  /// deleteChannel
  Future<dynamic> channelsDelete(String channelId) async {
    return _client.delete(ApiPaths.backendPath('/admin/channels/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}'));
  }

  /// listCredentials
  Future<dynamic> credentialsList() async {
    return _client.get(ApiPaths.backendPath('/admin/credentials'));
  }

  /// saveCredential
  Future<dynamic> credentialsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/credentials'), body: payload, contentType: 'application/json');
  }

  /// deleteCredential
  Future<dynamic> credentialsProvidersKeysDelete(String tenantId, String providerId, String keyReference) async {
    return _client.delete(ApiPaths.backendPath('/admin/credentials/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}/providers/${serializePathParameter(providerId, const PathParameterSpec('providerId', 'simple', false))}/keys/${serializePathParameter(keyReference, const PathParameterSpec('keyReference', 'simple', false))}'));
  }

  /// reloadExtensionRuntimes
  Future<dynamic> extensionsRuntimeReloadsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/extensions/runtime_reloads'), body: payload, contentType: 'application/json');
  }

  /// listRuntimeStatuses
  Future<dynamic> extensionsRuntimeStatusesList() async {
    return _client.get(ApiPaths.backendPath('/admin/extensions/runtime_statuses'));
  }

  /// listRateLimitPolicies
  Future<dynamic> gatewayRateLimitPoliciesList() async {
    return _client.get(ApiPaths.backendPath('/admin/gateway/rate_limit_policies'));
  }

  /// createRateLimitPolicy
  Future<dynamic> gatewayRateLimitPoliciesCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/gateway/rate_limit_policies'), body: payload, contentType: 'application/json');
  }

  /// listRateLimitWindows
  Future<dynamic> gatewayRateLimitWindowsList() async {
    return _client.get(ApiPaths.backendPath('/admin/gateway/rate_limit_windows'));
  }

  /// listMarketingCampaigns
  Future<dynamic> marketingCampaignsList() async {
    return _client.get(ApiPaths.backendPath('/admin/marketing/campaigns'));
  }

  /// saveMarketingCampaign
  Future<dynamic> marketingCampaignsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/marketing/campaigns'), body: payload, contentType: 'application/json');
  }

  /// updateMarketingCampaignStatus
  Future<dynamic> marketingCampaignsStatus(String marketingCampaignId, Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, const PathParameterSpec('marketingCampaignId', 'simple', false))}/status'), body: payload, contentType: 'application/json');
  }

  /// listModelPrices
  Future<dynamic> modelPricesList() async {
    return _client.get(ApiPaths.backendPath('/admin/model_prices'));
  }

  /// saveModelPrice
  Future<dynamic> modelPricesCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/model_prices'), body: payload, contentType: 'application/json');
  }

  /// deleteModelPrice
  Future<dynamic> modelPricesProvidersDelete(String channelId, String modelId, String proxyProviderId) async {
    return _client.delete(ApiPaths.backendPath('/admin/model_prices/${serializePathParameter(channelId, const PathParameterSpec('channelId', 'simple', false))}/models/${serializePathParameter(modelId, const PathParameterSpec('modelId', 'simple', false))}/providers/${serializePathParameter(proxyProviderId, const PathParameterSpec('proxyProviderId', 'simple', false))}'));
  }

  /// listModels
  Future<dynamic> modelsList() async {
    return _client.get(ApiPaths.backendPath('/admin/models'));
  }

  /// saveModel
  Future<dynamic> modelsCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/models'), body: payload, contentType: 'application/json');
  }

  /// deleteModel
  Future<dynamic> modelsProvidersDelete(String externalName, String providerId) async {
    return _client.delete(ApiPaths.backendPath('/admin/models/${serializePathParameter(externalName, const PathParameterSpec('externalName', 'simple', false))}/providers/${serializePathParameter(providerId, const PathParameterSpec('providerId', 'simple', false))}'));
  }

  /// listProviders
  Future<dynamic> providersList() async {
    return _client.get(ApiPaths.backendPath('/admin/providers'));
  }

  /// saveProvider
  Future<dynamic> providersCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/providers'), body: payload, contentType: 'application/json');
  }

  /// deleteProvider
  Future<dynamic> providersDelete(String providerId) async {
    return _client.delete(ApiPaths.backendPath('/admin/providers/${serializePathParameter(providerId, const PathParameterSpec('providerId', 'simple', false))}'));
  }

  /// listRoutingDecisionLogs
  Future<dynamic> routingDecisionLogsList() async {
    return _client.get(ApiPaths.backendPath('/admin/routing/decision_logs'));
  }

  /// listProviderHealthSnapshots
  Future<dynamic> routingHealthSnapshotsRetrieve() async {
    return _client.get(ApiPaths.backendPath('/admin/routing/health_snapshots'));
  }

  /// listRoutingProfiles
  Future<dynamic> routingProfilesList() async {
    return _client.get(ApiPaths.backendPath('/admin/routing/profiles'));
  }

  /// createRoutingProfile
  Future<dynamic> routingProfilesCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/routing/profiles'), body: payload, contentType: 'application/json');
  }

  /// listCompiledRoutingSnapshots
  Future<dynamic> routingSnapshotsList() async {
    return _client.get(ApiPaths.backendPath('/admin/routing/snapshots'));
  }

  /// listStorageAuditTrail
  Future<dynamic> storageAuditList() async {
    return _client.get(ApiPaths.backendPath('/admin/storage/audit'));
  }

  /// getGlobalStorageConfig
  Future<dynamic> storageConfigRetrieve() async {
    return _client.get(ApiPaths.backendPath('/admin/storage/config'));
  }

  /// saveGlobalStorageConfig
  Future<dynamic> storageConfigCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/storage/config'), body: payload, contentType: 'application/json');
  }

  /// getTenantStorageConfig
  Future<dynamic> storageConfigTenantsRetrieve(String tenantId) async {
    return _client.get(ApiPaths.backendPath('/admin/storage/config/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'));
  }

  /// saveTenantStorageConfig
  Future<dynamic> storageConfigTenantsCreate(String tenantId, Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/storage/config/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'), body: payload, contentType: 'application/json');
  }

  /// deleteTenantStorageConfig
  Future<dynamic> storageConfigTenantsDelete(String tenantId) async {
    return _client.delete(ApiPaths.backendPath('/admin/storage/config/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'));
  }

  /// getTenantEffectiveStorageConfig
  Future<dynamic> storageEffectiveTenantsRetrieve(String tenantId) async {
    return _client.get(ApiPaths.backendPath('/admin/storage/effective/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'));
  }

  /// listStorageProviders
  Future<dynamic> storageProvidersList() async {
    return _client.get(ApiPaths.backendPath('/admin/storage/providers'));
  }

  /// validateGlobalStorageConfig
  Future<dynamic> storageValidationCreate(Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/storage/validate'), body: payload, contentType: 'application/json');
  }

  /// validateTenantStorageConfig
  Future<dynamic> storageValidationTenantsCreate(String tenantId, Map<String, dynamic> body) async {
    final payload = body;
    return _client.post(ApiPaths.backendPath('/admin/storage/validate/tenants/${serializePathParameter(tenantId, const PathParameterSpec('tenantId', 'simple', false))}'), body: payload, contentType: 'application/json');
  }

  /// listUsageRecords
  Future<dynamic> usageRecordsList() async {
    return _client.get(ApiPaths.backendPath('/admin/usage/records'));
  }

  /// getUsageSummary
  Future<dynamic> usageSummaryRetrieve() async {
    return _client.get(ApiPaths.backendPath('/admin/usage/summary'));
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
