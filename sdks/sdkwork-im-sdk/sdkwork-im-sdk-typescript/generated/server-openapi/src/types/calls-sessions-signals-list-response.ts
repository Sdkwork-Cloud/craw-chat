import type { RtcSignalEvent } from './rtc-signal-event';

export interface CallsSessionsSignalsListResponse {
  code: 0;
  data: unknown & { items: RtcSignalEvent[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
