import type { MessageReactionMutationResult } from './message-reaction-mutation-result';

export interface MessagesReactionsRemoveResponse {
  code: 0;
  data: unknown & { item: MessageReactionMutationResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
