import type { RtcSignalEvent } from './rtc-signal-event';

export interface CallsSessionsSignalsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
