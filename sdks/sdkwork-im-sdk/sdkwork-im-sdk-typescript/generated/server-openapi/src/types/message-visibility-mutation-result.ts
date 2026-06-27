export interface MessageVisibilityMutationResult {
  tenantId: string;
  conversationId: string;
  messageId: string;
  messageSeq: number;
  principalKind: string;
  principalId: string;
  isDeleted: boolean;
  updatedAt: string;
}
