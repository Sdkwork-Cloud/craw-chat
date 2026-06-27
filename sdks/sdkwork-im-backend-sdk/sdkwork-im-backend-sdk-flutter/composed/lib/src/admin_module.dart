import 'package:im_backend_api_generated/im_backend_api_generated.dart';

import 'context.dart';

class ImBackendAdminModule {
  final ImBackendSdkContext context;

  ImBackendAdminModule(this.context);

  AdminApi get raw => context.transportClient.admin;

  Future<dynamic> listApiKeyGroups() {
    return apiKeyGroupsList();
  }

  Future<dynamic> createApiKeyGroup(Map<String, dynamic> body) {
    return apiKeyGroupsCreate(body);
  }

  Future<dynamic> updateApiKeyGroup(String groupId, Map<String, dynamic> body) {
    return apiKeyGroupsUpdate(groupId, body);
  }

  Future<dynamic> deleteApiKeyGroup(String groupId) {
    return apiKeyGroupsDelete(groupId);
  }

  Future<dynamic> updateApiKeyGroupStatus(
    String groupId,
    Map<String, dynamic> body,
  ) {
    return apiKeyGroupsStatus(groupId, body);
  }

  Future<dynamic> listApiKeys() {
    return apiKeysList();
  }

  Future<dynamic> createApiKey(Map<String, dynamic> body) {
    return apiKeysCreate(body);
  }

  Future<dynamic> updateApiKey(String hashedKey, Map<String, dynamic> body) {
    return apiKeysUpdate(hashedKey, body);
  }

  Future<dynamic> deleteApiKey(String hashedKey) {
    return apiKeysDelete(hashedKey);
  }

  Future<dynamic> updateApiKeyStatus(
    String hashedKey,
    Map<String, dynamic> body,
  ) {
    return apiKeysStatus(hashedKey, body);
  }

  Future<dynamic> apiKeyGroupsList() {
    return context.transportClient.admin.apiKeyGroupsList();
  }

  Future<dynamic> apiKeyGroupsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.apiKeyGroupsCreate(body);
  }

  Future<dynamic> apiKeyGroupsUpdate(
    String groupId,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.apiKeyGroupsUpdate(groupId, body);
  }

  Future<dynamic> apiKeyGroupsDelete(String groupId) {
    return context.transportClient.admin.apiKeyGroupsDelete(groupId);
  }

  Future<dynamic> apiKeyGroupsStatus(
    String groupId,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.apiKeyGroupsStatus(groupId, body);
  }

  Future<dynamic> apiKeysList() {
    return context.transportClient.admin.apiKeysList();
  }

  Future<dynamic> apiKeysCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.apiKeysCreate(body);
  }

  Future<dynamic> apiKeysUpdate(
    String hashedKey,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.apiKeysUpdate(hashedKey, body);
  }

  Future<dynamic> apiKeysDelete(String hashedKey) {
    return context.transportClient.admin.apiKeysDelete(hashedKey);
  }

  Future<dynamic> apiKeysStatus(
    String hashedKey,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.apiKeysStatus(hashedKey, body);
  }

  Future<dynamic> billingEventsList() {
    return context.transportClient.admin.billingEventsList();
  }

  Future<dynamic> billingEventsSummaryRetrieve() {
    return context.transportClient.admin.billingEventsSummaryRetrieve();
  }

  Future<dynamic> billingSummaryRetrieve() {
    return context.transportClient.admin.billingSummaryRetrieve();
  }

  Future<dynamic> channelModelsList() {
    return context.transportClient.admin.channelModelsList();
  }

  Future<dynamic> channelModelsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.channelModelsCreate(body);
  }

  Future<dynamic> channelModelsDelete(String channelId, String modelId) {
    return context.transportClient.admin
        .channelModelsDelete(channelId, modelId);
  }

  Future<dynamic> channelsList() {
    return context.transportClient.admin.channelsList();
  }

  Future<dynamic> channelsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.channelsCreate(body);
  }

  Future<dynamic> channelsDelete(String channelId) {
    return context.transportClient.admin.channelsDelete(channelId);
  }

  Future<dynamic> credentialsList() {
    return context.transportClient.admin.credentialsList();
  }

  Future<dynamic> credentialsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.credentialsCreate(body);
  }

  Future<dynamic> credentialsProvidersKeysDelete(
    String tenantId,
    String providerId,
    String keyReference,
  ) {
    return context.transportClient.admin.credentialsProvidersKeysDelete(
      tenantId,
      providerId,
      keyReference,
    );
  }

  Future<dynamic> extensionsRuntimeReloadsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.extensionsRuntimeReloadsCreate(body);
  }

  Future<dynamic> extensionsRuntimeStatusesList() {
    return context.transportClient.admin.extensionsRuntimeStatusesList();
  }

  Future<dynamic> gatewayRateLimitPoliciesList() {
    return context.transportClient.admin.gatewayRateLimitPoliciesList();
  }

  Future<dynamic> gatewayRateLimitPoliciesCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.gatewayRateLimitPoliciesCreate(body);
  }

  Future<dynamic> gatewayRateLimitWindowsList() {
    return context.transportClient.admin.gatewayRateLimitWindowsList();
  }

  Future<dynamic> marketingCampaignsList() {
    return context.transportClient.admin.marketingCampaignsList();
  }

  Future<dynamic> marketingCampaignsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.marketingCampaignsCreate(body);
  }

  Future<dynamic> marketingCampaignsStatus(
    String marketingCampaignId,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.marketingCampaignsStatus(
      marketingCampaignId,
      body,
    );
  }

  Future<dynamic> modelPricesList() {
    return context.transportClient.admin.modelPricesList();
  }

  Future<dynamic> modelPricesCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.modelPricesCreate(body);
  }

  Future<dynamic> modelPricesProvidersDelete(
    String channelId,
    String modelId,
    String proxyProviderId,
  ) {
    return context.transportClient.admin.modelPricesProvidersDelete(
      channelId,
      modelId,
      proxyProviderId,
    );
  }

  Future<dynamic> modelsList() {
    return context.transportClient.admin.modelsList();
  }

  Future<dynamic> modelsCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.modelsCreate(body);
  }

  Future<dynamic> modelsProvidersDelete(
      String externalName, String providerId) {
    return context.transportClient.admin.modelsProvidersDelete(
      externalName,
      providerId,
    );
  }

  Future<dynamic> providersList() {
    return context.transportClient.admin.providersList();
  }

  Future<dynamic> providersCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.providersCreate(body);
  }

  Future<dynamic> providersDelete(String providerId) {
    return context.transportClient.admin.providersDelete(providerId);
  }

  Future<dynamic> routingDecisionLogsList() {
    return context.transportClient.admin.routingDecisionLogsList();
  }

  Future<dynamic> routingHealthSnapshotsRetrieve() {
    return context.transportClient.admin.routingHealthSnapshotsRetrieve();
  }

  Future<dynamic> routingProfilesList() {
    return context.transportClient.admin.routingProfilesList();
  }

  Future<dynamic> routingProfilesCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.routingProfilesCreate(body);
  }

  Future<dynamic> routingSnapshotsList() {
    return context.transportClient.admin.routingSnapshotsList();
  }

  Future<dynamic> storageAuditList() {
    return context.transportClient.admin.storageAuditList();
  }

  Future<dynamic> storageConfigRetrieve() {
    return context.transportClient.admin.storageConfigRetrieve();
  }

  Future<dynamic> storageConfigCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.storageConfigCreate(body);
  }

  Future<dynamic> storageConfigTenantsRetrieve(String tenantId) {
    return context.transportClient.admin.storageConfigTenantsRetrieve(tenantId);
  }

  Future<dynamic> storageConfigTenantsCreate(
    String tenantId,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.storageConfigTenantsCreate(
      tenantId,
      body,
    );
  }

  Future<dynamic> storageConfigTenantsDelete(String tenantId) {
    return context.transportClient.admin.storageConfigTenantsDelete(tenantId);
  }

  Future<dynamic> storageEffectiveTenantsRetrieve(String tenantId) {
    return context.transportClient.admin.storageEffectiveTenantsRetrieve(
      tenantId,
    );
  }

  Future<dynamic> storageProvidersList() {
    return context.transportClient.admin.storageProvidersList();
  }

  Future<dynamic> storageValidationCreate(Map<String, dynamic> body) {
    return context.transportClient.admin.storageValidationCreate(body);
  }

  Future<dynamic> storageValidationTenantsCreate(
    String tenantId,
    Map<String, dynamic> body,
  ) {
    return context.transportClient.admin.storageValidationTenantsCreate(
      tenantId,
      body,
    );
  }

  Future<dynamic> usageRecordsList() {
    return context.transportClient.admin.usageRecordsList();
  }

  Future<dynamic> usageSummaryRetrieve() {
    return context.transportClient.admin.usageSummaryRetrieve();
  }
}
