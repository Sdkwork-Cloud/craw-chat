import type { SocialDirectChatSnapshotResponse } from './social-direct-chat-snapshot-response';

export interface SocialDirectChatsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
