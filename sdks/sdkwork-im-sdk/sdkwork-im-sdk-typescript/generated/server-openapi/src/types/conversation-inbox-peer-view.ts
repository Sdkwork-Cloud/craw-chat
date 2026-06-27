export interface ConversationInboxPeerView {
  principalKind: string;
  principalId: string;
  userId?: string | null;
  chatId?: string | null;
  displayName?: string | null;
  avatarUrl?: string | null;
  relationshipState?: string | null;
}
