import type {
  AddConversationMemberRequest,
  BindDirectChatRequest,
  ConversationProfileView,
  ConversationPreferencesView,
  CreateAgentDialogRequest,
  CreateAgentHandoffRequest,
  CreateConversationRequest,
  CreateConversationResult,
  CreateSystemChannelRequest,
  CreateThreadConversationRequest,
  MessageInteractionSummaryView,
  PostMessageResult,
  PostMessageRequest,
  QueryParams,
  ReadCursorView,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
} from '../generated/server-openapi/dist/index.js';
import type {
  ConversationMessageListResponse,
  ConversationInboxPage,
  ListMembersResponse,
  PinnedMessagesResponse,
} from './openapi-compat-types';
import { requireStringIdentifier } from './identifier-boundary';
import type { ImTransportClientLike, MessageHistoryListParams } from './transport-client-like';

export type { MessageHistoryListParams } from './transport-client-like';

export class ImConversationsModule {
  constructor(private readonly transportClient: ImTransportClientLike) {}

  create(body: CreateConversationRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.create(body);
  }

  list(params?: QueryParams): Promise<ConversationInboxPage> {
    return this.transportClient.chat.inbox.list(params);
  }

  createAgentDialog(body: CreateAgentDialogRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.agentDialogs.create(body);
  }

  createAgentHandoff(body: CreateAgentHandoffRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.agentHandoffs.create(body);
  }

  createSystemChannel(body: CreateSystemChannelRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.systemChannels.create(body);
  }

  createThreadConversation(body: CreateThreadConversationRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.threads.create(body);
  }

  bindDirectChat(body: BindDirectChatRequest): Promise<CreateConversationResult> {
    return this.transportClient.chat.conversations.directChats.bindings.create(body);
  }

  listMessages(
    conversationId: string,
    params?: MessageHistoryListParams,
  ): Promise<ConversationMessageListResponse> {
    return this.transportClient.chat.conversations.messages.list(
      requireStringIdentifier(conversationId, 'conversationId'),
      params,
    );
  }

  postMessage(conversationId: string, body: PostMessageRequest): Promise<PostMessageResult> {
    return this.transportClient.chat.conversations.messages.create(
      requireStringIdentifier(conversationId, 'conversationId'),
      body,
    );
  }

  postText(
    conversationId: string,
    text: string,
    body: Omit<PostMessageRequest, 'text'> = {},
  ): Promise<PostMessageResult> {
    return this.postMessage(conversationId, { ...body, text });
  }

  updateReadCursor(conversationId: string, body: { readSeq: number }): Promise<ReadCursorView> {
    return this.transportClient.chat.conversations.readCursor.update(
      requireStringIdentifier(conversationId, 'conversationId'),
      body,
    );
  }

  getMessageInteractionSummary(
    conversationId: string,
    messageId: string,
  ): Promise<MessageInteractionSummaryView> {
    return this.transportClient.chat.conversations.messages.interactionSummary.retrieve(
      requireStringIdentifier(conversationId, 'conversationId'),
      requireStringIdentifier(messageId, 'messageId'),
    );
  }

  listPinnedMessages(conversationId: string): Promise<PinnedMessagesResponse> {
    return this.transportClient.chat.conversations.pins.list(
      requireStringIdentifier(conversationId, 'conversationId'),
    );
  }

  getPreferences(conversationId: string): Promise<ConversationPreferencesView> {
    return this.transportClient.chat.conversations.preferences.retrieve(
      requireStringIdentifier(conversationId, 'conversationId'),
    );
  }

  updatePreferences(
    conversationId: string,
    body: UpdateConversationPreferencesRequest,
  ): Promise<ConversationPreferencesView> {
    return this.transportClient.chat.conversations.preferences.update(
      requireStringIdentifier(conversationId, 'conversationId'),
      body,
    );
  }

  getProfile(conversationId: string): Promise<ConversationProfileView> {
    return this.transportClient.chat.conversations.profile.retrieve(
      requireStringIdentifier(conversationId, 'conversationId'),
    );
  }

  updateProfile(
    conversationId: string,
    body: UpdateConversationProfileRequest,
  ): Promise<ConversationProfileView> {
    return this.transportClient.chat.conversations.profile.update(
      requireStringIdentifier(conversationId, 'conversationId'),
      body,
    );
  }

  listMembers(conversationId: string, params?: QueryParams): Promise<ListMembersResponse> {
    return this.transportClient.chat.conversations.members.list(
      requireStringIdentifier(conversationId, 'conversationId'),
      params,
    );
  }

  addMember(conversationId: string, body: AddConversationMemberRequest): Promise<unknown> {
    return this.transportClient.chat.conversations.members.add(
      requireStringIdentifier(conversationId, 'conversationId'),
      body,
    );
  }

  removeMember(conversationId: string, body: unknown): Promise<unknown> {
    return this.transportClient.chat.conversations.members.remove(
      requireStringIdentifier(conversationId, 'conversationId'),
      body,
    );
  }

  leave(conversationId: string): Promise<unknown> {
    return this.transportClient.chat.conversations.members.leave(
      requireStringIdentifier(conversationId, 'conversationId'),
    );
  }

  acceptInvitation(conversationId: string): Promise<import('../generated/server-openapi/dist/index.js').ConversationMember> {
    return this.transportClient.chat.conversations.members.acceptInvitation(
      requireStringIdentifier(conversationId, 'conversationId'),
    );
  }
}
