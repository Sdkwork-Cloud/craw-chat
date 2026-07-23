import type { ContactView } from './contact-view';

export interface SocialContactsListResponse {
  code: 0;
  data: unknown & { items: ContactView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
