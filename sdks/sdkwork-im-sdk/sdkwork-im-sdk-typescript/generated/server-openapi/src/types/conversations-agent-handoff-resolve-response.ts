import type { AckResponse } from './ack-response';

export interface ConversationsAgentHandoffResolveResponse {
  code: 0;
  data: unknown & { item: AckResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
