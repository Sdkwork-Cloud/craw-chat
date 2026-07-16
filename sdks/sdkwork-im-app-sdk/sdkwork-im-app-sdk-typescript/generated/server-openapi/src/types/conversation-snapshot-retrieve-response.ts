import type { PortalConversationSnapshot } from './portal-conversation-snapshot';

export interface ConversationSnapshotRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
