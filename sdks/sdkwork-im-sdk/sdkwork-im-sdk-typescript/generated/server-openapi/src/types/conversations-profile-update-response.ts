import type { ConversationProfileView } from './conversation-profile-view';

export interface ConversationsProfileUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
