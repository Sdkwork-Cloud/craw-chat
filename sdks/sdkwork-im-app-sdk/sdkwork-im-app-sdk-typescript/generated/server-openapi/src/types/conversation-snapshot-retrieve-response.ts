import type { PortalConversationSnapshot } from './portal-conversation-snapshot';

export interface ConversationSnapshotRetrieveResponse {
  code: 0;
  data: unknown & { item: PortalConversationSnapshot; };
  /** Server-owned request correlation id. */
  traceId: string;
}
