import { imApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AckResponse, AddConversationMemberRequest, BindDirectChatRequest, ChangeConversationMemberRoleRequest, ContactsResponse, ConversationMember, ConversationPreferencesView, ConversationProfileView, ConversationSummaryView, CreateAgentDialogRequest, CreateConversationRequest, CreateConversationResult, CreateRoomRequest, DeleteMessageFavoriteResponse, EditMessageRequest, EnterRoomResponse, FavoriteMessageRequest, FavoriteMessagesResponse, InboxResponse, ListMembersResponse, MemberDirectoryResponse, MessageFavoriteType, MessageFavoriteView, MessageInteractionSummaryView, MessagePinMutationResult, MessageReactionMutationResult, MessageReactionRequest, MessageVisibilityMutationResult, PinnedMessagesResponse, PostedMessageResponse, PostMessageRequest, ReadCursorView, RemoveConversationMemberRequest, RoomView, TimelineResponse, TransferConversationOwnerRequest, UpdateConversationPreferencesRequest, UpdateConversationProfileRequest, UpdateReadCursorRequest } from '../types';


export class ChatRoomsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a live, chat, or game room bound to a group conversation */
  async create(body: CreateRoomRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(imApiPath(`/chat/rooms`), body, undefined, undefined, 'application/json');
  }

/** Get room metadata and active member count */
  async get(roomId: string): Promise<RoomView> {
    return this.client.get<RoomView>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}`));
  }

/** Enter a room as the authenticated principal */
  async enter(roomId: string): Promise<EnterRoomResponse> {
    return this.client.post<EnterRoomResponse>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}/enter`));
  }

/** Leave a room as the authenticated principal */
  async leave(roomId: string): Promise<EnterRoomResponse> {
    return this.client.post<EnterRoomResponse>(imApiPath(`/chat/rooms/${serializePathParameter(roomId, { name: 'roomId', style: 'simple', explode: false })}/leave`));
  }
}

export class ChatMessagesPinApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Pin a message */
  async create(messageId: string): Promise<MessagePinMutationResult> {
    return this.client.post<MessagePinMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/pin`));
  }

/** Unpin a message */
  async delete(messageId: string): Promise<MessagePinMutationResult> {
    return this.client.post<MessagePinMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/unpin`));
  }
}

export class ChatMessagesReactionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Add a message reaction */
  async create(messageId: string, body: MessageReactionRequest): Promise<MessageReactionMutationResult> {
    return this.client.post<MessageReactionMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/reactions`), body, undefined, undefined, 'application/json');
  }

/** Remove a message reaction */
  async delete(messageId: string, body: MessageReactionRequest): Promise<MessageReactionMutationResult> {
    return this.client.post<MessageReactionMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/reactions/remove`), body, undefined, undefined, 'application/json');
  }
}

export class ChatMessagesVisibilityApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Delete message visibility for the current principal */
  async delete(messageId: string): Promise<MessageVisibilityMutationResult> {
    return this.client.delete<MessageVisibilityMutationResult>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/visibility`));
  }
}

export interface ChatMessagesFavoritesListParams {
  limit?: number;
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
  async list(params?: ChatMessagesFavoritesListParams): Promise<FavoriteMessagesResponse> {
    const query = buildQueryString([
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'favoriteType', value: params?.favoriteType, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<FavoriteMessagesResponse>(appendQueryString(imApiPath(`/chat/messages/favorites`), query));
  }

/** Favorite a message */
  async create(messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView> {
    return this.client.post<MessageFavoriteView>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/favorites`), body, undefined, undefined, 'application/json');
  }

/** Delete a message favorite */
  async delete(favoriteId: string): Promise<DeleteMessageFavoriteResponse> {
    return this.client.delete<DeleteMessageFavoriteResponse>(imApiPath(`/chat/messages/favorites/${serializePathParameter(favoriteId, { name: 'favoriteId', style: 'simple', explode: false })}`));
  }
}

export class ChatMessagesApi {
  private client: HttpClient;
  public readonly favorites: ChatMessagesFavoritesApi;
  public readonly visibility: ChatMessagesVisibilityApi;
  public readonly reactions: ChatMessagesReactionsApi;
  public readonly pin: ChatMessagesPinApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.favorites = new ChatMessagesFavoritesApi(client);
    this.visibility = new ChatMessagesVisibilityApi(client);
    this.reactions = new ChatMessagesReactionsApi(client);
    this.pin = new ChatMessagesPinApi(client);
  }


/** Edit a message */
  async edit(messageId: string, body: EditMessageRequest): Promise<PostedMessageResponse> {
    return this.client.post<PostedMessageResponse>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/edit`), body, undefined, undefined, 'application/json');
  }

/** Recall a message */
  async recall(messageId: string): Promise<PostedMessageResponse> {
    return this.client.post<PostedMessageResponse>(imApiPath(`/chat/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/recall`));
  }
}

export class ChatConversationsPinsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List pinned messages */
  async list(conversationId: string): Promise<PinnedMessagesResponse> {
    return this.client.get<PinnedMessagesResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/pins`));
  }
}

export class ChatConversationsMessagesInteractionSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve message interaction summary */
  async retrieve(conversationId: string, messageId: string): Promise<MessageInteractionSummaryView> {
    return this.client.get<MessageInteractionSummaryView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages/${serializePathParameter(messageId, { name: 'messageId', style: 'simple', explode: false })}/interaction_summary`));
  }
}

export interface ChatConversationsMessagesListParams {
  afterSeq?: number;
  limit?: number;
}

export class ChatConversationsMessagesApi {
  private client: HttpClient;
  public readonly interactionSummary: ChatConversationsMessagesInteractionSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.interactionSummary = new ChatConversationsMessagesInteractionSummaryApi(client);
  }


/** List conversation message timeline */
  async list(conversationId: string, params?: ChatConversationsMessagesListParams): Promise<TimelineResponse> {
    const query = buildQueryString([
      { name: 'afterSeq', value: params?.afterSeq, style: 'form', explode: true, allowReserved: false },
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<TimelineResponse>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages`), query));
  }

/** Post a conversation message */
  async create(conversationId: string, body: PostMessageRequest): Promise<PostedMessageResponse> {
    return this.client.post<PostedMessageResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/messages`), body, undefined, undefined, 'application/json');
  }
}

export class ChatConversationsMemberDirectoryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List member directory */
  async list(conversationId: string): Promise<MemberDirectoryResponse> {
    return this.client.get<MemberDirectoryResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/member_directory`));
  }
}

export class ChatConversationsReadCursorApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve read cursor */
  async retrieve(conversationId: string): Promise<ReadCursorView> {
    return this.client.get<ReadCursorView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/read_cursor`));
  }

/** Update read cursor */
  async update(conversationId: string, body: UpdateReadCursorRequest): Promise<ReadCursorView> {
    return this.client.post<ReadCursorView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/read_cursor`), body, undefined, undefined, 'application/json');
  }
}

export class ChatConversationsProfileApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve conversation profile */
  async retrieve(conversationId: string): Promise<ConversationProfileView> {
    return this.client.get<ConversationProfileView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/profile`));
  }

/** Update conversation profile */
  async update(conversationId: string, body: UpdateConversationProfileRequest): Promise<ConversationProfileView> {
    return this.client.patch<ConversationProfileView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/profile`), body, undefined, undefined, 'application/json');
  }
}

export class ChatConversationsPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve conversation preferences */
  async retrieve(conversationId: string): Promise<ConversationPreferencesView> {
    return this.client.get<ConversationPreferencesView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/preferences`));
  }

/** Update conversation preferences */
  async update(conversationId: string, body: UpdateConversationPreferencesRequest): Promise<ConversationPreferencesView> {
    return this.client.patch<ConversationPreferencesView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/preferences`), body, undefined, undefined, 'application/json');
  }
}

export interface ChatConversationsMembersListParams {
  limit?: number;
  cursor?: string;
}

export class ChatConversationsMembersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List conversation members */
  async list(conversationId: string, params?: ChatConversationsMembersListParams): Promise<ListMembersResponse> {
    const query = buildQueryString([
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ListMembersResponse>(appendQueryString(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members`), query));
  }

/** Add a conversation member */
  async add(conversationId: string, body: AddConversationMemberRequest): Promise<ConversationMember> {
    return this.client.post<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/add`), body, undefined, undefined, 'application/json');
  }

/** Remove a conversation member */
  async remove(conversationId: string, body: RemoveConversationMemberRequest): Promise<AckResponse> {
    return this.client.post<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/remove`), body, undefined, undefined, 'application/json');
  }

/** Transfer conversation owner */
  async transferOwner(conversationId: string, body: TransferConversationOwnerRequest): Promise<ConversationMember> {
    return this.client.post<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/transfer_owner`), body, undefined, undefined, 'application/json');
  }

/** Change conversation member role */
  async changeRole(conversationId: string, body: ChangeConversationMemberRoleRequest): Promise<ConversationMember> {
    return this.client.post<ConversationMember>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/change_role`), body, undefined, undefined, 'application/json');
  }

/** Leave a conversation */
  async leave(conversationId: string): Promise<AckResponse> {
    return this.client.post<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/members/leave`));
  }
}

export class ChatConversationsDirectChatsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Bind a direct chat conversation */
  async bind(body: BindDirectChatRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(imApiPath(`/chat/conversations/direct_chats/bindings`), body, undefined, undefined, 'application/json');
  }
}

export class ChatConversationsThreadsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a thread conversation */
  async create(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(imApiPath(`/chat/conversations/threads`), body, undefined, undefined, 'application/json');
  }
}

export class ChatConversationsSystemChannelsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a system channel */
  async create(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(imApiPath(`/chat/conversations/system_channels`), body, undefined, undefined, 'application/json');
  }

/** Publish a system channel message */
  async publish(conversationId: string, body: PostMessageRequest): Promise<PostedMessageResponse> {
    return this.client.post<PostedMessageResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/system_channel/publish`), body, undefined, undefined, 'application/json');
  }
}

export class ChatConversationsAgentHandoffsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create an agent handoff */
  async create(body: CreateAgentDialogRequest): Promise<AckResponse> {
    return this.client.post<AckResponse>(imApiPath(`/chat/conversations/agent_handoffs`), body, undefined, undefined, 'application/json');
  }

/** Retrieve agent handoff state */
  async retrieve(conversationId: string): Promise<AckResponse> {
    return this.client.get<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff`));
  }

/** Accept agent handoff */
  async accept(conversationId: string): Promise<AckResponse> {
    return this.client.post<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/accept`));
  }

/** Resolve agent handoff */
  async resolve(conversationId: string): Promise<AckResponse> {
    return this.client.post<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/resolve`));
  }

/** Close agent handoff */
  async close(conversationId: string): Promise<AckResponse> {
    return this.client.post<AckResponse>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}/agent_handoff/close`));
  }
}

export class ChatConversationsAgentDialogsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create an agent dialog */
  async create(body: CreateAgentDialogRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(imApiPath(`/chat/conversations/agent_dialogs`), body, undefined, undefined, 'application/json');
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
    this.preferences = new ChatConversationsPreferencesApi(client);
    this.profile = new ChatConversationsProfileApi(client);
    this.readCursor = new ChatConversationsReadCursorApi(client);
    this.memberDirectory = new ChatConversationsMemberDirectoryApi(client);
    this.messages = new ChatConversationsMessagesApi(client);
    this.pins = new ChatConversationsPinsApi(client);
  }


/** Create a conversation */
  async create(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.client.post<CreateConversationResult>(imApiPath(`/chat/conversations`), body, undefined, undefined, 'application/json');
  }

/** Retrieve conversation summary */
  async retrieve(conversationId: string): Promise<ConversationSummaryView> {
    return this.client.get<ConversationSummaryView>(imApiPath(`/chat/conversations/${serializePathParameter(conversationId, { name: 'conversationId', style: 'simple', explode: false })}`));
  }
}

export interface ChatInboxRetrieveParams {
  limit?: number;
  cursor?: string;
}

export class ChatInboxApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve current inbox window */
  async retrieve(params?: ChatInboxRetrieveParams): Promise<InboxResponse> {
    const query = buildQueryString([
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<InboxResponse>(appendQueryString(imApiPath(`/chat/inbox`), query));
  }
}

export interface ChatContactsListParams {
  limit?: number;
  cursor?: string;
}

export class ChatContactsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List IM contacts */
  async list(params?: ChatContactsListParams): Promise<ContactsResponse> {
    const query = buildQueryString([
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ContactsResponse>(appendQueryString(imApiPath(`/chat/contacts`), query));
  }
}

export class ChatApi {
  private client: HttpClient;
  public readonly contacts: ChatContactsApi;
  public readonly inbox: ChatInboxApi;
  public readonly conversations: ChatConversationsApi;
  public readonly messages: ChatMessagesApi;
  public readonly rooms: ChatRoomsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.contacts = new ChatContactsApi(client);
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
