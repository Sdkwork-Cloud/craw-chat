import type { SocialUserBlockCommitResponse } from './social-user-block-commit-response';

export interface SocialUserBlocksCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
