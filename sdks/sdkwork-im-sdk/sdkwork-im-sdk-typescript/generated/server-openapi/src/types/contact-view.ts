export interface ContactView {
  tenantId: string;
  ownerUserId: string;
  targetUserId: string;
  contactType: string;
  relationshipState: string;
  friendshipId: string;
  directChatId?: string | null;
  conversationId?: string | null;
  establishedAt: string;
  lastInteractionAt: string;
}
