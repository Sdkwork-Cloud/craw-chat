import type { RealtimeEventView } from './realtime-event-view';

export interface RealtimeEventsResponse {
  items: RealtimeEventView[];
  nextCursor?: string | null;
  hasMore: boolean;
}
