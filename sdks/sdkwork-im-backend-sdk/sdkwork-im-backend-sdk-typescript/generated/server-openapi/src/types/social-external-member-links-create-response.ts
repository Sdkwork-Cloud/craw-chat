import type { SocialExternalMemberLinkCommitResponse } from './social-external-member-link-commit-response';

export interface SocialExternalMemberLinksCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
