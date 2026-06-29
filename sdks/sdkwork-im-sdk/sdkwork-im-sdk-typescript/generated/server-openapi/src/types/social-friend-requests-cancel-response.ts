import type { SocialFriendRequestMutationResponse } from './social-friend-request-mutation-response';

export interface SocialFriendRequestsCancelResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
