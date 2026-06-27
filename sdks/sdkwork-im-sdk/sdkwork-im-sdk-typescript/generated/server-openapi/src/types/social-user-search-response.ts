import type { SocialUserSearchResult } from './social-user-search-result';

export interface SocialUserSearchResponse {
  items: SocialUserSearchResult[];
  nextCursor?: string | null;
  hasMore: boolean;
}
