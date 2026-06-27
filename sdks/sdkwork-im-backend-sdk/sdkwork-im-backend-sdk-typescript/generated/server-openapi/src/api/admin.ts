import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { LooseJsonObject, LooseJsonValue } from '../types';


export class AdminUsageSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getUsageSummary */
  async retrieve(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/usage/summary`));
  }
}

export class AdminUsageRecordsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listUsageRecords */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/usage/records`));
  }
}

export class AdminUsageApi {
  private client: HttpClient;
  public readonly records: AdminUsageRecordsApi;
  public readonly summary: AdminUsageSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.records = new AdminUsageRecordsApi(client);
    this.summary = new AdminUsageSummaryApi(client);
  }

}

export class AdminStorageValidationTenantsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** validateTenantStorageConfig */
  async create(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/storage/validate/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class AdminStorageValidationApi {
  private client: HttpClient;
  public readonly tenants: AdminStorageValidationTenantsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.tenants = new AdminStorageValidationTenantsApi(client);
  }


/** validateGlobalStorageConfig */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/storage/validate`), body, undefined, undefined, 'application/json');
  }
}

export class AdminStorageProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listStorageProviders */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/storage/providers`));
  }
}

export class AdminStorageEffectiveTenantsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getTenantEffectiveStorageConfig */
  async retrieve(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/storage/effective/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`));
  }
}

export class AdminStorageEffectiveApi {
  private client: HttpClient;
  public readonly tenants: AdminStorageEffectiveTenantsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.tenants = new AdminStorageEffectiveTenantsApi(client);
  }

}

export class AdminStorageConfigTenantsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getTenantStorageConfig */
  async retrieve(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`));
  }

/** saveTenantStorageConfig */
  async create(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** deleteTenantStorageConfig */
  async delete(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`));
  }
}

export class AdminStorageConfigApi {
  private client: HttpClient;
  public readonly tenants: AdminStorageConfigTenantsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.tenants = new AdminStorageConfigTenantsApi(client);
  }


/** getGlobalStorageConfig */
  async retrieve(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/storage/config`));
  }

/** saveGlobalStorageConfig */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/storage/config`), body, undefined, undefined, 'application/json');
  }
}

export class AdminStorageAuditApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listStorageAuditTrail */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/storage/audit`));
  }
}

export class AdminStorageApi {
  private client: HttpClient;
  public readonly audit: AdminStorageAuditApi;
  public readonly config: AdminStorageConfigApi;
  public readonly effective: AdminStorageEffectiveApi;
  public readonly providers: AdminStorageProvidersApi;
  public readonly validation: AdminStorageValidationApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.audit = new AdminStorageAuditApi(client);
    this.config = new AdminStorageConfigApi(client);
    this.effective = new AdminStorageEffectiveApi(client);
    this.providers = new AdminStorageProvidersApi(client);
    this.validation = new AdminStorageValidationApi(client);
  }

}

export class AdminRoutingSnapshotsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listCompiledRoutingSnapshots */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/routing/snapshots`));
  }
}

export class AdminRoutingProfilesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRoutingProfiles */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/routing/profiles`));
  }

/** createRoutingProfile */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/routing/profiles`), body, undefined, undefined, 'application/json');
  }
}

export class AdminRoutingHealthSnapshotsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listProviderHealthSnapshots */
  async retrieve(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/routing/health_snapshots`));
  }
}

export class AdminRoutingDecisionLogsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRoutingDecisionLogs */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/routing/decision_logs`));
  }
}

export class AdminRoutingApi {
  private client: HttpClient;
  public readonly decisionLogs: AdminRoutingDecisionLogsApi;
  public readonly healthSnapshots: AdminRoutingHealthSnapshotsApi;
  public readonly profiles: AdminRoutingProfilesApi;
  public readonly snapshots: AdminRoutingSnapshotsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.decisionLogs = new AdminRoutingDecisionLogsApi(client);
    this.healthSnapshots = new AdminRoutingHealthSnapshotsApi(client);
    this.profiles = new AdminRoutingProfilesApi(client);
    this.snapshots = new AdminRoutingSnapshotsApi(client);
  }

}

export class AdminProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listProviders */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/providers`));
  }

/** saveProvider */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/providers`), body, undefined, undefined, 'application/json');
  }

/** deleteProvider */
  async delete(providerId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}`));
  }
}

export class AdminModelsProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteModel */
  async delete(externalName: string | number, providerId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/models/${serializePathParameter(externalName, { name: 'externalName', style: 'simple', explode: false })}/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}`));
  }
}

export class AdminModelsApi {
  private client: HttpClient;
  public readonly providers: AdminModelsProvidersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.providers = new AdminModelsProvidersApi(client);
  }


/** listModels */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/models`));
  }

/** saveModel */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/models`), body, undefined, undefined, 'application/json');
  }
}

export class AdminModelPricesModelsProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteModelPrice */
  async delete(channelId: string | number, modelId: string | number, proxyProviderId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/model_prices/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/models/${serializePathParameter(modelId, { name: 'modelId', style: 'simple', explode: false })}/providers/${serializePathParameter(proxyProviderId, { name: 'proxyProviderId', style: 'simple', explode: false })}`));
  }
}

export class AdminModelPricesModelsApi {
  private client: HttpClient;
  public readonly providers: AdminModelPricesModelsProvidersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.providers = new AdminModelPricesModelsProvidersApi(client);
  }

}

export class AdminModelPricesApi {
  private client: HttpClient;
  public readonly models: AdminModelPricesModelsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.models = new AdminModelPricesModelsApi(client);
  }


/** listModelPrices */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/model_prices`));
  }

/** saveModelPrice */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/model_prices`), body, undefined, undefined, 'application/json');
  }
}

export class AdminMarketingCampaignsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listMarketingCampaigns */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/marketing/campaigns`));
  }

/** saveMarketingCampaign */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/marketing/campaigns`), body, undefined, undefined, 'application/json');
  }

/** updateMarketingCampaignStatus */
  async status(marketingCampaignId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, { name: 'marketingCampaignId', style: 'simple', explode: false })}/status`), body, undefined, undefined, 'application/json');
  }
}

export class AdminMarketingApi {
  private client: HttpClient;
  public readonly campaigns: AdminMarketingCampaignsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.campaigns = new AdminMarketingCampaignsApi(client);
  }

}

export class AdminGatewayRateLimitWindowsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRateLimitWindows */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/gateway/rate_limit_windows`));
  }
}

export class AdminGatewayRateLimitPoliciesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRateLimitPolicies */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/gateway/rate_limit_policies`));
  }

/** createRateLimitPolicy */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/gateway/rate_limit_policies`), body, undefined, undefined, 'application/json');
  }
}

export class AdminGatewayApi {
  private client: HttpClient;
  public readonly rateLimitPolicies: AdminGatewayRateLimitPoliciesApi;
  public readonly rateLimitWindows: AdminGatewayRateLimitWindowsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.rateLimitPolicies = new AdminGatewayRateLimitPoliciesApi(client);
    this.rateLimitWindows = new AdminGatewayRateLimitWindowsApi(client);
  }

}

export class AdminExtensionsRuntimeStatusesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRuntimeStatuses */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/extensions/runtime_statuses`));
  }
}

export class AdminExtensionsRuntimeReloadsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** reloadExtensionRuntimes */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/extensions/runtime_reloads`), body, undefined, undefined, 'application/json');
  }
}

export class AdminExtensionsApi {
  private client: HttpClient;
  public readonly runtimeReloads: AdminExtensionsRuntimeReloadsApi;
  public readonly runtimeStatuses: AdminExtensionsRuntimeStatusesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.runtimeReloads = new AdminExtensionsRuntimeReloadsApi(client);
    this.runtimeStatuses = new AdminExtensionsRuntimeStatusesApi(client);
  }

}

export class AdminCredentialsProvidersKeysApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteCredential */
  async delete(tenantId: string | number, providerId: string | number, keyReference: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/credentials/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}/keys/${serializePathParameter(keyReference, { name: 'keyReference', style: 'simple', explode: false })}`));
  }
}

export class AdminCredentialsProvidersApi {
  private client: HttpClient;
  public readonly keys: AdminCredentialsProvidersKeysApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.keys = new AdminCredentialsProvidersKeysApi(client);
  }

}

export class AdminCredentialsApi {
  private client: HttpClient;
  public readonly providers: AdminCredentialsProvidersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.providers = new AdminCredentialsProvidersApi(client);
  }


/** listCredentials */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/credentials`));
  }

/** saveCredential */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/credentials`), body, undefined, undefined, 'application/json');
  }
}

export class AdminChannelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listChannels */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/channels`));
  }

/** saveChannel */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/channels`), body, undefined, undefined, 'application/json');
  }

/** deleteChannel */
  async delete(channelId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`));
  }
}

export class AdminChannelModelsModelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteChannelModel */
  async delete(channelId: string | number, modelId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/channel_models/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/models/${serializePathParameter(modelId, { name: 'modelId', style: 'simple', explode: false })}`));
  }
}

export class AdminChannelModelsApi {
  private client: HttpClient;
  public readonly models: AdminChannelModelsModelsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.models = new AdminChannelModelsModelsApi(client);
  }


/** listChannelModels */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/channel_models`));
  }

/** saveChannelModel */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/channel_models`), body, undefined, undefined, 'application/json');
  }
}

export class AdminBillingSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getBillingSummary */
  async retrieve(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/billing/summary`));
  }
}

export class AdminBillingEventsSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getBillingEventSummary */
  async retrieve(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/billing/events/summary`));
  }
}

export class AdminBillingEventsApi {
  private client: HttpClient;
  public readonly summary: AdminBillingEventsSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.summary = new AdminBillingEventsSummaryApi(client);
  }


/** listBillingEvents */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/billing/events`));
  }
}

export class AdminBillingApi {
  private client: HttpClient;
  public readonly events: AdminBillingEventsApi;
  public readonly summary: AdminBillingSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.events = new AdminBillingEventsApi(client);
    this.summary = new AdminBillingSummaryApi(client);
  }

}

export class AdminApiKeysApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listApiKeys */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/api_keys`));
  }

/** createApiKey */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/api_keys`), body, undefined, undefined, 'application/json');
  }

/** updateApiKey */
  async update(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.put<LooseJsonValue>(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** deleteApiKey */
  async delete(hashedKey: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}`));
  }

/** updateApiKeyStatus */
  async status(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}/status`), body, undefined, undefined, 'application/json');
  }
}

export class AdminApiKeyGroupsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listApiKeyGroups */
  async list(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/admin/api_key_groups`));
  }

/** createApiKeyGroup */
  async create(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/api_key_groups`), body, undefined, undefined, 'application/json');
  }

/** updateApiKeyGroup */
  async update(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.patch<LooseJsonValue>(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** deleteApiKeyGroup */
  async delete(groupId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`));
  }

/** updateApiKeyGroupStatus */
  async status(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/status`), body, undefined, undefined, 'application/json');
  }
}

export class AdminApi {
  private client: HttpClient;
  public readonly apiKeyGroups: AdminApiKeyGroupsApi;
  public readonly apiKeys: AdminApiKeysApi;
  public readonly billing: AdminBillingApi;
  public readonly channelModels: AdminChannelModelsApi;
  public readonly channels: AdminChannelsApi;
  public readonly credentials: AdminCredentialsApi;
  public readonly extensions: AdminExtensionsApi;
  public readonly gateway: AdminGatewayApi;
  public readonly marketing: AdminMarketingApi;
  public readonly modelPrices: AdminModelPricesApi;
  public readonly models: AdminModelsApi;
  public readonly providers: AdminProvidersApi;
  public readonly routing: AdminRoutingApi;
  public readonly storage: AdminStorageApi;
  public readonly usage: AdminUsageApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.apiKeyGroups = new AdminApiKeyGroupsApi(client);
    this.apiKeys = new AdminApiKeysApi(client);
    this.billing = new AdminBillingApi(client);
    this.channelModels = new AdminChannelModelsApi(client);
    this.channels = new AdminChannelsApi(client);
    this.credentials = new AdminCredentialsApi(client);
    this.extensions = new AdminExtensionsApi(client);
    this.gateway = new AdminGatewayApi(client);
    this.marketing = new AdminMarketingApi(client);
    this.modelPrices = new AdminModelPricesApi(client);
    this.models = new AdminModelsApi(client);
    this.providers = new AdminProvidersApi(client);
    this.routing = new AdminRoutingApi(client);
    this.storage = new AdminStorageApi(client);
    this.usage = new AdminUsageApi(client);
  }

}

export function createAdminApi(client: HttpClient): AdminApi {
  return new AdminApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
