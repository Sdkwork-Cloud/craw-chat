import type { ArchiveGroupConversationResponse } from './archive-group-conversation-response';

export interface ConversationsArchiveResponse {
  code: 0;
  data: unknown & ArchiveGroupConversationResponse;
  /** Server-owned request correlation id. */
  traceId: string;
}
