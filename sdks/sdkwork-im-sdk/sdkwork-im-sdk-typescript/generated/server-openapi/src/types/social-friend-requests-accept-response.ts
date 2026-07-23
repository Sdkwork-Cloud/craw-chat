import type { SocialFriendRequestAcceptanceResponse } from './social-friend-request-acceptance-response';

export interface SocialFriendRequestsAcceptResponse {
  code: 0;
  data: unknown & { item: SocialFriendRequestAcceptanceResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
