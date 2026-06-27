export interface BindDirectChatRequest {
  conversationId?: string | null;
  directChatId?: string | null;
  leftActorId?: string | null;
  leftActorKind?: string | null;
  rightActorId?: string | null;
  rightActorKind?: string | null;
  targetUserId?: string;
}
