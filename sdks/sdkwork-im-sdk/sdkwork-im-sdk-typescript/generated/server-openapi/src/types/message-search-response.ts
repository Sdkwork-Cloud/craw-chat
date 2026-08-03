import type { MessageSearchHit } from './message-search-hit';
import type { PageInfo } from './page-info';

export interface MessageSearchResponse {
  code: 0;
  data: unknown & { items: MessageSearchHit[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
