import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';



export class ProviderPrincipalProfileHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve principal-profile provider health */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(appApiPath(`/principal/profiles/provider_health`));
  }
}

export class ProviderMediaHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve media provider health */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(appApiPath(`/media/provider_health`));
  }
}

export class ProviderApi {
  private client: HttpClient;
  public readonly mediaHealth: ProviderMediaHealthApi;
  public readonly principalProfileHealth: ProviderPrincipalProfileHealthApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.mediaHealth = new ProviderMediaHealthApi(client);
    this.principalProfileHealth = new ProviderPrincipalProfileHealthApi(client);
  }

}

export function createProviderApi(client: HttpClient): ProviderApi {
  return new ProviderApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
