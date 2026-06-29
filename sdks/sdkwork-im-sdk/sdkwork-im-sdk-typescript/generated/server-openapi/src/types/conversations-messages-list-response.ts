import type { PageInfo } from './page-info';
import type { TimelineViewEntry } from './timeline-view-entry';

export interface ConversationsMessagesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
