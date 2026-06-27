import type { TimelineViewEntry } from './timeline-view-entry';

export interface TimelineResponse {
  items: TimelineViewEntry[];
  nextAfterSeq?: number | null;
  hasMore: boolean;
}
