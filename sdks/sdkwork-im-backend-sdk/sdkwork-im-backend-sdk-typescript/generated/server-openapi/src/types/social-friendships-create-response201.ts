import type { SocialFriendshipCommitResponse } from './social-friendship-commit-response';

export interface SocialFriendshipsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
