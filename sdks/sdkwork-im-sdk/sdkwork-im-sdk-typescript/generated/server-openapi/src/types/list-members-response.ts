import type { ConversationMember } from './conversation-member';

export interface ListMembersResponse {
  items: ConversationMember[];
  nextCursor?: string | null;
  hasMore: boolean;
}
