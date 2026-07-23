import type { ConversationInboxEntry } from './conversation-inbox-entry';

export interface InboxListResponse {
  code: 0;
  data: unknown & { items: ConversationInboxEntry[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
