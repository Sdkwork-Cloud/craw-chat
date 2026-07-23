import type { SocialExternalConnectionSnapshotResponse } from './social-external-connection-snapshot-response';

export interface SocialExternalConnectionsRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialExternalConnectionSnapshotResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
