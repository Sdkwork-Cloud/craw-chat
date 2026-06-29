import type { SocialExternalConnectionSnapshotResponse } from './social-external-connection-snapshot-response';

export interface SocialExternalConnectionsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
