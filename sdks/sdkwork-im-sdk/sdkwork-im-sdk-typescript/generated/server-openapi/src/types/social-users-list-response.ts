import type { SocialUserSearchResult } from './social-user-search-result';

export interface SocialUsersListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
