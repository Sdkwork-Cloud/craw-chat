import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkImConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { PresenceApi, createPresenceApi } from './api/presence';
import { RealtimeApi, createRealtimeApi } from './api/realtime';
import { CallsApi, createCallsApi } from './api/calls';
import { SocialApi, createSocialApi } from './api/social';
import { ChatApi, createChatApi } from './api/chat';
import { StreamsApi, createStreamsApi } from './api/streams';
import { SpacesApi, createSpacesApi } from './api/spaces';

export class SdkworkImClient {
  private httpClient: HttpClient;

  public readonly presence: PresenceApi;
  public readonly realtime: RealtimeApi;
  public readonly calls: CallsApi;
  public readonly social: SocialApi;
  public readonly chat: ChatApi;
  public readonly streams: StreamsApi;
  public readonly spaces: SpacesApi;

  constructor(config: SdkworkImConfig) {
    this.httpClient = createHttpClient(config);
    this.presence = createPresenceApi(this.httpClient);

    this.realtime = createRealtimeApi(this.httpClient);

    this.calls = createCallsApi(this.httpClient);

    this.social = createSocialApi(this.httpClient);

    this.chat = createChatApi(this.httpClient);

    this.streams = createStreamsApi(this.httpClient);

    this.spaces = createSpacesApi(this.httpClient);
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

export function createClient(config: SdkworkImConfig): SdkworkImClient {
  return new SdkworkImClient(config);
}

export default SdkworkImClient;
