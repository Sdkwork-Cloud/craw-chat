export interface ContactView {
  tenantId: string;
  ownerUserId: string;
  targetUserId: string;
  displayName?: string | null;
  avatarUrl?: string | null;
  chatId?: string | null;
  contactType: string;
  relationshipState: string;
  friendshipId: string;
  directChatId?: string | null;
  conversationId?: string | null;
  establishedAt: string;
  lastInteractionAt: string;
  isStarred: boolean;
  isBlocked: boolean;
  remark?: string | null;
  updatedAt: string;
}
