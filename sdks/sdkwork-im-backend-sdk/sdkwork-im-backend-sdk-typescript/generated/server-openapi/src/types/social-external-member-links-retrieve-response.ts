import type { SocialExternalMemberLinkSnapshotResponse } from './social-external-member-link-snapshot-response';

export interface SocialExternalMemberLinksRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
