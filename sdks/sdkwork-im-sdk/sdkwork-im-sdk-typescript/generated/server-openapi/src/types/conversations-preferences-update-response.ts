import type { ConversationPreferencesView } from './conversation-preferences-view';

export interface ConversationsPreferencesUpdateResponse {
  code: 0;
  data: unknown & { item: ConversationPreferencesView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
