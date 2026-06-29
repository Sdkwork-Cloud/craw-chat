import type { SocialFriendshipMutationResponse } from './social-friendship-mutation-response';

export interface SocialFriendshipsRemoveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
