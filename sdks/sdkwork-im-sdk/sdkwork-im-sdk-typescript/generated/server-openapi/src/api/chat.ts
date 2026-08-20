import { imApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { AckResponse, AddConversationMemberRequest, BindDirectChatRequest, ChangeConversationMemberRoleRequest, ConversationAgentAssignments, ConversationInboxEntry, ConversationMember, ConversationMessageEntry, ConversationPreferencesView, ConversationProfileView, ConversationSummaryView, CreateAgentDialogRequest, CreateAgentHandoffRequest, CreateConversationRequest, CreateConversationResult, CreateRoomRequest, CreateSystemChannelRequest, CreateThreadConversationRequest, EditMessageRequest, EnterRoomResponse, FavoriteMessageRequest, MessageFavoriteType, MessageFavoriteView, MessageInteractionSummaryView, MessageMutationResult, MessagePinMutationResult, MessageReactionMutationResult, MessageReactionRequest, MessageSearchHit, PageInfo, PostMessageRequest, PostMessageResult, ReadCursorView, RecallMessageRequest, RemoveConversationMemberRequest, RoomView, TransferConversationOwnerRequest, UpdateConversationAgentsRequest, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest, UpdateReadCursorRequest, WelcomeEnsureView } from '../types';


export class ChatRoomsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a live, chat, or game room bound to a group conversation */
  async create(body: CreateRoomRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/rooms`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Retrieve room metadata and active member count */
  async retrieve(roomId: string, requestOptions?: ApiRequestOptions): Promise<RoomView> {
    return this.client.request<RoomView>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Enter a room as the authenticated principal */
  async enter(roomId: string, requestOptions?: ApiRequestOptions): Promise<EnterRoomResponse> {
    return this.client.request<EnterRoomResponse>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}/enter`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Leave a room as the authenticated principal */
  async leave(roomId: string, requestOptions?: ApiRequestOptions): Promise<EnterRoomResponse> {
    return this.client.request<EnterRoomResponse>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}/leave`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ChatMessagesReactionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Add a message reaction */
  async create(messageId: string, body: MessageReactionRequest, requestOptions?: ApiRequestOptions): Promise<MessageReactionMutationResult> {
    return this.client.request<MessageReactionMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/reactions`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Remove a message reaction */
  async remove(messageId: string, body: MessageReactionRequest, requestOptions?: ApiRequestOptions): Promise<MessageReactionMutationResult> {
    return this.client.request<MessageReactionMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/reactions/remove`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatMessagesVisibilityApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Delete message visibility for the current principal */
  async delete(messageId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/visibility`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
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
    return this.client.request<{ items: MessageFavoriteView[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/chat/messages/favorites`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Favorite a message */
  async create(messageId: string, body: FavoriteMessageRequest, requestOptions?: ApiRequestOptions): Promise<MessageFavoriteView> {
    return this.client.request<MessageFavoriteView>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/favorites`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Delete a message favorite */
  async delete(favoriteId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(imApiPath(`/chat/messages/favorites/${serializePathParameter(favoriteId, { name: 'favoriteId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export interface ChatMessagesSearchParams {
  q: string;
  conversationId?: string;
  pageSize?: number;
  cursor?: string;
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


/** Search conversation message history */
  async search(params: ChatMessagesSearchParams, requestOptions?: ApiRequestOptions): Promise<{ items: MessageSearchHit[]; pageInfo: PageInfo; }> {
    const query = buildQueryString([
      { name: 'q', value: params.q, style: 'form', explode: true, allowReserved: false },
      { name: 'conversationId', value: params.conversationId, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<{ items: MessageSearchHit[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/chat/messages/search`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Edit a message */
  async edit(messageId: string, body: EditMessageRequest, requestOptions?: ApiRequestOptions): Promise<MessageMutationResult> {
    return this.client.request<MessageMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/edit`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Recall a message */
  async recall(messageId: string, body: RecallMessageRequest, requestOptions?: ApiRequestOptions): Promise<MessageMutationResult> {
    return this.client.request<MessageMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/recall`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Pin a message */
  async pin(messageId: string, requestOptions?: ApiRequestOptions): Promise<MessagePinMutationResult> {
    return this.client.request<MessagePinMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/pin`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Unpin a message */
  async unpin(messageId: string, requestOptions?: ApiRequestOptions): Promise<MessagePinMutationResult> {
    return this.client.request<MessagePinMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/unpin`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
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
    return this.client.request<{ items: MessageInteractionSummaryView[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/pins`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class ChatConversationsMessagesInteractionSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve message interaction summary */
  async retrieve(conversationId: string, messageId: string, requestOptions?: ApiRequestOptions): Promise<MessageInteractionSummaryView> {
    return this.client.request<MessageInteractionSummaryView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/interaction_summary`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
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
    return this.client.request<{ items: ConversationMessageEntry[]; pageInfo: PageInfo; highWatermark: number; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Post a conversation message */
  async create(conversationId: string, body: PostMessageRequest, requestOptions?: ApiRequestOptions): Promise<PostMessageResult> {
    return this.client.request<PostMessageResult>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
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
    return this.client.request<{ items: ConversationMember[]; pageInfo: PageInfo; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/member_directory`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class ChatConversationsReadCursorApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve read cursor */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ReadCursorView> {
    return this.client.request<ReadCursorView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/read_cursor`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update read cursor */
  async update(conversationId: string, body: UpdateReadCursorRequest, requestOptions?: ApiRequestOptions): Promise<ReadCursorView> {
    return this.client.request<ReadCursorView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/read_cursor`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsProfileApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve conversation profile */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationProfileView> {
    return this.client.request<ConversationProfileView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/profile`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update conversation profile */
  async update(conversationId: string, body: UpdateConversationProfileRequest, requestOptions?: ApiRequestOptions): Promise<ConversationProfileView> {
    return this.client.request<ConversationProfileView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/profile`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve conversation preferences */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationPreferencesView> {
    return this.client.request<ConversationPreferencesView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/preferences`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update conversation preferences */
  async update(conversationId: string, body: UpdateConversationPreferencesRequest, requestOptions?: ApiRequestOptions): Promise<ConversationPreferencesView> {
    return this.client.request<ConversationPreferencesView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/preferences`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsAgentsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve assigned group agents */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationAgentAssignments> {
    return this.client.request<ConversationAgentAssignments>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agents`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update assigned group agents */
  async update(conversationId: string, body: UpdateConversationAgentsRequest, requestOptions?: ApiRequestOptions): Promise<ConversationAgentAssignments> {
    return this.client.request<ConversationAgentAssignments>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agents`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsMembersCurrentApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve the current conversation member */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/current`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
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
    return this.client.request<{ items: ConversationMember[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Add a conversation member */
  async add(conversationId: string, body: AddConversationMemberRequest, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/add`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Remove a conversation member */
  async remove(conversationId: string, body: RemoveConversationMemberRequest, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/remove`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Transfer conversation owner */
  async transferOwner(conversationId: string, body: TransferConversationOwnerRequest, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/transfer_owner`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Change conversation member role */
  async changeRole(conversationId: string, body: ChangeConversationMemberRoleRequest, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/change_role`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Leave a conversation */
  async leave(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/leave`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Accept a conversation invitation */
  async acceptInvitation(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationMember> {
    return this.client.request<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/accept_invitation`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsDirectChatsBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a direct chat conversation binding */
  async create(body: BindDirectChatRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/direct_chats/bindings`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsDirectChatsApi {
  public readonly bindings: ChatConversationsDirectChatsBindingsApi;

  constructor(client: HttpClient) {
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
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/threads`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsSystemChannelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a system channel */
  async create(body: CreateSystemChannelRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/system_channels`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Publish a system channel message */
  async publish(conversationId: string, body: PostMessageRequest, requestOptions?: ApiRequestOptions): Promise<PostMessageResult> {
    return this.client.request<PostMessageResult>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/system_channel/publish`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsAgentHandoffsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create an agent handoff */
  async create(body: CreateAgentHandoffRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/agent_handoffs`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Retrieve agent handoff state */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Accept agent handoff */
  async accept(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/accept`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Resolve agent handoff */
  async resolve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/resolve`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Close agent handoff */
  async close(conversationId: string, requestOptions?: ApiRequestOptions): Promise<AckResponse> {
    return this.client.request<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/close`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ChatConversationsAgentDialogsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create an agent dialog */
  async create(body: CreateAgentDialogRequest, requestOptions?: ApiRequestOptions): Promise<CreateConversationResult> {
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations/agent_dialogs`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
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
    return this.client.request<CreateConversationResult>(imApiPath(`/chat/conversations`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Retrieve conversation summary */
  async retrieve(conversationId: string, requestOptions?: ApiRequestOptions): Promise<ConversationSummaryView> {
    return this.client.request<ConversationSummaryView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ChatMeWelcomeApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Ensure the current user received the system-agent Welcome message */
  async ensure(requestOptions?: ApiRequestOptions): Promise<WelcomeEnsureView> {
    return this.client.request<WelcomeEnsureView>(imApiPath(`/chat/me/welcome/ensure`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class ChatMeApi {
  public readonly welcome: ChatMeWelcomeApi;

  constructor(client: HttpClient) {
    this.welcome = new ChatMeWelcomeApi(client);
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
    return this.client.request<{ items: ConversationInboxEntry[]; pageInfo: { mode: 'cursor'; nextCursor?: string | null; hasMore: boolean; }; }>(appendQueryString(imApiPath(`/chat/inbox`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class ChatApi {
  public readonly inbox: ChatInboxApi;
  public readonly me: ChatMeApi;
  public readonly conversations: ChatConversationsApi;
  public readonly messages: ChatMessagesApi;
  public readonly rooms: ChatRoomsApi;

  constructor(client: HttpClient) {
    this.inbox = new ChatInboxApi(client);
    this.me = new ChatMeApi(client);
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
