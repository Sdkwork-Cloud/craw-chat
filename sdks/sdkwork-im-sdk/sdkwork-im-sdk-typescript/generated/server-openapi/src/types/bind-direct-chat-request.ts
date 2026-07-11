export interface BindDirectChatRequest {
  conversationId?: string | null;
  directChatId?: string | null;
  leftActorId: string;
  leftActorKind: string;
  rightActorId: string;
  rightActorKind: string;
}
