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

export type PcLiveConnectionStatus = 'idle' | 'connecting' | 'open' | 'closed' | 'error';

export interface PcRealtimeConnectionManagerConfig {
  getClient?: () => Promise<ImSdkClient> | ImSdkClient;
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
let connectionDrainPromise: Promise<void> | null = null;
let connectionStatus: PcLiveConnectionStatus = 'idle';
let connectionGeneration = 0;
let reconnectAttempt = 0;
let reconnectTimer: ReturnType<typeof setTimeout> | undefined;
let consecutiveFailures = 0;
let circuitOpenUntil = 0;
let totalConnectionsCreated = 0;
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

function isPromiseLike<T>(value: Promise<T> | T): value is Promise<T> {
  return typeof (value as Promise<T>).then === 'function';
}

function resolveClient(): Promise<ImSdkClient> | ImSdkClient {
  if (managerConfig.getClient) {
    return managerConfig.getClient();
  }
  return import('./imSdkClient').then(({ getImSdkClientWithSession }) => getImSdkClientWithSession());
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

export interface PcLiveConnectionRecoveryOptions {
  force?: boolean;
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
    try {
      listener(reason);
    } catch {
      // Authentication cleanup must continue even when a UI observer fails.
    }
  }
}

function notifyConnectionOpen(connection: ImLiveConnection): void {
  for (const listener of openListeners) {
    try {
      listener(connection);
    } catch {
      // One observer must not block subscription synchronization or recovery.
    }
  }
}

function clearReconnectTimer(): void {
  if (reconnectTimer) {
    clearTimeout(reconnectTimer);
    reconnectTimer = undefined;
  }
}

function recordConnectionFailure(): void {
  consecutiveFailures += 1;
  if (consecutiveFailures >= CIRCUIT_BREAKER_FAILURE_THRESHOLD) {
    circuitOpenUntil = Math.max(
      circuitOpenUntil,
      Date.now() + CIRCUIT_BREAKER_COOLDOWN_MS,
    );
  }
}

function detachConnectionListeners(): void {
  lifecycleUnsub?.();
  errorUnsub?.();
  lifecycleUnsub = undefined;
  errorUnsub = undefined;
}

function queueInFlightConnectionDrain(reason: string): void {
  const inFlightConnection = sharedConnectionPromise;
  if (!inFlightConnection) {
    return;
  }

  const previousDrain = connectionDrainPromise;
  const drain = async (): Promise<void> => {
    try {
      const connection = await inFlightConnection;
      connection.disconnect(1000, reason);
    } catch {
      // Superseded connection attempts already close themselves before rejecting.
    }
  };
  const queuedDrain = (previousDrain ? previousDrain.catch(() => undefined) : Promise.resolve())
    .then(drain);
  let finalDrain: Promise<void>;
  finalDrain = queuedDrain.finally(() => {
    if (connectionDrainPromise === finalDrain) {
      connectionDrainPromise = null;
    }
  });
  connectionDrainPromise = finalDrain;
}

function resetConnectionState(reason = 'PC live connection state reset'): void {
  queueInFlightConnectionDrain(reason);
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
  const activeConversationIds = new Set(conversationIds);
  for (const [conversationId, unsubscribe] of [...conversationUnsubs.entries()]) {
    if (activeConversationIds.has(conversationId)) {
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
          try {
            handler(message, context);
          } catch {
            // Isolate independent consumers sharing the same wire subscription.
          }
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
          if (
            eventType
            && listener.eventTypes.length > 0
            && !listener.eventTypes.includes(eventType)
          ) {
            continue;
          }
          try {
            listener.handler(context);
          } catch {
            // Isolate independent consumers sharing the same scope subscription.
          }
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
  const staleConnection = sharedConnection;
  connectionGeneration += 1;
  resetConnectionState(reason);
  staleConnection?.disconnect(1000, reason);
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;
}

function handleConnectionLost(
  connection: ImLiveConnection,
  generation: number,
  triggerReconnect: boolean,
  closeStaleConnection = false,
): void {
  if (generation !== connectionGeneration || sharedConnection !== connection) {
    return;
  }
  detachConnectionListeners();
  clearWireSubscriptions();
  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'closed';
  if (closeStaleConnection) {
    connection.disconnect(1000, 'live connection lost');
  }

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
    || sharedConnectionPromise
    || connectionDrainPromise
    || connectionStatus === 'connecting'
    || !hasSubscriptionDemand()
    || !isAppSdkSessionAuthenticated(resolveSession())
  ) {
    return;
  }

  const circuitCooldownRemaining = circuitOpenUntil - Date.now();
  if (circuitCooldownRemaining > 0) {
    reconnectTimer = setTimeout(() => {
      reconnectTimer = undefined;
      void ensurePcLiveConnection().catch(() => undefined);
    }, circuitCooldownRemaining);
    return;
  }
  circuitOpenUntil = 0;

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
  let failureRecorded = false;

  const recordCurrentConnectionFailure = (): void => {
    if (failureRecorded) {
      return;
    }
    failureRecorded = true;
    recordConnectionFailure();
  };

  const nextLifecycleUnsub = connection.lifecycle.onStateChange((state) => {
    if (generation !== connectionGeneration || sharedConnection !== connection) {
      return;
    }
    if (state.status === 'open') {
      connectionStatus = 'open';
      reconnectAttempt = 0;
      consecutiveFailures = 0;
      circuitOpenUntil = 0;
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
      recordCurrentConnectionFailure();
      handleConnectionLost(connection, generation, true, true);
      return;
    }
    if (state.status === 'closed') {
      recordCurrentConnectionFailure();
      handleConnectionLost(connection, generation, true);
    }
  });
  if (generation === connectionGeneration && sharedConnection === connection) {
    lifecycleUnsub = nextLifecycleUnsub;
  } else {
    nextLifecycleUnsub();
  }

  const nextErrorUnsub = connection.lifecycle.onError((error) => {
    if (generation !== connectionGeneration || sharedConnection !== connection) {
      return;
    }
    if (isAuthenticationFailure(error)) {
      notifyAuthenticationFailure('websocket authentication failed');
      disposePcLiveConnection('websocket authentication failed');
      return;
    }
    if (isFatalLiveConnectionError(error)) {
      recordCurrentConnectionFailure();
      handleConnectionLost(connection, generation, true, true);
    }
  });
  if (generation === connectionGeneration && sharedConnection === connection) {
    errorUnsub = nextErrorUnsub;
  } else {
    nextErrorUnsub();
  }
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
  totalConnectionsCreated += 1;

  const deviceId = resolveDeviceId();
  const resolvedClient = resolveClient();
  const client = isPromiseLike(resolvedClient) ? await resolvedClient : resolvedClient;
  const connection = await client.connect({
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
  if (generation !== connectionGeneration || sharedConnection !== connection) {
    throw new Error('PC live connection closed during setup');
  }
  return connection;
}

function installBrowserRecoveryHooks(): void {
  if (browserHooksInstalled || typeof window === 'undefined') {
    return;
  }
  browserHooksInstalled = true;

  window.addEventListener('online', () => {
    recoverPcLiveConnection('browser online', { force: true });
  });
  window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
    invalidatePcLiveConnection('auth session changed');
  });
  if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') {
        recoverPcLiveConnection('browser visible', { force: true });
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

export function getPcLiveConnectionDiagnostics(): {
  status: PcLiveConnectionStatus;
  totalConnectionsCreated: number;
  hasSharedConnection: boolean;
  isConnecting: boolean;
  isDraining: boolean;
  reconnectAttempt: number;
  consecutiveFailures: number;
  circuitOpen: boolean;
} {
  return {
    status: connectionStatus,
    totalConnectionsCreated,
    hasSharedConnection: sharedConnection !== null,
    isConnecting: sharedConnectionPromise !== null,
    isDraining: connectionDrainPromise !== null,
    reconnectAttempt,
    consecutiveFailures,
    circuitOpen: Date.now() < circuitOpenUntil,
  };
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
  if (connectionDrainPromise) {
    await connectionDrainPromise;
    return ensurePcLiveConnection();
  }
  if (Date.now() < circuitOpenUntil) {
    scheduleReconnect();
    throw new Error('PC live connection circuit breaker is cooling down');
  }

  const attemptGeneration = connectionGeneration + 1;
  let attemptPromise: Promise<ImLiveConnection>;
  attemptPromise = connectSharedLiveConnection()
    .then((connection) => {
      if (sharedConnectionPromise === attemptPromise) {
        sharedConnectionPromise = null;
      }
      return connection;
    })
    .catch((error: unknown) => {
      if (sharedConnectionPromise === attemptPromise) {
        sharedConnectionPromise = null;
      }
      if (attemptGeneration !== connectionGeneration) {
        throw error;
      }
      const failureAlreadyRecorded = connectionStatus === 'closed';
      connectionStatus = 'error';
      if (!failureAlreadyRecorded) {
        recordConnectionFailure();
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
  sharedConnectionPromise = attemptPromise;

  return sharedConnectionPromise;
}

export function recoverPcLiveConnection(
  reason = 'realtime recovery requested',
  options: PcLiveConnectionRecoveryOptions = {},
): void {
  if (!hasSubscriptionDemand() || !isAppSdkSessionAuthenticated(resolveSession())) {
    return;
  }
  if (sharedConnectionPromise) {
    return;
  }
  if (connectionDrainPromise) {
    void ensurePcLiveConnection().catch(() => undefined);
    return;
  }
  if (sharedConnection && (connectionStatus === 'open' || connectionStatus === 'connecting')) {
    syncWireSubscriptionsWhenReady(sharedConnection);
    return;
  }
  if (!options.force && (connectionStatus === 'open' || connectionStatus === 'connecting')) {
    return;
  }
  clearReconnectTimer();
  connectionGeneration += 1;
  if (sharedConnection) {
    const staleConnection = sharedConnection;
    resetConnectionState(reason);
    staleConnection.disconnect(1000, reason);
  } else {
    resetConnectionState(reason);
  }

  void ensurePcLiveConnection().catch(() => undefined);
}

function invalidatePcLiveConnection(reason: string): void {
  clearReconnectTimer();
  connectionGeneration += 1;
  clearWireSubscriptions();
  sharedConnection?.disconnect(1000, reason);
  resetConnectionState(reason);
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
    teardownIfIdle('connection lease released');
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
    teardownIfIdle('conversation subscription closed');
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
    eventTypes: scope.eventTypes ?? [],
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
    teardownIfIdle('scope subscription closed');
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
  connectionDrainPromise = null;
  totalConnectionsCreated = 0;
}
