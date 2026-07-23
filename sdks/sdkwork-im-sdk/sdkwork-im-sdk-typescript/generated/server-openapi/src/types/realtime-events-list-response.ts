import type { RealtimeEventView } from './realtime-event-view';

export interface RealtimeEventsListResponse {
  code: 0;
  data: unknown & { items: RealtimeEventView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
