import type { SocialFriendRequestCommitResponse } from './social-friend-request-commit-response';

export interface SocialFriendRequestsDeclineResponse {
  code: 0;
  data: unknown & { item: SocialFriendRequestCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
