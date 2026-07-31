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
  CreateConversationRequest,
  CreateConversationResult,
  FavoriteMessageRequest,
  FavoriteMessagesResponse,
  ImConnectOptions,
  ImDecodedMessage,
  ImLiveConnection,
  ImLiveConnectionState,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
  ImSubscription,
  MessageFavoriteView,
  MessageHistoryListParams,
  PostMessageResult,
  QueryParams,
  SocialFriendRequestMutationResponse,
  SocialUserSearchResponse,
  UpdateConversationPreferencesRequest,
} from "@sdkwork/im-sdk";
