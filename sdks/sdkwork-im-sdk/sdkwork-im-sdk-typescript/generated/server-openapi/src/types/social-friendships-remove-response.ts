import type { SocialFriendshipMutationResponse } from './social-friendship-mutation-response';

export interface SocialFriendshipsRemoveResponse {
  code: 0;
  data: unknown & { item: SocialFriendshipMutationResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
