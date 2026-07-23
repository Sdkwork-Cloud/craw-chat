import type { ConversationProfileView } from './conversation-profile-view';

export interface ConversationsProfileUpdateResponse {
  code: 0;
  data: unknown & { item: ConversationProfileView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
