export interface SubmitFriendRequestRequest {
  eventId: string;
  requestMessage?: string | null;
  requestedAt: string;
  requesterUserId: string;
  targetUserId: string;
}
