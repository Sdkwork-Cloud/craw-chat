import type { SocialFriendRequestAcceptanceResponse } from './social-friend-request-acceptance-response';

export interface SocialFriendRequestsAcceptResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
