import type { SocialDirectChatSnapshotResponse } from './social-direct-chat-snapshot-response';

export interface SocialDirectChatsRetrieveResponse {
  code: 0;
  data: unknown & { item: SocialDirectChatSnapshotResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
