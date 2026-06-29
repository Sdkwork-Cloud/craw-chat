import type { ReadCursorView } from './read-cursor-view';

export interface ConversationsReadCursorUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
