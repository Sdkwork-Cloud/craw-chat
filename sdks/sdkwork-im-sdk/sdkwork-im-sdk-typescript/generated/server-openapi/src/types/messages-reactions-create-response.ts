import type { MessageReactionMutationResult } from './message-reaction-mutation-result';

export interface MessagesReactionsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
