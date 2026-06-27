import { imApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { PresenceHeartbeatRequest, PresenceView } from '../types';


export class PresenceMeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve current principal presence */
  async retrieve(): Promise<PresenceView> {
    return this.client.get<PresenceView>(imApiPath(`/presence/me`));
  }
}

export class PresenceHeartbeatApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Publish current client route presence heartbeat */
  async create(body: PresenceHeartbeatRequest): Promise<PresenceView> {
    return this.client.post<PresenceView>(imApiPath(`/presence/heartbeat`), body, undefined, undefined, 'application/json');
  }
}

export class PresenceApi {
  private client: HttpClient;
  public readonly heartbeat: PresenceHeartbeatApi;
  public readonly me: PresenceMeApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.heartbeat = new PresenceHeartbeatApi(client);
    this.me = new PresenceMeApi(client);
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
