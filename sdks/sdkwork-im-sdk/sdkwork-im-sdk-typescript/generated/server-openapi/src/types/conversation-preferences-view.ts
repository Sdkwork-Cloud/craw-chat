export interface ConversationPreferencesView {
  tenantId: string;
  conversationId: string;
  principalKind: string;
  principalId: string;
  isPinned: boolean;
  isMuted: boolean;
  isMarkedUnread: boolean;
  isHidden: boolean;
  updatedAt: string;
}
