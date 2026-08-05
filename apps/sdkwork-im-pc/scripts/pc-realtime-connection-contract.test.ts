import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import type {
  ImCallParticipantCredential,
  ImCallSession,
  ImDecodedMessage,
  ImLiveConnection,
  ImLiveConnectionState,
  ImMessageContext,
  ImRealtimeEventContext,
} from '@sdkwork/im-sdk';

const managerText = readFileSync(
  './packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager.ts',
  'utf8',
);
const chatServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ChatService.ts',
  'utf8',
);
const contactServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/ContactService.ts',
  'utf8',
);
const callServiceText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/services/CallService.ts',
  'utf8',
);
const realtimeSdkText = readFileSync(
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime.ts',
  'utf8',
);

assert.match(
  managerText,
  /sharedConnectionPromise/u,
  'PC realtime manager must dedupe in-flight connect attempts',
);
assert.match(
  managerText,
  /sharedConnectionPromise[\s\S]*recoverPcLiveConnection/u,
  'PC realtime recovery must be aware of in-flight connect attempts',
);
assert.match(
  realtimeSdkText,
  /events\.nack/u,
  'IM SDK realtime client must support events.nack ARQ recovery',
);
assert.match(
  managerText,
  /recoverPcLiveConnection[\s\S]*connectionStatus === 'open'[\s\S]*connectionStatus === 'connecting'/u,
  'PC realtime recovery must skip healthy connections',
);
assert.match(
  managerText,
  /CIRCUIT_BREAKER_FAILURE_THRESHOLD/u,
  'PC realtime manager must include circuit breaker protection',
);
assert.match(
  managerText,
  /connectionStatus = 'connecting'/u,
  'PC realtime manager must not mark the connection open before lifecycle open',
);
assert.match(
  managerText,
  /state\.status === 'open'[\s\S]*syncWireSubscriptions\(connection\)/u,
  'PC realtime wire subscription sync must run on lifecycle open',
);
assert.match(
  managerText,
  /syncWireSubscriptionsWhenReady[\s\S]*connectionStatus !== 'open'/u,
  'PC realtime wire subscription sync must defer until lifecycle open',
);
assert.doesNotMatch(
  managerText,
  /\.then\(\(connection\) => \{[\s\S]*syncWireSubscriptions\(connection\)/u,
  'PC realtime manager must not sync wire subscriptions immediately after connect resolves',
);
assert.doesNotMatch(
  managerText,
  /connectionStatus = 'open'[\s\S]*syncWireSubscriptions\(connection\)[\s\S]*lifecycleUnsub = connection\.lifecycle\.onStateChange/u,
  'PC realtime manager must not sync wire subscriptions before lifecycle subscription',
);
assert.doesNotMatch(
  chatServiceText,
  /this\.client\(\)\.connect\(/u,
  'ChatService must not open dedicated websocket connections',
);
assert.match(
  chatServiceText,
  /subscribePcConversationMessages/u,
  'ChatService must subscribe through the shared PC realtime manager',
);
assert.match(
  chatServiceText,
  /recoverPcLiveConnection/u,
  'ChatService must delegate realtime recovery to the shared manager',
);
assert.doesNotMatch(
  contactServiceText,
  /this\.client\(\)\.connect\(/u,
  'ContactService must not open dedicated websocket connections',
);
assert.match(
  contactServiceText,
  /subscribePcRealtimeScope/u,
  'ContactService must subscribe friend-request scopes through the shared manager',
);
assert.match(
  callServiceText,
  /watchIncoming\(\{[\s\S]*connection,/u,
  'CallService must reuse the shared live connection for incoming call watch',
);
assert.match(
  callServiceText,
  /acquirePcLiveConnectionLease/u,
  'CallService must hold a shared-connection lease while watching incoming calls',
);

type StateHandler = (state: ImLiveConnectionState) => void;
type ErrorHandler = (error: unknown) => void;

class Deferred<T> {
  promise: Promise<T>;

  reject!: (reason?: unknown) => void;

  resolve!: (value: T | PromiseLike<T>) => void;

  constructor() {
    this.promise = new Promise<T>((resolve, reject) => {
      this.resolve = resolve;
      this.reject = reject;
    });
  }
}

interface ScheduledTimer {
  callback: () => void;
  runAt: number;
}

class FakeTimerScheduler {
  now = 1_000_000;

  private nextId = 1;

  private readonly timers = new Map<number, ScheduledTimer>();

  clearTimeout(handle: unknown): void {
    if (typeof handle === 'number') {
      this.timers.delete(handle);
    }
  }

  nextDelay(): number | undefined {
    let nextRunAt: number | undefined;
    for (const timer of this.timers.values()) {
      nextRunAt = nextRunAt === undefined ? timer.runAt : Math.min(nextRunAt, timer.runAt);
    }
    return nextRunAt === undefined ? undefined : nextRunAt - this.now;
  }

  setTimeout(callback: () => void, delay = 0): number {
    const id = this.nextId;
    this.nextId += 1;
    this.timers.set(id, {
      callback,
      runAt: this.now + Math.max(0, delay),
    });
    return id;
  }

  advanceBy(duration: number): void {
    const target = this.now + duration;
    while (true) {
      const next = [...this.timers.entries()]
        .filter(([, timer]) => timer.runAt <= target)
        .sort((left, right) => left[1].runAt - right[1].runAt || left[0] - right[0])[0];
      if (!next) {
        break;
      }
      const [id, timer] = next;
      this.timers.delete(id);
      this.now = timer.runAt;
      timer.callback();
    }
    this.now = target;
  }
}

class FakeLiveConnection implements ImLiveConnection {
  readonly disconnects: Array<{ code?: number; reason?: string }> = [];
  readonly stateHandlers = new Set<StateHandler>();
  readonly errorHandlers = new Set<ErrorHandler>();
  readonly syncedConversations: string[][] = [];

  private readonly conversationHandlers = new Map<
    string,
    Set<(message: ImDecodedMessage, context: ImMessageContext) => void>
  >();

  private readonly scopeHandlers = new Map<
    string,
    Set<(event: Record<string, unknown>, context: ImRealtimeEventContext) => void>
  >();

  private currentState: ImLiveConnectionState = { status: 'connecting' };

  disconnect(code?: number, reason?: string): void {
    this.disconnects.push({ code, reason });
    this.emitState({ status: 'closed', reason });
  }

  emitOpen(): void {
    this.emitState({ status: 'open' });
  }

  emitState(state: ImLiveConnectionState): void {
    this.currentState = state;
    for (const handler of this.stateHandlers) {
      handler(state);
    }
  }

  emitError(error: unknown): void {
    for (const handler of this.errorHandlers) {
      handler(error);
    }
  }

  emitConversation(
    conversationId: string,
    message: ImDecodedMessage,
    context: ImMessageContext,
  ): void {
    for (const handler of this.conversationHandlers.get(conversationId) ?? []) {
      handler(message, context);
    }
  }

  emitScope(
    scopeType: string,
    scopeId: string,
    context: ImRealtimeEventContext,
  ): void {
    for (const handler of this.scopeHandlers.get(`${scopeType}:${scopeId}`) ?? []) {
      handler({}, context);
    }
  }

  events = {
    onConversation: (
      _conversationId: string,
      _handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void,
    ) => () => undefined,
    onScope: (
      scopeType: string,
      scopeId: string,
      handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void,
    ) => {
      const key = `${scopeType}:${scopeId}`;
      const handlers = this.scopeHandlers.get(key) ?? new Set();
      handlers.add(handler);
      this.scopeHandlers.set(key, handlers);
      return () => {
        handlers.delete(handler);
        if (handlers.size === 0) {
          this.scopeHandlers.delete(key);
        }
      };
    },
  };

  lifecycle = {
    onError: (handler: ErrorHandler) => {
      this.errorHandlers.add(handler);
      return () => {
        this.errorHandlers.delete(handler);
      };
    },
    onStateChange: (handler: StateHandler) => {
      this.stateHandlers.add(handler);
      handler(this.currentState);
      return () => {
        this.stateHandlers.delete(handler);
      };
    },
  };

  messages = {
    onConversation: (
      conversationId: string,
      handler: (message: ImDecodedMessage, context: ImMessageContext) => void,
    ) => {
      const handlers = this.conversationHandlers.get(conversationId) ?? new Set();
      handlers.add(handler);
      this.conversationHandlers.set(conversationId, handlers);
      return () => {
        handlers.delete(handler);
        if (handlers.size === 0) {
          this.conversationHandlers.delete(conversationId);
        }
      };
    },
  };

  subscriptions = {
    syncConversations: (conversationIds: string[]) => {
      this.syncedConversations.push([...conversationIds]);
    },
    syncScopes: () => undefined,
  };
}

function createCallSession(
  rtcSessionId: string,
  state: string,
  conversationId = 'conversation-call-state',
): ImCallSession {
  return {
    tenantId: '100001',
    rtcSessionId,
    conversationId,
    initiatorId: 'user-peer',
    initiatorKind: 'user',
    rtcMode: 'video',
    state,
    startedAt: '2026-07-12T00:00:00.000Z',
  };
}

async function waitForCondition(predicate: () => boolean, label: string): Promise<void> {
  const startedAt = Date.now();
  while (!predicate()) {
    if (Date.now() - startedAt > 1000) {
      throw new Error(`timed out waiting for ${label}`);
    }
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
}

async function flushMicrotasks(): Promise<void> {
  for (let index = 0; index < 8; index += 1) {
    await Promise.resolve();
  }
}

async function runSingleFlightRecoveryContract(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  manager.resetPcRealtimeConnectionManagerForTests();

  const firstConnection = new FakeLiveConnection();
  const firstConnect = new Deferred<FakeLiveConnection>();
  let connectCount = 0;

  manager.configurePcRealtimeConnectionManager({
    getClient: () => ({
      connect: () => {
        connectCount += 1;
        return firstConnect.promise;
      },
    } as never),
    getSession: () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
  });

  const unsubscribe = manager.subscribePcConversationMessages('conversation-1', () => undefined);
  assert.equal(connectCount, 1, 'first subscription must start exactly one websocket connect');

  manager.recoverPcLiveConnection('browser online', { force: true });
  manager.recoverPcLiveConnection('browser visible', { force: true });
  await Promise.resolve();

  assert.equal(
    connectCount,
    1,
    'PC realtime recovery must not open another websocket while the shared connection is still connecting',
  );
  assert.equal(
    manager.getPcLiveConnectionDiagnostics().totalConnectionsCreated,
    1,
    'PC realtime diagnostics must count a single in-flight connection during recovery bursts',
  );

  firstConnect.resolve(firstConnection);
  const activeConnection = await manager.ensurePcLiveConnection();
  assert.equal(activeConnection, firstConnection, 'recovery must keep the original in-flight connection as the singleton');
  firstConnection.emitOpen();

  manager.recoverPcLiveConnection('browser online', { force: true });
  await Promise.resolve();

  assert.equal(
    connectCount,
    1,
    'PC realtime recovery must not replace an already open healthy shared websocket',
  );
  assert.equal(firstConnection.disconnects.length, 0, 'healthy recovery must not disconnect the singleton websocket');

  unsubscribe();
  manager.resetPcRealtimeConnectionManagerForTests();
}

async function runInvalidateDuringConnectContract(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  manager.resetPcRealtimeConnectionManagerForTests();

  const firstConnection = new FakeLiveConnection();
  const secondConnection = new FakeLiveConnection();
  const firstConnect = new Deferred<FakeLiveConnection>();
  const secondConnect = new Deferred<FakeLiveConnection>();
  let connectCount = 0;

  manager.configurePcRealtimeConnectionManager({
    getClient: () => ({
      connect: () => {
        connectCount += 1;
        return connectCount === 1 ? firstConnect.promise : secondConnect.promise;
      },
    } as never),
    getSession: () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
  });

  manager.subscribePcConversationMessages('conversation-before-session-change', () => undefined);
  assert.equal(connectCount, 1, 'initial subscription must start one websocket connect');

  manager.disposePcLiveConnection('session changed while websocket is connecting');
  manager.subscribePcConversationMessages('conversation-after-session-change', () => undefined);
  await Promise.resolve();

  assert.equal(
    connectCount,
    1,
    'PC realtime manager must not create a replacement websocket before the invalidated in-flight connect is drained',
  );

  firstConnect.resolve(firstConnection);
  await waitForCondition(
    () => connectCount === 2,
    'replacement websocket connect after stale in-flight attempt drains',
  );

  assert.deepEqual(firstConnection.disconnects, [
    {
      code: 1000,
      reason: 'stale PC live connection attempt',
    },
  ]);
  assert.equal(connectCount, 2, 'PC realtime manager must create one replacement websocket after the stale attempt closes');

  const activeConnectionPromise = manager.ensurePcLiveConnection();
  secondConnect.resolve(secondConnection);
  const activeConnection = await activeConnectionPromise;
  assert.equal(activeConnection, secondConnection, 'replacement websocket must become the shared singleton after drain');

  manager.resetPcRealtimeConnectionManagerForTests();
}

async function runBackoffAndCircuitRecoveryContract(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  manager.resetPcRealtimeConnectionManagerForTests();

  const timers = new FakeTimerScheduler();
  const originalSetTimeout = globalThis.setTimeout;
  const originalClearTimeout = globalThis.clearTimeout;
  const originalDateNow = Date.now;
  const originalRandom = Math.random;
  const connections: FakeLiveConnection[] = [];
  let connectCount = 0;

  globalThis.setTimeout = timers.setTimeout.bind(timers) as typeof globalThis.setTimeout;
  globalThis.clearTimeout = timers.clearTimeout.bind(timers) as typeof globalThis.clearTimeout;
  Date.now = () => timers.now;
  Math.random = () => 0.5;

  try {
    manager.configurePcRealtimeConnectionManager({
      getClient: () => ({
        connect: async () => {
          connectCount += 1;
          const connection = new FakeLiveConnection();
          connections.push(connection);
          return connection;
        },
      } as never),
      getSession: () => ({
        accessToken: 'access-token',
        authToken: 'auth-token',
      }),
    });

    const unsubscribe = manager.subscribePcConversationMessages(
      'conversation-backoff',
      () => undefined,
    );
    await flushMicrotasks();
    assert.equal(connectCount, 1);

    connections[0]?.emitState({ status: 'error', reason: 'websocket_upstream_connect' });
    await flushMicrotasks();
    assert.equal(manager.getPcLiveConnectionDiagnostics().consecutiveFailures, 1);
    assert.equal(manager.getPcLiveConnectionDiagnostics().reconnectAttempt, 1);
    assert.equal(timers.nextDelay(), 1_000, 'first failed handshake must retry after base delay');

    connections[0]?.emitOpen();
    assert.equal(
      manager.getPcLiveConnectionStatus(),
      'closed',
      'late open from a stale connection must not replace the current manager state',
    );

    timers.advanceBy(1_000);
    await flushMicrotasks();
    assert.equal(connectCount, 2);
    assert.equal(
      manager.getPcLiveConnectionDiagnostics().consecutiveFailures,
      1,
      'a resolved connection object must not reset failures before lifecycle open',
    );
    assert.equal(manager.getPcLiveConnectionDiagnostics().reconnectAttempt, 1);

    connections[1]?.emitState({ status: 'error', reason: 'websocket_upstream_connect' });
    await flushMicrotasks();
    assert.equal(manager.getPcLiveConnectionDiagnostics().consecutiveFailures, 2);
    assert.equal(manager.getPcLiveConnectionDiagnostics().reconnectAttempt, 2);
    assert.equal(timers.nextDelay(), 2_000, 'second failed handshake must use exponential backoff');

    for (const delay of [2_000, 4_000, 8_000]) {
      timers.advanceBy(delay);
      await flushMicrotasks();
      const connection = connections.at(-1);
      assert.ok(connection, 'reconnect attempt must create a connection');
      connection.emitState({ status: 'error', reason: 'websocket_upstream_connect' });
      await flushMicrotasks();
    }

    const openCircuitDiagnostics = manager.getPcLiveConnectionDiagnostics();
    assert.equal(openCircuitDiagnostics.consecutiveFailures, 5);
    assert.equal(openCircuitDiagnostics.circuitOpen, true);
    assert.equal(
      timers.nextDelay(),
      60_000,
      'open circuit must schedule one automatic retry at cooldown expiry',
    );

    timers.advanceBy(59_999);
    await flushMicrotasks();
    assert.equal(connectCount, 5, 'circuit cooldown must suppress premature reconnect attempts');

    timers.advanceBy(1);
    await flushMicrotasks();
    assert.equal(connectCount, 6, 'circuit cooldown expiry must wake the shared connection manager');
    connections.at(-1)?.emitOpen();
    assert.deepEqual(
      manager.getPcLiveConnectionDiagnostics(),
      {
        status: 'open',
        totalConnectionsCreated: 6,
        hasSharedConnection: true,
        isConnecting: false,
        isDraining: false,
        reconnectAttempt: 0,
        consecutiveFailures: 0,
        circuitOpen: false,
      },
      'only lifecycle open may reset reconnect and circuit-breaker state',
    );

    unsubscribe();
  } finally {
    manager.resetPcRealtimeConnectionManagerForTests();
    globalThis.setTimeout = originalSetTimeout;
    globalThis.clearTimeout = originalClearTimeout;
    Date.now = originalDateNow;
    Math.random = originalRandom;
  }
}

async function runReconnectCancellationContract(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  manager.resetPcRealtimeConnectionManagerForTests();

  const timers = new FakeTimerScheduler();
  const originalSetTimeout = globalThis.setTimeout;
  const originalClearTimeout = globalThis.clearTimeout;
  const originalDateNow = Date.now;
  const originalRandom = Math.random;
  const connections: FakeLiveConnection[] = [];
  let connectCount = 0;

  globalThis.setTimeout = timers.setTimeout.bind(timers) as typeof globalThis.setTimeout;
  globalThis.clearTimeout = timers.clearTimeout.bind(timers) as typeof globalThis.clearTimeout;
  Date.now = () => timers.now;
  Math.random = () => 0.5;

  try {
    manager.configurePcRealtimeConnectionManager({
      getClient: () => ({
        connect: async () => {
          connectCount += 1;
          const connection = new FakeLiveConnection();
          connections.push(connection);
          return connection;
        },
      } as never),
      getSession: () => ({
        accessToken: 'access-token',
        authToken: 'auth-token',
      }),
    });

    const unsubscribe = manager.subscribePcConversationMessages(
      'conversation-cancel-reconnect',
      () => undefined,
    );
    await flushMicrotasks();
    connections[0]?.emitState({ status: 'error', reason: 'websocket_upstream_connect' });
    await flushMicrotasks();
    assert.equal(timers.nextDelay(), 1_000);

    unsubscribe();
    timers.advanceBy(1_000);
    await flushMicrotasks();
    assert.equal(connectCount, 1, 'released demand must cancel pending reconnect timers');
    assert.equal(manager.getPcLiveConnectionStatus(), 'idle');
  } finally {
    manager.resetPcRealtimeConnectionManagerForTests();
    globalThis.setTimeout = originalSetTimeout;
    globalThis.clearTimeout = originalClearTimeout;
    Date.now = originalDateNow;
    Math.random = originalRandom;
  }
}

async function runListenerIsolationContract(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  manager.resetPcRealtimeConnectionManagerForTests();

  const connection = new FakeLiveConnection();
  let openListenerCalls = 0;
  let authenticationListenerCalls = 0;
  let conversationHandlerCalls = 0;
  let wildcardScopeHandlerCalls = 0;
  let filteredScopeHandlerCalls = 0;

  manager.configurePcRealtimeConnectionManager({
    getClient: () => ({
      connect: async () => connection,
    } as never),
    getSession: () => ({
      accessToken: 'access-token',
      authToken: 'auth-token',
    }),
  });

  const releaseFailingOpenListener = manager.onPcLiveConnectionOpen(() => {
    throw new Error('stale open listener');
  });
  const releaseOpenListener = manager.onPcLiveConnectionOpen(() => {
    openListenerCalls += 1;
  });
  const releaseFailingAuthenticationListener = manager.onPcLiveAuthenticationFailure(() => {
    throw new Error('stale authentication listener');
  });
  const releaseAuthenticationListener = manager.onPcLiveAuthenticationFailure(() => {
    authenticationListenerCalls += 1;
  });
  const releaseFailingConversationHandler = manager.subscribePcConversationMessages(
    'conversation-listener-isolation',
    () => {
      throw new Error('stale conversation listener');
    },
  );
  const releaseConversationHandler = manager.subscribePcConversationMessages(
    'conversation-listener-isolation',
    () => {
      conversationHandlerCalls += 1;
    },
  );
  const releaseWildcardScopeHandler = manager.subscribePcRealtimeScope(
    {
      scopeType: 'user',
      scopeId: 'user-listener-isolation',
    },
    () => {
      wildcardScopeHandlerCalls += 1;
      throw new Error('stale scope listener');
    },
  );
  const releaseFilteredScopeHandler = manager.subscribePcRealtimeScope(
    {
      scopeType: 'user',
      scopeId: 'user-listener-isolation',
      eventTypes: ['friend_request.submitted'],
    },
    () => {
      filteredScopeHandlerCalls += 1;
    },
  );

  await flushMicrotasks();
  assert.doesNotThrow(() => connection.emitOpen());
  assert.equal(openListenerCalls, 1, 'one failing open listener must not starve later listeners');

  assert.doesNotThrow(() => connection.emitConversation(
    'conversation-listener-isolation',
    {} as ImDecodedMessage,
    {} as ImMessageContext,
  ));
  assert.equal(
    conversationHandlerCalls,
    1,
    'one failing conversation consumer must not starve later consumers sharing the wire',
  );

  assert.doesNotThrow(() => connection.emitScope(
    'user',
    'user-listener-isolation',
    { eventType: 'friend_request.submitted' } as ImRealtimeEventContext,
  ));
  assert.equal(wildcardScopeHandlerCalls, 1, 'an omitted eventTypes list must remain a wildcard');
  assert.equal(
    filteredScopeHandlerCalls,
    1,
    'one failing scope consumer must not starve later consumers sharing the wire',
  );

  assert.doesNotThrow(() => connection.emitError({ status: 401 }));
  assert.equal(
    authenticationListenerCalls,
    1,
    'one failing authentication observer must not block logout cleanup listeners',
  );
  assert.equal(manager.getPcLiveConnectionStatus(), 'idle');

  releaseFailingOpenListener();
  releaseOpenListener();
  releaseFailingAuthenticationListener();
  releaseAuthenticationListener();
  releaseFailingConversationHandler();
  releaseConversationHandler();
  releaseWildcardScopeHandler();
  releaseFilteredScopeHandler();
  manager.resetPcRealtimeConnectionManagerForTests();
}

async function runChatAgentAssignmentRealtimeContract(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  const { createSdkworkChatService } = await import(
    '../packages/sdkwork-im-pc-chat/src/services/ChatService'
  );
  manager.resetPcRealtimeConnectionManagerForTests();

  const connection = new FakeLiveConnection();
  const conversationId = 'c_agent_assignment_realtime';
  const session = {
    accessToken: 'access-token',
    authToken: 'auth-token',
    user: { id: 'user-self' },
  };
  const client = {
    connect: async () => connection,
    chat: {
      inbox: {
        list: async () => ({
          items: [{
            conversationId,
            conversationType: 'group',
            unreadCount: 0,
            lastActivityAt: '2026-07-12T00:00:00.000Z',
            lastMessageSeq: 0,
          }],
          pageInfo: { hasMore: false },
        }),
      },
    },
  };
  const service = createSdkworkChatService({
    getClient: () => client as never,
    getSession: () => session as never,
  });
  const snapshots: Array<Array<{ id: string; agentAssignments?: Array<{ agentId: string }>; agentAssignmentGeneration?: number }>> = [];
  const unsubscribe = service.subscribeChats((chats) => {
    snapshots.push(chats.map((chat) => ({
      id: chat.id,
      agentAssignments: chat.agentAssignments?.map(({ agentId }) => ({ agentId })),
      agentAssignmentGeneration: chat.agentAssignmentGeneration,
    })));
  });
  const unsubscribeMessages = service.subscribeMessages(conversationId, () => undefined);

  try {
    await flushMicrotasks();
    connection.emitOpen();
    await flushMicrotasks();
    await waitForCondition(
      () => snapshots.some((snapshot) => snapshot.some((chat) => chat.id === conversationId)),
      'initial chat list hydration',
    );

    let ackCount = 0;
    // Assignment relays are emitted on the conversation scope. The chat
    // service must subscribe that scope when it opens a live conversation.
    connection.emitScope('conversation', conversationId, {
      eventId: 'evt_agents_replaced_2',
      eventType: 'conversation.agents_replaced',
      scopeType: 'conversation',
      scopeId: conversationId,
      sequence: 2,
      receivedAt: '2026-07-12T00:00:01.000Z',
      // The conversation outbox relay sends an event envelope with the
      // business payload nested under `payload`; the PC adapter must also
      // accept this production wire shape (direct publishers send the body
      // without the envelope).
      payload: {
        eventId: 'conversation:conversation.agents_replaced:evt_agents_replaced_2',
        eventType: 'conversation.agents_replaced',
        aggregateId: conversationId,
        payload: {
          conversationId,
          previousGeneration: 1,
          agentAssignments: {
            generation: 2,
            source: 'conversation_override',
            agents: [
              { agentId: 'agent.writer', revisionId: 'revision.writer.v2' },
              { agentId: 'agent.reviewer', revisionId: 'revision.reviewer.v1' },
            ],
          },
          replacedAt: '2026-07-12T00:00:01.000Z',
        },
      },
      ack: async () => {
        ackCount += 1;
      },
    });
    await flushMicrotasks();

    const immediate = snapshots.at(-1)?.find((chat) => chat.id === conversationId);
    assert.deepEqual(
      immediate,
      {
        id: conversationId,
        agentAssignments: [
          { agentId: 'agent.writer' },
          { agentId: 'agent.reviewer' },
        ],
        agentAssignmentGeneration: 2,
      },
      'agent replacement events must update the local chat list before the inbox refresh completes',
    );
    assert.equal(ackCount, 1, 'agent replacement events must be acknowledged exactly once');

    connection.emitScope('conversation', conversationId, {
      eventId: 'evt_agents_replaced_conflict',
      eventType: 'conversation.agents_replaced',
      scopeType: 'conversation',
      scopeId: conversationId,
      sequence: 3,
      receivedAt: '2026-07-12T00:00:02.000Z',
      payload: {
        conversationId,
        previousGeneration: 1,
        agentAssignments: {
          generation: 2,
          source: 'conversation_override',
          agents: [{ agentId: 'agent.conflicting', revisionId: 'revision.conflicting.v1' }],
        },
        replacedAt: '2026-07-12T00:00:02.000Z',
      },
      ack: async () => {
        ackCount += 1;
      },
    });
    await flushMicrotasks();
    assert.deepEqual(
      snapshots.at(-1)?.find((chat) => chat.id === conversationId)?.agentAssignments?.map(({ agentId }) => agentId),
      ['agent.writer', 'agent.reviewer'],
      'a same-generation conflicting assignment snapshot must not overwrite local CAS state',
    );
    assert.equal(ackCount, 2, 'conflicting duplicate assignment events must still be acknowledged');

    const refreshed = await service.listChatsPage({ cursor: 'after-agent-event' });
    assert.deepEqual(
      refreshed.items[0]?.agentAssignments?.map(({ agentId }) => agentId),
      ['agent.writer', 'agent.reviewer'],
      'inbox hydration must retain the authoritative assignment order after a realtime refresh',
    );
    assert.equal(
      refreshed.items[0]?.agentAssignmentGeneration,
      2,
      'inbox hydration must retain the assignment generation after a realtime refresh',
    );
  } finally {
    unsubscribeMessages();
    unsubscribe();
    manager.resetPcRealtimeConnectionManagerForTests();
  }
}

async function runCallServiceStateContracts(): Promise<void> {
  const manager = await import('../packages/sdkwork-im-pc-core/src/sdk/pcRealtimeConnectionManager');
  const { createSdkworkCallService } = await import(
    '../packages/sdkwork-im-pc-chat/src/services/CallService'
  );
  manager.resetPcRealtimeConnectionManagerForTests();

  const connection = new FakeLiveConnection();
  const credentialDeferred = new Deferred<ImCallParticipantCredential>();
  const muteAudioDeferred = new Deferred<void>();
  let startDeferred: Deferred<ImCallSession> | undefined;
  let callListener: ((session: ImCallSession) => void) | undefined;
  let credentialRequestCount = 0;
  let inviteCount = 0;
  let joinCount = 0;
  let publishCount = 0;
  const endedSessionIds: string[] = [];

  const client = {
    connect: async () => connection,
    calls: {
      accept: async (rtcSessionId: string) => createCallSession(rtcSessionId, 'accepted'),
      end: async (rtcSessionId: string) => {
        endedSessionIds.push(rtcSessionId);
        return createCallSession(rtcSessionId, 'ended');
      },
      invite: async (rtcSessionId: string) => {
        inviteCount += 1;
        return createCallSession(rtcSessionId, 'started');
      },
      issueParticipantCredential: async (rtcSessionId: string) => {
        credentialRequestCount += 1;
        return credentialDeferred.promise.then((credential) => ({
          ...credential,
          rtcSessionId,
        }));
      },
      reject: async (rtcSessionId: string) => createCallSession(rtcSessionId, 'rejected'),
      retrieve: async (rtcSessionId: string) => createCallSession(rtcSessionId, 'started'),
      start: async () => {
        if (!startDeferred) {
          throw new Error('start deferred was not configured');
        }
        return startDeferred.promise;
      },
      subscribe: (listener: (session: ImCallSession) => void) => {
        callListener = listener;
        return () => {
          if (callListener === listener) {
            callListener = undefined;
          }
        };
      },
      watchIncoming: async () => null,
    },
  };
  const mediaService = {
    bindLocalVideoElement: async () => undefined,
    bindRemoteVideoElement: async () => undefined,
    join: async () => {
      joinCount += 1;
    },
    leave: async () => undefined,
    muteAudio: async () => muteAudioDeferred.promise,
    muteVideo: async () => undefined,
    publish: async () => {
      publishCount += 1;
    },
  };
  const session = {
    accessToken: 'access-token',
    authToken: 'auth-token',
    context: {
      userId: 'user-self',
    },
  };
  const service = createSdkworkCallService({
    getClient: () => client as never,
    readSession: () => session as never,
    rtcMediaService: mediaService,
  });

  try {
    const observedStates: string[] = [];
    const releaseFailingInitialObserver = service.subscribe(() => {
      throw new Error('initial observer failure must stay isolated');
    });
    releaseFailingInitialObserver();
    service.subscribe((snapshot) => {
      if (snapshot.state === 'connected') {
        throw new Error('observer failure must stay isolated');
      }
    });
    service.subscribe((snapshot) => {
      observedStates.push(snapshot.state);
    });

    await service.watchIncomingCalls(['conversation-call-state']);
    connection.emitOpen();
    assert.ok(callListener, 'incoming call watch must install one call session listener');

    callListener(createCallSession('rtc-call-a', 'started'));
    assert.equal(service.getSnapshot().state, 'ringing');
    callListener(createCallSession('rtc-call-a', 'accepted'));
    callListener(createCallSession('rtc-call-a', 'accepted'));
    await flushMicrotasks();
    assert.equal(service.getSnapshot().state, 'connected');
    assert.equal(
      credentialRequestCount,
      1,
      'duplicate connected events must share one participant credential request',
    );
    assert.equal(
      observedStates.includes('connected'),
      true,
      'a throwing observer must not starve later call-state observers',
    );

    credentialDeferred.resolve({
      tenantId: '100001',
      rtcSessionId: 'rtc-call-a',
      participantId: 'user-self',
      credential: 'credential-a',
      expiresAt: '2026-07-12T00:10:00.000Z',
    });
    await flushMicrotasks();
    assert.equal(joinCount, 1, 'credential single-flight must start RTC media exactly once');
    assert.equal(publishCount, 1, 'credential single-flight must publish local media exactly once');

    callListener(createCallSession('rtc-call-a', 'started'));
    assert.equal(
      service.getSnapshot().state,
      'connected',
      'late ringing state must not regress an already connected call',
    );

    const mutePromise = service.setAudioMuted(true);
    callListener(createCallSession('rtc-call-a', 'ended'));
    muteAudioDeferred.reject(new Error('audio device failed'));
    await assert.rejects(mutePromise, /audio device failed/u);
    assert.equal(
      service.getSnapshot().state,
      'ended',
      'mute rollback must not restore a snapshot superseded by a terminal call event',
    );

    callListener(createCallSession('rtc-call-a', 'accepted'));
    await flushMicrotasks();
    assert.equal(
      service.getSnapshot().state,
      'ended',
      'late accepted state must not resurrect a terminal call',
    );
    assert.equal(credentialRequestCount, 1, 'terminal calls must not request fresh RTC credentials');

    startDeferred = new Deferred<ImCallSession>();
    const outgoingPromise = service.startOutgoingCall({
      conversationId: 'conversation-call-state',
      targetName: 'Peer',
      targetUserId: 'user-peer',
      type: 'video',
    });
    const staleOutgoingSessionId = service.getSnapshot().rtcSessionId;
    assert.ok(staleOutgoingSessionId, 'outgoing call must allocate a runtime session id');
    callListener(createCallSession(staleOutgoingSessionId, 'ended'));
    callListener(createCallSession('rtc-call-b', 'started'));
    startDeferred.resolve(createCallSession(staleOutgoingSessionId, 'started'));
    await outgoingPromise;

    assert.equal(service.getSnapshot().rtcSessionId, 'rtc-call-b');
    assert.equal(service.getSnapshot().state, 'ringing');
    assert.equal(inviteCount, 0, 'stale outgoing create response must not continue to invite');
    assert.equal(
      endedSessionIds.includes(staleOutgoingSessionId),
      true,
      'a superseded outgoing session must be closed best-effort on the signaling service',
    );
  } finally {
    manager.resetPcRealtimeConnectionManagerForTests();
  }
}

async function runContactServiceContracts(): Promise<void> {
  const { createSdkworkContactService } = await import(
    '../packages/sdkwork-im-pc-chat/src/services/ContactService'
  );
  const listPageSizes: number[] = [];
  const operations: string[] = [];
  const client = {
    social: {
      contacts: {
        list: async ({ pageSize }: { pageSize: number }) => {
          listPageSizes.push(pageSize);
          return {
            items: [],
            pageInfo: {
              hasMore: false,
            },
          };
        },
        preferences: {
          update: async (userId: string, update: { isBlocked?: boolean }) => {
            operations.push(`preference:${userId}:${String(update.isBlocked)}`);
            return {
              tenantId: '100001',
              ownerUserId: 'user-self',
              targetUserId: userId,
              isStarred: false,
              remark: '',
              isBlocked: update.isBlocked ?? false,
              updatedAt: '2026-07-12T00:00:00.000Z',
            };
          },
        },
      },
      userBlocks: {
        create: async () => {
          throw new Error('contact blacklist changes must use the coordinated preferences API');
        },
        delete: async () => {
          throw new Error('contact blacklist changes must use the coordinated preferences API');
        },
      },
    },
  };

  const service = createSdkworkContactService(() => client as never);
  await service.listContactsPage({ pageSize: -10 });
  await service.listContactsPage({ pageSize: 3.9 });
  await service.listContactsPage({ pageSize: 10_000 });
  assert.deepEqual(
    listPageSizes,
    [20, 3, 200],
    'contact pagination must apply the SDKWork default, integer normalization, and maximum',
  );

  await service.addToBlacklist('user-peer');
  await service.removeFromBlacklist('user-peer');
  assert.deepEqual(
    operations,
    [
      'preference:user-peer:true',
      'preference:user-peer:false',
    ],
    'blacklist changes must use the server API that coordinates contact preferences and user blocks',
  );

  operations.length = 0;
  const restartedService = createSdkworkContactService(() => client as never);
  await restartedService.removeFromBlacklist('user-after-restart');
  assert.deepEqual(
    operations,
    [
      'preference:user-after-restart:false',
    ],
    'unblocking after restart must not depend on an in-memory block id',
  );

  const blacklistUpdates: Array<Deferred<Record<string, unknown>>> = [];
  const raceClient = {
    social: {
      contacts: {
        preferences: {
          update: async () => {
            const deferred = new Deferred<Record<string, unknown>>();
            blacklistUpdates.push(deferred);
            return deferred.promise;
          },
        },
      },
    },
  };
  const raceService = createSdkworkContactService(() => raceClient as never);
  const raceServiceInternals = raceService as unknown as {
    handleAuthSessionChanged(event?: Event): void;
  };
  const switchContactSession = (userId: string): void => {
    raceServiceInternals.handleAuthSessionChanged({
      detail: {
        session: {
          user: { id: userId },
        },
      },
    } as unknown as Event);
  };
  const resolveBlacklistUpdate = (index: number, userId: string): void => {
    blacklistUpdates[index]?.resolve({
      tenantId: '100001',
      ownerUserId: 'user-self',
      targetUserId: userId,
      isStarred: false,
      remark: '',
      isBlocked: true,
      updatedAt: '2026-07-12T00:00:00.000Z',
    });
  };

  switchContactSession('account-a');
  const staleBlacklistRequest = raceService.addToBlacklist('shared-user');
  const staleBlacklistOutcome = staleBlacklistRequest.then(
    () => undefined,
    (error: unknown) => error,
  );
  await flushMicrotasks();
  assert.equal(blacklistUpdates.length, 1);

  switchContactSession('account-b');
  const activeBlacklistRequest = raceService.addToBlacklist('shared-user');
  await flushMicrotasks();
  assert.equal(blacklistUpdates.length, 2);

  resolveBlacklistUpdate(0, 'shared-user');
  const staleBlacklistError = await staleBlacklistOutcome;
  assert.match(
    String(staleBlacklistError),
    /Contact session changed/u,
    'the stale account mutation must fail before updating the new account cache',
  );

  await raceService.addToBlacklist('shared-user');
  assert.equal(
    blacklistUpdates.length,
    2,
    'stale request cleanup must not remove the new account in-flight marker',
  );
  resolveBlacklistUpdate(1, 'shared-user');
  await activeBlacklistRequest;
}

await runSingleFlightRecoveryContract();
await runInvalidateDuringConnectContract();
await runBackoffAndCircuitRecoveryContract();
await runReconnectCancellationContract();
await runListenerIsolationContract();
await runChatAgentAssignmentRealtimeContract();
await runCallServiceStateContracts();
await runContactServiceContracts();

console.log('sdkwork im pc realtime connection contract passed.');
