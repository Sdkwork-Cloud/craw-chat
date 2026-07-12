import type { RtcParticipantCredential } from './rtc-participant-credential';

export interface CallsSessionsCredentialsRefreshResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
