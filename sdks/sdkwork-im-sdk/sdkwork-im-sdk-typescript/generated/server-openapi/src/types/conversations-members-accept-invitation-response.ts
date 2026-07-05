import type { ConversationMember } from './conversation-member';

export interface ConversationsMembersAcceptInvitationResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
