import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { PortalSnapshot, PortalWorkspaceView } from '../types';


export class PortalWorkspaceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the current tenant workspace snapshot */
  async retrieve(): Promise<PortalWorkspaceView> {
    return this.client.get<PortalWorkspaceView>(appApiPath(`/portal/workspace`));
  }
}

export class PortalRealtimeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant realtime snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/realtime`));
  }
}

export class PortalMediaApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant media snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/media`));
  }
}

export class PortalHomeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant portal home snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/home`));
  }
}

export class PortalGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant governance snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/governance`));
  }
}

export class PortalDashboardApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant dashboard snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/dashboard`));
  }
}

export class PortalConversationSnapshotApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant conversations snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/conversations`));
  }
}

export class PortalAutomationApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant automation snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/automation`));
  }
}

export class PortalAccessApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Read the tenant portal sign-in snapshot */
  async retrieve(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(appApiPath(`/portal/access`));
  }
}

export class PortalApi {
  private client: HttpClient;
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
    this.client = client;
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

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
