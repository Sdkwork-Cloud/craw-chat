import type { ReadCursorView } from './read-cursor-view';

export interface ConversationsReadCursorUpdateResponse {
  code: 0;
  data: unknown & { item: ReadCursorView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
