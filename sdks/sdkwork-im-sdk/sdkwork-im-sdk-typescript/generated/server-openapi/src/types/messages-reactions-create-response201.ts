import type { MessageReactionMutationResult } from './message-reaction-mutation-result';

export interface MessagesReactionsCreateResponse201 {
  code: 0;
  data: unknown & { item: MessageReactionMutationResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
