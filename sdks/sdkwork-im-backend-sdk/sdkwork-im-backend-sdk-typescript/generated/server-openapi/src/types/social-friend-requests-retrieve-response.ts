import type { SocialFriendRequestSnapshotResponse } from './social-friend-request-snapshot-response';

export interface SocialFriendRequestsRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialFriendRequestSnapshotResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
