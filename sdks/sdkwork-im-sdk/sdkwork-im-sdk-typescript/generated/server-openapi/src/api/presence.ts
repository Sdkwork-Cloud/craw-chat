import { imApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { PresenceHeartbeatRequest, PresenceView } from '../types';


export class PresenceMeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve current principal presence */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<PresenceView> {
    return this.client.request<PresenceView>(imApiPath(`/presence/me`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class PresenceApi {
  private client: HttpClient;
  public readonly me: PresenceMeApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.me = new PresenceMeApi(client);
  }


/** Publish current client route presence heartbeat */
  async heartbeat(body: PresenceHeartbeatRequest, requestOptions?: ApiRequestOptions): Promise<PresenceView> {
    return this.client.request<PresenceView>(imApiPath(`/presence/heartbeat`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export function createPresenceApi(client: HttpClient): PresenceApi {
  return new PresenceApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
