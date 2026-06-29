import type { CreateConversationResult } from './create-conversation-result';

export interface ConversationsThreadsCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
