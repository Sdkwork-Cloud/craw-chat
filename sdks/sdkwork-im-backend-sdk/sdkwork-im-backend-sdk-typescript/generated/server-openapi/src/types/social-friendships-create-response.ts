import type { SocialFriendshipCommitResponse } from './social-friendship-commit-response';

export interface SocialFriendshipsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
