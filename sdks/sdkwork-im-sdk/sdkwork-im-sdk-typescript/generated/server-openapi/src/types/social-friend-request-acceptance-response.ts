import type { DirectChat } from './direct-chat';
import type { FriendRequest } from './friend-request';
import type { Friendship } from './friendship';
import type { SocialFriendRequestAcceptedConversation } from './social-friend-request-accepted-conversation';

export interface SocialFriendRequestAcceptanceResponse {
  friendRequest: FriendRequest;
  friendship: Friendship;
  directChat: DirectChat;
  conversation: SocialFriendRequestAcceptedConversation;
}
