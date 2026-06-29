import type { ConversationMember } from './conversation-member';

export interface ConversationsMembersAddResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
