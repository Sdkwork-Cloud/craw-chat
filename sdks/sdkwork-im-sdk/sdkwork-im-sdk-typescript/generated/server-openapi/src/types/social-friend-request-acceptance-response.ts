import type { CreateConversationResult } from './create-conversation-result';
import type { DirectChat } from './direct-chat';
import type { FriendRequest } from './friend-request';
import type { Friendship } from './friendship';

export interface SocialFriendRequestAcceptanceResponse {
  friendRequest: FriendRequest;
  friendship: Friendship;
  directChat: DirectChat;
  conversation: CreateConversationResult;
}
