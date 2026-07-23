import type { SocialExternalMemberLinkSnapshotResponse } from './social-external-member-link-snapshot-response';

export interface SocialExternalMemberLinksRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialExternalMemberLinkSnapshotResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
