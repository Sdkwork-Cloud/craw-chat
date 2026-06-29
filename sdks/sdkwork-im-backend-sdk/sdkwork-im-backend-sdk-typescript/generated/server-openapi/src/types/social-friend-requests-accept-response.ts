import type { SocialFriendRequestCommitResponse } from './social-friend-request-commit-response';

export interface SocialFriendRequestsAcceptResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
