import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { PortalAccessSnapshot, PortalConversationSnapshot, PortalDashboardSnapshot, PortalGovernanceSnapshot, PortalModuleSnapshot, PortalRealtimeSnapshot, PortalWorkspaceView } from '../types';


export class PortalWorkspaceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the current tenant workspace snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalWorkspaceView> {
    return this.client.request<PortalWorkspaceView>(appApiPath(`/portal/workspace`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalRealtimeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant realtime snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalRealtimeSnapshot> {
    return this.client.request<PortalRealtimeSnapshot>(appApiPath(`/portal/realtime`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalMediaApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant media snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalModuleSnapshot> {
    return this.client.request<PortalModuleSnapshot>(appApiPath(`/portal/media`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalHomeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant portal home snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalModuleSnapshot> {
    return this.client.request<PortalModuleSnapshot>(appApiPath(`/portal/home`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant governance snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalGovernanceSnapshot> {
    return this.client.request<PortalGovernanceSnapshot>(appApiPath(`/portal/governance`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalDashboardApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant dashboard snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalDashboardSnapshot> {
    return this.client.request<PortalDashboardSnapshot>(appApiPath(`/portal/dashboard`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalConversationSnapshotApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant conversations snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalConversationSnapshot> {
    return this.client.request<PortalConversationSnapshot>(appApiPath(`/portal/conversations`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalAutomationApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant automation snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalModuleSnapshot> {
    return this.client.request<PortalModuleSnapshot>(appApiPath(`/portal/automation`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalAccessApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant portal access snapshot */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PortalAccessSnapshot> {
    return this.client.request<PortalAccessSnapshot>(appApiPath(`/portal/access`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PortalApi {
  public readonly access: PortalAccessApi;
  public readonly automation: PortalAutomationApi;
  public readonly conversationSnapshot: PortalConversationSnapshotApi;
  public readonly dashboard: PortalDashboardApi;
  public readonly governance: PortalGovernanceApi;
  public readonly home: PortalHomeApi;
  public readonly media: PortalMediaApi;
  public readonly realtime: PortalRealtimeApi;
  public readonly workspace: PortalWorkspaceApi;

  constructor(client: HttpClient) {
    this.access = new PortalAccessApi(client);
    this.automation = new PortalAutomationApi(client);
    this.conversationSnapshot = new PortalConversationSnapshotApi(client);
    this.dashboard = new PortalDashboardApi(client);
    this.governance = new PortalGovernanceApi(client);
    this.home = new PortalHomeApi(client);
    this.media = new PortalMediaApi(client);
    this.realtime = new PortalRealtimeApi(client);
    this.workspace = new PortalWorkspaceApi(client);
  }

}

export function createPortalApi(client: HttpClient): PortalApi {
  return new PortalApi(client);
}
