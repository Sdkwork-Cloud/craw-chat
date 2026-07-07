import type { ConversationInboxEntry } from './conversation-inbox-entry';

export interface InboxListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
