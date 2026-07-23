import type { SocialFriendRequestPendingCountResponse } from './social-friend-request-pending-count-response';

export interface SocialFriendRequestsPendingCountRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialFriendRequestPendingCountResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
