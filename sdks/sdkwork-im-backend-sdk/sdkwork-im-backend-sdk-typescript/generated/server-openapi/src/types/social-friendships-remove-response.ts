import type { SocialFriendshipCommitResponse } from './social-friendship-commit-response';

export interface SocialFriendshipsRemoveResponse {
  code: 0;
  data: unknown & { item: SocialFriendshipCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
