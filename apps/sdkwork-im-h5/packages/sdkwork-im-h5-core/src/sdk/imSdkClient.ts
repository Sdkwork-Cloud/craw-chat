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
  ContactPreferencesView,
  ConversationInboxEntry,
  ConversationInboxPage,
  ConversationMember,
  ConversationMessageEntry,
  ConversationMessageListResponse,
  ConversationPreferencesView,
  ConversationProfileView,
  ConversationSummaryView,
  CreateConversationRequest,
  CreateConversationResult,
  EditMessageRequest,
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
  MessageMutationResult,
  MessagePinMutationResult,
  MessageReplyReference,
  MessageHistoryListParams,
  MessageSearchHit,
  MessageSearchPage,
  MessageSearchParams,
  PinnedMessagesResponse,
  PostMessageResult,
  QueryParams,
  RecallMessageRequest,
  SocialFriendRequestMutationResponse,
  SocialFriendRequestAcceptanceResponse,
  SocialFriendRequestListResponse,
  SocialUserSearchResponse,
  UpdateContactPreferencesRequest,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
} from "@sdkwork/im-sdk";
