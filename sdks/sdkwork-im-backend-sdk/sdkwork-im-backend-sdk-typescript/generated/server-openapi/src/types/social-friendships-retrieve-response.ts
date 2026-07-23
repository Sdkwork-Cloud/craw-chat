import type { SocialFriendshipSnapshotResponse } from './social-friendship-snapshot-response';

export interface SocialFriendshipsRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialFriendshipSnapshotResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
