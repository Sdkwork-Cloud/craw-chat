import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';



export class AutomationGovernanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve automation governance */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/automation/governance`));
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
