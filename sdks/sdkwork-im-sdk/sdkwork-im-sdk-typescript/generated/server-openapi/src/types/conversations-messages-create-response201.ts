import type { PostMessageResult } from './post-message-result';

export interface ConversationsMessagesCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
