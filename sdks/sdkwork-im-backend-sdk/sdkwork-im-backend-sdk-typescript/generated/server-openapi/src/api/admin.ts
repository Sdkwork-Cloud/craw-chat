import { backendApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { LooseJsonObject, LooseJsonValue, SdkWorkPageData } from '../types';


export class AdminUsageSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getUsageSummary */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/usage/summary`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminUsageRecordsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminUsageRecordsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listUsageRecords */
  async list(params?: AdminUsageRecordsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/usage/records`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class AdminUsageApi {
  public readonly records: AdminUsageRecordsApi;
  public readonly summary: AdminUsageSummaryApi;

  constructor(client: HttpClient) {
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
  async create(tenantId: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/validate/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
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
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/validate`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminStorageProvidersListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminStorageProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listStorageProviders */
  async list(params?: AdminStorageProvidersListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/storage/providers`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class AdminStorageEffectiveTenantsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getTenantEffectiveStorageConfig */
  async retrieve(tenantId: string | number, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/effective/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class AdminStorageEffectiveApi {
  public readonly tenants: AdminStorageEffectiveTenantsApi;

  constructor(client: HttpClient) {
    this.tenants = new AdminStorageEffectiveTenantsApi(client);
  }

}

export class AdminStorageConfigTenantsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getTenantStorageConfig */
  async retrieve(tenantId: string | number, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** saveTenantStorageConfig */
  async create(tenantId: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** deleteTenantStorageConfig */
  async delete(tenantId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
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
  async retrieve(requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/config`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** saveGlobalStorageConfig */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/storage/config`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminStorageAuditListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminStorageAuditApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listStorageAuditTrail */
  async list(params?: AdminStorageAuditListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/storage/audit`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class AdminStorageApi {
  public readonly audit: AdminStorageAuditApi;
  public readonly config: AdminStorageConfigApi;
  public readonly effective: AdminStorageEffectiveApi;
  public readonly providers: AdminStorageProvidersApi;
  public readonly validation: AdminStorageValidationApi;

  constructor(client: HttpClient) {
    this.audit = new AdminStorageAuditApi(client);
    this.config = new AdminStorageConfigApi(client);
    this.effective = new AdminStorageEffectiveApi(client);
    this.providers = new AdminStorageProvidersApi(client);
    this.validation = new AdminStorageValidationApi(client);
  }

}

export interface AdminRoutingSnapshotsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminRoutingSnapshotsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listCompiledRoutingSnapshots */
  async list(params?: AdminRoutingSnapshotsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/routing/snapshots`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface AdminRoutingProfilesListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminRoutingProfilesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRoutingProfiles */
  async list(params?: AdminRoutingProfilesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/routing/profiles`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** createRoutingProfile */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/routing/profiles`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminRoutingHealthSnapshotsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listProviderHealthSnapshots */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/routing/health_snapshots`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminRoutingDecisionLogsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminRoutingDecisionLogsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRoutingDecisionLogs */
  async list(params?: AdminRoutingDecisionLogsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/routing/decision_logs`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class AdminRoutingApi {
  public readonly decisionLogs: AdminRoutingDecisionLogsApi;
  public readonly healthSnapshots: AdminRoutingHealthSnapshotsApi;
  public readonly profiles: AdminRoutingProfilesApi;
  public readonly snapshots: AdminRoutingSnapshotsApi;

  constructor(client: HttpClient) {
    this.decisionLogs = new AdminRoutingDecisionLogsApi(client);
    this.healthSnapshots = new AdminRoutingHealthSnapshotsApi(client);
    this.profiles = new AdminRoutingProfilesApi(client);
    this.snapshots = new AdminRoutingSnapshotsApi(client);
  }

}

export interface AdminProvidersListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listProviders */
  async list(params?: AdminProvidersListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/providers`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveProvider */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/providers`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** deleteProvider */
  async delete(providerId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class AdminModelsProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteModel */
  async delete(externalName: string | number, providerId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/models/${serializePathParameter(externalName, { name: 'externalName', style: 'simple', explode: false })}/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export interface AdminModelsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminModelsApi {
  private client: HttpClient;
  public readonly providers: AdminModelsProvidersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.providers = new AdminModelsProvidersApi(client);
  }


/** listModels */
  async list(params?: AdminModelsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/models`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveModel */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/models`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminModelPricesModelsProvidersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteModelPrice */
  async delete(channelId: string | number, modelId: string | number, proxyProviderId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/model_prices/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/models/${serializePathParameter(modelId, { name: 'modelId', style: 'simple', explode: false })}/providers/${serializePathParameter(proxyProviderId, { name: 'proxyProviderId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class AdminModelPricesModelsApi {
  public readonly providers: AdminModelPricesModelsProvidersApi;

  constructor(client: HttpClient) {
    this.providers = new AdminModelPricesModelsProvidersApi(client);
  }

}

export interface AdminModelPricesListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminModelPricesApi {
  private client: HttpClient;
  public readonly models: AdminModelPricesModelsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.models = new AdminModelPricesModelsApi(client);
  }


/** listModelPrices */
  async list(params?: AdminModelPricesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/model_prices`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveModelPrice */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/model_prices`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminMarketingCampaignsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminMarketingCampaignsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listMarketingCampaigns */
  async list(params?: AdminMarketingCampaignsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/marketing/campaigns`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveMarketingCampaign */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/marketing/campaigns`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** updateMarketingCampaignStatus */
  async status(marketingCampaignId: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, { name: 'marketingCampaignId', style: 'simple', explode: false })}/status`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminMarketingApi {
  public readonly campaigns: AdminMarketingCampaignsApi;

  constructor(client: HttpClient) {
    this.campaigns = new AdminMarketingCampaignsApi(client);
  }

}

export interface AdminGatewayRateLimitWindowsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminGatewayRateLimitWindowsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRateLimitWindows */
  async list(params?: AdminGatewayRateLimitWindowsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/gateway/rate_limit_windows`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface AdminGatewayRateLimitPoliciesListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminGatewayRateLimitPoliciesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRateLimitPolicies */
  async list(params?: AdminGatewayRateLimitPoliciesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/gateway/rate_limit_policies`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** createRateLimitPolicy */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/gateway/rate_limit_policies`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminGatewayApi {
  public readonly rateLimitPolicies: AdminGatewayRateLimitPoliciesApi;
  public readonly rateLimitWindows: AdminGatewayRateLimitWindowsApi;

  constructor(client: HttpClient) {
    this.rateLimitPolicies = new AdminGatewayRateLimitPoliciesApi(client);
    this.rateLimitWindows = new AdminGatewayRateLimitWindowsApi(client);
  }

}

export interface AdminExtensionsRuntimeStatusesListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminExtensionsRuntimeStatusesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listRuntimeStatuses */
  async list(params?: AdminExtensionsRuntimeStatusesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/extensions/runtime_statuses`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class AdminExtensionsRuntimeReloadsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** reloadExtensionRuntimes */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/extensions/runtime_reloads`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminExtensionsApi {
  public readonly runtimeReloads: AdminExtensionsRuntimeReloadsApi;
  public readonly runtimeStatuses: AdminExtensionsRuntimeStatusesApi;

  constructor(client: HttpClient) {
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
  async delete(tenantId: string | number, providerId: string | number, keyReference: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/credentials/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}/keys/${serializePathParameter(keyReference, { name: 'keyReference', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class AdminCredentialsProvidersApi {
  public readonly keys: AdminCredentialsProvidersKeysApi;

  constructor(client: HttpClient) {
    this.keys = new AdminCredentialsProvidersKeysApi(client);
  }

}

export interface AdminCredentialsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminCredentialsApi {
  private client: HttpClient;
  public readonly providers: AdminCredentialsProvidersApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.providers = new AdminCredentialsProvidersApi(client);
  }


/** listCredentials */
  async list(params?: AdminCredentialsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/credentials`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveCredential */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/credentials`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminChannelsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminChannelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listChannels */
  async list(params?: AdminChannelsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/channels`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveChannel */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/channels`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** deleteChannel */
  async delete(channelId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class AdminChannelModelsModelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** deleteChannelModel */
  async delete(channelId: string | number, modelId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/channel_models/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/models/${serializePathParameter(modelId, { name: 'modelId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export interface AdminChannelModelsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminChannelModelsApi {
  private client: HttpClient;
  public readonly models: AdminChannelModelsModelsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.models = new AdminChannelModelsModelsApi(client);
  }


/** listChannelModels */
  async list(params?: AdminChannelModelsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/channel_models`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** saveChannelModel */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/channel_models`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminBillingSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getBillingSummary */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/billing/summary`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class AdminBillingEventsSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** getBillingEventSummary */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/billing/events/summary`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminBillingEventsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminBillingEventsApi {
  private client: HttpClient;
  public readonly summary: AdminBillingEventsSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.summary = new AdminBillingEventsSummaryApi(client);
  }


/** listBillingEvents */
  async list(params?: AdminBillingEventsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/billing/events`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class AdminBillingApi {
  public readonly events: AdminBillingEventsApi;
  public readonly summary: AdminBillingSummaryApi;

  constructor(client: HttpClient) {
    this.events = new AdminBillingEventsApi(client);
    this.summary = new AdminBillingSummaryApi(client);
  }

}

export interface AdminApiKeysListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminApiKeysApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listApiKeys */
  async list(params?: AdminApiKeysListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/api_keys`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** createApiKey */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/api_keys`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** updateApiKey */
  async update(hashedKey: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** deleteApiKey */
  async delete(hashedKey: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }

/** updateApiKeyStatus */
  async status(hashedKey: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}/status`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface AdminApiKeyGroupsListParams {
  pageSize?: number;
  cursor?: string;
  page?: number;
  q?: string;
}

export class AdminApiKeyGroupsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** listApiKeyGroups */
  async list(params?: AdminApiKeyGroupsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/admin/api_key_groups`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** createApiKeyGroup */
  async create(body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/api_key_groups`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** updateApiKeyGroup */
  async update(groupId: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** deleteApiKeyGroup */
  async delete(groupId: string | number, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }

/** updateApiKeyGroupStatus */
  async status(groupId: string | number, body: LooseJsonObject, requestOptions?: ApiRequestOptions): Promise<LooseJsonValue> {
    return this.client.request<LooseJsonValue>(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/status`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class AdminApi {
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
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
