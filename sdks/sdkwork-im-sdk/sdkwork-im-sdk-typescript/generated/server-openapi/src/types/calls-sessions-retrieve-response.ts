import type { RtcSession } from './rtc-session';

export interface CallsSessionsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
