import type { ConversationMember } from './conversation-member';

export interface ConversationsMembersChangeRoleResponse {
  code: 0;
  data: unknown & { item: ConversationMember; };
  /** Server-owned request correlation id. */
  traceId: string;
}
