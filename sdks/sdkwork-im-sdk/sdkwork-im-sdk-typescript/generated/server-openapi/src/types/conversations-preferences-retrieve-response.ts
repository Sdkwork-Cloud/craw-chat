import type { ConversationPreferencesView } from './conversation-preferences-view';

export interface ConversationsPreferencesRetrieveResponse {
  code: 0;
  data: unknown & { item: ConversationPreferencesView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
