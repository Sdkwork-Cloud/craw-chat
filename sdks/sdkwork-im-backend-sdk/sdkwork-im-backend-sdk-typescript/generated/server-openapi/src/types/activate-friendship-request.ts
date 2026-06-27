export interface ActivateFriendshipRequest {
  directChatId?: string | null;
  establishedAt: string;
  eventId: string;
  friendshipId: string;
  initiatorUserId: string;
  peerUserId: string;
}
