import assert from 'node:assert/strict';
import type {
  ImDecodedMessage,
  ImLiveConnection,
  ImLiveConnectionState,
  ImMessageContext,
  ImRealtimeEventContext,
  ImSdkClient,
} from '@sdkwork/im-sdk';
import { createSdkworkChatService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService';
import {
  acquirePcLiveConnectionLease,
  configurePcRealtimeConnectionManager,
  resetPcRealtimeConnectionManagerForTests,
  subscribePcRealtimeScope,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager';
import type { SdkworkChatSession } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session';

type MessageListener = (message: ImDecodedMessage, context: ImMessageContext) => void;
type ScopeEventListener = (event: Record<string, unknown>, context: ImRealtimeEventContext) => void;
type StateListener = (state: ImLiveConnectionState) => void;

const authenticatedSession: SdkworkChatSession = {
  accessToken: 'test-access-token',
  authToken: 'test-auth-token',
  context: {
    appId: 'sdkwork-im-pc',
    authLevel: 'password',
    dataScope: [],
    deploymentMode: 'saas',
    environment: 'dev',
    permissionScope: [],
    sessionId: 'test-session',
    tenantId: 'test-tenant',
    userId: 'current-user',
  },
  sessionId: 'test-session',
  user: {
    id: 'current-user',
    userId: 'current-user',
  },
};

function createRealtimeChatService(getClient: () => ImSdkClient) {
  return createSdkworkChatService({
    getClient,
    getSession: () => authenticatedSession,
  });
}

async function flushRealtimeMicrotasks(): Promise<void> {
  for (let index = 0; index < 4; index += 1) {
    await Promise.resolve();
  }
}

async function flushChatListCoalesceWindow(): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, 420));
  await flushRealtimeMicrotasks();
}

class FakeLiveConnection implements ImLiveConnection {
  readonly disconnectCalls: Array<{ code?: number; reason?: string }> = [];
  readonly messageListeners = new Map<string, Set<MessageListener>>();
  readonly messageSubscribeCounts = new Map<string, number>();
  readonly messageUnsubscribeCounts = new Map<string, number>();
  readonly syncedConversationSnapshots: string[][] = [];
  readonly scopeEventListeners = new Map<string, Set<ScopeEventListener>>();
  readonly syncedScopeSnapshots: Array<Array<{ eventTypes?: string[]; scopeId: string; scopeType: string }>> = [];

  private readonly errorListeners = new Set<(error: unknown) => void>();
  private readonly stateListeners = new Set<StateListener>();

  disconnect(code?: number, reason?: string): void {
    this.disconnectCalls.push({ code, reason });
  }

  events = {
    onConversation: (conversationId: string, handler: ScopeEventListener) => {
      const key = `conversation:${conversationId}`;
      const listeners = this.scopeEventListeners.get(key) ?? new Set<ScopeEventListener>();
      listeners.add(handler);
      this.scopeEventListeners.set(key, listeners);
      return () => {
        listeners.delete(handler);
      };
    },
    onScope: (scopeType: string, scopeId: string, handler: ScopeEventListener) => {
      const key = `${scopeType}:${scopeId}`;
      const listeners = this.scopeEventListeners.get(key) ?? new Set<ScopeEventListener>();
      listeners.add(handler);
      this.scopeEventListeners.set(key, listeners);
      return () => {
        listeners.delete(handler);
      };
    },
  };

  lifecycle = {
    onError: (handler: (error: unknown) => void) => {
      this.errorListeners.add(handler);
      return () => {
        this.errorListeners.delete(handler);
      };
    },
    onStateChange: (handler: StateListener) => {
      this.stateListeners.add(handler);
      handler({ status: 'open' });
      return () => {
        this.stateListeners.delete(handler);
      };
    },
  };

  messages = {
    onConversation: (conversationId: string, handler: MessageListener) => {
      const listeners = this.messageListeners.get(conversationId) ?? new Set<MessageListener>();
      listeners.add(handler);
      this.messageListeners.set(conversationId, listeners);
      this.messageSubscribeCounts.set(
        conversationId,
        (this.messageSubscribeCounts.get(conversationId) ?? 0) + 1,
      );
      return () => {
        listeners.delete(handler);
        this.messageUnsubscribeCounts.set(
          conversationId,
          (this.messageUnsubscribeCounts.get(conversationId) ?? 0) + 1,
        );
      };
    },
  };

  subscriptions = {
    syncConversations: (conversationIds: string[]) => {
      this.syncedConversationSnapshots.push([...conversationIds]);
    },
    syncScopes: (scopes: Array<{ eventTypes?: string[]; scopeId: string; scopeType: string }>) => {
      this.syncedScopeSnapshots.push(scopes.map((scope) => ({
        eventTypes: scope.eventTypes ? [...scope.eventTypes] : undefined,
        scopeId: scope.scopeId,
        scopeType: scope.scopeType,
      })));
    },
  };

  emitUserMessage(
    userId: string,
    conversationId: string,
    summary: string,
    sequence: number,
    options: { messageId?: string; messageSeq?: number; senderId?: string } = {},
  ): void {
    const messageId = options.messageId ?? `message-${conversationId}-${sequence}`;
    const payload = {
      body: {
        parts: [{ kind: 'text', text: summary }],
        renderHints: {
          sdkworkChatPcType: 'text',
        },
        summary,
      },
      conversationId,
      messageId,
      messageSeq: options.messageSeq ?? sequence,
      messageType: 'standard',
      occurredAt: '2026-06-08T10:00:00.000Z',
      sender: {
        id: options.senderId ?? 'user-live',
        kind: 'user',
        metadata: {},
      },
      summary,
    };
    const context: ImRealtimeEventContext = {
      ack: () => Promise.resolve(),
      eventId: `event-${conversationId}-${sequence}`,
      eventType: 'message.posted',
      payload,
      rawEvent: {
        eventId: `event-${conversationId}-${sequence}`,
        eventType: 'message.posted',
        payload,
        scopeId: userId,
        scopeType: 'user',
      },
      receivedAt: '2026-06-08T10:00:00.000Z',
      scopeId: userId,
      scopeType: 'user',
      sequence,
    };

    for (const listener of this.scopeEventListeners.get(`user:${userId}`) ?? []) {
      listener(context.rawEvent, context);
    }
  }

  emitMessage(
    conversationId: string,
    summary: string,
    sequence: number,
    options: { messageId?: string; messageSeq?: number } = {},
  ): void {
    const context: ImMessageContext = {
      ack: () => Promise.resolve(),
      conversationId,
      messageId: options.messageId ?? `message-${conversationId}-${sequence}`,
      receivedAt: '2026-06-08T10:00:00.000Z',
      sequence,
    };
    const message: ImDecodedMessage = {
      attachments: [],
      body: {
        parts: [{ kind: 'text', text: summary }],
        summary,
      },
      conversationId,
      messageId: context.messageId,
      messageSeq: options.messageSeq ?? sequence,
      messageType: 'standard',
      renderHints: {
        sdkworkChatPcType: 'text',
      },
      sender: {
        id: 'user-live',
        kind: 'user',
        metadata: {},
      },
      summary,
    };

    for (const listener of this.messageListeners.get(conversationId) ?? []) {
      listener(message, context);
    }
  }

  emitRtcSignal(
    conversationId: string,
    signalType: 'rtc.invite' | 'rtc.accept' | 'rtc.reject' | 'rtc.end',
    sequence: number,
    payload: Record<string, unknown>,
    options: { messageId?: string; senderId?: string } = {},
  ): void {
    const context: ImMessageContext = {
      ack: () => Promise.resolve(),
      conversationId,
      messageId: options.messageId ?? `signal-${conversationId}-${sequence}`,
      payload: {
        body: {
          parts: [
            {
              kind: 'signal',
              payload: JSON.stringify(payload),
              signalType,
            },
          ],
        },
        conversationId,
        messageId: options.messageId ?? `signal-${conversationId}-${sequence}`,
        messageSeq: sequence,
      },
      receivedAt: `2026-06-08T10:00:0${sequence}.000Z`,
      sequence,
      sender: {
        principalId: options.senderId ?? 'user-signal',
      },
    };
    const message: ImDecodedMessage = {
      attachments: [],
      body: {
        parts: [
          {
            kind: 'signal',
            payload: JSON.stringify(payload),
            signalType,
          },
        ],
        summary: signalType,
      },
      conversationId,
      messageId: context.messageId,
      messageSeq: sequence,
      messageType: 'signal',
      sender: {
        id: options.senderId ?? 'user-signal',
        kind: 'user',
        metadata: {},
      },
      summary: signalType,
      type: 'signal',
    };

    for (const listener of this.messageListeners.get(conversationId) ?? []) {
      listener(message, context);
    }
  }

  emitError(error: unknown): void {
    for (const listener of this.errorListeners) {
      listener(error);
    }
  }

  emitState(state: ImLiveConnectionState): void {
    for (const listener of this.stateListeners) {
      listener(state);
    }
  }
}

async function main(): Promise<void> {
  resetPcRealtimeConnectionManagerForTests();
  const terminalHistoryConnections: FakeLiveConnection[] = [];
  const terminalHistoryListCalls: Array<{ afterSeq?: number; conversationId: string; pageSize?: number }> = [];
  const terminalHistoryClient = {
    chat: {
      inbox: {
        async list() {
          return {
            items: [
              {
                tenantId: 'test-tenant',
                conversationId: 'terminal-history-chat-1',
                conversationType: 'group',
                displayName: 'Terminal history chat',
                lastActivityAt: '2026-06-08T10:00:00.000Z',
                lastMessageAt: '2026-06-08T10:00:00.000Z',
                lastMessageId: 'terminal-history-message-1',
                lastMessageSeq: 1,
                lastSenderId: 'user-history',
                lastSummary: 'terminal first page',
                messageCount: 1,
                unreadCount: 0,
              },
            ],
            pageInfo: { hasMore: false, mode: 'cursor' },
          };
        },
      },
    },
    conversations: {
      async listMessages(
        conversationId: string,
        params?: { afterSeq?: number; pageSize?: number },
      ) {
        terminalHistoryListCalls.push({
          afterSeq: params?.afterSeq,
          conversationId,
          pageSize: params?.pageSize,
        });
        if (params?.afterSeq === 0) {
          return {
            highWatermark: 1,
            items: [
              {
                body: {
                  parts: [{ kind: 'text', text: 'terminal first page' }],
                  renderHints: { sdkworkChatPcType: 'text' },
                  summary: 'terminal first page',
                },
                conversationId,
                deliveryMode: 'discrete',
                messageId: 'terminal-history-message-1',
                messageSeq: 1,
                messageType: 'standard',
                occurredAt: '2026-06-08T10:00:00.000Z',
                sender: { id: 'user-history', kind: 'user', metadata: {} },
                summary: 'terminal first page',
              },
            ],
            pageInfo: { hasMore: false, mode: 'cursor', pageSize: 20 },
          };
        }
        return {
          highWatermark: 1,
          items: [],
          pageInfo: { hasMore: false, mode: 'cursor', pageSize: 20 },
        };
      },
    },
    async connect() {
      const connection = new FakeLiveConnection();
      terminalHistoryConnections.push(connection);
      return connection;
    },
  } as unknown as ImSdkClient;
  const terminalHistoryService = createRealtimeChatService(() => terminalHistoryClient);
  await terminalHistoryService.listChatsPage();
  const terminalInitialLoad = terminalHistoryService.getMessages('terminal-history-chat-1');
  const terminalHistoryNotifications: string[] = [];
  const unsubscribeTerminalHistory = terminalHistoryService.subscribeMessages(
    'terminal-history-chat-1',
    (message) => {
      terminalHistoryNotifications.push(message.content);
    },
  );
  const terminalHistoryMessages = await terminalInitialLoad;
  await flushRealtimeMicrotasks();
  assert.deepEqual(
    terminalHistoryListCalls,
    [{ afterSeq: 0, conversationId: 'terminal-history-chat-1', pageSize: 20 }],
    'initial live subscription catch-up must not request another message page after the first history page returns hasMore=false',
  );
  assert.deepEqual(
    terminalHistoryMessages.map((message) => message.content),
    ['terminal first page'],
    'initial terminal history page must still populate the opened conversation',
  );
  assert.deepEqual(
    terminalHistoryNotifications,
    [],
    'terminal initial history load must not be replayed through the live subscription handler',
  );
  assert.equal(
    terminalHistoryConnections[0].messageSubscribeCounts.get('terminal-history-chat-1'),
    1,
    'terminal history regression must still create the realtime wire subscription',
  );
  unsubscribeTerminalHistory();
  resetPcRealtimeConnectionManagerForTests();

  const connections: FakeLiveConnection[] = [];
  const connectCalls: Array<{
    deviceId?: string;
    conversations?: string[];
  }> = [];
  let inboxListCalls = 0;
  let messageHistoryListCalls = 0;
  const readCursorUpdates: Array<{ conversationId: string; readSeq: number }> = [];
  const fakeClient = {
    chat: {
      inbox: {
        async list() {
          inboxListCalls += 1;
          return {
            hasMore: false,
            items: [],
          };
        },
      },
    },
    conversations: {
      async listMessages() {
        messageHistoryListCalls += 1;
        return {
          highWatermark: 999,
          items: [],
          pageInfo: {
            hasMore: false,
            mode: 'cursor',
            nextCursor: null,
          },
        };
      },
      async updateReadCursor(conversationId: string, body: { readSeq: number }) {
        readCursorUpdates.push({ conversationId, readSeq: body.readSeq });
      },
    },
    async connect(options?: { deviceId?: string; subscriptions?: { conversations?: string[] } }) {
      connectCalls.push({
        deviceId: options?.deviceId,
        conversations: options?.subscriptions?.conversations,
      });
      const connection = new FakeLiveConnection();
      connections.push(connection);
      return connection;
    },
  } as unknown as ImSdkClient;

  const service = createRealtimeChatService(() => fakeClient);
  const chatOneMessages: string[] = [];
  const chatTwoMessages: string[] = [];

  const chatListUpdates: string[][] = [];
  const chatListSnapshots: Array<Array<{
    id: string;
    isMarkedUnread?: boolean;
    lastContent?: string;
    unreadCount: number;
  }>> = [];
  const unsubscribeChats = service.subscribeChats((nextChats) => {
    chatListSnapshots.push(nextChats.map((chat) => ({
      id: chat.id,
      isMarkedUnread: chat.isMarkedUnread,
      lastContent: chat.lastMessage?.content,
      unreadCount: chat.unreadCount,
    })));
    chatListUpdates.push(nextChats.map((chat) => `${chat.id}:${chat.lastMessage?.content ?? ''}`));
  });
  await Promise.resolve();
  await Promise.resolve();

  assert.equal(
    connectCalls.length,
    1,
    'ChatService must open a user-scoped realtime subscription for conversation list self-healing',
  );
  assert.deepEqual(
    connections[0].syncedScopeSnapshots.at(-1),
    [
      {
        eventTypes: [
          'message.posted',
          'conversation.updated',
          'conversation.created',
          'conversation.member_joined',
          'conversation.member_role_changed',
          'conversation.member_removed',
          'conversation.member_left',
          'conversation.owner_transferred',
        ],
        scopeId: 'current-user',
        scopeType: 'user',
      },
    ],
    'ChatService conversation list subscription must listen on the current user scope, not only known conversations',
  );
  connections[0].emitUserMessage('current-user', 'new-friend-chat-1', 'hello from a friend', 11, {
    messageId: 'message-new-friend-chat-1',
    messageSeq: 1,
    senderId: 'friend-user-1',
  });
  await flushChatListCoalesceWindow();
  assert.deepEqual(
    chatListUpdates.at(-1),
    ['new-friend-chat-1:hello from a friend'],
    'ChatService must locally create a conversation list item when a friend message arrives for an unknown conversation',
  );
  const selfHealedChatSnapshot = chatListSnapshots.at(-1)?.find((chat) => chat.id === 'new-friend-chat-1');
  assert.ok(
    selfHealedChatSnapshot && (selfHealedChatSnapshot.unreadCount > 0 || selfHealedChatSnapshot.isMarkedUnread),
    'ChatService must mark a locally self-healed unknown conversation as unread so the chat list shows a red dot',
  );
  const selfHealedChat = (await service.getChats()).find((chat) => chat.id === 'new-friend-chat-1');
  assert.ok(
    selfHealedChat && (selfHealedChat.unreadCount > 0 || selfHealedChat.isMarkedUnread),
    'ChatService getChats must preserve the unread marker for locally self-healed unknown conversations',
  );
  assert.deepEqual(
    (await service.getChats()).map((chat) => `${chat.id}:${chat.lastMessage?.content ?? ''}`),
    ['new-friend-chat-1:hello from a friend'],
    'ChatService getChats must preserve locally created unknown conversations even before inbox catches up',
  );
  const inboxCallsBeforeBurst = inboxListCalls;
  connections[0].emitUserMessage('current-user', 'burst-chat-1', 'burst hello 1', 21, {
    messageId: 'message-burst-chat-1',
    messageSeq: 1,
    senderId: 'friend-user-burst',
  });
  connections[0].emitUserMessage('current-user', 'burst-chat-1', 'burst hello 2', 22, {
    messageId: 'message-burst-chat-2',
    messageSeq: 2,
    senderId: 'friend-user-burst',
  });
  connections[0].emitUserMessage('current-user', 'burst-chat-2', 'burst hello 3', 23, {
    messageId: 'message-burst-chat-3',
    messageSeq: 1,
    senderId: 'friend-user-burst',
  });
  await flushChatListCoalesceWindow();
  assert.ok(
    inboxListCalls >= inboxCallsBeforeBurst && inboxListCalls <= inboxCallsBeforeBurst + 1,
    'ChatService must coalesce burst realtime conversation-list refreshes into at most one inbox page request',
  );
  const messageHistoryCallsBeforeActiveRead = messageHistoryListCalls;
  service.setReadFocusContext({
    activeConversationId: 'active-read-chat-1',
    isWindowFocused: true,
  });
  connections[0].emitUserMessage('current-user', 'active-read-chat-1', 'active read hello 1', 31, {
    messageId: 'message-active-read-chat-1',
    messageSeq: 31,
    senderId: 'friend-user-active-read',
  });
  connections[0].emitUserMessage('current-user', 'active-read-chat-1', 'active read hello 2', 32, {
    messageId: 'message-active-read-chat-2',
    messageSeq: 32,
    senderId: 'friend-user-active-read',
  });
  connections[0].emitUserMessage('current-user', 'active-read-chat-1', 'active read hello 3', 33, {
    messageId: 'message-active-read-chat-3',
    messageSeq: 33,
    senderId: 'friend-user-active-read',
  });
  await flushRealtimeMicrotasks();
  assert.equal(
    messageHistoryListCalls,
    messageHistoryCallsBeforeActiveRead,
    'active realtime read cursor sync must use the event sequence and must not issue per-message listMessages lookups',
  );
  assert.deepEqual(
    readCursorUpdates.filter((update) => update.conversationId === 'active-read-chat-1'),
    [{ conversationId: 'active-read-chat-1', readSeq: 33 }],
    'active realtime read cursor sync must coalesce burst messages into one max-sequence read update',
  );
  await flushChatListCoalesceWindow();
  service.setReadFocusContext({
    activeConversationId: undefined,
    isWindowFocused: true,
  });
  const openedAfterUserScopeMessages: string[] = [];
  const unsubscribeOpenedAfterUserScope = service.subscribeMessages('new-friend-chat-1', (message) => {
    openedAfterUserScopeMessages.push(message.content);
  });
  await Promise.resolve();
  connections[0].emitMessage('new-friend-chat-1', 'hello from a friend via conversation replay', 12, {
    messageId: 'message-new-friend-chat-1',
    messageSeq: 1,
  });
  await Promise.resolve();
  assert.deepEqual(
    openedAfterUserScopeMessages,
    [],
    'ChatService must not replay a user-scope self-healed message as a new opened-conversation notification',
  );
  unsubscribeOpenedAfterUserScope();
  const activeDualScopeMessages: string[] = [];
  const unsubscribeActiveDualScope = service.subscribeMessages('active-dual-scope-chat-1', (message) => {
    activeDualScopeMessages.push(message.content);
  });
  await Promise.resolve();
  connections[0].emitUserMessage('current-user', 'active-dual-scope-chat-1', 'active dual scope hello', 12, {
    messageId: 'message-active-dual-scope-chat-1',
    messageSeq: 1,
    senderId: 'friend-user-2',
  });
  connections[0].emitMessage('active-dual-scope-chat-1', 'active dual scope hello via conversation scope', 13, {
    messageId: 'message-active-dual-scope-chat-1',
    messageSeq: 1,
  });
  await Promise.resolve();
  assert.deepEqual(
    activeDualScopeMessages,
    ['active dual scope hello'],
    'ChatService must notify an opened conversation once when user-scope and conversation-scope realtime deliver the same message',
  );
  unsubscribeActiveDualScope();
  unsubscribeChats();
  connectCalls.length = 0;
  connections.length = 0;

  const chatListReconnectCallbacks: Array<() => void> = [];
  const chatListOriginalSetTimeout = globalThis.setTimeout;
  const chatListOriginalClearTimeout = globalThis.clearTimeout;
  Object.defineProperty(globalThis, 'setTimeout', {
    configurable: true,
    value: ((callback: () => void) => {
      chatListReconnectCallbacks.push(callback);
      return { sdkworkTimer: chatListReconnectCallbacks.length };
    }) as typeof setTimeout,
  });
  Object.defineProperty(globalThis, 'clearTimeout', {
    configurable: true,
    value: (() => undefined) as typeof clearTimeout,
  });
  try {
    const chatListOnlyConnections: FakeLiveConnection[] = [];
    const chatListOnlyClient = {
      chat: {
        inbox: {
          async list() {
            return { hasMore: false, items: [] };
          },
        },
      },
      async connect() {
        const connection = new FakeLiveConnection();
        chatListOnlyConnections.push(connection);
        return connection;
      },
    } as unknown as ImSdkClient;
    const chatListOnlyService = createRealtimeChatService(() => chatListOnlyClient);
    const unsubscribeChatListOnly = chatListOnlyService.subscribeChats(() => undefined);
    await Promise.resolve();
    await Promise.resolve();
    const callbacksBeforeDrop = chatListReconnectCallbacks.length;
    chatListOnlyConnections[0].emitState({ status: 'closed', reason: 'chat list socket dropped' });
    assert.equal(
      chatListReconnectCallbacks.length,
      callbacksBeforeDrop + 1,
      'ChatService must schedule reconnect when only the conversation-list realtime subscription is active',
    );
    chatListReconnectCallbacks.at(-1)?.();
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(
      chatListOnlyConnections.length,
      2,
      'ChatService must restart user-scope conversation-list realtime after a dropped connection',
    );
    assert.deepEqual(
      chatListOnlyConnections[1].syncedScopeSnapshots.at(-1),
      [
        {
          eventTypes: [
            'message.posted',
            'conversation.updated',
            'conversation.created',
            'conversation.member_joined',
            'conversation.member_role_changed',
            'conversation.member_removed',
            'conversation.member_left',
            'conversation.owner_transferred',
          ],
          scopeId: 'current-user',
          scopeType: 'user',
        },
      ],
      'ChatService must resubscribe the user-scope conversation list after reconnect',
    );
    unsubscribeChatListOnly();
  } finally {
    Object.defineProperty(globalThis, 'setTimeout', {
      configurable: true,
      value: chatListOriginalSetTimeout,
    });
    Object.defineProperty(globalThis, 'clearTimeout', {
      configurable: true,
      value: chatListOriginalClearTimeout,
    });
  }

  const unsubscribeChatOne = service.subscribeMessages('chat-1', (message) => {
    chatOneMessages.push(message.content);
  });
  const unsubscribeChatTwo = service.subscribeMessages('chat-2', (message) => {
    chatTwoMessages.push(message.content);
  });
  await Promise.resolve();

  assert.equal(
    connectCalls.length,
    1,
    'ChatService must reuse the chat-list realtime connection for all live conversation subscriptions',
  );
  assert.deepEqual(
    connectCalls[0].conversations,
    [],
    'ChatService must not bind the device connection to a single conversation at websocket creation time',
  );
  assert.deepEqual(
    connections[0].syncedConversationSnapshots.at(-1),
    ['chat-1', 'chat-2'],
    'ChatService must synchronize active conversation subscriptions over the existing IM realtime connection',
  );

  connections[0].emitMessage('chat-1', 'first live message', 1);
  connections[0].emitMessage('chat-1', 'first live message redelivered', 1);
  connections[0].emitMessage('chat-1', 'second live message high realtime seq', 30, {
    messageId: 'message-chat-1-message-seq-2',
    messageSeq: 2,
  });
  connections[0].emitMessage('chat-2', 'second live message', 2);
  assert.deepEqual(
    chatOneMessages,
    ['first live message', 'second live message high realtime seq'],
    'ChatService must not notify live message handlers twice for the same message id',
  );
  assert.deepEqual(chatTwoMessages, ['second live message']);

  const callUpdates: Array<{ content: string; id: string; senderId: string; type: string }> = [];
  const unsubscribeCallChat = service.subscribeMessages('call-chat-1', (message) => {
    callUpdates.push({
      content: message.content,
      id: message.id,
      senderId: message.senderId,
      type: message.type,
    });
  });
  await Promise.resolve();
  connections[0].emitRtcSignal('call-chat-1', 'rtc.invite', 1, {
    conversationId: 'call-chat-1',
    initiatorId: 'caller-user',
    receiverId: 'callee-user',
    rtcMode: 'video',
    rtcSessionId: 'rtc-session-1',
    state: 'ringing',
  }, { messageId: 'rtc-invite-message-1', senderId: 'caller-user' });
  connections[0].emitRtcSignal('call-chat-1', 'rtc.accept', 2, {
    actorId: 'callee-user',
    conversationId: 'call-chat-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-session-1',
    state: 'connected',
  }, { messageId: 'rtc-accept-message-2', senderId: 'callee-user' });
  connections[0].emitRtcSignal('call-chat-1', 'rtc.end', 3, {
    actorId: 'callee-user',
    conversationId: 'call-chat-1',
    rtcMode: 'video',
    rtcSessionId: 'rtc-session-1',
    state: 'ended',
  }, { messageId: 'rtc-end-message-3', senderId: 'callee-user' });

  assert.equal(
    new Set(callUpdates.map((message) => message.id)).size,
    1,
    'ChatService must collapse multiple RTC signaling messages for one rtcSessionId into a single call message id',
  );
  assert.deepEqual(
    callUpdates.map((message) => message.id),
    ['call:rtc-session-1', 'call:rtc-session-1', 'call:rtc-session-1'],
    'ChatService must emit updates for the same call message instead of separate signaling rows',
  );
  assert.deepEqual(
    callUpdates.map((message) => message.type),
    ['video_call', 'video_call', 'video_call'],
    'ChatService must render RTC signaling as call messages, not generic system messages',
  );
  assert.equal(
    callUpdates.at(-1)?.senderId,
    'caller-user',
    'The aggregated call message must stay anchored to the initiator so sender/receiver display remains stable',
  );
  assert.match(
    callUpdates[1]?.content ?? '',
    /caller-user.*callee-user.*(接通|accepted)/u,
    'The updated call message must describe who initiated the call and who accepted it',
  );
  assert.match(
    callUpdates.at(-1)?.content ?? '',
    /callee-user\s+\u5df2\u6302\u65ad/u,
    'The ended call message must describe which participant hung up the call',
  );
  unsubscribeCallChat();

  const nestedCallUpdates: Array<{ content: string; id: string; senderId: string; type: string }> = [];
  const unsubscribeNestedCallChat = service.subscribeMessages('call-chat-nested-1', (message) => {
    nestedCallUpdates.push({
      content: message.content,
      id: message.id,
      senderId: message.senderId,
      type: message.type,
    });
  });
  await Promise.resolve();
  connections[0].emitRtcSignal('call-chat-nested-1', 'rtc.invite', 4, {
    signalPayload: JSON.stringify({
      conversationId: 'call-chat-nested-1',
      initiatorId: 'nested-caller',
      receiverId: 'nested-callee',
      rtcMode: 'voice',
      rtcSessionId: 'rtc-session-nested-1',
      state: 'started',
    }),
  }, { messageId: 'rtc-nested-invite-message-4', senderId: 'nested-caller' });

  assert.equal(
    nestedCallUpdates.length,
    1,
    'ChatService must render RTC call messages when signal fields are nested in signalPayload',
  );
  assert.equal(nestedCallUpdates[0]?.id, 'call:rtc-session-nested-1');
  assert.equal(nestedCallUpdates[0]?.senderId, 'nested-caller');
  assert.equal(nestedCallUpdates[0]?.type, 'video_call');
  unsubscribeNestedCallChat();

  unsubscribeChatOne();
  assert.deepEqual(
    connections[0].disconnectCalls,
    [],
    'ChatService must keep the shared realtime connection open while at least one conversation is still subscribed',
  );
  assert.deepEqual(
    connections[0].syncedConversationSnapshots.at(-1),
    ['chat-2'],
    'ChatService must remove unsubscribed conversations from the server-side subscription snapshot',
  );

  unsubscribeChatTwo();
  assert.deepEqual(
    connections[0].syncedConversationSnapshots.at(-1),
    [],
    'ChatService must clear the server-side subscription snapshot before closing the shared realtime connection',
  );
  assert.deepEqual(
    connections[0].disconnectCalls,
    [{ code: 1000, reason: 'conversation subscription closed' }],
    'ChatService must disconnect the shared realtime connection after the final live subscription closes',
  );

  const originalSetTimeout = globalThis.setTimeout;
  const originalClearTimeout = globalThis.clearTimeout;
  const reconnectDelays: number[] = [];
  const reconnectCallbacks: Array<() => void> = [];
  Object.defineProperty(globalThis, 'setTimeout', {
    configurable: true,
    value: ((callback: () => void, delay?: number) => {
      reconnectDelays.push(typeof delay === 'number' ? delay : 0);
      reconnectCallbacks.push(callback);
      return { sdkworkTimer: reconnectDelays.length };
    }) as typeof setTimeout,
  });
  Object.defineProperty(globalThis, 'clearTimeout', {
    configurable: true,
    value: (() => undefined) as typeof clearTimeout,
  });

  try {
    const reconnectConnections: FakeLiveConnection[] = [];
    let reconnectAttempts = 0;
    const reconnectClient = {
      async connect() {
        reconnectAttempts += 1;
        if (reconnectAttempts <= 2) {
          throw new Error(`connect failed ${reconnectAttempts}`);
        }
        const connection = new FakeLiveConnection();
        reconnectConnections.push(connection);
        return connection;
      },
    } as unknown as ImSdkClient;
    const reconnectService = createRealtimeChatService(() => reconnectClient);
    const unsubscribeRetryChat = reconnectService.subscribeMessages('retry-chat-1', () => undefined);

    await flushRealtimeMicrotasks();
    assert.equal(reconnectAttempts, 1);
    assert.equal(reconnectDelays.length, 1);
    assert.ok(
      reconnectDelays[0] >= 800 && reconnectDelays[0] <= 1200,
      `first reconnect delay must use jittered base backoff; got ${reconnectDelays[0]}`,
    );

    reconnectCallbacks[0]();
    await flushRealtimeMicrotasks();
    assert.equal(reconnectAttempts, 2);
    assert.equal(reconnectDelays.length, 2);
    assert.ok(
      reconnectDelays[1] >= 1600 && reconnectDelays[1] <= 2400,
      `second reconnect delay must use jittered exponential backoff; got ${reconnectDelays[1]}`,
    );

    reconnectCallbacks[1]();
    await flushRealtimeMicrotasks();
    assert.equal(reconnectAttempts, 3);
    assert.equal(reconnectConnections.length, 1);

    reconnectConnections[0].emitState({ status: 'closed', reason: 'network dropped after recovery' });
    assert.equal(reconnectDelays.length, 3);
    assert.ok(
      reconnectDelays[2] >= 800 && reconnectDelays[2] <= 1200,
      `reconnect delay must reset after a successful live session; got ${reconnectDelays[2]}`,
    );

    unsubscribeRetryChat();
  } finally {
    Object.defineProperty(globalThis, 'setTimeout', {
      configurable: true,
      value: originalSetTimeout,
    });
    Object.defineProperty(globalThis, 'clearTimeout', {
      configurable: true,
      value: originalClearTimeout,
    });
  }

  const lifecycleErrorTimeoutCallbacks: Array<() => void> = [];
  Object.defineProperty(globalThis, 'setTimeout', {
    configurable: true,
    value: ((callback: () => void) => {
      lifecycleErrorTimeoutCallbacks.push(callback);
      return { sdkworkTimer: lifecycleErrorTimeoutCallbacks.length };
    }) as typeof setTimeout,
  });
  Object.defineProperty(globalThis, 'clearTimeout', {
    configurable: true,
    value: (() => undefined) as typeof clearTimeout,
  });

  try {
    const lifecycleErrorConnections: FakeLiveConnection[] = [];
    const lifecycleErrorClient = {
      async connect() {
        const connection = new FakeLiveConnection();
        lifecycleErrorConnections.push(connection);
        return connection;
      },
    } as unknown as ImSdkClient;
    const lifecycleErrorService = createRealtimeChatService(() => lifecycleErrorClient);
    const unsubscribeLifecycleErrorChat = lifecycleErrorService.subscribeMessages('lifecycle-error-chat-1', () => undefined);
    await Promise.resolve();
    await Promise.resolve();

    lifecycleErrorConnections[0].emitError({
      code: 'websocket_upstream_unavailable',
      message: 'upstream websocket connection dropped',
      type: 'error',
    });

    assert.equal(
      lifecycleErrorTimeoutCallbacks.length,
      1,
      'ChatService must schedule realtime reconnect when the SDK lifecycle emits a transport error',
    );
    lifecycleErrorTimeoutCallbacks[0]();
    await flushRealtimeMicrotasks();
    assert.equal(
      lifecycleErrorConnections.length,
      2,
      'ChatService must restart the shared realtime connection after lifecycle errors',
    );
    unsubscribeLifecycleErrorChat();

    const nonFatalLifecycleErrorService = createRealtimeChatService(() => lifecycleErrorClient);
    const unsubscribeNonFatalLifecycleErrorChat = nonFatalLifecycleErrorService.subscribeMessages(
      'non-fatal-lifecycle-error-chat-1',
      () => undefined,
    );
    await Promise.resolve();
    await Promise.resolve();
    const nonFatalConnectionCount = lifecycleErrorConnections.length;
    lifecycleErrorConnections.at(-1)?.emitError({
      code: 'subscription_forbidden',
      message: 'conversation access denied',
      type: 'error',
    });
    assert.equal(
      lifecycleErrorTimeoutCallbacks.length,
      1,
      'ChatService must not reconnect the shared realtime connection for non-fatal subscription errors',
    );
    assert.equal(lifecycleErrorConnections.length, nonFatalConnectionCount);
    unsubscribeNonFatalLifecycleErrorChat();
  } finally {
    Object.defineProperty(globalThis, 'setTimeout', {
      configurable: true,
      value: originalSetTimeout,
    });
    Object.defineProperty(globalThis, 'clearTimeout', {
      configurable: true,
      value: originalClearTimeout,
    });
  }

  const messageHistoryClient = {
    conversations: {
      async getMessageInteractionSummary() {
        return { reactionCounts: [] };
      },
      async listMessages() {
        return {
          hasMore: false,
          items: [
            {
              body: {
                parts: [
                  {
                    kind: 'signal',
                    payload: JSON.stringify({
                      conversationId: 'message-history-call-chat-1',
                      initiatorId: 'message-history-caller',
                      receiverId: 'message-history-callee',
                      rtcMode: 'voice',
                      rtcSessionId: 'message-history-rtc-session-1',
                      state: 'started',
                    }),
                    signalType: 'rtc.invite',
                  },
                ],
                summary: 'rtc.invite',
              },
              conversationId: 'message-history-call-chat-1',
              deliveryMode: 'discrete',
              messageId: 'message-history-rtc-invite-message',
              messageSeq: 1,
              messageType: 'signal',
              occurredAt: '2026-06-08T10:00:01.000Z',
              sender: { id: 'message-history-caller', kind: 'user', metadata: {} },
              summary: 'rtc.invite',
            },
            {
              body: {
                parts: [
                  {
                    kind: 'signal',
                    payload: JSON.stringify({
                      actorId: 'message-history-callee',
                      conversationId: 'message-history-call-chat-1',
                      rtcMode: 'voice',
                      rtcSessionId: 'message-history-rtc-session-1',
                      state: 'rejected',
                    }),
                    signalType: 'rtc.reject',
                  },
                ],
                summary: 'rtc.reject',
              },
              conversationId: 'message-history-call-chat-1',
              deliveryMode: 'discrete',
              messageId: 'message-history-rtc-reject-message',
              messageSeq: 2,
              messageType: 'signal',
              occurredAt: '2026-06-08T10:00:02.000Z',
              sender: { id: 'message-history-callee', kind: 'user', metadata: {} },
              summary: 'rtc.reject',
            },
          ],
        };
      },
    },
  } as unknown as ImSdkClient;
  const messageHistoryService = createRealtimeChatService(() => messageHistoryClient);
  const messageHistoryMessages = await messageHistoryService.getMessages('message-history-call-chat-1');
  assert.equal(
    messageHistoryMessages.length,
    1,
    'ChatService must collapse message history RTC signaling entries into one call message after reload/offline catch-up',
  );
  assert.equal(messageHistoryMessages[0]?.id, 'call:message-history-rtc-session-1');
  assert.equal(messageHistoryMessages[0]?.type, 'video_call');
  assert.equal(messageHistoryMessages[0]?.senderId, 'message-history-caller');
  assert.match(
    messageHistoryMessages[0]?.content ?? '',
    /message-history-caller.*message-history-callee.*(拒绝|rejected)/u,
    'The collapsed message history call message must keep the latest rejected state',
  );

  const catchupTimeoutCallbacks: Array<() => void> = [];
  Object.defineProperty(globalThis, 'setTimeout', {
    configurable: true,
    value: ((callback: () => void) => {
      catchupTimeoutCallbacks.push(callback);
      return { sdkworkTimer: catchupTimeoutCallbacks.length };
    }) as typeof setTimeout,
  });
  Object.defineProperty(globalThis, 'clearTimeout', {
    configurable: true,
    value: (() => undefined) as typeof clearTimeout,
  });

  try {
    const catchupConnections: FakeLiveConnection[] = [];
    const catchupListCalls: Array<{ afterSeq?: number; conversationId: string; pageSize?: number }> = [];
    let catchupListAfterSeqOneCall = 0;
    const catchupClient = {
      async connect() {
        const connection = new FakeLiveConnection();
        catchupConnections.push(connection);
        return connection;
      },
      conversations: {
        async getMessageInteractionSummary() {
          return { reactionCounts: [] };
        },
        async listMessages(
          conversationId: string,
          params?: { afterSeq?: number; pageSize?: number },
        ) {
          catchupListCalls.push({
            afterSeq: params?.afterSeq,
            conversationId,
            pageSize: params?.pageSize,
          });
          if (params?.afterSeq === 0) {
            return {
              items: [
                {
                  conversationId,
                  deliveryMode: 'discrete',
                  messageId: 'catchup-message-1',
                  messageSeq: 1,
                  messageType: 'standard',
                  occurredAt: '2026-06-08T10:00:00.000Z',
                  sender: { id: 'user-history', kind: 'user', metadata: {} },
                  summary: 'already loaded',
                  body: {
                    parts: [{ kind: 'text', text: 'already loaded' }],
                    renderHints: { sdkworkChatPcType: 'text' },
                    summary: 'already loaded',
                  },
                },
              ],
              pageInfo: { hasMore: false, mode: 'cursor' },
            };
          }
          if (params?.afterSeq === 1) {
            catchupListAfterSeqOneCall += 1;
            return {
              items: catchupListAfterSeqOneCall === 1
                ? []
                : [
                    {
                      conversationId,
                      deliveryMode: 'discrete',
                      messageId: 'catchup-message-2',
                      messageSeq: 2,
                      messageType: 'standard',
                      occurredAt: '2026-06-08T10:00:05.000Z',
                      sender: { id: 'user-offline', kind: 'user', metadata: {} },
                      summary: 'missed while offline',
                      body: {
                        parts: [{ kind: 'text', text: 'missed while offline' }],
                        renderHints: { sdkworkChatPcType: 'text' },
                        summary: 'missed while offline',
                      },
                    },
                  ],
              pageInfo: {
                hasMore: catchupListAfterSeqOneCall > 1,
                mode: 'cursor',
                nextCursor: catchupListAfterSeqOneCall > 1 ? '2' : undefined,
              },
            };
          }
          if (params?.afterSeq === 2) {
            return {
              items: [
                {
                  conversationId,
                  deliveryMode: 'discrete',
                  messageId: 'catchup-message-3',
                  messageSeq: 3,
                  messageType: 'standard',
                  occurredAt: '2026-06-08T10:00:06.000Z',
                  sender: { id: 'user-offline', kind: 'user', metadata: {} },
                  summary: 'second missed while offline',
                  body: {
                    parts: [{ kind: 'text', text: 'second missed while offline' }],
                    renderHints: { sdkworkChatPcType: 'text' },
                    summary: 'second missed while offline',
                  },
                },
              ],
              pageInfo: { hasMore: true, mode: 'cursor', nextCursor: '3' },
            };
          }
          if (params?.afterSeq === 3) {
            return {
              items: [
                {
                  conversationId,
                  deliveryMode: 'discrete',
                  messageId: 'catchup-message-4',
                  messageSeq: 4,
                  messageType: 'standard',
                  occurredAt: '2026-06-08T10:00:07.000Z',
                  sender: { id: 'user-offline', kind: 'user', metadata: {} },
                  summary: 'third missed while offline',
                  body: {
                    parts: [{ kind: 'text', text: 'third missed while offline' }],
                    renderHints: { sdkworkChatPcType: 'text' },
                    summary: 'third missed while offline',
                  },
                },
              ],
              pageInfo: { hasMore: false, mode: 'cursor' },
            };
          }
          return { items: [], pageInfo: { hasMore: false, mode: 'cursor' } };
        },
      },
    } as unknown as ImSdkClient;

    const catchupService = createRealtimeChatService(() => catchupClient);
    await catchupService.getMessages('catchup-chat-1');
    const catchupMessages: string[] = [];
    const unsubscribeCatchupChat = catchupService.subscribeMessages('catchup-chat-1', (message) => {
      catchupMessages.push(message.content);
    });
    await Promise.resolve();
    await Promise.resolve();

    assert.deepEqual(
      catchupMessages,
      [],
      'initial live subscription catch-up must not replay already loaded messages',
    );
    catchupConnections[0].emitMessage('catchup-chat-1', 'live before disconnect', 30, {
      messageId: 'catchup-message-2',
      messageSeq: 2,
    });
    catchupConnections[0].emitState({ status: 'closed', reason: 'network dropped' });
    await Promise.resolve();
    catchupTimeoutCallbacks[0]();
    await Promise.resolve();
    await Promise.resolve();
    catchupConnections[1].emitState({ status: 'open' });
    await Promise.resolve();
    await Promise.resolve();
    await new Promise((resolve) => originalSetTimeout(resolve, 0));

    assert.deepEqual(
      catchupMessages,
      ['live before disconnect', 'second missed while offline', 'third missed while offline'],
      'ChatService must pull every missed message history page after a recovered realtime connection',
    );
    assert.deepEqual(
      catchupListCalls.map((call) => call.afterSeq),
      [0, 2, 3],
      'ChatService must not probe an empty page after initial hasMore=false and must reconnect catch-up from messageSeq checkpoints',
    );
    unsubscribeCatchupChat();
  } finally {
    Object.defineProperty(globalThis, 'setTimeout', {
      configurable: true,
      value: originalSetTimeout,
    });
    Object.defineProperty(globalThis, 'clearTimeout', {
      configurable: true,
      value: originalClearTimeout,
    });
  }

  const recoveryConnections: FakeLiveConnection[] = [];
  const recoveryListCalls: Array<{ afterSeq?: number; conversationId: string }> = [];
  const recoveryClient = {
    async connect() {
      const connection = new FakeLiveConnection();
      recoveryConnections.push(connection);
      return connection;
    },
    conversations: {
      async getMessageInteractionSummary() {
        return { reactionCounts: [] };
      },
      async listMessages(conversationId: string, params?: { afterSeq?: number }) {
        recoveryListCalls.push({
          afterSeq: params?.afterSeq,
          conversationId,
        });
        if (params?.afterSeq === 1) {
          return {
            hasMore: false,
            items: [
              {
                conversationId,
                deliveryMode: 'discrete',
                messageId: 'recovery-message-2',
                messageSeq: 2,
                messageType: 'standard',
                occurredAt: '2026-06-08T10:00:10.000Z',
                sender: { id: 'user-recovery', kind: 'user', metadata: {} },
                summary: 'missed during network recovery',
                body: {
                  parts: [{ kind: 'text', text: 'missed during network recovery' }],
                  renderHints: { sdkworkChatPcType: 'text' },
                  summary: 'missed during network recovery',
                },
              },
            ],
          };
        }
        return { hasMore: false, items: [] };
      },
    },
  } as unknown as ImSdkClient;
  const recoveryService = createRealtimeChatService(() => recoveryClient);
  const recoveryMessages: string[] = [];
  const unsubscribeRecoveryChat = recoveryService.subscribeMessages('recovery-chat-1', (message) => {
    recoveryMessages.push(message.content);
  });
  await Promise.resolve();
  await Promise.resolve();
  recoveryConnections[0].emitMessage('recovery-chat-1', 'live before network recovery', 1, {
    messageId: 'recovery-message-1',
    messageSeq: 1,
  });

  recoveryService.recoverRealtimeConnection('network restored');
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => originalSetTimeout(resolve, 0));

  assert.equal(
    recoveryConnections.length,
    2,
    'ChatService must immediately restart the shared realtime session when the host reports network recovery',
  );
  assert.deepEqual(
    recoveryConnections[0].disconnectCalls.at(-1),
    { code: 1000, reason: 'network restored' },
    'ChatService must close the stale websocket with the recovery reason before reconnecting',
  );
  assert.deepEqual(
    recoveryConnections[1].syncedConversationSnapshots.at(-1),
    ['recovery-chat-1'],
    'ChatService must resubscribe active conversations after network recovery',
  );
  assert.deepEqual(
    recoveryMessages,
    ['live before network recovery', 'missed during network recovery'],
    'ChatService must catch up missed messages immediately after a recovery-triggered reconnect',
  );
  assert.deepEqual(
    recoveryListCalls,
    [{ conversationId: 'recovery-chat-1', afterSeq: 1 }],
    'ChatService recovery catch-up must resume from the last delivered message sequence',
  );
  unsubscribeRecoveryChat();

  const pendingConnectionResolvers: Array<(connection: FakeLiveConnection) => void> = [];
  const pendingClient = {
    connect() {
      return new Promise<ImLiveConnection>((resolve) => {
        pendingConnectionResolvers.push(resolve);
      });
    },
  } as unknown as ImSdkClient;
  const pendingRecoveryService = createRealtimeChatService(() => pendingClient);
  const unsubscribePendingRecoveryChat = pendingRecoveryService.subscribeMessages(
    'pending-recovery-chat-1',
    () => undefined,
  );
  await Promise.resolve();
  assert.equal(
    pendingConnectionResolvers.length,
    1,
    'ChatService must start the first realtime connection for a live subscription',
  );

  pendingRecoveryService.recoverRealtimeConnection('network restored');
  await Promise.resolve();
  assert.equal(
    pendingConnectionResolvers.length,
    2,
    'ChatService must start a fresh realtime connection when recovery fires during websocket handshake',
  );

  const recoveredConnection = new FakeLiveConnection();
  pendingConnectionResolvers[1](recoveredConnection);
  await Promise.resolve();
  await Promise.resolve();
  assert.deepEqual(
    recoveredConnection.syncedConversationSnapshots.at(-1),
    ['pending-recovery-chat-1'],
    'ChatService must bind active subscriptions to the newest recovered websocket',
  );

  const staleConnection = new FakeLiveConnection();
  pendingConnectionResolvers[0](staleConnection);
  await Promise.resolve();
  await Promise.resolve();
  assert.deepEqual(
    staleConnection.disconnectCalls,
    [{ code: 1000, reason: 'stale PC live connection attempt' }],
    'ChatService must close a stale websocket handshake that resolves after a recovered connection',
  );
  assert.deepEqual(
    staleConnection.syncedConversationSnapshots,
    [],
    'ChatService must not subscribe conversations on stale recovered-over websocket handshakes',
  );
  unsubscribePendingRecoveryChat();

  const staleRejectTimeoutCallbacks: Array<() => void> = [];
  Object.defineProperty(globalThis, 'setTimeout', {
    configurable: true,
    value: ((callback: () => void) => {
      staleRejectTimeoutCallbacks.push(callback);
      return { sdkworkTimer: staleRejectTimeoutCallbacks.length };
    }) as typeof setTimeout,
  });
  Object.defineProperty(globalThis, 'clearTimeout', {
    configurable: true,
    value: (() => undefined) as typeof clearTimeout,
  });

  try {
    const staleRejectors: Array<(error: unknown) => void> = [];
    const staleRejectResolvers: Array<(connection: FakeLiveConnection) => void> = [];
    const staleRejectClient = {
      connect() {
        return new Promise<ImLiveConnection>((resolve, reject) => {
          staleRejectResolvers.push(resolve);
          staleRejectors.push(reject);
        });
      },
    } as unknown as ImSdkClient;
    const staleRejectService = createRealtimeChatService(() => staleRejectClient);
    const unsubscribeStaleRejectChat = staleRejectService.subscribeMessages(
      'stale-reject-chat-1',
      () => undefined,
    );
    await Promise.resolve();
    assert.equal(staleRejectResolvers.length, 1);

    staleRejectService.recoverRealtimeConnection('network restored');
    await Promise.resolve();
    assert.equal(staleRejectResolvers.length, 2);

    const staleRejectRecoveredConnection = new FakeLiveConnection();
    staleRejectResolvers[1](staleRejectRecoveredConnection);
    await Promise.resolve();
    await Promise.resolve();
    staleRejectors[0](new Error('stale connect failed'));
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(
      staleRejectTimeoutCallbacks.length,
      0,
      'ChatService must not schedule reconnect when a stale websocket handshake fails after recovery',
    );
    assert.equal(
      staleRejectResolvers.length,
      2,
      'ChatService must ignore stale websocket handshake failures after a newer recovered connection is active',
    );
    assert.deepEqual(
      staleRejectRecoveredConnection.disconnectCalls,
      [],
      'ChatService must not disconnect a healthy recovered websocket because an older handshake failed later',
    );
    unsubscribeStaleRejectChat();
  } finally {
    Object.defineProperty(globalThis, 'setTimeout', {
      configurable: true,
      value: originalSetTimeout,
    });
    Object.defineProperty(globalThis, 'clearTimeout', {
      configurable: true,
      value: originalClearTimeout,
    });
  }

  const browserRecoveryEvents = new Map<string, Set<() => void>>();
  const browserRecoveryConnections: FakeLiveConnection[] = [];
  const originalWindow = Object.prototype.hasOwnProperty.call(globalThis, 'window')
    ? globalThis.window
    : undefined;
  const originalDocument = Object.prototype.hasOwnProperty.call(globalThis, 'document')
    ? globalThis.document
    : undefined;
  Object.defineProperty(globalThis, 'window', {
    configurable: true,
    value: {
      addEventListener: (type: string, handler: () => void) => {
        const handlers = browserRecoveryEvents.get(type) ?? new Set<() => void>();
        handlers.add(handler);
        browserRecoveryEvents.set(type, handlers);
      },
      removeEventListener: (type: string, handler: () => void) => {
        browserRecoveryEvents.get(type)?.delete(handler);
      },
    },
  });
  Object.defineProperty(globalThis, 'document', {
    configurable: true,
    value: {
      addEventListener: (type: string, handler: () => void) => {
        const handlers = browserRecoveryEvents.get(type) ?? new Set<() => void>();
        handlers.add(handler);
        browserRecoveryEvents.set(type, handlers);
      },
      removeEventListener: (type: string, handler: () => void) => {
        browserRecoveryEvents.get(type)?.delete(handler);
      },
      visibilityState: 'hidden',
    },
  });

  try {
    const browserRecoveryClient = {
      async connect() {
        const connection = new FakeLiveConnection();
        browserRecoveryConnections.push(connection);
        return connection;
      },
    } as unknown as ImSdkClient;
    const browserRecoveryService = createRealtimeChatService(() => browserRecoveryClient);
    const unsubscribeBrowserRecoveryChat = browserRecoveryService.subscribeMessages(
      'browser-recovery-chat-1',
      () => undefined,
    );
    await Promise.resolve();
    await Promise.resolve();

    browserRecoveryEvents.get('online')?.forEach((handler) => handler());
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(
      browserRecoveryConnections.length,
      2,
      'ChatService must recover the shared realtime session when the browser online event fires',
    );
    assert.deepEqual(
      browserRecoveryConnections[0].disconnectCalls.at(-1),
      { code: 1000, reason: 'browser online' },
      'ChatService must tag browser online recovery disconnects with a diagnosable reason',
    );

    Object.defineProperty(globalThis.document, 'visibilityState', {
      configurable: true,
      value: 'visible',
    });
    browserRecoveryEvents.get('visibilitychange')?.forEach((handler) => handler());
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(
      browserRecoveryConnections.length,
      3,
      'ChatService must recover the shared realtime session when a browser page becomes visible again',
    );
    assert.deepEqual(
      browserRecoveryConnections[1].disconnectCalls.at(-1),
      { code: 1000, reason: 'browser visible' },
      'ChatService must tag browser visible recovery disconnects with a diagnosable reason',
    );

    unsubscribeBrowserRecoveryChat();
    browserRecoveryEvents.get('online')?.forEach((handler) => handler());
    await Promise.resolve();
    await Promise.resolve();
    assert.equal(
      browserRecoveryConnections.length,
      3,
      'ChatService must ignore browser recovery signals when there are no active live subscriptions',
    );
  } finally {
    if (originalWindow === undefined) {
      Reflect.deleteProperty(globalThis, 'window');
    } else {
      Object.defineProperty(globalThis, 'window', {
        configurable: true,
        value: originalWindow,
      });
    }
    if (originalDocument === undefined) {
      Reflect.deleteProperty(globalThis, 'document');
    } else {
      Object.defineProperty(globalThis, 'document', {
        configurable: true,
        value: originalDocument,
      });
    }
  }

  resetPcRealtimeConnectionManagerForTests();
  const leaseOnlyConnections: FakeLiveConnection[] = [];
  const leaseOnlyClient = {
    async connect() {
      const connection = new FakeLiveConnection();
      leaseOnlyConnections.push(connection);
      return connection;
    },
  } as unknown as ImSdkClient;
  configurePcRealtimeConnectionManager({
    getClient: () => leaseOnlyClient,
    getDeviceId: () => 'lease-device-1',
    getSession: () => authenticatedSession,
  });
  const releaseLeaseOnlyConversation = acquirePcLiveConnectionLease('lease-only-watch', {
    conversationIds: ['lease-only-conversation-1'],
  });
  await flushRealtimeMicrotasks();
  assert.equal(
    leaseOnlyConnections[0].messageSubscribeCounts.get('lease-only-conversation-1'),
    1,
    'PC realtime manager must create one wire listener for a lease-only conversation',
  );
  assert.deepEqual(
    leaseOnlyConnections[0].syncedConversationSnapshots.at(-1),
    ['lease-only-conversation-1'],
    'PC realtime manager must include lease-only conversations in subscription sync',
  );
  const unsubscribeLeaseOnlyScope = subscribePcRealtimeScope(
    {
      eventTypes: ['conversation.created'],
      scopeId: 'current-user',
      scopeType: 'user',
    },
    () => undefined,
  );
  await flushRealtimeMicrotasks();
  assert.equal(
    leaseOnlyConnections[0].messageSubscribeCounts.get('lease-only-conversation-1'),
    1,
    'PC realtime manager must not resubscribe lease-only conversations during later scope syncs',
  );
  assert.equal(
    leaseOnlyConnections[0].messageUnsubscribeCounts.get('lease-only-conversation-1') ?? 0,
    0,
    'PC realtime manager must not drop lease-only conversation listeners while the lease is active',
  );
  unsubscribeLeaseOnlyScope();
  releaseLeaseOnlyConversation();
  resetPcRealtimeConnectionManagerForTests();

  console.log('sdkwork-im-pc live subscription session contract passed');
}

void main();
