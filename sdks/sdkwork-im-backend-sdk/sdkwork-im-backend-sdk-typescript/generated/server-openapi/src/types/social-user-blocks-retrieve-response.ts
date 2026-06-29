import type { SocialUserBlockSnapshotResponse } from './social-user-block-snapshot-response';

export interface SocialUserBlocksRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
