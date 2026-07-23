import type { AckResponse } from './ack-response';

export interface RealtimeEventsAckResponse {
  code: 0;
  data: unknown & { item: AckResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
