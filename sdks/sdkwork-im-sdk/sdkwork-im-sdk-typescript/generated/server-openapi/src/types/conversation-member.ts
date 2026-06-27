import type { MembershipState } from './membership-state';

export interface ConversationMember {
  tenantId: string;
  conversationId: string;
  memberId: string;
  principalId: string;
  principalKind: string;
  role: string;
  state: MembershipState;
  joinedAt: string;
}
