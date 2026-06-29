import type { MessagePinMutationResult } from './message-pin-mutation-result';

export interface MessagesPinCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
