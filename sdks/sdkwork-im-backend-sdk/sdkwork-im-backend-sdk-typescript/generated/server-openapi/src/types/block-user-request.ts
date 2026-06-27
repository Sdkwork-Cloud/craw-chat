export interface BlockUserRequest {
  blockId: string;
  blockedUserId: string;
  blockerUserId: string;
  directChatId?: string | null;
  effectiveAt: string;
  eventId: string;
  expiresAt?: string | null;
  scope: string;
}
