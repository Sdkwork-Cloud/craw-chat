import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { SDKWORK_MAX_PAGE_SIZE } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appSdkResponseHelpers.ts';
import type { SdkworkChatSession } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts';
import {
  createSdkworkChatService,
  type ChatService,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ChatService.ts';

interface ReadCursorCall {
  conversationId: string;
  readSeq: number;
}

interface Deferred<T> {
  promise: Promise<T>;
  reject: (reason?: unknown) => void;
  resolve: (value: T) => void;
}

interface InboxEntry {
  conversationId: string;
  conversationType: 'single';
  displayName: string;
  lastActivityAt: string;
  lastMessageSeq: number;
  preferences: {
    isPinned: boolean;
  };
  unreadCount: number;
}

interface InboxListParams {
  cursor?: string;
  pageSize?: number;
}

interface InboxPage {
  items: InboxEntry[];
  pageInfo: {
    hasMore: boolean;
    mode: 'cursor';
    nextCursor?: string;
  };
}

interface MessagePage {
  items: Array<{
    body: {
      parts: Array<{ kind: 'text'; text: string }>;
      renderHints: { sdkworkChatPcType: 'text' };
      summary: string;
    };
    conversationId: string;
    deliveryMode: 'discrete';
    messageId: string;
    messageSeq: number;
    messageType: 'standard';
    occurredAt: string;
    sender: {
      id: string;
      kind: 'user';
      metadata: Record<string, never>;
    };
    summary: string;
  }>;
  pageInfo: {
    hasMore: boolean;
    mode: 'cursor';
    nextCursor?: string;
  };
}

interface PostMessageResult {
  messageId: string;
  messageSeq: number;
}

interface Harness {
  inboxListCalls: InboxListParams[];
  readCursorCalls: ReadCursorCall[];
  service: ChatService;
}

const LOCAL_CONVERSATION_CACHE_CAP = SDKWORK_MAX_PAGE_SIZE * 10;
const LOCAL_MESSAGES_PER_CONVERSATION_CAP = SDKWORK_MAX_PAGE_SIZE;
const CONCURRENT_RESOURCE_CAP = SDKWORK_MAX_PAGE_SIZE;

interface TestMessage {
  chatId: string;
  content: string;
  id: string;
  senderId: string;
  timestamp: number;
  type: 'text';
}

interface TestLiveSubscription {
  chatId: string;
  handlers: Set<(message: TestMessage) => void>;
  notifiedMessageVersions: Map<string, string>;
}

interface ChatServiceInternals {
  activeMessageHistoryLoads: Set<Promise<TestMessage[]>>;
  authSessionGeneration: number;
  chatListHandlers: Set<(chats: Array<{ id: string }>) => void>;
  chatListRefreshPromise?: Promise<void>;
  conversationViewState: Map<string, { isHidden?: boolean; isMarkedUnread?: boolean }>;
  conversationWireUnsubs: Map<string, () => void>;
  emitChatList: (generation?: number) => Promise<void>;
  getMessagesPromises: Map<string, Promise<TestMessage[]>>;
  handleAuthSessionChanged: () => void;
  handleLiveMessage: (
    fallbackChatId: string,
    decodedMessage: unknown,
    context: unknown,
    generation?: number,
  ) => void;
  handleLiveScopeEvent: (context: unknown, generation?: number) => void;
  inboxFirstPageCache?: {
    expiresAt: number;
    generation: number;
    pageSize: number;
    promise: Promise<unknown>;
  };
  inboxFirstPageCaches: Map<number, unknown>;
  inboxFirstPagePromise?: Promise<unknown>;
  inboxFirstPagePromises: Map<number, unknown>;
  lastChatListSnapshot: Array<{ id: string }>;
  latestReadSeq: Map<string, number>;
  liveInboxWireUnsub?: () => void;
  liveSubscriptions: Map<string, TestLiveSubscription>;
  loadMoreMessagesPromises: Map<string, Promise<TestMessage[]>>;
  localConversationCacheRecency: Map<string, undefined>;
  localMessages: Map<string, TestMessage[]>;
  messageHistoryPaginationState: Map<string, { hasMore: boolean; nextCursor?: string }>;
  notifyLiveSubscription: (
    subscription: TestLiveSubscription,
    message: TestMessage,
  ) => void;
  pendingRealtimeReadCursorSeqs: Map<string, number>;
  queueRealtimeReadCursorSync: (chatId: string, readSeq: number, generation?: number) => void;
  readCursorInFlightCounts: Map<string, number>;
  releaseInboxWireSubscription: () => void;
  realtimeReadCursorSyncPromise?: Promise<void>;
  runEmitChatList: (generation?: number) => Promise<void>;
  setLocalMessages: (chatId: string, messages: TestMessage[]) => void;
  upsertLocalMessage: (
    chatId: string,
    message: TestMessage,
    preferNew?: boolean,
  ) => TestMessage;
}

function readInternals(service: ChatService): ChatServiceInternals {
  return service as unknown as ChatServiceInternals;
}

function createDeferred<T>(): Deferred<T> {
  let reject!: (reason?: unknown) => void;
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    reject = rejectPromise;
    resolve = resolvePromise;
  });
  return { promise, reject, resolve };
}

function createInboxEntries(count: number, prefix = 'conversation'): InboxEntry[] {
  return Array.from({ length: count }, (_, ordinal) => ({
    conversationId: `${prefix}-${ordinal}`,
    conversationType: 'single' as const,
    displayName: `Conversation ${ordinal}`,
    lastActivityAt: new Date(Date.UTC(2026, 0, 1) + ordinal * 1_000).toISOString(),
    lastMessageSeq: ordinal + 1,
    preferences: {
      isPinned: true,
    },
    unreadCount: ordinal % 3,
  }));
}

function createLocalMessage(conversationId: string, messageSeq: number): TestMessage {
  return {
    chatId: conversationId,
    content: `message ${messageSeq}`,
    id: `${conversationId}:message:${messageSeq}`,
    senderId: 'u_sender',
    timestamp: Date.UTC(2026, 0, 1) + messageSeq * 1_000,
    type: 'text',
  };
}

function createMessagePage(
  conversationId: string,
  messageSeq: number,
  options: { hasMore?: boolean; nextCursor?: string } = {},
): MessagePage {
  const hasMore = options.hasMore ?? true;
  return {
    items: [
      {
        body: {
          parts: [{ kind: 'text', text: `message ${messageSeq}` }],
          renderHints: { sdkworkChatPcType: 'text' },
          summary: `message ${messageSeq}`,
        },
        conversationId,
        deliveryMode: 'discrete',
        messageId: `${conversationId}:message:${messageSeq}`,
        messageSeq,
        messageType: 'standard',
        occurredAt: new Date(Date.UTC(2026, 0, 1) + messageSeq * 1_000).toISOString(),
        sender: {
          id: 'u_sender',
          kind: 'user',
          metadata: {},
        },
        summary: `message ${messageSeq}`,
      },
    ],
    pageInfo: {
      hasMore,
      mode: 'cursor',
      ...(hasMore
        ? { nextCursor: options.nextCursor ?? `opaque:${conversationId}:older:${messageSeq}` }
        : {}),
    },
  };
}

function parseInboxCursor(cursor: string | undefined): number {
  if (!cursor) {
    return 0;
  }
  const offset = Number.parseInt(cursor.split(':').at(-1) ?? '', 10);
  assert.ok(Number.isSafeInteger(offset) && offset >= 0, `invalid test cursor: ${cursor}`);
  return offset;
}

function createInboxPage(
  entries: InboxEntry[],
  params: InboxListParams = {},
): InboxPage {
  const pageSize = params.pageSize ?? SDKWORK_MAX_PAGE_SIZE;
  const offset = parseInboxCursor(params.cursor);
  const items = entries.slice(offset, offset + pageSize);
  const nextOffset = offset + items.length;
  const hasMore = nextOffset < entries.length;
  return {
    items,
    pageInfo: {
      hasMore,
      mode: 'cursor',
      ...(hasMore ? { nextCursor: `opaque:inbox:${nextOffset}` } : {}),
    },
  };
}

function createHarness(options: {
  getSession?: () => SdkworkChatSession | null;
  inboxEntries: InboxEntry[];
  listInbox?: (params: InboxListParams) => Promise<InboxPage>;
  listMessages?: (
    conversationId: string,
    params: { cursor?: string; pageSize?: number } | undefined,
  ) => Promise<MessagePage>;
  postText?: (conversationId: string, content: string) => Promise<PostMessageResult>;
  updatePreferences?: (
    conversationId: string,
    preferences: Record<string, boolean>,
  ) => Promise<void>;
  updateReadCursor?: (conversationId: string, readSeq: number) => Promise<void>;
}): Harness {
  const inboxListCalls: InboxListParams[] = [];
  const readCursorCalls: ReadCursorCall[] = [];
  const fakeClient = {
    chat: {
      inbox: {
        async list(params: InboxListParams = {}) {
          inboxListCalls.push({ ...params });
          return options.listInbox?.(params) ?? createInboxPage(options.inboxEntries, params);
        },
      },
    },
    conversations: {
      async listMessages(
        conversationId: string,
        params?: { cursor?: string; pageSize?: number },
      ) {
        return options.listMessages?.(conversationId, params)
          ?? createMessagePage(conversationId, 1);
      },
      async postText(conversationId: string, content: string) {
        return options.postText?.(conversationId, content) ?? {
          messageId: `${conversationId}:posted`,
          messageSeq: 1,
        };
      },
      async updatePreferences(
        conversationId: string,
        preferences: Record<string, boolean>,
      ) {
        await options.updatePreferences?.(conversationId, preferences);
      },
      async updateReadCursor(conversationId: string, body: { readSeq: number }) {
        readCursorCalls.push({ conversationId, readSeq: body.readSeq });
        await options.updateReadCursor?.(conversationId, body.readSeq);
      },
    },
  } as unknown as ImSdkClient;

  return {
    inboxListCalls,
    readCursorCalls,
    service: createSdkworkChatService({
      getClient: () => fakeClient,
      getSession: options.getSession ?? (() => null),
    }),
  };
}

async function loadAllInboxPages(service: ChatService, initialCursor?: string): Promise<void> {
  let cursor = initialCursor;
  do {
    const page = await service.listChatsPage({
      ...(cursor ? { cursor } : {}),
      pageSize: SDKWORK_MAX_PAGE_SIZE,
    });
    cursor = page.hasMore ? page.nextCursor : undefined;
  } while (cursor);
}

function readCursorCallCount(calls: ReadCursorCall[], conversationId: string): number {
  return calls.filter((call) => call.conversationId === conversationId).length;
}

async function waitForCondition(
  predicate: () => boolean,
  description: string,
): Promise<void> {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise<void>((resolve) => setImmediate(resolve));
  }
  assert.fail(`timed out waiting for ${description}`);
}

async function assertInboxFirstPagePromiseSettlesAndTtlExpires(): Promise<void> {
  const originalNow = Date.now;
  let now = 10_000;
  Date.now = () => now;
  const harness = createHarness({
    inboxEntries: createInboxEntries(3, 'ttl'),
  });
  const internals = readInternals(harness.service);

  try {
    await harness.service.listChatsPage({ pageSize: 10 });
    assert.equal(
      internals.inboxFirstPagePromise,
      undefined,
      'a settled first-page request must release its in-flight slot',
    );
    assert.deepEqual(
      {
        generation: internals.inboxFirstPageCache?.generation,
        pageSize: internals.inboxFirstPageCache?.pageSize,
        requests: harness.inboxListCalls.length,
      },
      { generation: 0, pageSize: 10, requests: 1 },
      'the short TTL cache must record the normalized request identity',
    );

    await harness.service.listChatsPage({ pageSize: 10 });
    assert.equal(harness.inboxListCalls.length, 1, 'a fresh first-page TTL must reuse the response');

    now += 801;
    await harness.service.listChatsPage({ pageSize: 10 });
    assert.equal(harness.inboxListCalls.length, 2, 'an expired first-page TTL must refresh from the SDK');
  } finally {
    Date.now = originalNow;
  }
}

async function assertRejectedInboxFirstPageIsNotCached(): Promise<void> {
  const expectedError = new Error('inbox unavailable');
  let attempts = 0;
  const harness = createHarness({
    inboxEntries: [],
    listInbox: async (params) => {
      attempts += 1;
      if (attempts === 1) {
        throw expectedError;
      }
      return createInboxPage(createInboxEntries(1, 'retry-inbox'), params);
    },
  });
  const internals = readInternals(harness.service);

  await assert.rejects(
    harness.service.listChatsPage({ pageSize: 10 }),
    (error: unknown) => error === expectedError,
  );
  assert.deepEqual(
    {
      cacheEntries: internals.inboxFirstPageCaches.size,
      cachedPromise: internals.inboxFirstPageCache,
      inFlightEntries: internals.inboxFirstPagePromises.size,
      inFlightPromise: internals.inboxFirstPagePromise,
    },
    { cacheEntries: 0, cachedPromise: undefined, inFlightEntries: 0, inFlightPromise: undefined },
    'a rejected first page must release all in-flight state without entering the TTL cache',
  );

  const retryPage = await harness.service.listChatsPage({ pageSize: 10 });
  assert.deepEqual(retryPage.items.map((chat) => chat.id), ['retry-inbox-0']);
  assert.equal(attempts, 2, 'the next first-page call must retry the generated SDK');
}

async function assertInboxFirstPageIdentityIncludesPageSize(): Promise<void> {
  const requests: Array<Deferred<InboxPage>> = [];
  const harness = createHarness({
    inboxEntries: [],
    listInbox: async () => {
      const deferred = createDeferred<InboxPage>();
      requests.push(deferred);
      return deferred.promise;
    },
  });

  const smallPagePromise = harness.service.listChatsPage({ pageSize: 5 });
  await waitForCondition(() => requests.length === 1, 'the five-row inbox request');
  const largePagePromise = harness.service.listChatsPage({ pageSize: 10 });
  await waitForCondition(
    () => requests.length === 2,
    'a distinct ten-row inbox request instead of cross-page-size reuse',
  );
  const duplicateSmallPagePromise = harness.service.listChatsPage({ pageSize: 5 });
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(
    requests.length,
    2,
    'alternating page sizes must not bypass bounded in-flight deduplication',
  );

  requests[0]?.resolve(createInboxPage(createInboxEntries(5, 'small'), { pageSize: 5 }));
  requests[1]?.resolve(createInboxPage(createInboxEntries(10, 'large'), { pageSize: 10 }));
  const [smallPage, largePage, duplicateSmallPage] = await Promise.all([
    smallPagePromise,
    largePagePromise,
    duplicateSmallPagePromise,
  ]);
  assert.deepEqual(
    [smallPage.items.length, largePage.items.length, duplicateSmallPage.items.length],
    [5, 10, 5],
    'concurrent first pages with different normalized sizes must not share a response',
  );
}

async function assertInboxPageSizeNormalization(): Promise<void> {
  const harness = createHarness({ inboxEntries: [] });
  const cursor = 'opaque:inbox:0';

  await harness.service.listChatsPage({ cursor });
  await harness.service.listChatsPage({ cursor, pageSize: Number.NaN });
  await harness.service.listChatsPage({ cursor, pageSize: -1 });
  await harness.service.listChatsPage({ cursor, pageSize: 10.9 });
  await harness.service.listChatsPage({ cursor, pageSize: 200 });
  await harness.service.listChatsPage({ cursor, pageSize: 201 });

  assert.deepEqual(
    harness.inboxListCalls.map((call) => call.pageSize),
    [20, 20, 20, 10, 200, 200],
    'inbox page sizes must use default 20, floor positive values, and cap at the standard max 200',
  );
}

async function assertAuthGenerationFencesInboxCompletion(): Promise<void> {
  const requests: Array<Deferred<InboxPage>> = [];
  const oldEntry = { ...createInboxEntries(1, 'old-auth')[0]!, lastMessageSeq: 901 };
  const newEntry = { ...createInboxEntries(1, 'new-auth')[0]!, lastMessageSeq: 17 };
  const harness = createHarness({
    inboxEntries: [],
    listInbox: async () => {
      const deferred = createDeferred<InboxPage>();
      requests.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);

  const oldRequest = harness.service.listChatsPage({ pageSize: 10 });
  await waitForCondition(() => requests.length === 1, 'the old-account inbox request');
  internals.handleAuthSessionChanged();
  const newRequest = harness.service.listChatsPage({ pageSize: 10 });
  await waitForCondition(() => requests.length === 2, 'the new-account inbox request');

  requests[1]?.resolve(createInboxPage([newEntry], { pageSize: 10 }));
  const newPage = await newRequest;
  requests[0]?.resolve(createInboxPage([oldEntry], { pageSize: 10 }));
  const stalePage = await oldRequest;

  assert.deepEqual(
    {
      cachedNewSeq: internals.latestReadSeq.get(newEntry.conversationId),
      cachedOldSeq: internals.latestReadSeq.get(oldEntry.conversationId),
      currentSnapshot: internals.lastChatListSnapshot.map((chat) => chat.id),
      newPage: newPage.items.map((chat) => chat.id),
      stalePage: stalePage.items,
      staleViewState: internals.conversationViewState.get(oldEntry.conversationId),
    },
    {
      cachedNewSeq: 17,
      cachedOldSeq: undefined,
      currentSnapshot: [newEntry.conversationId],
      newPage: [newEntry.conversationId],
      stalePage: [],
      staleViewState: undefined,
    },
    'an old inbox completion must not hydrate, snapshot, persist, or expose data in the new session',
  );
}

async function assertAuthGenerationFencesInboxWhenOldCompletesFirst(): Promise<void> {
  const requests: Array<Deferred<InboxPage>> = [];
  const oldEntry = { ...createInboxEntries(1, 'old-first')[0]!, lastMessageSeq: 301 };
  const newEntry = { ...createInboxEntries(1, 'new-later')[0]!, lastMessageSeq: 19 };
  const harness = createHarness({
    inboxEntries: [],
    listInbox: async () => {
      const deferred = createDeferred<InboxPage>();
      requests.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);

  const oldRequest = harness.service.listChatsPage({ pageSize: 10 });
  await waitForCondition(() => requests.length === 1, 'the old-first inbox request');
  internals.handleAuthSessionChanged();
  const newRequest = harness.service.listChatsPage({ pageSize: 10 });
  await waitForCondition(() => requests.length === 2, 'the new-later inbox request');
  const currentRequestPromise = internals.inboxFirstPagePromise;

  requests[0]?.resolve(createInboxPage([oldEntry], { pageSize: 10 }));
  const stalePage = await oldRequest;
  assert.deepEqual(
    {
      currentRequestRetained: internals.inboxFirstPagePromise === currentRequestPromise,
      latestReadSeqSize: internals.latestReadSeq.size,
      snapshot: internals.lastChatListSnapshot,
      stalePage: stalePage.items,
      viewStateSize: internals.conversationViewState.size,
    },
    {
      currentRequestRetained: true,
      latestReadSeqSize: 0,
      snapshot: [],
      stalePage: [],
      viewStateSize: 0,
    },
    'old-first completion must not mutate state or release the new-account request slot',
  );

  requests[1]?.resolve(createInboxPage([newEntry], { pageSize: 10 }));
  const newPage = await newRequest;
  assert.deepEqual(
    {
      currentSeq: internals.latestReadSeq.get(newEntry.conversationId),
      newPage: newPage.items.map((chat) => chat.id),
      snapshot: internals.lastChatListSnapshot.map((chat) => chat.id),
    },
    { currentSeq: 19, newPage: [newEntry.conversationId], snapshot: [newEntry.conversationId] },
  );
}

async function assertAuthGenerationFencesChatListRefreshes(): Promise<void> {
  const requests: Array<Deferred<InboxPage>> = [];
  const notifications: string[][] = [];
  const oldEntry = createInboxEntries(1, 'old-refresh')[0]!;
  const newEntry = createInboxEntries(1, 'new-refresh')[0]!;
  const harness = createHarness({
    getSession: () => ({ accessToken: 'new-access-token', authToken: 'new-auth-token' }),
    inboxEntries: [],
    listInbox: async () => {
      const deferred = createDeferred<InboxPage>();
      requests.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);
  internals.chatListHandlers.add((chats) => {
    notifications.push(chats.map((chat) => chat.id));
  });

  const oldRefresh = internals.runEmitChatList();
  await waitForCondition(() => requests.length === 1, 'the old-account chat-list refresh');
  internals.handleAuthSessionChanged();
  assert.deepEqual(
    notifications,
    [[]],
    'auth reset must immediately clear the previous account from existing list subscribers',
  );
  assert.equal(
    typeof internals.liveInboxWireUnsub,
    'function',
    'auth reset must restore the inbox wire when chat-list subscriber demand remains',
  );
  const newRefresh = internals.runEmitChatList();
  await waitForCondition(() => requests.length === 2, 'the new-account chat-list refresh');

  const currentRefreshPromise = internals.chatListRefreshPromise;
  assert.ok(currentRefreshPromise, 'the new-account refresh must own the current in-flight slot');
  requests[0]?.resolve(createInboxPage([oldEntry]));
  await oldRefresh;
  assert.equal(
    internals.chatListRefreshPromise,
    currentRefreshPromise,
    'the old refresh finally block must not clear the new-account refresh slot',
  );
  assert.deepEqual(notifications, [[]], 'the old refresh must not notify current-session handlers');

  requests[1]?.resolve(createInboxPage([newEntry]));
  await newRefresh;
  assert.deepEqual(notifications, [[], [newEntry.conversationId]]);
  assert.equal(internals.chatListRefreshPromise, undefined);
  internals.releaseInboxWireSubscription();
}

async function assertAuthResetSettlesCoalescedRefreshPromise(): Promise<void> {
  const harness = createHarness({ inboxEntries: [] });
  const internals = readInternals(harness.service);
  internals.chatListHandlers.add(() => undefined);

  let settled = false;
  const scheduledRefresh = internals.emitChatList().then(() => {
    settled = true;
  });
  internals.handleAuthSessionChanged();
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(settled, true, 'clearing the auth-stale coalesce timer must settle its outer promise');
  await scheduledRefresh;
}

async function assertStaleRealtimeCallbacksOnlyAcknowledge(): Promise<void> {
  const harness = createHarness({ inboxEntries: [] });
  const internals = readInternals(harness.service);
  const staleGeneration = internals.authSessionGeneration;
  internals.handleAuthSessionChanged();
  let acknowledgements = 0;
  const acknowledge = async () => {
    acknowledgements += 1;
  };

  internals.handleLiveMessage(
    'stale-direct',
    {
      attachments: [],
      body: { parts: [{ kind: 'text', text: 'stale direct' }], renderHints: {}, summary: 'stale direct' },
      conversationId: 'stale-direct',
      messageId: 'stale-direct:1',
      messageSeq: 41,
      occurredAt: '2026-01-01T00:00:00.000Z',
      renderHints: {},
      sender: { id: 'u_sender', kind: 'user', metadata: {} },
      text: 'stale direct',
      type: 'text',
    },
    { ack: acknowledge },
    staleGeneration,
  );
  internals.handleLiveScopeEvent(
    {
      ack: acknowledge,
      eventType: 'message.posted',
      payload: {
        conversationId: 'stale-inbox',
        messageId: 'stale-inbox:1',
        messageSeq: 42,
        summary: 'stale inbox',
      },
    },
    staleGeneration,
  );
  await waitForCondition(() => acknowledgements === 2, 'both stale realtime acknowledgements');

  assert.deepEqual(
    {
      inboxRequests: harness.inboxListCalls.length,
      latestReadSeqSize: internals.latestReadSeq.size,
      localMessageSize: internals.localMessages.size,
      viewStateSize: internals.conversationViewState.size,
    },
    { inboxRequests: 0, latestReadSeqSize: 0, localMessageSize: 0, viewStateSize: 0 },
    'stale direct and inbox callbacks must only ACK without touching current-session state',
  );
}

async function assertAuthGenerationFencesRealtimeReadCursorSync(): Promise<void> {
  const cursorRequests: Array<Deferred<void>> = [];
  const harness = createHarness({
    inboxEntries: [],
    updateReadCursor: async () => {
      const deferred = createDeferred<void>();
      cursorRequests.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);
  const conversationId = 'shared-auth-conversation';

  internals.queueRealtimeReadCursorSync(conversationId, 99);
  await waitForCondition(() => cursorRequests.length === 1, 'the old-account read-cursor update');
  internals.handleAuthSessionChanged();
  internals.latestReadSeq.set(conversationId, 5);
  internals.queueRealtimeReadCursorSync(conversationId, 10);
  await waitForCondition(() => cursorRequests.length === 2, 'the new-account read-cursor update');

  const currentSyncPromise = internals.realtimeReadCursorSyncPromise;
  assert.ok(currentSyncPromise, 'the new-account read cursor must own the current sync slot');
  cursorRequests[0]?.resolve(undefined);
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.deepEqual(
    {
      currentReadSeq: internals.latestReadSeq.get(conversationId),
      currentSyncRetained: internals.realtimeReadCursorSyncPromise === currentSyncPromise,
      inFlightCount: internals.readCursorInFlightCounts.get(conversationId),
    },
    { currentReadSeq: 5, currentSyncRetained: true, inFlightCount: 1 },
    'old read-cursor completion must not write or release the new generation state',
  );

  cursorRequests[1]?.resolve(undefined);
  await currentSyncPromise;
  assert.deepEqual(
    {
      currentReadSeq: internals.latestReadSeq.get(conversationId),
      inFlightCount: internals.readCursorInFlightCounts.get(conversationId),
      pendingCount: internals.pendingRealtimeReadCursorSeqs.size,
      syncPromise: internals.realtimeReadCursorSyncPromise,
    },
    { currentReadSeq: 10, inFlightCount: undefined, pendingCount: 0, syncPromise: undefined },
  );
}

async function assertAuthGenerationFencesMarkAsReadCursorCompletion(): Promise<void> {
  const cursorRequests: Array<Deferred<void>> = [];
  const preferenceCalls: Array<{ conversationId: string; preferences: Record<string, boolean> }> = [];
  const harness = createHarness({
    inboxEntries: [],
    updatePreferences: async (conversationId, preferences) => {
      preferenceCalls.push({ conversationId, preferences });
    },
    updateReadCursor: async () => {
      const deferred = createDeferred<void>();
      cursorRequests.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);
  const conversationId = 'mark-read-shared-auth';
  internals.latestReadSeq.set(conversationId, 99);

  const oldMark = harness.service.markAsRead(conversationId);
  await waitForCondition(() => cursorRequests.length === 1, 'the old-account explicit read cursor');
  internals.handleAuthSessionChanged();
  internals.latestReadSeq.set(conversationId, 5);
  const newMark = harness.service.markAsRead(conversationId);
  await waitForCondition(() => cursorRequests.length === 2, 'the new-account explicit read cursor');

  cursorRequests[0]?.resolve(undefined);
  await oldMark;
  assert.deepEqual(
    {
      inFlightCount: internals.readCursorInFlightCounts.get(conversationId),
      latestReadSeq: internals.latestReadSeq.get(conversationId),
      preferenceCalls: preferenceCalls.length,
    },
    { inFlightCount: 1, latestReadSeq: 5, preferenceCalls: 0 },
    'the old mark-as-read completion must not write or release new-account ownership',
  );

  cursorRequests[1]?.resolve(undefined);
  await newMark;
  assert.deepEqual(
    {
      inFlightCount: internals.readCursorInFlightCounts.get(conversationId),
      latestReadSeq: internals.latestReadSeq.get(conversationId),
      markedUnread: internals.conversationViewState.get(conversationId)?.isMarkedUnread,
      preferenceCalls: preferenceCalls.length,
    },
    { inFlightCount: undefined, latestReadSeq: 5, markedUnread: false, preferenceCalls: 1 },
  );
}

async function assertAuthGenerationFencesMarkAsReadPreferenceCompletion(): Promise<void> {
  const preferenceRequests: Array<Deferred<void>> = [];
  const harness = createHarness({
    inboxEntries: [],
    updatePreferences: async () => {
      const deferred = createDeferred<void>();
      preferenceRequests.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);
  const conversationId = 'mark-read-preference-auth';

  const oldMark = harness.service.markAsRead(conversationId);
  await waitForCondition(() => preferenceRequests.length === 1, 'the old-account read preference update');
  internals.handleAuthSessionChanged();
  internals.conversationViewState.set(conversationId, { isMarkedUnread: true });
  preferenceRequests[0]?.resolve(undefined);
  await oldMark;

  assert.equal(
    internals.conversationViewState.get(conversationId)?.isMarkedUnread,
    true,
    'an old preference completion must not overwrite the new account conversation state',
  );
}

async function assertAuthGenerationFencesSendCompletion(): Promise<void> {
  const postDeferred = createDeferred<PostMessageResult>();
  let postCalls = 0;
  const harness = createHarness({
    inboxEntries: [],
    postText: async () => {
      postCalls += 1;
      return postDeferred.promise;
    },
  });
  const internals = readInternals(harness.service);
  const oldConversationId = 'send-old-account';
  const newConversationId = 'send-new-account';
  const sendPromise = harness.service.sendMessage(oldConversationId, 'old account message');
  const rejectedSend = assert.rejects(
    sendPromise,
    /Chat session changed while sending message\./u,
  );
  await waitForCondition(() => postCalls === 1, 'the old-account message post');

  internals.handleAuthSessionChanged();
  const newMessage = createLocalMessage(newConversationId, 5);
  internals.setLocalMessages(newConversationId, [newMessage]);
  internals.latestReadSeq.set(oldConversationId, 5);
  postDeferred.resolve({ messageId: 'old-server-message', messageSeq: 99 });
  await rejectedSend;

  assert.deepEqual(
    {
      newMessages: internals.localMessages.get(newConversationId),
      oldMessages: internals.localMessages.get(oldConversationId),
      oldReadSeqInNewGeneration: internals.latestReadSeq.get(oldConversationId),
    },
    {
      newMessages: [newMessage],
      oldMessages: undefined,
      oldReadSeqInNewGeneration: 5,
    },
    'an old send completion must not write messages or sequence state into the new generation',
  );
}

async function assertInboxOnlyStateUsesTheGlobalBound(): Promise<void> {
  const oldestConversationId = 'conversation-0';
  const newestConversationId = `conversation-${LOCAL_CONVERSATION_CACHE_CAP}`;
  const harness = createHarness({
    inboxEntries: createInboxEntries(LOCAL_CONVERSATION_CACHE_CAP + 1),
  });

  await loadAllInboxPages(harness.service);
  await harness.service.markAsRead(oldestConversationId);
  await harness.service.markAsRead(newestConversationId);
  const oldestChat = await harness.service.updateChat(oldestConversationId, { activeCount: 1 });
  const newestChat = await harness.service.updateChat(newestConversationId, { activeCount: 1 });

  assert.deepEqual(
    {
      newestPinnedProjection: newestChat.isPinned,
      newestReadCursorCalls: readCursorCallCount(harness.readCursorCalls, newestConversationId),
      oldestPinnedProjection: oldestChat.isPinned,
      oldestReadCursorCalls: readCursorCallCount(harness.readCursorCalls, oldestConversationId),
    },
    {
      newestPinnedProjection: true,
      newestReadCursorCalls: 1,
      oldestPinnedProjection: undefined,
      oldestReadCursorCalls: 0,
    },
    'inbox-only pagination must evict the oldest read-sequence and view-state companions',
  );
}

async function assertFocusedAndLiveConversationsAreProtected(): Promise<void> {
  const focusedConversationId = 'protected-0';
  const liveConversationId = 'protected-1';
  const firstRemovableConversationId = 'protected-2';
  const harness = createHarness({
    inboxEntries: createInboxEntries(LOCAL_CONVERSATION_CACHE_CAP + 2, 'protected'),
  });
  harness.service.setReadFocusContext({
    activeConversationId: focusedConversationId,
    isWindowFocused: true,
  });
  const unsubscribe = harness.service.subscribeMessages(liveConversationId, () => undefined);

  try {
    await loadAllInboxPages(harness.service);
    await harness.service.markAsRead(focusedConversationId);
    await harness.service.markAsRead(liveConversationId);
    await harness.service.markAsRead(firstRemovableConversationId);

    assert.deepEqual(
      {
        firstRemovableReadCursorCalls: readCursorCallCount(
          harness.readCursorCalls,
          firstRemovableConversationId,
        ),
        focusedReadCursorCalls: readCursorCallCount(
          harness.readCursorCalls,
          focusedConversationId,
        ),
        liveReadCursorCalls: readCursorCallCount(harness.readCursorCalls, liveConversationId),
      },
      {
        firstRemovableReadCursorCalls: 0,
        focusedReadCursorCalls: 1,
        liveReadCursorCalls: 1,
      },
      'focused and live conversations must survive eviction ahead of older unprotected state',
    );
  } finally {
    unsubscribe();
    harness.service.setReadFocusContext({ activeConversationId: undefined });
  }
}

async function assertMessageRequestsAreProtectedAndRebounded(): Promise<void> {
  const getConversationId = 'get-in-flight';
  const loadConversationId = 'load-in-flight';
  const getDeferred = createDeferred<MessagePage>();
  const loadDeferred = createDeferred<MessagePage>();
  const harness = createHarness({
    inboxEntries: createInboxEntries(LOCAL_CONVERSATION_CACHE_CAP, 'request-inbox'),
    listMessages: async (conversationId, params) => {
      if (conversationId === getConversationId) {
        return getDeferred.promise;
      }
      if (conversationId === loadConversationId && params?.cursor) {
        return loadDeferred.promise;
      }
      return createMessagePage(conversationId, 10, {
        nextCursor: `opaque:${conversationId}:older`,
      });
    },
  });

  await harness.service.getMessages(loadConversationId);
  await harness.service.markAsUnread(getConversationId);
  const getPromise = harness.service.getMessages(getConversationId);
  const loadPromise = harness.service.loadMoreMessages(loadConversationId);

  await loadAllInboxPages(harness.service);
  getDeferred.resolve(createMessagePage(getConversationId, 20));
  loadDeferred.resolve(createMessagePage(loadConversationId, 9, { hasMore: false }));
  await Promise.all([getPromise, loadPromise]);

  await harness.service.markAsRead(getConversationId);
  await harness.service.markAsRead(loadConversationId);
  await harness.service.markAsRead('request-inbox-0');
  assert.deepEqual(
    {
      firstInboxReadCursorCalls: readCursorCallCount(
        harness.readCursorCalls,
        'request-inbox-0',
      ),
      getReadCursorCalls: readCursorCallCount(harness.readCursorCalls, getConversationId),
      getStillHasMore: harness.service.hasMoreMessages(getConversationId),
      loadReadCursorCalls: readCursorCallCount(harness.readCursorCalls, loadConversationId),
    },
    {
      firstInboxReadCursorCalls: 0,
      getReadCursorCalls: 1,
      getStillHasMore: true,
      loadReadCursorCalls: 1,
    },
    'get/load promises must protect state, and async completion must refresh and re-bound it',
  );
}

async function assertReadCursorInFlightStateIsProtected(): Promise<void> {
  const activeConversationId = 'read-in-flight-0';
  const firstRemovableConversationId = 'read-in-flight-1';
  const readCursorDeferred = createDeferred<void>();
  const entries = createInboxEntries(LOCAL_CONVERSATION_CACHE_CAP + 1, 'read-in-flight');
  const harness = createHarness({
    inboxEntries: entries,
    updateReadCursor: async (conversationId) => {
      if (conversationId === activeConversationId) {
        await readCursorDeferred.promise;
      }
    },
  });

  const firstPage = await harness.service.listChatsPage({ pageSize: 1 });
  assert.equal(firstPage.nextCursor, 'opaque:inbox:1');
  const markAsReadPromise = harness.service.markAsRead(activeConversationId);
  await loadAllInboxPages(harness.service, firstPage.nextCursor);

  const activeChat = await harness.service.updateChat(activeConversationId, { activeCount: 1 });
  await harness.service.markAsRead(firstRemovableConversationId);
  assert.deepEqual(
    {
      activePinnedProjection: activeChat.isPinned,
      firstRemovableReadCursorCalls: readCursorCallCount(
        harness.readCursorCalls,
        firstRemovableConversationId,
      ),
    },
    {
      activePinnedProjection: true,
      firstRemovableReadCursorCalls: 0,
    },
    'an in-flight read cursor update must protect its conversation from eviction',
  );

  readCursorDeferred.resolve();
  await markAsReadPromise;
}

async function assertDeleteRemovesEveryConversationCacheCompanion(): Promise<void> {
  const conversationId = 'deleted-conversation';
  const harness = createHarness({
    inboxEntries: createInboxEntries(1, 'deleted-conversation'),
    listMessages: async () => createMessagePage(conversationId, 42),
  });

  await harness.service.getMessages(conversationId);
  assert.equal(harness.service.hasMoreMessages(conversationId), true);
  await harness.service.deleteChat(conversationId);
  await harness.service.markAsRead(conversationId);

  assert.deepEqual(
    {
      hasMoreMessages: harness.service.hasMoreMessages(conversationId),
      readCursorCalls: readCursorCallCount(harness.readCursorCalls, conversationId),
    },
    {
      hasMoreMessages: false,
      readCursorCalls: 0,
    },
    'deleting a conversation must remove messages, pagination, read sequence, and companion state',
  );
}

async function assertAuthGenerationFencesGetMessageCompletions(): Promise<void> {
  const conversationId = 'session-generation-get';
  const firstDeferred = createDeferred<MessagePage>();
  const secondDeferred = createDeferred<MessagePage>();
  let listMessageCalls = 0;
  const harness = createHarness({
    inboxEntries: [],
    listMessages: async () => {
      listMessageCalls += 1;
      if (listMessageCalls === 1) {
        return firstDeferred.promise;
      }
      if (listMessageCalls === 2) {
        return secondDeferred.promise;
      }
      return createMessagePage(conversationId, 300, { hasMore: false });
    },
  });
  const internals = readInternals(harness.service);

  const stalePromise = harness.service.getMessages(conversationId);
  await waitForCondition(() => listMessageCalls === 1, 'the old-session message request');
  internals.handleAuthSessionChanged();
  const currentPromise = harness.service.getMessages(conversationId);
  await waitForCondition(() => listMessageCalls === 2, 'the new-session message request');

  firstDeferred.resolve(createMessagePage(conversationId, 100));
  const staleResult = await stalePromise;
  assert.deepEqual(staleResult, [], 'an old-session getMessages result must not escape into new cache state');
  assert.equal(
    harness.service.hasMoreMessages(conversationId),
    false,
    'an old-session response must not restore pagination state after auth cache reset',
  );

  const joinedCurrentPromise = harness.service.getMessages(conversationId);
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(
    listMessageCalls,
    2,
    'the old request finally handler must not delete a newer same-chat request',
  );

  secondDeferred.resolve(createMessagePage(conversationId, 200, { hasMore: false }));
  const [currentResult, joinedResult] = await Promise.all([
    currentPromise,
    joinedCurrentPromise,
  ]);
  const expectedIds = [`${conversationId}:message:200`];
  assert.deepEqual(currentResult.map((message) => message.id), expectedIds);
  assert.deepEqual(joinedResult.map((message) => message.id), expectedIds);

  const lateStaleConversationId = 'session-generation-get-late-stale';
  const lateStaleDeferred = createDeferred<MessagePage>();
  const earlyCurrentDeferred = createDeferred<MessagePage>();
  let lateStaleCalls = 0;
  const lateStaleHarness = createHarness({
    inboxEntries: [],
    listMessages: async () => {
      lateStaleCalls += 1;
      return lateStaleCalls === 1
        ? lateStaleDeferred.promise
        : earlyCurrentDeferred.promise;
    },
  });
  const lateStalePromise = lateStaleHarness.service.getMessages(lateStaleConversationId);
  await waitForCondition(() => lateStaleCalls === 1, 'the second old-session request');
  readInternals(lateStaleHarness.service).handleAuthSessionChanged();
  const earlyCurrentPromise = lateStaleHarness.service.getMessages(lateStaleConversationId);
  await waitForCondition(() => lateStaleCalls === 2, 'the second new-session request');
  earlyCurrentDeferred.resolve(createMessagePage(lateStaleConversationId, 500, { hasMore: false }));
  await earlyCurrentPromise;
  lateStaleDeferred.resolve(createMessagePage(lateStaleConversationId, 400));
  assert.deepEqual(
    await lateStalePromise,
    [],
    'an old caller must not receive messages already cached for the new authenticated session',
  );
}

async function assertAuthGenerationFencesLoadMoreCompletions(): Promise<void> {
  const conversationId = 'session-generation-load-more';
  const staleLoadDeferred = createDeferred<MessagePage>();
  const currentLoadDeferred = createDeferred<MessagePage>();
  let listMessageCalls = 0;
  const harness = createHarness({
    inboxEntries: [],
    listMessages: async (_requestedConversationId, params) => {
      listMessageCalls += 1;
      if (!params?.cursor && listMessageCalls === 1) {
        return createMessagePage(conversationId, 50, { nextCursor: 'old-session-cursor' });
      }
      if (params?.cursor === 'old-session-cursor') {
        return staleLoadDeferred.promise;
      }
      if (!params?.cursor && listMessageCalls === 3) {
        return createMessagePage(conversationId, 100, { nextCursor: 'new-session-cursor' });
      }
      if (params?.cursor === 'new-session-cursor') {
        return currentLoadDeferred.promise;
      }
      return createMessagePage(conversationId, 300, { hasMore: false });
    },
  });
  const internals = readInternals(harness.service);

  await harness.service.getMessages(conversationId);
  const staleLoadPromise = harness.service.loadMoreMessages(conversationId);
  await waitForCondition(() => listMessageCalls === 2, 'the old-session load-more request');
  internals.handleAuthSessionChanged();
  await harness.service.getMessages(conversationId);
  const currentLoadPromise = harness.service.loadMoreMessages(conversationId);
  await waitForCondition(() => listMessageCalls === 4, 'the new-session load-more request');

  staleLoadDeferred.resolve(createMessagePage(conversationId, 49, { hasMore: false }));
  assert.deepEqual(
    await staleLoadPromise,
    [],
    'an old-session loadMoreMessages result must not escape into new cache state',
  );
  const joinedCurrentLoadPromise = harness.service.loadMoreMessages(conversationId);
  await new Promise<void>((resolve) => setImmediate(resolve));
  assert.equal(
    listMessageCalls,
    4,
    'the old load-more finally handler must not delete a newer same-chat request',
  );

  currentLoadDeferred.resolve(createMessagePage(conversationId, 99, { hasMore: false }));
  const [currentResult, joinedResult] = await Promise.all([
    currentLoadPromise,
    joinedCurrentLoadPromise,
  ]);
  const expectedIds = [`${conversationId}:message:99`];
  assert.deepEqual(currentResult.map((message) => message.id), expectedIds);
  assert.deepEqual(joinedResult.map((message) => message.id), expectedIds);
  assert.deepEqual(
    internals.localMessages.get(conversationId)?.map((message) => message.id),
    [
      `${conversationId}:message:99`,
      `${conversationId}:message:100`,
    ],
    'stale history must not be merged into the current-session retained window',
  );
}

async function assertProtectedCacheStateStillHasAHardBound(): Promise<void> {
  const harness = createHarness({ inboxEntries: [] });
  const internals = readInternals(harness.service);
  const focusedConversationId = 'all-protected-0';
  harness.service.setReadFocusContext({ activeConversationId: focusedConversationId });

  for (let ordinal = 0; ordinal < LOCAL_CONVERSATION_CACHE_CAP + 2; ordinal += 1) {
    const conversationId = `all-protected-${ordinal}`;
    internals.liveSubscriptions.set(conversationId, {
      chatId: conversationId,
      handlers: new Set(),
      notifiedMessageVersions: new Map(),
    });
    internals.setLocalMessages(conversationId, [createLocalMessage(conversationId, ordinal + 1)]);
  }

  assert.deepEqual(
    {
      cacheEntries: internals.localConversationCacheRecency.size,
      focusedRetained: internals.localMessages.has(focusedConversationId),
      newestRetained: internals.localMessages.has(
        `all-protected-${LOCAL_CONVERSATION_CACHE_CAP + 1}`,
      ),
      oldestNonFocusedRetained: internals.localMessages.has('all-protected-1'),
    },
    {
      cacheEntries: LOCAL_CONVERSATION_CACHE_CAP,
      focusedRetained: true,
      newestRetained: true,
      oldestNonFocusedRetained: false,
    },
    'the cache must evict oldest protected state as a last resort while retaining focus and recency',
  );
  internals.liveSubscriptions.clear();
}

async function assertLiveAndMessageLoadMetadataHaveIndependentBounds(): Promise<void> {
  const messageDeferreds: Array<Deferred<MessagePage>> = [];
  const harness = createHarness({
    inboxEntries: [],
    listMessages: async () => {
      const deferred = createDeferred<MessagePage>();
      messageDeferreds.push(deferred);
      return deferred.promise;
    },
  });
  const internals = readInternals(harness.service);
  const unsubs: Array<() => void> = [];

  for (let ordinal = 0; ordinal < CONCURRENT_RESOURCE_CAP; ordinal += 1) {
    unsubs.push(harness.service.subscribeMessages(`bounded-live-${ordinal}`, () => undefined));
  }
  assert.throws(
    () => harness.service.subscribeMessages('bounded-live-overflow', () => undefined),
    /live conversation subscription limit/iu,
  );
  assert.deepEqual(
    {
      liveSubscriptions: internals.liveSubscriptions.size,
      wireSubscriptions: internals.conversationWireUnsubs.size,
    },
    {
      liveSubscriptions: CONCURRENT_RESOURCE_CAP,
      wireSubscriptions: CONCURRENT_RESOURCE_CAP,
    },
  );
  for (const unsubscribe of unsubs) {
    unsubscribe();
  }

  const messagePromises = Array.from(
    { length: CONCURRENT_RESOURCE_CAP },
    (_, ordinal) => harness.service.getMessages(`bounded-load-${ordinal}`),
  );
  await waitForCondition(
    () => messageDeferreds.length === CONCURRENT_RESOURCE_CAP,
    'all bounded concurrent message requests',
  );
  internals.messageHistoryPaginationState.set('bounded-load-more-overflow', {
    hasMore: true,
    nextCursor: 'overflow-cursor',
  });
  await assert.rejects(
    harness.service.loadMoreMessages('bounded-load-more-overflow'),
    /concurrent message history load limit/iu,
  );
  assert.equal(
    internals.getMessagesPromises.size + internals.loadMoreMessagesPromises.size,
    CONCURRENT_RESOURCE_CAP,
    'get and load-more requests must share one hard concurrent metadata bound',
  );
  assert.equal(
    internals.activeMessageHistoryLoads.size,
    CONCURRENT_RESOURCE_CAP,
    'the aggregate request bound must retain in-flight work independently of per-session indexes',
  );

  internals.handleAuthSessionChanged();
  assert.equal(
    internals.activeMessageHistoryLoads.size,
    CONCURRENT_RESOURCE_CAP,
    'auth cache reset must not forget requests that are still executing',
  );
  for (const [ordinal, deferred] of messageDeferreds.entries()) {
    deferred.resolve(createMessagePage(`bounded-load-${ordinal}`, ordinal + 1, { hasMore: false }));
  }
  await Promise.all(messagePromises);
  assert.equal(internals.activeMessageHistoryLoads.size, 0);
}

async function assertNotificationVersionsTrackOnlyTheRetainedMessageWindow(): Promise<void> {
  const conversationId = 'bounded-notification-versions';
  const harness = createHarness({ inboxEntries: [] });
  const internals = readInternals(harness.service);
  let notificationCount = 0;
  const unsubscribe = harness.service.subscribeMessages(conversationId, () => {
    notificationCount += 1;
  });
  const subscription = internals.liveSubscriptions.get(conversationId);
  assert.ok(subscription);

  const totalMessages = LOCAL_MESSAGES_PER_CONVERSATION_CAP * 5;
  for (let messageSeq = 1; messageSeq <= totalMessages; messageSeq += 1) {
    const message = createLocalMessage(conversationId, messageSeq);
    internals.upsertLocalMessage(conversationId, message);
    internals.notifyLiveSubscription(subscription, message);
  }

  const retainedMessages = internals.localMessages.get(conversationId) ?? [];
  const retainedIds = new Set(retainedMessages.map((message) => message.id));
  assert.equal(retainedMessages.length, LOCAL_MESSAGES_PER_CONVERSATION_CAP);
  assert.equal(subscription.notifiedMessageVersions.size, LOCAL_MESSAGES_PER_CONVERSATION_CAP);
  assert.ok(
    [...subscription.notifiedMessageVersions.keys()].every((messageId) => retainedIds.has(messageId)),
    'notification versions must not retain IDs outside the retained message window',
  );

  const oldestRetainedMessage = retainedMessages[0];
  assert.ok(oldestRetainedMessage);
  internals.notifyLiveSubscription(subscription, oldestRetainedMessage);
  assert.equal(notificationCount, totalMessages, 'duplicate versions must not notify handlers twice');
  assert.equal(
    [...subscription.notifiedMessageVersions.keys()].at(-1),
    oldestRetainedMessage.id,
    'a duplicate notification lookup must refresh version-map recency',
  );

  const newestMessage = createLocalMessage(conversationId, totalMessages + 1);
  internals.upsertLocalMessage(conversationId, newestMessage);
  internals.notifyLiveSubscription(subscription, newestMessage);
  assert.equal(subscription.notifiedMessageVersions.size, LOCAL_MESSAGES_PER_CONVERSATION_CAP);
  assert.equal(
    subscription.notifiedMessageVersions.has(oldestRetainedMessage.id),
    false,
    'trimming local messages must prune the matching notification version immediately',
  );
  unsubscribe();
}

async function assertDeleteFencesDeferredLoadsAndQueuedDirectMessages(): Promise<void> {
  const conversationId = 'concurrent-delete';
  const getDeferred = createDeferred<MessagePage>();
  const loadDeferred = createDeferred<MessagePage>();
  const deleteDeferred = createDeferred<void>();
  let deletePreferenceCalls = 0;
  const harness = createHarness({
    inboxEntries: [],
    listMessages: async (_requestedConversationId, params) => (
      params?.cursor ? loadDeferred.promise : getDeferred.promise
    ),
    updatePreferences: async (_requestedConversationId, preferences) => {
      if (preferences.isHidden) {
        deletePreferenceCalls += 1;
        await deleteDeferred.promise;
      }
    },
  });
  const internals = readInternals(harness.service);
  internals.messageHistoryPaginationState.set(conversationId, {
    hasMore: true,
    nextCursor: 'delete-cursor',
  });
  let notificationCount = 0;
  harness.service.subscribeMessages(conversationId, () => {
    notificationCount += 1;
  });
  const deletedSubscription = internals.liveSubscriptions.get(conversationId);
  assert.ok(deletedSubscription);
  const getPromise = harness.service.getMessages(conversationId);
  const loadPromise = harness.service.loadMoreMessages(conversationId);
  const deletePromise = harness.service.deleteChat(conversationId);
  await waitForCondition(() => deletePreferenceCalls === 1, 'the deferred delete preference write');
  deleteDeferred.resolve();
  await deletePromise;

  assert.deepEqual(
    {
      hiddenTombstone: internals.conversationViewState.get(conversationId)?.isHidden,
      retainedDetachedHandlers: deletedSubscription.handlers.size,
      liveSubscriptionRetained: internals.liveSubscriptions.has(conversationId),
      tombstoneInBoundedLru: internals.localConversationCacheRecency.has(conversationId),
      wireSubscriptionRetained: internals.conversationWireUnsubs.has(conversationId),
    },
    {
      hiddenTombstone: true,
      retainedDetachedHandlers: 0,
      liveSubscriptionRetained: false,
      tombstoneInBoundedLru: true,
      wireSubscriptionRetained: false,
    },
    'delete must replace cache state with a bounded tombstone and close live resources',
  );

  let ackCount = 0;
  internals.handleLiveMessage(
    conversationId,
    {
      attachments: [],
      body: {
        parts: [{ kind: 'text', text: 'queued after delete' }],
        renderHints: {},
        summary: 'queued after delete',
      },
      conversationId,
      messageId: `${conversationId}:queued`,
      messageSeq: 999,
      messageType: 'standard',
      occurredAt: '2026-01-01T00:00:00.000Z',
      renderHints: {},
      sender: { id: 'u_sender', kind: 'user', metadata: {} },
      summary: 'queued after delete',
      text: 'queued after delete',
      type: 'text',
    },
    {
      ack: async () => {
        ackCount += 1;
      },
      conversationId,
      messageId: `${conversationId}:queued`,
      payload: { conversationId, messageSeq: 999 },
      rawEvent: {},
      receivedAt: '2026-01-01T00:00:00.000Z',
      sender: { id: 'u_sender', kind: 'user', metadata: {} },
      sequence: 999,
    },
  );
  await waitForCondition(() => ackCount === 1, 'the hidden-conversation realtime acknowledgement');

  getDeferred.resolve(createMessagePage(conversationId, 100));
  loadDeferred.resolve(createMessagePage(conversationId, 99, { hasMore: false }));
  const [getResult, loadResult] = await Promise.all([getPromise, loadPromise]);
  assert.deepEqual(
    {
      cachedMessages: internals.localMessages.get(conversationId),
      getResult,
      hasMoreMessages: harness.service.hasMoreMessages(conversationId),
      loadResult,
      notificationCount,
    },
    {
      cachedMessages: undefined,
      getResult: [],
      hasMoreMessages: false,
      loadResult: [],
      notificationCount: 0,
    },
    'deferred request results and queued direct messages must not resurrect a hidden conversation',
  );
}

async function assertRecentDeleteTombstoneSurvivesProtectedPressure(): Promise<void> {
  const deletedConversationId = 'recent-delete-under-pressure';
  const harness = createHarness({ inboxEntries: [] });
  const internals = readInternals(harness.service);
  for (let ordinal = 0; ordinal < LOCAL_CONVERSATION_CACHE_CAP; ordinal += 1) {
    const conversationId = `delete-pressure-${ordinal}`;
    internals.liveSubscriptions.set(conversationId, {
      chatId: conversationId,
      handlers: new Set(),
      notifiedMessageVersions: new Map(),
    });
    internals.setLocalMessages(conversationId, [createLocalMessage(conversationId, ordinal + 1)]);
  }

  await harness.service.deleteChat(deletedConversationId);
  assert.deepEqual(
    {
      cacheEntries: internals.localConversationCacheRecency.size,
      hiddenTombstone: internals.conversationViewState.get(deletedConversationId)?.isHidden,
      oldestProtectedRetained: internals.localMessages.has('delete-pressure-0'),
      tombstoneRetained: internals.localConversationCacheRecency.has(deletedConversationId),
    },
    {
      cacheEntries: LOCAL_CONVERSATION_CACHE_CAP,
      hiddenTombstone: true,
      oldestProtectedRetained: false,
      tombstoneRetained: true,
    },
    'a newly written hidden tombstone must survive pressure ahead of older protected cache state',
  );
  internals.liveSubscriptions.clear();
}

async function main(): Promise<void> {
  const checks = new Map<string, () => Promise<void>>([
    ['inbox-first-page-lifecycle', assertInboxFirstPagePromiseSettlesAndTtlExpires],
    ['inbox-first-page-rejection', assertRejectedInboxFirstPageIsNotCached],
    ['inbox-first-page-page-size', assertInboxFirstPageIdentityIncludesPageSize],
    ['inbox-page-size-normalization', assertInboxPageSizeNormalization],
    ['auth-inbox-fence', assertAuthGenerationFencesInboxCompletion],
    ['auth-inbox-old-first-fence', assertAuthGenerationFencesInboxWhenOldCompletesFirst],
    ['auth-chat-list-refresh-fence', assertAuthGenerationFencesChatListRefreshes],
    ['auth-coalesce-settle', assertAuthResetSettlesCoalescedRefreshPromise],
    ['auth-realtime-callback-fence', assertStaleRealtimeCallbacksOnlyAcknowledge],
    ['auth-read-cursor-fence', assertAuthGenerationFencesRealtimeReadCursorSync],
    ['auth-mark-read-cursor-fence', assertAuthGenerationFencesMarkAsReadCursorCompletion],
    ['auth-mark-read-preference-fence', assertAuthGenerationFencesMarkAsReadPreferenceCompletion],
    ['auth-send-completion-fence', assertAuthGenerationFencesSendCompletion],
    ['inbox-bound', assertInboxOnlyStateUsesTheGlobalBound],
    ['protected-priority', assertFocusedAndLiveConversationsAreProtected],
    ['request-protection', assertMessageRequestsAreProtectedAndRebounded],
    ['read-cursor-protection', assertReadCursorInFlightStateIsProtected],
    ['delete-companions', assertDeleteRemovesEveryConversationCacheCompanion],
    ['auth-get-fence', assertAuthGenerationFencesGetMessageCompletions],
    ['auth-load-more-fence', assertAuthGenerationFencesLoadMoreCompletions],
    ['protected-hard-bound', assertProtectedCacheStateStillHasAHardBound],
    ['resource-bounds', assertLiveAndMessageLoadMetadataHaveIndependentBounds],
    ['notification-window', assertNotificationVersionsTrackOnlyTheRetainedMessageWindow],
    ['concurrent-delete', assertDeleteFencesDeferredLoadsAndQueuedDirectMessages],
    ['delete-tombstone-pressure', assertRecentDeleteTombstoneSurvivesProtectedPressure],
  ]);
  const selectedCheck = process.argv[2];
  if (selectedCheck) {
    const check = checks.get(selectedCheck);
    assert.ok(check, `unknown bounded-cache check: ${selectedCheck}`);
    await check();
  } else {
    for (const check of checks.values()) {
      await check();
    }
  }
  console.log('sdkwork im pc bounded conversation cache contract passed.');
}

void main();
