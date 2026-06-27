export interface ConversationProfileView {
  tenantId: string;
  conversationId: string;
  displayName: string;
  avatarUrl: string;
  notice: string;
  updatedAt: string;
  updatedByPrincipalKind?: string | null;
  updatedByPrincipalId?: string | null;
}
