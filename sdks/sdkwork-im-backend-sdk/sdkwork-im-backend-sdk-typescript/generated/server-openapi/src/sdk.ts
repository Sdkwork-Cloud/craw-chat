import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { OpsApi, createOpsApi } from './api/ops';
import { AuditApi, createAuditApi } from './api/audit';
import { AutomationApi, createAutomationApi } from './api/automation';
import { ControlApi, createControlApi } from './api/control';
import { AdminApi, createAdminApi } from './api/admin';

export class SdkworkImBackendClient {
  private httpClient: HttpClient;

  public readonly ops: OpsApi;
  public readonly audit: AuditApi;
  public readonly automation: AutomationApi;
  public readonly control: ControlApi;
  public readonly admin: AdminApi;

  constructor(config: SdkworkBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.ops = createOpsApi(this.httpClient);

    this.audit = createAuditApi(this.httpClient);

    this.automation = createAutomationApi(this.httpClient);

    this.control = createControlApi(this.httpClient);

    this.admin = createAdminApi(this.httpClient);
  }
  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkBackendConfig): SdkworkImBackendClient {
  return new SdkworkImBackendClient(config);
}

export { SdkworkImBackendClient as SdkworkBackendClient };

export default SdkworkImBackendClient;
