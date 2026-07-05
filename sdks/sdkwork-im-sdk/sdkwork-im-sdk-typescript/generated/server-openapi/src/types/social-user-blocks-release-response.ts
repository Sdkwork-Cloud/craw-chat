import type { SocialUserBlockMutationResponse } from './social-user-block-mutation-response';

export interface SocialUserBlocksReleaseResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
