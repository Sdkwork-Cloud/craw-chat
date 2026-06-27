export interface ConversationSummaryView {
  tenantId: string;
  conversationId: string;
  messageCount: number;
  lastMessageSeq: number;
  lastSummary?: string | null;
  lastMessageAt?: string;
}
