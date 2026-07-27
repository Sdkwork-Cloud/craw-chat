import { imApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { AckResponse, AddConversationMemberRequest, BindDirectChatRequest, ChangeConversationMemberRoleRequest, ConversationAgentAssignments, ConversationInboxEntry, ConversationMember, ConversationMessageEntry, ConversationPreferencesView, ConversationProfileView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateRoomRequest, CreateSystemChannelRequest, CreateThreadConversationRequest, EditMessageRequest, EnterRoomResponse, FavoriteMessageRequest, MessageFavoriteType, MessageFavoriteView, MessageInteractionSummaryView, MessageMutationResult, MessagePinMutationResult, MessageReactionMutationResult, MessageReactionRequest, PageInfo, PostMessageRequest, PostMessageResult, ReadCursorView, RecallMessageRequest, RemoveConversationMemberRequest, RoomView, TransferConversationOwnerRequest, UpdateConversationAgentsRequest, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest, UpdateReadCursorRequest } from '../types';


export class ChatRoomsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a live, chat, or game room bound to a group conversation */
  async create(body: CreateRoomRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/rooms`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Retrieve room metadata and active member count */
  async retrieve(roomId: string, requestOptions?: ApiRequestOptions): Promise<RoomView> {
    return this.client.request<RoomView>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Enter a room as the authenticated principal */
  async enter(roomId: string, requestOptions?: ApiRequestOptions): Promise<EnterRoomResponse> {
    return this.client.request<EnterRoomResponse>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}/enter`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }

/** Leave a room as the authenticated principal */
  async leave(roomId: string, requestOptions?: ApiRequestOptions): Promise<EnterRoomResponse> {
    return this.client.request<EnterRoomResponse>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}/leave`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }
}

export class ChatMessagesReactionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Add a message reaction */
  async create(messageId: string, body: MessageReactionRequest, requestOptions?: ApiRequestOptions): Promise<MessageReactionMutationResult> {
    return this.client.request<MessageReactionMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/reactions`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Remove a message reaction */
  async remove(messageId: string, body: MessageReactionRequest, requestOptions?: ApiRequestOptions): Promise<MessageReactionMutationResult> {
    return this.client.request<MessageReactionMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/reactions/remove`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }
}

export class ChatMessagesVisibilityApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Delete message visibility for the current principal */
  async delete(messageId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/visibility`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export interface ChatMessagesFavoritesListParams {
  pageSize?: number;
  cursor?: string;
  favoriteType?: MessageFavoriteType;
  q?: string;
}

export class ChatMessagesFavoritesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List message favorites */
  async list(params?: ChatMessagesFavoritesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: MessageFavoriteView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'favoriteType', value: params?.favoriteType, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MessageFavoriteView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/chat/messages/favorites`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Favorite a message */
  async create(messageId: string, body: FavoriteMessageRequest, requestOptions?: ApiRequestOptions): Promise<MessageFavoriteView> {
    return this.client.request<MessageFavoriteView>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/favorites`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Delete a message favorite */
  async delete(favoriteId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/chat/messages/favorites/${serializePathParameter(favoriteId, { name: 'favoriteId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'DELETE' as any });
  }
}

export class ChatMessagesApi {
  private client: HttpClient;
  public readonly favorites: ChatMessagesFavoritesApi;
  public readonly visibility: ChatMessagesVisibilityApi;
  public readonly reactions: ChatMessagesReactionsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.favorites = new ChatMessagesFavoritesApi(client);
    this.visibility = new ChatMessagesVisibilityApi(client);
    this.reactions = new ChatMessagesReactionsApi(client);
  }


/** Edit a message */
  async edit(messageId: string, body: EditMessageRequest, requestOptions?: ApiRequestOptions): Promise<MessageMutationResult> {
    return this.client.request<MessageMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/edit`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Recall a message */
  async recall(messageId: string, body: RecallMessageRequest, requestOptions?: ApiRequestOptions): Promise<MessageMutationResult> {
    return this.client.request<MessageMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/recall`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Pin a message */
  async pin(messageId: string, requestOptions?: ApiRequestOptions): Promise<MessagePinMutationResult> {
    return this.client.request<MessagePinMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/pin`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }

/** Unpin a message */
  async unpin(messageId: string, requestOptions?: ApiRequestOptions): Promise<MessagePinMutationResult> {
    return this.client.request<MessagePinMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/unpin`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }
}

export interface ChatConversationsPinsListParams {
  cursor?: string;
  pageSize?: number;
}

export class ChatConversationsPinsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List pinned messages */
  async list(conversationId: string, params?: ChatConversationsPinsListParams, requestOptions?: ApiRequestOptions): Promise<{ items: MessageInteractionSummaryView[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MessageInteractionSummaryView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/pins`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }
}

export class ChatConversationsMessagesInteractionSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve message interaction summary */
  async retrieve(conversationId: string, messageId: string, requestOptions?: ApiRequestOptions): Promise<MessageInteractionSummaryView> {
    return this.client.request<MessageInteractionSummaryView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/interaction_summary`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }
}

export interface ChatConversationsMessagesListParams {
  cursor?: string;
  pageSize?: number;
}

export class ChatConversationsMessagesApi {
  private client: HttpClient;
  public readonly interactionSummary: ChatConversationsMessagesInteractionSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.interactionSummary = new ChatConversationsMessagesInteractionSummaryApi(client);
  }


/** List conversation message history */
  async list(conversationId: string, params?: ChatConversationsMessagesListParams, requestOptions?: ApiRequestOptions): Promise<{ items: ConversationMessageEntry[]; pageInfo: PageInfo; highWatermark: number; }> {
    const query = buildQueryString([
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: ConversationMessageEntry[]; pageInfo: PageInfo; highWatermark: number; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Post a conversation message */
  async create(conversationId: string, body: PostMessageRequest, requestOptions?: ApiRequestOptions): Promise<PostMessageResult> {
    return this.client.request<PostMessageResult>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }
}

export interface ChatConversationsMemberDirectoryListParams {
  cursor?: string;
  pageSize?: number;
}

export class ChatConversationsMemberDirectoryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List member directory */
  async list(conversationId: string, params?: ChatConversationsMemberDirectoryListParams, requestOptions?: ApiRequestOptions): Promise<{ items: ConversationMember[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: ConversationMember[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/member_directory`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }
}

export class ChatConversationsReadCursorApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve read cursor */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ReadCursorView> {
    return this.client.request<ReadCursorView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/read_cursor`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Update read cursor */
  async update(conversationId: string, body: UpdateReadCursorRequest, requestOptions?: ApiRequestOptions): Promise<ReadCursorView> {
    return this.client.request<ReadCursorView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/read_cursor`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsProfileApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve conversation profile */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationProfileView> {
    return this.client.request<ConversationProfileView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/profile`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Update conversation profile */
  async update(conversationId: string, body: UpdateConversationProfileRequest, requestOptions?: ApiRequestOptions): Promise<ConversationProfileView> {
    return this.client.request<ConversationProfileView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/profile`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve conversation preferences */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationPreferencesView> {
    return this.client.request<ConversationPreferencesView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/preferences`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Update conversation preferences */
  async update(conversationId: string, body: UpdateConversationPreferencesRequest, requestOptions?: ApiRequestOptions): Promise<ConversationPreferencesView> {
    return this.client.request<ConversationPreferencesView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/preferences`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PATCH' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsAgentsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve assigned group agents */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationAgentAssignments> {
    return this.client.request<ConversationAgentAssignments>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agents`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Update assigned group agents */
  async update(conversationId: string, body: UpdateConversationAgentsRequest, requestOptions?: ApiRequestOptions): Promise<ConversationAgentAssignments> {
    return this.client.request<ConversationAgentAssignments>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agents`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'PUT' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsMembersCurrentApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve the current conversation member */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/current`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }
}

export interface ChatConversationsMembersListParams {
  pageSize?: number;
  cursor?: string;
}

export class ChatConversationsMembersApi {
  private client: HttpClient;
  public readonly current: ChatConversationsMembersCurrentApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.current = new ChatConversationsMembersCurrentApi(client);
  }


/** List conversation members */
  async list(conversationId: string, params?: ChatConversationsMembersListParams, requestOptions?: ApiRequestOptions): Promise<{ items: ConversationMember[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: ConversationMember[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Add a conversation member */
  async add(conversationId: string, body: AddConversationMemberRequest, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/add`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Remove a conversation member */
  async remove(conversationId: string, body: RemoveConversationMemberRequest, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/remove`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Transfer conversation owner */
  async transferOwner(conversationId: string, body: TransferConversationOwnerRequest, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/transfer_owner`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Change conversation member role */
  async changeRole(conversationId: string, body: ChangeConversationMemberRoleRequest, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/change_role`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Leave a conversation */
  async leave(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/leave`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }

/** Accept a conversation invitation */
  async acceptInvitation(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/accept_invitation`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }
}

export class ChatConversationsDirectChatsBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a direct chat conversation binding */
  async create(body: BindDirectChatRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/direct_chats/bindings`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsDirectChatsApi {
  private client: HttpClient;
  public readonly bindings: ChatConversationsDirectChatsBindingsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.bindings = new ChatConversationsDirectChatsBindingsApi(client);
  }

}

export class ChatConversationsThreadsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a thread conversation */
  async create(body: CreateThreadConversationRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/threads`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsSystemChannelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a system channel */
  async create(body: CreateSystemChannelRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/system_channels`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Publish a system channel message */
  async publish(conversationId: string, body: PostMessageRequest, requestOptions?: ApiRequestOptions): Promise<PostMessageResult> {
    return this.client.request<PostMessageResult>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/system_channel/publish`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsAgentHandoffsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create an agent handoff */
  async create(body: CreateAgentHandoffRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/agent_handoffs`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Retrieve agent handoff state */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }

/** Accept agent handoff */
  async accept(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/accept`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }

/** Resolve agent handoff */
  async resolve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/resolve`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }

/** Close agent handoff */
  async close(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/close`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any });
  }
}

export class ChatConversationsAgentDialogsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create an agent dialog */
  async create(body: CreateAgentDialogRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/agent_dialogs`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }
}

export class ChatConversationsApi {
  private client: HttpClient;
  public readonly agentDialogs: ChatConversationsAgentDialogsApi;
  public readonly agentHandoffs: ChatConversationsAgentHandoffsApi;
  public readonly systemChannels: ChatConversationsSystemChannelsApi;
  public readonly threads: ChatConversationsThreadsApi;
  public readonly directChats: ChatConversationsDirectChatsApi;
  public readonly members: ChatConversationsMembersApi;
  public readonly agents: ChatConversationsAgentsApi;
  public readonly preferences: ChatConversationsPreferencesApi;
  public readonly profile: ChatConversationsProfileApi;
  public readonly readCursor: ChatConversationsReadCursorApi;
  public readonly memberDirectory: ChatConversationsMemberDirectoryApi;
  public readonly messages: ChatConversationsMessagesApi;
  public readonly pins: ChatConversationsPinsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.agentDialogs = new ChatConversationsAgentDialogsApi(client);
    this.agentHandoffs = new ChatConversationsAgentHandoffsApi(client);
    this.systemChannels = new ChatConversationsSystemChannelsApi(client);
    this.threads = new ChatConversationsThreadsApi(client);
    this.directChats = new ChatConversationsDirectChatsApi(client);
    this.members = new ChatConversationsMembersApi(client);
    this.agents = new ChatConversationsAgentsApi(client);
    this.preferences = new ChatConversationsPreferencesApi(client);
    this.profile = new ChatConversationsProfileApi(client);
    this.readCursor = new ChatConversationsReadCursorApi(client);
    this.memberDirectory = new ChatConversationsMemberDirectoryApi(client);
    this.messages = new ChatConversationsMessagesApi(client);
    this.pins = new ChatConversationsPinsApi(client);
  }


/** Create a conversation */
  async create(body: CreateConversationRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json' });
  }

/** Retrieve conversation summary */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationSummaryView> {
    return this.client.request<ConversationSummaryView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }
}

export interface ChatInboxListParams {
  pageSize?: number;
  cursor?: string;
  conversationType?: string;
  q?: string;
}

export class ChatInboxApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List current inbox window */
  async list(params?: ChatInboxListParams, requestOptions?: ApiRequestOptions): Promise<{ items: ConversationInboxEntry[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'conversation_type', value: params?.conversationType, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: ConversationInboxEntry[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/chat/inbox`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any });
  }
}

export class ChatApi {
  private client: HttpClient;
  public readonly inbox: ChatInboxApi;
  public readonly conversations: ChatConversationsApi;
  public readonly messages: ChatMessagesApi;
  public readonly rooms: ChatRoomsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.inbox = new ChatInboxApi(client);
    this.conversations = new ChatConversationsApi(client);
    this.messages = new ChatMessagesApi(client);
    this.rooms = new ChatRoomsApi(client);
  }

}

export function createChatApi(client: HttpClient): ChatApi {
  return new ChatApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
