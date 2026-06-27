import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { AutomationApi, createAutomationApi } from './api/automation';
import { NotificationsApi, createNotificationsApi } from './api/notifications';
import { PortalApi, createPortalApi } from './api/portal';
import { ProviderApi, createProviderApi } from './api/provider';

export class SdkworkImAppClient {
  private httpClient: HttpClient;

  public readonly automation: AutomationApi;
  public readonly notifications: NotificationsApi;
  public readonly portal: PortalApi;
  public readonly provider: ProviderApi;

  constructor(config: SdkworkAppConfig) {
    this.httpClient = createHttpClient(config);
    this.automation = createAutomationApi(this.httpClient);

    this.notifications = createNotificationsApi(this.httpClient);

    this.portal = createPortalApi(this.httpClient);

    this.provider = createProviderApi(this.httpClient);
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

export function createClient(config: SdkworkAppConfig): SdkworkImAppClient {
  return new SdkworkImAppClient(config);
}

export { SdkworkImAppClient as SdkworkAppClient };

export default SdkworkImAppClient;
