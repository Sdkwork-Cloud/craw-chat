export interface CreateAgentHandoffRequest {
  conversationId: string;
  targetId: string;
  targetKind: string;
  handoffSessionId: string;
  handoffReason?: string | null;
}
