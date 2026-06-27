import type {
  ImDecodedMessage,
  ImLiveConnection,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
  ImSdkClient,
  ImSubscription,
} from '@sdkwork/im-sdk';
import {
  isAppSdkSessionAuthenticated,
  readAppSdkSessionTokens,
  SDKWORK_IM_SESSION_CHANGED_EVENT,
  type SdkworkChatSession,
} from './session';
import { getImSdkClientWithSession } from './imSdkClient';

export type PcLiveConnectionStatus = 'idle' | 'connecting' | 'open' | 'closed' | 'error';

export interface PcRealtimeConnectionManagerConfig {
  getClient?: () => ImSdkClient;
  getDeviceId?: () => string | undefined;
  getSession?: () => SdkworkChatSession | null;
}

type ConversationMessageHandler = (message: ImDecodedMessage, context: ImMessageContext) => void;
type ScopeEventHandler = (context: ImRealtimeEventContext) => void;
type OpenListener = (connection: ImLiveConnection) => void;
type AuthenticationFailureListener = (reason: string) => void;

interface ConversationRegistration {
  handlers: Set<ConversationMessageHandler>;
}

interface ScopeListenerRegistration {
  eventTypes: readonly string[];
  handler: ScopeEventHandler;
}

interface ScopeRegistration {
  listeners: Set<ScopeListenerRegistration>;
  scopeId: string;
  scopeType: string;
}

const RECONNECT_BASE_DELAY_MS = 1000;
const RECONNECT_MAX_DELAY_MS = 30000;
const RECONNECT_JITTER_RATIO = 0.2;
const CIRCUIT_BREAKER_FAILURE_THRESHOLD = 5;
const CIRCUIT_BREAKER_COOLDOWN_MS = 60_000;

let managerConfig: PcRealtimeConnectionManagerConfig = {};
let sharedConnection: ImLiveConnection | null = null;
let sharedConnectionPromise: Promise<ImLiveConnection> | null = null;
let connectionStatus: PcLiveConnectionStatus = 'idle';
let connectionGeneration = 0;
let reconnectAttempt = 0;
let reconnectTimer: ReturnType<typeof setTimeout> | undefined;
let consecutiveFailures = 0;
let circuitOpenUntil = 0;
let lifecycleUnsub: ImSubscription | undefined;
let errorUnsub: ImSubscription | undefined;
let browserHooksInstalled = false;

const conversationRegistrations = new Map<string, ConversationRegistration>();
const scopeRegistrations = new Map<string, ScopeRegistration>();
const connectionLeases = new Set<string>();
const leasedConversationIds = new Map<string, ReadonlySet<string>>();
const conversationUnsubs = new Map<string, ImSubscription>();
const scopeUnsubs = new Map<string, ImSubscription>();
const openListeners = new Set<OpenListener>();
const authenticationFailureListeners = new Set<AuthenticationFailureListener>();

function resolveClient(): ImSdkClient {
  return managerConfig.getClient?.() ?? getImSdkClientWithSession();
}

function resolveSession(): SdkworkChatSession | null {
  return managerConfig.getSession?.() ?? readAppSdkSessionTokens();
}

function resolveDeviceId(): string | undefined {
  return managerConfig.getDeviceId?.();
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function isAuthenticationFailure(error: unknown): boolean {
  const record = toRecord(error);
  const code = pickString(record.code);
  const message = pickString(record.message, record.error, record.reason);
  const status = Number(record.status ?? record.statusCode ?? record.httpStatus ?? record.http_status);
  return status === 401
    || Boolean(code && /(?:auth|session|token|unauthori(?:s|z)ed)/iu.test(code))
    || Boolean(message && /(?:auth|session|token).*(?:failed|expired|invalid|required)|unauthori(?:s|z)ed/iu.test(message));
}

function isFatalLiveConnectionError(error: unknown): boolean {
  const code = pickString(toRecord(error).code);
  return Boolean(code && (
    /^websocket_(?:auth|upstream|connect|heartbeat)/u.test(code)
    || /(?:auth|session|token).*(?:failed|expired|invalid|required)/iu.test(code)
  ));
}

function computeReconnectDelay(attempt: number): number {
  const normalizedAttempt = Math.max(1, attempt);
  const exponentialDelay = Math.min(
    RECONNECT_MAX_DELAY_MS,
    RECONNECT_BASE_DELAY_MS * (2 ** (normalizedAttempt - 1)),
  );
  const jitterSpan = exponentialDelay * RECONNECT_JITTER_RATIO;
  return Math.round(exponentialDelay - jitterSpan + Math.random() * jitterSpan * 2);
}

function scopeRegistryKey(scope: Pick<ImRealtimeScopeSubscription, 'scopeId' | 'scopeType'>): string {
  return `${scope.scopeType}:${scope.scopeId}`;
}

export interface PcLiveConnectionLeaseOptions {
  conversationIds?: readonly string[];
}

function collectActiveConversationIds(): string[] {
  const conversationIds = new Set(conversationRegistrations.keys());
  for (const conversationIdSet of leasedConversationIds.values()) {
    for (const conversationId of conversationIdSet) {
      conversationIds.add(conversationId);
    }
  }
  return [...conversationIds];
}

function hasSubscriptionDemand(): boolean {
  return conversationRegistrations.size > 0
    || scopeRegistrations.size > 0
    || connectionLeases.size > 0;
}

function notifyAuthenticationFailure(reason: string): void {
  for (const listener of authenticationFailureListeners) {
    listener(reason);
  }
}

function notifyConnectionOpen(connection: ImLiveConnection): void {
  for (const listener of openListeners) {
    listener(connection);
  }
}

function clearReconnectTimer(): void {
  if (reconnectTimer) {
    clearTimeout(reconnectTimer);
    reconnectTimer = undefined;
  }
}

function detachConnectionListeners(): void {
  lifecycleUnsub?.();
  errorUnsub?.();
  lifecycleUnsub = undefined;
  errorUnsub = undefined;
}

function resetConnectionState(): void {
  detachConnectionListeners();
  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'idle';
}

function clearWireSubscriptions(): void {
  for (const unsubscribe of conversationUnsubs.values()) {
    unsubscribe();
  }
  conversationUnsubs.clear();
  for (const unsubscribe of scopeUnsubs.values()) {
    unsubscribe();
  }
  scopeUnsubs.clear();
}

function buildMergedScopeSubscriptions(): ImRealtimeScopeSubscription[] {
  const merged = new Map<string, ImRealtimeScopeSubscription>();
  for (const registration of scopeRegistrations.values()) {
    const key = scopeRegistryKey(registration);
    const eventTypes = new Set<string>();
    for (const listener of registration.listeners) {
      for (const eventType of listener.eventTypes) {
        eventTypes.add(eventType);
      }
    }
    merged.set(key, {
      scopeType: registration.scopeType,
      scopeId: registration.scopeId,
      eventTypes: [...eventTypes],
    });
  }
  return [...merged.values()];
}

function syncWireSubscriptions(connection: ImLiveConnection): void {
  const conversationIds = collectActiveConversationIds();
  for (const [conversationId, unsubscribe] of [...conversationUnsubs.entries()]) {
    if (conversationRegistrations.has(conversationId)) {
      continue;
    }
    unsubscribe();
    conversationUnsubs.delete(conversationId);
  }
  for (const conversationId of conversationIds) {
    if (conversationUnsubs.has(conversationId)) {
      continue;
    }
    conversationUnsubs.set(
      conversationId,
      connection.messages.onConversation(conversationId, (message, context) => {
        const registration = conversationRegistrations.get(conversationId);
        if (!registration) {
          return;
        }
        for (const handler of registration.handlers) {
          handler(message, context);
        }
      }),
    );
  }

  const mergedScopes = buildMergedScopeSubscriptions();
  const mergedScopeKeys = new Set(mergedScopes.map((scope) => scopeRegistryKey(scope)));
  for (const [scopeKey, unsubscribe] of [...scopeUnsubs.entries()]) {
    if (mergedScopeKeys.has(scopeKey)) {
      continue;
    }
    unsubscribe();
    scopeUnsubs.delete(scopeKey);
  }
  for (const scope of mergedScopes) {
    const scopeKey = scopeRegistryKey(scope);
    if (scopeUnsubs.has(scopeKey)) {
      continue;
    }
    scopeUnsubs.set(
      scopeKey,
      connection.events.onScope(scope.scopeType, scope.scopeId, (_event, context) => {
        const registration = scopeRegistrations.get(scopeKey);
        if (!registration) {
          return;
        }
        const eventType = context.eventType;
        for (const listener of registration.listeners) {
          if (eventType && !listener.eventTypes.includes(eventType)) {
            continue;
          }
          listener.handler(context);
        }
      }),
    );
  }

  connection.subscriptions.syncConversations(conversationIds);
  connection.subscriptions.syncScopes(mergedScopes);
}

function syncWireSubscriptionsWhenReady(connection: ImLiveConnection): void {
  if (connectionStatus !== 'open') {
    return;
  }
  syncWireSubscriptions(connection);
}

function teardownIfIdle(reason = 'no live subscriptions'): void {
  if (hasSubscriptionDemand()) {
    return;
  }
  clearReconnectTimer();
  clearWireSubscriptions();
  sharedConnection?.disconnect(1000, reason);
  resetConnectionState();
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;
}

function handleConnectionLost(triggerReconnect: boolean): void {
  detachConnectionListeners();
  clearWireSubscriptions();
  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'closed';

  if (!triggerReconnect || !hasSubscriptionDemand()) {
    teardownIfIdle('live connection closed');
    return;
  }
  if (!isAppSdkSessionAuthenticated(resolveSession())) {
    return;
  }
  scheduleReconnect();
}

function scheduleReconnect(): void {
  if (
    reconnectTimer
    || !hasSubscriptionDemand()
    || !isAppSdkSessionAuthenticated(resolveSession())
  ) {
    return;
  }
  if (Date.now() < circuitOpenUntil) {
    return;
  }

  reconnectAttempt += 1;
  reconnectTimer = setTimeout(() => {
    reconnectTimer = undefined;
    void ensurePcLiveConnection().catch(() => undefined);
  }, computeReconnectDelay(reconnectAttempt));
}

function bindConnection(connection: ImLiveConnection, generation: number): void {
  detachConnectionListeners();
  sharedConnection = connection;
  connectionStatus = 'connecting';
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;

  lifecycleUnsub = connection.lifecycle.onStateChange((state) => {
    if (generation !== connectionGeneration) {
      return;
    }
    if (state.status === 'open') {
      connectionStatus = 'open';
      syncWireSubscriptions(connection);
      notifyConnectionOpen(connection);
      return;
    }
    if (state.status === 'connecting') {
      connectionStatus = 'connecting';
      return;
    }
    if (state.status === 'error') {
      connectionStatus = 'error';
      if (state.reason && isAuthenticationFailure({ message: state.reason })) {
        notifyAuthenticationFailure(state.reason);
        disposePcLiveConnection('websocket authentication failed');
        return;
      }
      handleConnectionLost(true);
      return;
    }
    if (state.status === 'closed') {
      handleConnectionLost(true);
    }
  });

  errorUnsub = connection.lifecycle.onError((error) => {
    if (generation !== connectionGeneration) {
      return;
    }
    if (isAuthenticationFailure(error)) {
      notifyAuthenticationFailure('websocket authentication failed');
      disposePcLiveConnection('websocket authentication failed');
      return;
    }
    if (isFatalLiveConnectionError(error)) {
      consecutiveFailures += 1;
      if (consecutiveFailures >= CIRCUIT_BREAKER_FAILURE_THRESHOLD) {
        circuitOpenUntil = Date.now() + CIRCUIT_BREAKER_COOLDOWN_MS;
      }
      handleConnectionLost(true);
    }
  });
}

async function connectSharedLiveConnection(): Promise<ImLiveConnection> {
  if (!isAppSdkSessionAuthenticated(resolveSession())) {
    throw new Error('PC live connection requires an authenticated session');
  }
  if (!hasSubscriptionDemand()) {
    throw new Error('PC live connection requires at least one subscription');
  }

  const generation = connectionGeneration + 1;
  connectionGeneration = generation;
  connectionStatus = 'connecting';

  const deviceId = resolveDeviceId();
  const connection = await resolveClient().connect({
    ...(deviceId ? { deviceId } : {}),
    subscriptions: {
      conversations: [],
      scopes: [],
    },
  });

  if (generation !== connectionGeneration) {
    connection.disconnect(1000, 'stale PC live connection attempt');
    throw new Error('PC live connection attempt superseded');
  }
  if (!hasSubscriptionDemand()) {
    connection.disconnect(1000, 'PC live subscriptions removed during connect');
    throw new Error('PC live subscriptions removed during connect');
  }

  bindConnection(connection, generation);
  return connection;
}

function installBrowserRecoveryHooks(): void {
  if (browserHooksInstalled || typeof window === 'undefined') {
    return;
  }
  browserHooksInstalled = true;

  window.addEventListener('online', () => {
    recoverPcLiveConnection('browser online');
  });
  window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
    invalidatePcLiveConnection('auth session changed');
  });
  if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') {
        recoverPcLiveConnection('browser visible');
      }
    });
  }
}

export function configurePcRealtimeConnectionManager(
  config: PcRealtimeConnectionManagerConfig,
): void {
  managerConfig = {
    ...managerConfig,
    ...config,
  };
  installBrowserRecoveryHooks();
}

export function getPcLiveConnectionStatus(): PcLiveConnectionStatus {
  return connectionStatus;
}

export function getPcLiveConnectionIfReady(): ImLiveConnection | null {
  return sharedConnection;
}

export async function ensurePcLiveConnection(): Promise<ImLiveConnection> {
  installBrowserRecoveryHooks();
  if (!hasSubscriptionDemand()) {
    throw new Error('PC live connection requires at least one subscription');
  }
  if (!isAppSdkSessionAuthenticated(resolveSession())) {
    throw new Error('PC live connection requires an authenticated session');
  }
  if (sharedConnection) {
    return sharedConnection;
  }
  if (sharedConnectionPromise) {
    return sharedConnectionPromise;
  }

  sharedConnectionPromise = connectSharedLiveConnection()
    .then((connection) => {
      sharedConnectionPromise = null;
      return connection;
    })
    .catch((error: unknown) => {
      sharedConnectionPromise = null;
      connectionStatus = 'error';
      consecutiveFailures += 1;
      if (consecutiveFailures >= CIRCUIT_BREAKER_FAILURE_THRESHOLD) {
        circuitOpenUntil = Date.now() + CIRCUIT_BREAKER_COOLDOWN_MS;
      }
      if (
        hasSubscriptionDemand()
        && isAppSdkSessionAuthenticated(resolveSession())
        && !isAuthenticationFailure(error)
      ) {
        scheduleReconnect();
      }
      throw error;
    });

  return sharedConnectionPromise;
}

export function recoverPcLiveConnection(reason = 'realtime recovery requested'): void {
  if (!hasSubscriptionDemand() || !isAppSdkSessionAuthenticated(resolveSession())) {
    return;
  }
  if (connectionStatus === 'open' || connectionStatus === 'connecting') {
    return;
  }

  clearReconnectTimer();
  if (sharedConnection) {
    const staleConnection = sharedConnection;
    resetConnectionState();
    staleConnection.disconnect(1000, reason);
  } else {
    resetConnectionState();
  }

  void ensurePcLiveConnection().catch(() => undefined);
}

function invalidatePcLiveConnection(reason: string): void {
  clearReconnectTimer();
  connectionGeneration += 1;
  clearWireSubscriptions();
  sharedConnection?.disconnect(1000, reason);
  resetConnectionState();
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;
}

export function disposePcLiveConnection(reason = 'session ended'): void {
  invalidatePcLiveConnection(reason);
  conversationRegistrations.clear();
  scopeRegistrations.clear();
  connectionLeases.clear();
  leasedConversationIds.clear();
}

export function acquirePcLiveConnectionLease(
  leaseKey: string,
  options: PcLiveConnectionLeaseOptions = {},
): () => void {
  installBrowserRecoveryHooks();
  connectionLeases.add(leaseKey);
  if (options.conversationIds && options.conversationIds.length > 0) {
    leasedConversationIds.set(leaseKey, new Set(options.conversationIds));
  } else {
    leasedConversationIds.delete(leaseKey);
  }
  void ensurePcLiveConnection()
    .then((connection) => {
      syncWireSubscriptionsWhenReady(connection);
    })
    .catch(() => undefined);
  return () => {
    connectionLeases.delete(leaseKey);
    leasedConversationIds.delete(leaseKey);
    if (sharedConnection) {
      syncWireSubscriptionsWhenReady(sharedConnection);
    }
    teardownIfIdle();
  };
}

export function onPcLiveConnectionOpen(listener: OpenListener): () => void {
  openListeners.add(listener);
  return () => {
    openListeners.delete(listener);
  };
}

export function onPcLiveAuthenticationFailure(listener: AuthenticationFailureListener): () => void {
  authenticationFailureListeners.add(listener);
  return () => {
    authenticationFailureListeners.delete(listener);
  };
}

export function subscribePcConversationMessages(
  conversationId: string,
  handler: ConversationMessageHandler,
): () => void {
  installBrowserRecoveryHooks();
  let registration = conversationRegistrations.get(conversationId);
  if (!registration) {
    registration = { handlers: new Set() };
    conversationRegistrations.set(conversationId, registration);
  }
  registration.handlers.add(handler);
  void ensurePcLiveConnection()
    .then((connection) => {
      syncWireSubscriptionsWhenReady(connection);
    })
    .catch(() => undefined);

  return () => {
    const activeRegistration = conversationRegistrations.get(conversationId);
    if (!activeRegistration) {
      return;
    }
    activeRegistration.handlers.delete(handler);
    if (activeRegistration.handlers.size > 0) {
      return;
    }
    conversationRegistrations.delete(conversationId);
    const unsubscribe = conversationUnsubs.get(conversationId);
    unsubscribe?.();
    conversationUnsubs.delete(conversationId);
    if (sharedConnection) {
      syncWireSubscriptionsWhenReady(sharedConnection);
    }
    teardownIfIdle();
  };
}

export function subscribePcRealtimeScope(
  scope: ImRealtimeScopeSubscription,
  handler: ScopeEventHandler,
): () => void {
  installBrowserRecoveryHooks();
  const scopeKey = scopeRegistryKey(scope);
  let registration = scopeRegistrations.get(scopeKey);
  if (!registration) {
    registration = {
      listeners: new Set(),
      scopeId: scope.scopeId,
      scopeType: scope.scopeType,
    };
    scopeRegistrations.set(scopeKey, registration);
  }
  const listenerRegistration: ScopeListenerRegistration = {
    eventTypes: scope.eventTypes,
    handler,
  };
  registration.listeners.add(listenerRegistration);
  void ensurePcLiveConnection()
    .then((connection) => {
      syncWireSubscriptionsWhenReady(connection);
    })
    .catch(() => undefined);

  return () => {
    const activeRegistration = scopeRegistrations.get(scopeKey);
    if (!activeRegistration) {
      return;
    }
    activeRegistration.listeners.delete(listenerRegistration);
    if (activeRegistration.listeners.size > 0) {
      if (sharedConnection) {
        syncWireSubscriptionsWhenReady(sharedConnection);
      }
      return;
    }
    scopeRegistrations.delete(scopeKey);
    const unsubscribe = scopeUnsubs.get(scopeKey);
    unsubscribe?.();
    scopeUnsubs.delete(scopeKey);
    if (sharedConnection) {
      syncWireSubscriptionsWhenReady(sharedConnection);
    }
    teardownIfIdle();
  };
}

export function resetPcRealtimeConnectionManagerForTests(): void {
  disposePcLiveConnection('test reset');
  managerConfig = {};
  browserHooksInstalled = false;
  openListeners.clear();
  authenticationFailureListeners.clear();
  connectionLeases.clear();
  leasedConversationIds.clear();
}
