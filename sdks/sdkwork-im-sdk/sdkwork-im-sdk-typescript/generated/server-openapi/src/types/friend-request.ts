export interface FriendRequest {
  tenantId: string;
  requesterUserId: string;
  targetUserId: string;
  status: string;
  requestMessage?: string | null;
  createdAt: string;
  updatedAt: string;
}
