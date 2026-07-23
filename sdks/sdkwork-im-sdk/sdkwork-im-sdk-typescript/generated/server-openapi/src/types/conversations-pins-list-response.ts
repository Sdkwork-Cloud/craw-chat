import type { MessageInteractionSummaryView } from './message-interaction-summary-view';
import type { PageInfo } from './page-info';

export interface ConversationsPinsListResponse {
  code: 0;
  data: unknown & { items: MessageInteractionSummaryView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
