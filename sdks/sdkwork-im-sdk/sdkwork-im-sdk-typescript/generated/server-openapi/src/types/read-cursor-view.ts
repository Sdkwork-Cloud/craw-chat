export interface ReadCursorView {
  tenantId: string;
  conversationId: string;
  memberId: string;
  principalId: string;
  principalKind: string;
  deviceId?: string | null;
  readSeq: string;
  lastReadMessageId?: string | null;
  updatedAt: string;
  unreadCount: string;
}
