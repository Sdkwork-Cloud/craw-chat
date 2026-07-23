import type { RtcSessionMutationResponse } from './rtc-session-mutation-response';

export interface CallsSessionsInviteResponse {
  code: 0;
  data: unknown & { item: RtcSessionMutationResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
