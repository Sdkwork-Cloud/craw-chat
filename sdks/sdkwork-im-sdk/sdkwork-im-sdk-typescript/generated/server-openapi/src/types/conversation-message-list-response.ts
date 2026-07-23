import type { ConversationMessageEntry } from './conversation-message-entry';
import type { PageInfo } from './page-info';

export interface ConversationMessageListResponse {
  code: 0;
  data: unknown & { items: ConversationMessageEntry[]; pageInfo: PageInfo; highWatermark: number; };
  /** Server-owned request correlation id. */
  traceId: string;
}
