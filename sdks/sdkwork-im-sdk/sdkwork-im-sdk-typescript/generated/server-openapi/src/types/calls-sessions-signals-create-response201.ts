import type { RtcSignalEvent } from './rtc-signal-event';

export interface CallsSessionsSignalsCreateResponse201 {
  code: 0;
  data: unknown & { item: RtcSignalEvent; };
  /** Server-owned request correlation id. */
  traceId: string;
}
