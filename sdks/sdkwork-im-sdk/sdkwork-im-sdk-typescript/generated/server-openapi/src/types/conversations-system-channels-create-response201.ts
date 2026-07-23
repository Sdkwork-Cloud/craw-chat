import type { CreateConversationResult } from './create-conversation-result';

export interface ConversationsSystemChannelsCreateResponse201 {
  code: 0;
  data: unknown & { item: CreateConversationResult; };
  /** Server-owned request correlation id. */
  traceId: string;
}
