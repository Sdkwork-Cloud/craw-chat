export interface MessageReactionMutationResult {
  tenantId: string;
  conversationId: string;
  messageId: string;
  reactionKey: string;
  count: number;
  updatedAt: string;
}
