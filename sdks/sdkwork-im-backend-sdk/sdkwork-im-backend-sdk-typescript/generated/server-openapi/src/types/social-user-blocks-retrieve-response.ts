import type { SocialUserBlockSnapshotResponse } from './social-user-block-snapshot-response';

export interface SocialUserBlocksRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialUserBlockSnapshotResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
