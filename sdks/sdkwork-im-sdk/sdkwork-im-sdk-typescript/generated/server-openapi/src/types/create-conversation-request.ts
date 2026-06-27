export interface CreateConversationRequest {
  conversationId?: string | null;
  conversationType?: string | null;
  kind?: string | null;
  title?: string | null;
  memberIds?: string[];
}
