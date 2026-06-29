import type { FriendRequest } from './friend-request';

export interface SocialFriendRequestsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
