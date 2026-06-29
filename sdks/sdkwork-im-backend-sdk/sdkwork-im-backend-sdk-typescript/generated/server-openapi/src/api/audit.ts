import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { SdkWorkPageData } from '../types';


export class AuditExportApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Export audit bundle */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/audit/export`));
  }
}

export class AuditRecordsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List audit records */
  async list(): Promise<SdkWorkPageData> {
    return this.client.get<SdkWorkPageData>(backendApiPath(`/audit/records`));
  }

/** Record audit anchor */
  async create(): Promise<Record<string, unknown>> {
    return this.client.post<Record<string, unknown>>(backendApiPath(`/audit/records`));
  }
}

export class AuditApi {
  private client: HttpClient;
  public readonly records: AuditRecordsApi;
  public readonly export: AuditExportApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.records = new AuditRecordsApi(client);
    this.export = new AuditExportApi(client);
  }

}

export function createAuditApi(client: HttpClient): AuditApi {
  return new AuditApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
