import type {
  AckResponse,
  AddConversationMemberRequest,
  BindDirectChatRequest,
  CreateRtcSessionRequest,
  ContactPreferencesView,
  ContactRecommendationView,
  ContactTagView,
  CreateAgentDialogRequest,
  CreateContactRecommendationRequest,
  CreateContactTagRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateRoomRequest,
  EnterRoomResponse,
  RoomView,
  EditMessageRequest,
  FavoriteMessageRequest,
} from '@sdkwork/im-sdk-generated';
import type {
  ContactTagsResponse,
  ContactsResponse,
  DeleteContactTagResponse,
  DeleteMessageFavoriteResponse,
  FavoriteMessagesResponse,
  InboxResponse,
  ListMembersResponse,
  MessageVisibilityMutationResult,
  PinnedMessagesResponse,
  SocialFriendRequestListResponse,
  SocialUserSearchResponse,
  TimelineResponse,
} from './openapi-compat-types';
import type {
  MessageFavoriteView,
  MessageFavoriteType,
  MessageInteractionSummaryView,
  MessagePinMutationResult,
  MessageReactionMutationResult,
  MessageReactionRequest,
  PostMessageRequest,
  PostedMessageResponse,
  QueryParams,
  ReadCursorView,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  PostRtcSignalRequest,
  RtcSession,
  RtcSessionMutationResponse,
  RtcSignalEvent,
  RtcParticipantCredential,
  SocialFriendRequestAcceptanceResponse,
  SocialFriendRequestMutationResponse,
  SocialFriendshipMutationResponse,
  UpdateRtcSessionRequest,
  UpdateContactPreferencesRequest,
  UpdateContactTagRequest,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
  ConversationProfileView,
} from '@sdkwork/im-sdk-generated';

export type { QueryParams };

export interface ImTransportClientLike {
  chat: {
    contacts: {
      list(params?: QueryParams): Promise<ContactsResponse>;
    };
    inbox: {
      retrieve(params?: QueryParams): Promise<InboxResponse>;
    };
    rooms: {
      create(body: CreateRoomRequest): Promise<CreateConversationResult>;
      get(roomId: string | number): Promise<RoomView>;
      enter(roomId: string | number): Promise<EnterRoomResponse>;
      leave(roomId: string | number): Promise<EnterRoomResponse>;
    };
    conversations: {
      create(body: CreateConversationRequest): Promise<CreateConversationResult>;
      agentDialogs: {
        create(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
      };
      directChats: {
        bind(body: BindDirectChatRequest): Promise<CreateConversationResult>;
      };
      members: {
        list(conversationId: string | number, params?: QueryParams): Promise<ListMembersResponse>;
        add(conversationId: string | number, body: AddConversationMemberRequest): Promise<unknown>;
        remove(conversationId: string | number, body: unknown): Promise<unknown>;
        leave(conversationId: string | number): Promise<unknown>;
      };
      messages: {
        list(conversationId: string | number, params?: QueryParams): Promise<TimelineResponse>;
        create(conversationId: string | number, body: PostMessageRequest): Promise<PostedMessageResponse>;
        interactionSummary: {
          retrieve(conversationId: string | number, messageId: string | number): Promise<MessageInteractionSummaryView>;
        };
      };
      pins: {
        list(conversationId: string | number): Promise<PinnedMessagesResponse>;
      };
      preferences: {
        retrieve(conversationId: string | number): Promise<import('@sdkwork/im-sdk-generated').ConversationPreferencesView>;
        update(conversationId: string | number, body: UpdateConversationPreferencesRequest): Promise<import('@sdkwork/im-sdk-generated').ConversationPreferencesView>;
      };
      profile: {
        retrieve(conversationId: string | number): Promise<ConversationProfileView>;
        update(conversationId: string | number, body: UpdateConversationProfileRequest): Promise<ConversationProfileView>;
      };
      readCursor: {
        update(conversationId: string | number, body: { readSeq: number }): Promise<ReadCursorView>;
      };
    };
    messages: {
      edit(messageId: string | number, body: EditMessageRequest): Promise<PostedMessageResponse>;
      recall(messageId: string | number): Promise<PostedMessageResponse>;
      reactions: {
        create(messageId: string | number, body: MessageReactionRequest): Promise<MessageReactionMutationResult>;
        delete(messageId: string | number, body: MessageReactionRequest): Promise<MessageReactionMutationResult>;
      };
      pin: {
        create(messageId: string | number): Promise<MessagePinMutationResult>;
        delete(messageId: string | number): Promise<MessagePinMutationResult>;
      };
      visibility: {
        delete(messageId: string | number): Promise<MessageVisibilityMutationResult>;
      };
      favorites: {
        list(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse>;
        create(messageId: string | number, body: FavoriteMessageRequest): Promise<MessageFavoriteView>;
        delete(favoriteId: string | number): Promise<DeleteMessageFavoriteResponse>;
      };
    };
  };
  calls: {
    sessions: {
      create(body: CreateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      retrieve(rtcSessionId: string | number): Promise<RtcSession>;
      invite(rtcSessionId: string | number, body: InviteRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      accept(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      reject(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      end(rtcSessionId: string | number, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      signals: {
        create(rtcSessionId: string | number, body: PostRtcSignalRequest): Promise<RtcSignalEvent>;
      };
      credentials: {
        create(rtcSessionId: string | number, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential>;
      };
    };
  };
  social: {
    users: {
      list(params?: { q?: string; limit?: number; cursor?: string; }): Promise<SocialUserSearchResponse>;
    };
    friendRequests: {
      list(params?: QueryParams & { direction?: string; status?: string }): Promise<SocialFriendRequestListResponse>;
      create(body: { targetUserId: string; requestMessage?: string }): Promise<SocialFriendRequestMutationResponse>;
      accept(requestId: string | number): Promise<SocialFriendRequestAcceptanceResponse>;
      decline(requestId: string | number): Promise<SocialFriendRequestMutationResponse>;
      cancel(requestId: string | number): Promise<SocialFriendRequestMutationResponse>;
    };
    friendships: {
      remove(friendshipId: string | number): Promise<SocialFriendshipMutationResponse>;
    };
    contacts: {
      list(params?: QueryParams): Promise<ContactsResponse>;
      preferences: {
        retrieve(targetUserId: string | number): Promise<ContactPreferencesView>;
        update(targetUserId: string | number, body: UpdateContactPreferencesRequest): Promise<ContactPreferencesView>;
      };
      tags: {
        list(params?: QueryParams): Promise<ContactTagsResponse>;
        create(body: CreateContactTagRequest): Promise<ContactTagView>;
        update(
          tagId: string | number,
          body: UpdateContactTagRequest,
        ): Promise<ContactTagView>;
        delete(tagId: string | number): Promise<DeleteContactTagResponse>;
      };
      recommendations: {
        create(
          targetUserId: string | number,
          body: CreateContactRecommendationRequest,
        ): Promise<ContactRecommendationView>;
      };
    };
  };
  setAuthToken?(token: string): unknown;
  setAccessToken?(token: string): unknown;
  setTokenManager?(manager: unknown): unknown;
}

export type TransportAckResponse = AckResponse;
