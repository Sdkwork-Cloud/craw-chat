import { backendApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';



export class AutomationGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve automation governance */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/automation/governance`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class AutomationApi {
  public readonly governance: AutomationGovernanceApi;

  constructor(client: HttpClient) {
    this.governance = new AutomationGovernanceApi(client);
  }

}

export function createAutomationApi(client: HttpClient): AutomationApi {
  return new AutomationApi(client);
}
