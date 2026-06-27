import type { FriendRequest } from './friend-request';

export interface SocialFriendRequestListResponse {
  items: FriendRequest[];
  nextCursor?: string | null;
}
