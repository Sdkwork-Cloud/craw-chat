import type { SocialFriendshipSnapshotResponse } from './social-friendship-snapshot-response';

export interface SocialFriendshipsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
