import type { MessageSearchHitView } from './message-search-hit-view';

export interface MessagesSearchResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
