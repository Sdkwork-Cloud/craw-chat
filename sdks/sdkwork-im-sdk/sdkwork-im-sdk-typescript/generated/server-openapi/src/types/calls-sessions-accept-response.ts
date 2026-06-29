import type { RtcSessionMutationResponse } from './rtc-session-mutation-response';

export interface CallsSessionsAcceptResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
