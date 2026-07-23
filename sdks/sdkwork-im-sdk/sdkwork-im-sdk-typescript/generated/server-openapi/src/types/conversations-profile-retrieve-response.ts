import type { ConversationProfileView } from './conversation-profile-view';

export interface ConversationsProfileRetrieveResponse {
  code: 0;
  data: unknown & { item: ConversationProfileView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
