export interface MessagePinMutationResult {
  tenantId: string;
  conversationId: string;
  messageId: string;
  isPinned: boolean;
  updatedAt: string;
}
