import type { AckResponse } from './ack-response';

export interface ConversationsMembersRemoveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
