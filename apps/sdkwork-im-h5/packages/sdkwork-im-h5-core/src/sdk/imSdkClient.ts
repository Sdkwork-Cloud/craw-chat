import {
  ImSdkClient,
  type ImSdkClientOptions,
} from "@sdkwork/im-sdk";

let imSdkClient: ImSdkClient | null = null;

export function initImSdkClient(options: ImSdkClientOptions): ImSdkClient {
  imSdkClient = new ImSdkClient(options);
  return imSdkClient;
}

export function getImSdkClient(): ImSdkClient {
  if (!imSdkClient) {
    throw new Error("IM SDK client is not initialized");
  }
  return imSdkClient;
}

export function resetImSdkClient(): void {
  imSdkClient = null;
}

export type { ImSdkClient, ImSdkClientOptions };

export type {
  ContactsResponse,
  ConversationInboxEntry,
  ConversationInboxPage,
  ConversationMessageEntry,
  ConversationMessageListResponse,
  ConversationPreferencesView,
  ConversationProfileView,
  ConversationSummaryView,
  CreateConversationRequest,
  CreateConversationResult,
  FavoriteMessageRequest,
  FavoriteMessagesResponse,
  FriendRequest,
  ImConnectOptions,
  ImDecodedMessage,
  ImLiveConnection,
  ImLiveConnectionState,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
  ImSubscription,
  ListMembersResponse,
  MessageFavoriteView,
  MessageReplyReference,
  MessageHistoryListParams,
  MessageSearchHit,
  MessageSearchPage,
  MessageSearchParams,
  PostMessageResult,
  QueryParams,
  SocialFriendRequestMutationResponse,
  SocialFriendRequestAcceptanceResponse,
  SocialFriendRequestListResponse,
  SocialUserSearchResponse,
  UpdateConversationPreferencesRequest,
} from "@sdkwork/im-sdk";
