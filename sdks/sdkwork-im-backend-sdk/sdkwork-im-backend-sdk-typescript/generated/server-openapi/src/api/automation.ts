import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { GovernanceRetrieveResponse } from '../types';


export class AutomationGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve automation governance */
  async retrieve(): Promise<GovernanceRetrieveResponse> {
    return this.client.get<GovernanceRetrieveResponse>(backendApiPath(`/automation/governance`));
  }
}

export class AutomationApi {
  private client: HttpClient;
  public readonly governance: AutomationGovernanceApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.governance = new AutomationGovernanceApi(client);
  }

}

export function createAutomationApi(client: HttpClient): AutomationApi {
  return new AutomationApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
