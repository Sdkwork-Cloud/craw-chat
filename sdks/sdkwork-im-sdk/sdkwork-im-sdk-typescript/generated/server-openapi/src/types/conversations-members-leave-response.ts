import type { AckResponse } from './ack-response';

export interface ConversationsMembersLeaveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
