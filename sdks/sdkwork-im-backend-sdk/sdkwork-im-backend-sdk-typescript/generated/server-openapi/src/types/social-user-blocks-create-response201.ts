import type { SocialUserBlockCommitResponse } from './social-user-block-commit-response';

export interface SocialUserBlocksCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialUserBlockCommitResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
