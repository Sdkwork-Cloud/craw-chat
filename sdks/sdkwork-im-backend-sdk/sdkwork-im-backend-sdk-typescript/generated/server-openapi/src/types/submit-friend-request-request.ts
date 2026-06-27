export interface SubmitFriendRequestRequest {
  eventId: string;
  requestId: string;
  requestMessage?: string | null;
  requestedAt: string;
  requesterUserId: string;
  targetUserId: string;
}
