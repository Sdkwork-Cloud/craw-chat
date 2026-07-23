import type { ReadCursorView } from './read-cursor-view';

export interface ConversationsReadCursorRetrieveResponse {
  code: 0;
  data: unknown & { item: ReadCursorView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
