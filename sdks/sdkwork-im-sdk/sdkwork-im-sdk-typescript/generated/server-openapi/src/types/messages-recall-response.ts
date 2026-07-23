import type { MessageMutationResult } from './message-mutation-result';

export interface MessagesRecallResponse {
  code: 0;
  data: unknown & { item: MessageMutationResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
