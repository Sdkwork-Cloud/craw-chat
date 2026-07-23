import type { ConversationSummaryView } from './conversation-summary-view';

export interface ConversationsRetrieveResponse {
  code: 0;
  data: unknown & { item: ConversationSummaryView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
