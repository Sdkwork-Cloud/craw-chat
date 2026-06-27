import type { ConversationInboxEntry } from './conversation-inbox-entry';

export interface InboxResponse {
  items: ConversationInboxEntry[];
  nextCursor?: string | null;
  hasMore: boolean;
}
