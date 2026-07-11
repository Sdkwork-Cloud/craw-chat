import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import type {
  ImDecodedMessage,
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

class FakeLiveConnection implements ImLiveConnection {
  readonly disconnects: Array<{ code?: number; reason?: string }> = [];
  readonly stateHandlers = new Set<StateHandler>();
  readonly errorHandlers = new Set<ErrorHandler>();
  readonly syncedConversations: string[][] = [];

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

  events = {
    onConversation: (
      _conversationId: string,
      _handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void,
    ) => () => undefined,
    onScope: (
      _scopeType: string,
      _scopeId: string,
      _handler: (event: Record<string, unknown>, context: ImRealtimeEventContext) => void,
    ) => () => undefined,
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
      _conversationId: string,
      _handler: (message: ImDecodedMessage, context: ImMessageContext) => void,
    ) => () => undefined,
  };

  subscriptions = {
    syncConversations: (conversationIds: string[]) => {
      this.syncedConversations.push([...conversationIds]);
    },
    syncScopes: () => undefined,
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

await runSingleFlightRecoveryContract();
await runInvalidateDuringConnectContract();

console.log('sdkwork im pc realtime connection contract passed.');
