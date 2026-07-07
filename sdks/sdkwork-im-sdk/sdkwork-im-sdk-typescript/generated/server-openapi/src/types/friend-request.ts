export interface FriendRequest {
  tenantId: string;
  friendRequestId: string;
  requesterUserId: string;
  targetUserId: string;
  status: string;
  requestMessage?: string | null;
  expiredAt?: string;
  createdAt: string;
  updatedAt: string;
}
