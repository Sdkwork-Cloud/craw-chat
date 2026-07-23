import type { CreateConversationResult } from './create-conversation-result';

export interface ConversationsAgentHandoffsCreateResponse201 {
  code: 0;
  data: unknown & { item: CreateConversationResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
