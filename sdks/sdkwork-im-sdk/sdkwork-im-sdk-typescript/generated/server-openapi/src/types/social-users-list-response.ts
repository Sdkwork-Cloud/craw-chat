import type { SocialUserSearchResult } from './social-user-search-result';

export interface SocialUsersListResponse {
  code: 0;
  data: unknown & { items: SocialUserSearchResult[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
