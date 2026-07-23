import type { MessageInteractionSummaryView } from './message-interaction-summary-view';

export interface ConversationsMessagesInteractionSummaryRetrieveResponse {
  code: 0;
  data: unknown & { item: MessageInteractionSummaryView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
