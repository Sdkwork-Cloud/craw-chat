import type { PostedMessageResponse } from './posted-message-response';

export interface ConversationsSystemChannelPublishResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
