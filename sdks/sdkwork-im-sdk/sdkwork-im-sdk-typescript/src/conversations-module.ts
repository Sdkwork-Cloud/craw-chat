import type {
  AddConversationMemberRequest,
  BindDirectChatRequest,
  ConversationProfileView,
  ConversationPreferencesView,
  CreateAgentDialogRequest,
  CreateConversationRequest,
  CreateConversationResult,
  MessageInteractionSummaryView,
  PostMessageRequest,
  PostedMessageResponse,
  QueryParams,
  ReadCursorView,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
} from '@sdkwork/im-sdk-generated';
import type { InboxResponse, ListMembersResponse, PinnedMessagesResponse, TimelineResponse } from './openapi-compat-types';
import type { ImTransportClientLike } from './transport-client-like';

export class ImConversationsModule {
  constructor(private readonly transportClient: ImTransportClientLike) {}

  create(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.create(body);
  }

  list(params?: QueryParams): Promise<InboxResponse> {
    return this.transportClient.chat.inbox.retrieve(params);
  }

  createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.agentDialogs.create(body);
  }

  bindDirectChat(body: BindDirectChatRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.directChats.bind(body);
  }

  listMessages(conversationId: string | number, params?: QueryParams): Promise<TimelineResponse> {
    return this.transportClient.chat.conversations.messages.list(conversationId, params);
  }

  postMessage(conversationId: string | number, body: PostMessageRequest): Promise<PostedMessageResponse> {
    return this.transportClient.chat.conversations.messages.create(conversationId, body);
  }

  postText(
    conversationId: string | number,
    text: string,
    body: Omit<PostMessageRequest, 'text'> = {},
  ): Promise<PostedMessageResponse> {
    return this.postMessage(conversationId, { ...body, text });
  }

  updateReadCursor(conversationId: string | number, body: { readSeq: number }): Promise<ReadCursorView> {
    return this.transportClient.chat.conversations.readCursor.update(conversationId, body);
  }

  getMessageInteractionSummary(
    conversationId: string | number,
    messageId: string | number,
  ): Promise<MessageInteractionSummaryView> {
    return this.transportClient.chat.conversations.messages.interactionSummary.retrieve(conversationId, messageId);
  }

  listPinnedMessages(conversationId: string | number): Promise<PinnedMessagesResponse> {
    return this.transportClient.chat.conversations.pins.list(conversationId);
  }

  getPreferences(conversationId: string | number): Promise<ConversationPreferencesView> {
    return this.transportClient.chat.conversations.preferences.retrieve(conversationId);
  }

  updatePreferences(
    conversationId: string | number,
    body: UpdateConversationPreferencesRequest,
  ): Promise<ConversationPreferencesView> {
    return this.transportClient.chat.conversations.preferences.update(conversationId, body);
  }

  getProfile(conversationId: string | number): Promise<ConversationProfileView> {
    return this.transportClient.chat.conversations.profile.retrieve(conversationId);
  }

  updateProfile(
    conversationId: string | number,
    body: UpdateConversationProfileRequest,
  ): Promise<ConversationProfileView> {
    return this.transportClient.chat.conversations.profile.update(conversationId, body);
  }

  listMembers(conversationId: string | number, params?: QueryParams): Promise<ListMembersResponse> {
    return this.transportClient.chat.conversations.members.list(conversationId, params);
  }

  addMember(conversationId: string | number, body: AddConversationMemberRequest): Promise<unknown> {
    return this.transportClient.chat.conversations.members.add(conversationId, body);
  }

  removeMember(conversationId: string | number, body: unknown): Promise<unknown> {
    return this.transportClient.chat.conversations.members.remove(conversationId, body);
  }

  leave(conversationId: string | number): Promise<unknown> {
    return this.transportClient.chat.conversations.members.leave(conversationId);
  }
}
