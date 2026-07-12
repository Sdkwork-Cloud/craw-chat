import type {
  AckResponse,
  AddConversationMemberRequest,
  BindDirectChatRequest,
  BlockUserRequest,
  CreateRtcSessionRequest,
  ContactPreferencesView,
  ContactRecommendationView,
  ContactTagView,
  ConversationAgentAssignment,
  ConversationAgentAssignments,
  CreateAgentDialogRequest,
  CreateAgentHandoffRequest,
  CreateContactRecommendationRequest,
  CreateContactTagRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateSystemChannelRequest,
  CreateThreadConversationRequest,
  ConversationMember,
  EnterRoomResponse,
  RoomView,
  UpdateConversationAgentsRequest,
  EditMessageRequest,
  FavoriteMessageRequest,
  OpenApiUserBlockResponse,
} from '../generated/server-openapi/dist/index.js';
import type {
  ContactTagsResponse,
  ContactsResponse,
  DeleteContactTagResponse,
  DeleteMessageFavoriteResponse,
  FavoriteMessagesResponse,
  ConversationMessageListResponse,
  ConversationInboxPage,
  ListMembersResponse,
  PinnedMessagesResponse,
  SocialFriendRequestListResponse,
  SocialUserSearchResponse,
} from './openapi-compat-types.js';
import type {
  MessageFavoriteView,
  MessageFavoriteType,
  MessageInteractionSummaryView,
  MessageMutationResult,
  MessagePinMutationResult,
  PostMessageResult,
  MessageReactionMutationResult,
  MessageReactionRequest,
  PostMessageRequest,
  QueryParams,
  ReadCursorView,
  InviteRtcSessionRequest,
  IssueRtcParticipantCredentialRequest,
  PostRtcSignalRequest,
  RtcSession,
  RtcSessionMutationResponse,
  RtcSignalEvent,
  RtcParticipantCredential,
  RecallMessageRequest,
  SocialFriendRequestAcceptanceResponse,
  SocialFriendRequestMutationResponse,
  SocialFriendshipMutationResponse,
  UpdateRtcSessionRequest,
  UpdateContactPreferencesRequest,
  UpdateContactTagRequest,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
  ConversationProfileView,
} from '../generated/server-openapi/dist/index.js';

export type { QueryParams };

export interface MessageHistoryListParams {
  cursor?: string;
  pageSize?: number;
}

export interface ImCreateRoomRequest {
  conversationId?: string | null;
  roomId: string;
  roomKind: 'live' | 'chat' | 'game';
}

export type ImConversationAgentAssignment = ConversationAgentAssignment;

export type ImConversationAgentAssignmentSet = Omit<ConversationAgentAssignments, 'generation'> & {
  generation: number;
};

export type ImReplaceConversationAgentAssignmentsRequest = Omit<
  UpdateConversationAgentsRequest,
  'expectedGeneration'
> & {
  expectedGeneration: number;
};

export type ImReplaceConversationAgentAssignmentsResult = ImConversationAgentAssignmentSet;

export interface ImTransportClientLike {
  chat: {
    contacts: {
      list(params?: QueryParams): Promise<ContactsResponse>;
    };
    inbox: {
      list(params?: QueryParams): Promise<ConversationInboxPage>;
    };
    rooms: {
      create(body: ImCreateRoomRequest): Promise<CreateConversationResult>;
      retrieve(roomId: string): Promise<RoomView>;
      enter(roomId: string): Promise<EnterRoomResponse>;
      leave(roomId: string): Promise<EnterRoomResponse>;
    };
    conversations: {
      create(body: CreateConversationRequest): Promise<CreateConversationResult>;
      agentDialogs: {
        create(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
      };
      agentHandoffs: {
        create(body: CreateAgentHandoffRequest): Promise<CreateConversationResult>;
      };
      systemChannels: {
        create(body: CreateSystemChannelRequest): Promise<CreateConversationResult>;
        publish(conversationId: string, body: PostMessageRequest): Promise<PostMessageResult>;
      };
      threads: {
        create(body: CreateThreadConversationRequest): Promise<CreateConversationResult>;
      };
      directChats: {
        bindings: {
          create(body: BindDirectChatRequest): Promise<CreateConversationResult>;
        };
      };
      members: {
        list(conversationId: string, params?: QueryParams): Promise<ListMembersResponse>;
        current: {
          retrieve(conversationId: string): Promise<ConversationMember>;
        };
        add(conversationId: string, body: AddConversationMemberRequest): Promise<unknown>;
        remove(conversationId: string, body: unknown): Promise<unknown>;
        leave(conversationId: string): Promise<unknown>;
        acceptInvitation(conversationId: string): Promise<import('../generated/server-openapi/dist/index.js').ConversationMember>;
      };
      agents: {
        retrieve(conversationId: string): Promise<ConversationAgentAssignments>;
        update(
          conversationId: string,
          body: UpdateConversationAgentsRequest,
        ): Promise<ConversationAgentAssignments>;
      };
      messages: {
        list(
          conversationId: string,
          params?: MessageHistoryListParams,
        ): Promise<ConversationMessageListResponse>;
        create(conversationId: string, body: PostMessageRequest): Promise<PostMessageResult>;
        interactionSummary: {
          retrieve(conversationId: string, messageId: string): Promise<MessageInteractionSummaryView>;
        };
      };
      pins: {
        list(conversationId: string): Promise<PinnedMessagesResponse>;
      };
      preferences: {
        retrieve(conversationId: string): Promise<import('../generated/server-openapi/dist/index.js').ConversationPreferencesView>;
        update(conversationId: string, body: UpdateConversationPreferencesRequest): Promise<import('../generated/server-openapi/dist/index.js').ConversationPreferencesView>;
      };
      profile: {
        retrieve(conversationId: string): Promise<ConversationProfileView>;
        update(conversationId: string, body: UpdateConversationProfileRequest): Promise<ConversationProfileView>;
      };
      readCursor: {
        update(conversationId: string, body: { readSeq: number }): Promise<ReadCursorView>;
      };
    };
    messages: {
      edit(messageId: string, body: EditMessageRequest): Promise<MessageMutationResult>;
      recall(messageId: string, body?: RecallMessageRequest): Promise<MessageMutationResult>;
      reactions: {
        create(messageId: string, body: MessageReactionRequest): Promise<MessageReactionMutationResult>;
        remove(messageId: string, body: MessageReactionRequest): Promise<MessageReactionMutationResult>;
      };
      pin(messageId: string): Promise<MessagePinMutationResult>;
      unpin(messageId: string): Promise<MessagePinMutationResult>;
      visibility: {
        delete(messageId: string): Promise<void>;
      };
      favorites: {
        list(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse>;
        create(messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView>;
        delete(favoriteId: string): Promise<DeleteMessageFavoriteResponse>;
      };
    };
  };
  calls: {
    sessions: {
      create(body: CreateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      retrieve(rtcSessionId: string): Promise<RtcSession>;
      invite(rtcSessionId: string, body: InviteRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      accept(rtcSessionId: string, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      reject(rtcSessionId: string, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      end(rtcSessionId: string, body: UpdateRtcSessionRequest): Promise<RtcSessionMutationResponse>;
      signals: {
        create(rtcSessionId: string, body: PostRtcSignalRequest): Promise<RtcSignalEvent>;
        list(
          rtcSessionId: string,
          query?: { afterSignalSeq?: string; pageSize?: number; cursor?: string },
        ): Promise<{ items: RtcSignalEvent[]; pageInfo: { mode: string; hasMore?: boolean; nextCursor?: string | null } }>;
      };
      credentials: {
        create(rtcSessionId: string, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential>;
        refresh(rtcSessionId: string, body: IssueRtcParticipantCredentialRequest): Promise<RtcParticipantCredential>;
      };
    };
  };
  social: {
    users: {
      list(params?: { q?: string; pageSize?: number; cursor?: string; }): Promise<SocialUserSearchResponse>;
    };
    friendRequests: {
      list(params?: QueryParams & { direction?: string; status?: string }): Promise<SocialFriendRequestListResponse>;
      create(body: { targetUserId: string; requestMessage?: string }): Promise<SocialFriendRequestMutationResponse>;
      accept(requestId: string): Promise<SocialFriendRequestAcceptanceResponse>;
      decline(requestId: string): Promise<SocialFriendRequestMutationResponse>;
      cancel(requestId: string): Promise<SocialFriendRequestMutationResponse>;
      pendingCount(): Promise<{ count: number }>;
    };
    friendships: {
      remove(friendshipId: string): Promise<SocialFriendshipMutationResponse>;
    };
    userBlocks: {
      create(body: BlockUserRequest): Promise<OpenApiUserBlockResponse>;
      delete(blockId: string): Promise<void>;
    };
    contacts: {
      list(params?: QueryParams): Promise<ContactsResponse>;
      preferences: {
        retrieve(targetUserId: string): Promise<ContactPreferencesView>;
        update(targetUserId: string, body: UpdateContactPreferencesRequest): Promise<ContactPreferencesView>;
      };
      tags: {
        list(params?: QueryParams): Promise<ContactTagsResponse>;
        create(body: CreateContactTagRequest): Promise<ContactTagView>;
        update(
          tagId: string,
          body: UpdateContactTagRequest,
        ): Promise<ContactTagView>;
        delete(tagId: string): Promise<DeleteContactTagResponse>;
      };
      recommendations: {
        create(
          targetUserId: string,
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
