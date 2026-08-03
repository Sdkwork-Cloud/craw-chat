export interface FriendRequest {
  tenantId: string;
  friendRequestId: string;
  requesterUserId: string;
  targetUserId: string;
  status: string;
  requestMessage?: string | null;
  expiredAt?: string | null;
  createdAt: string;
  updatedAt: string;
}
