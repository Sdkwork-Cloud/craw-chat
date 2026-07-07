import type { MessagePinMutationResult } from './message-pin-mutation-result';

export interface MessagesPinResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
