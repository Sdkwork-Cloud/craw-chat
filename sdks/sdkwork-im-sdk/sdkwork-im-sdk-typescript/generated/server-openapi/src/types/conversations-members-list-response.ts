import type { ConversationMember } from './conversation-member';

export interface ConversationsMembersListResponse {
  code: 0;
  data: unknown & { items: ConversationMember[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; };
  /** Server-owned request correlation id. */
  traceId: string;
}
