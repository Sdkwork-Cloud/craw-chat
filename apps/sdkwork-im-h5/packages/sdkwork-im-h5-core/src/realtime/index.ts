import type {
  ImConnectOptions,
  ImDecodedMessage,
  ImLiveConnection,
  ImLiveConnectionState,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
  ImSdkClient,
  ImSubscription,
} from '../sdk';
import { getImSdkClient } from '../sdk';

export type ChatLiveConnectionStatus = 'idle' | 'connecting' | 'open' | 'closed' | 'error';

export type ConversationMessageHandler = (message: ImDecodedMessage, context: ImMessageContext) => void;
export type ConversationEventHandler = (event: Record<string, unknown>, context: ImRealtimeEventContext) => void;
export type ScopeEventHandler = ConversationEventHandler;
export type ConnectionOpenListener = (connection: ImLiveConnection) => void;

interface ConversationRegistration {
  messageHandlers: Set<ConversationMessageHandler>;
  eventHandlers: Set<ConversationEventHandler>;
}

interface ScopeRegistration {
  handlers: Set<ScopeEventHandler>;
  scopeId: string;
  scopeType: string;
}

// Reconnect policy constants, aligned with the PC reference manager
// (`sdkwork-im-pc-core/sdk/pcRealtimeConnectionManager.ts`).
const RECONNECT_BASE_DELAY_MS = 1000;
const RECONNECT_MAX_DELAY_MS = 30_000;
const RECONNECT_JITTER_RATIO = 0.2;
const CIRCUIT_BREAKER_FAILURE_THRESHOLD = 5;
const CIRCUIT_BREAKER_COOLDOWN_MS = 60_000;
const IM_H5_DEVICE_ID_STORAGE_KEY = 'sdkwork-im-h5-device-id';

const conversationRegistrations = new Map<string, ConversationRegistration>();
const scopeRegistrations = new Map<string, ScopeRegistration>();
const inboxRefreshHandlers = new Set<() => void>();
const openListeners = new Set<ConnectionOpenListener>();

let sharedConnection: ImLiveConnection | null = null;
let sharedConnectionPromise: Promise<ImLiveConnection> | null = null;
let connectionStatus: ChatLiveConnectionStatus = 'idle';
let connectionGeneration = 0;
let lifecycleUnsub: ImSubscription | undefined;
let stateUnsub: ImSubscription | undefined;
let conversationMessageUnsubs = new Map<string, ImSubscription>();
let conversationEventUnsubs = new Map<string, ImSubscription>();
let scopeUnsubs = new Map<string, ImSubscription>();

const conversationIds = new Set<string>();
const scopeSubscriptions: ImRealtimeScopeSubscription[] = [];
const connectionLeases = new Set<string>();
const conversationLeaseIds = new Map<string, Set<string>>();
const scopeLeaseIds = new Map<string, Set<string>>();
let nextLeaseSequence = 0;

// Recovery and reconnect state.
let reconnectAttempt = 0;
let reconnectTimer: ReturnType<typeof setTimeout> | undefined;
let consecutiveFailures = 0;
let circuitOpenUntil = 0;
let totalConnectionsCreated = 0;
let sessionActiveProvider: (() => boolean) | null = null;
let browserRecoveryHooksInstalled = false;
let cachedDeviceId: string | undefined;

function createConnectionLeaseId(scope: string): string {
  nextLeaseSequence += 1;
  return `${scope}:${nextLeaseSequence}`;
}

function connectForLease(
  leaseId: string,
  onReady: (connection: ImLiveConnection) => void,
): void {
  void ensureImLiveConnection()
    .then((connection) => {
      if (connectionLeases.has(leaseId)) {
        onReady(connection);
      }
    })
    .catch(() => undefined);
}

function ensureConversationRegistration(conversationId: string): ConversationRegistration {
  let registration = conversationRegistrations.get(conversationId);
  if (!registration) {
    registration = { messageHandlers: new Set(), eventHandlers: new Set() };
    conversationRegistrations.set(conversationId, registration);
  }
  return registration;
}

function ensureScopeRegistration(scopeType: string, scopeId: string): ScopeRegistration {
  const key = `${scopeType}:${scopeId}`;
  let registration = scopeRegistrations.get(key);
  if (!registration) {
    registration = { handlers: new Set(), scopeId, scopeType };
    scopeRegistrations.set(key, registration);
  }
  return registration;
}

function resolveConnectOptions(): ImConnectOptions {
  return {
    connectionTimeoutMs: 10_000,
    heartbeat: {
      intervalMs: 25_000,
      timeoutMs: 10_000,
    },
    subscriptions: {
      conversations: Array.from(conversationIds),
      scopes: scopeSubscriptions,
    },
    ...(resolveDeviceId() ? { deviceId: resolveDeviceId() } : {}),
  };
}

// ---------------------------------------------------------------- utilities

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

/**
 * A definitive credential rejection (expired/revoked/invalid token or HTTP
 * 401) must not trigger reconnect loops: the session is dead and only a
 * session change (`invalidateImLiveConnection`) can recover it. Transient
 * credential states such as `websocket_auth_tokens_not_ready` or
 * `websocket_auth_timeout` are NOT classified here, so the backoff retry can
 * self-heal once tokens are available.
 */
function isAuthenticationFailure(error: unknown): boolean {
  const record = toRecord(error);
  const code = pickString(record.code);
  const message = pickString(record.message, record.error, record.reason);
  const status = Number(record.status ?? record.statusCode ?? record.httpStatus ?? record.http_status);
  return status === 401
    || Boolean(code && /(?:auth_bind|session_expired|token_expired|invalid_token|unauthori(?:s|z)ed)/iu.test(code))
    || Boolean(message && (
      /(?:expired|invalid|unauthori(?:s|z)ed)/iu.test(message)
      || /(?:auth|session|token).*(?:rejected|revoked)/iu.test(message)
    ));
}

/**
 * Persistent per-installation device identifier so the gateway can track and
 * resume the H5 connection across page refreshes (mirrors the PC client id).
 */
function resolveDeviceId(): string | undefined {
  if (cachedDeviceId !== undefined) {
    return cachedDeviceId;
  }
  const storage = typeof globalThis.localStorage === 'undefined' ? null : globalThis.localStorage;
  if (!storage) {
    return undefined;
  }
  try {
    const existing = storage.getItem(IM_H5_DEVICE_ID_STORAGE_KEY);
    if (existing && existing.trim()) {
      cachedDeviceId = existing;
      return cachedDeviceId;
    }
    const generated = typeof globalThis.crypto?.randomUUID === 'function'
      ? globalThis.crypto.randomUUID()
      : `h5-${Date.now()}-${Math.random().toString(36).slice(2)}`;
    storage.setItem(IM_H5_DEVICE_ID_STORAGE_KEY, generated);
    cachedDeviceId = generated;
    return cachedDeviceId;
  } catch {
    return undefined;
  }
}

// ---------------------------------------------------------------- reconnect

function hasConnectionDemand(): boolean {
  return connectionLeases.size > 0;
}

function isSessionActive(): boolean {
  if (sessionActiveProvider) {
    try {
      return sessionActiveProvider();
    } catch {
      // A broken provider must not prevent reconnects from being scheduled.
      return true;
    }
  }
  // Without an injected provider the connection demand alone gates recovery.
  return true;
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

function scheduleReconnect(): void {
  if (
    reconnectTimer
    || sharedConnectionPromise
    || connectionStatus === 'connecting'
    || !hasConnectionDemand()
    || !isSessionActive()
  ) {
    return;
  }

  const circuitCooldownRemaining = circuitOpenUntil - Date.now();
  if (circuitCooldownRemaining > 0) {
    reconnectTimer = setTimeout(() => {
      reconnectTimer = undefined;
      void ensureImLiveConnection({ refreshInboxOnOpen: true }).catch(() => undefined);
    }, circuitCooldownRemaining);
    return;
  }
  circuitOpenUntil = 0;

  reconnectAttempt += 1;
  reconnectTimer = setTimeout(() => {
    reconnectTimer = undefined;
    void ensureImLiveConnection({ refreshInboxOnOpen: true }).catch(() => undefined);
  }, computeReconnectDelay(reconnectAttempt));
}

function installBrowserRecoveryHooks(): void {
  if (browserRecoveryHooksInstalled || typeof window === 'undefined') {
    return;
  }
  browserRecoveryHooksInstalled = true;

  window.addEventListener('online', () => {
    recoverImLiveConnection('browser online');
  });
  if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') {
        recoverImLiveConnection('browser visible');
      }
    });
  }
}

// ------------------------------------------------------ connection lifecycle

function clearWireSubscriptions(): void {
  for (const unsub of conversationMessageUnsubs.values()) {
    try {
      unsub();
    } catch {
      // ignore
    }
  }
  for (const unsub of conversationEventUnsubs.values()) {
    try {
      unsub();
    } catch {
      // ignore
    }
  }
  for (const unsub of scopeUnsubs.values()) {
    try {
      unsub();
    } catch {
      // ignore
    }
  }
  conversationMessageUnsubs = new Map();
  conversationEventUnsubs = new Map();
  scopeUnsubs = new Map();
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

async function openSharedConnection(refreshInboxOnOpen: boolean): Promise<ImLiveConnection> {
  const client: ImSdkClient = getImSdkClient();
  const options = resolveConnectOptions();
  connectionGeneration += 1;
  const generation = connectionGeneration;
  connectionStatus = 'connecting';
  totalConnectionsCreated += 1;

  const connection = await client.connect(options);

  if (generation !== connectionGeneration) {
    try {
      connection.disconnect();
    } catch {
      // ignore
    }
    throw new Error('im live connection superseded');
  }

  sharedConnection = connection;

  // `client.connect` resolves once the transport object exists; the CCP
  // handshake (auth.init -> hello -> auth_bind -> auth.ok) is still running.
  // The 'open' status and every failure decision come from the lifecycle
  // callbacks below so status always reflects the real wire state.
  lifecycleUnsub = connection.lifecycle.onError((error: unknown) => {
    if (generation !== connectionGeneration || sharedConnection !== connection) {
      return;
    }
    if (isAuthenticationFailure(error)) {
      connectionStatus = 'error';
      return;
    }
    handleConnectionLost(connection, generation, true);
  });

  stateUnsub = connection.lifecycle.onStateChange((state: ImLiveConnectionState) => {
    if (generation !== connectionGeneration || sharedConnection !== connection) {
      return;
    }
    if (state.status === 'open') {
      connectionStatus = 'open';
      reconnectAttempt = 0;
      consecutiveFailures = 0;
      circuitOpenUntil = 0;
      // A connection opened directly for a subscribing page must not fire the
      // inbox refresh (the page just loaded its own data); reconnects opened
      // by the recovery machinery refresh subscribers whose data went stale.
      syncLiveSubscriptions(connection, refreshInboxOnOpen);
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
        // Definitive credential rejection: wait for the next session change
        // instead of hammering the gateway with invalid tokens.
        return;
      }
      handleConnectionLost(connection, generation, true);
      return;
    }
    if (state.status === 'closed') {
      handleConnectionLost(connection, generation);
    }
  });

  return connection;
}

/**
 * The connection is dead: drop the cached references so the next
 * lease/ensure call opens a fresh connection (the SDK does not reconnect
 * automatically). Active registrations and leases are preserved so the
 * replacement connection re-attaches them, and a backoff retry is scheduled
 * when the session is still active.
 */
function handleConnectionLost(
  connection: ImLiveConnection,
  generation: number,
  closeStaleConnection = false,
): void {
  if (generation !== connectionGeneration || sharedConnection !== connection) {
    return;
  }
  recordConnectionFailure();

  try {
    lifecycleUnsub?.();
  } catch {
    // ignore
  }
  try {
    stateUnsub?.();
  } catch {
    // ignore
  }
  lifecycleUnsub = undefined;
  stateUnsub = undefined;
  clearWireSubscriptions();
  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'closed';
  if (closeStaleConnection) {
    try {
      connection.disconnect();
    } catch {
      // ignore
    }
  }

  if (!hasConnectionDemand() || !isSessionActive()) {
    teardownConnectionIfIdle();
    return;
  }
  scheduleReconnect();
}

function attachConversationListeners(connection: ImLiveConnection, conversationId: string): void {
  const registration = conversationRegistrations.get(conversationId);
  if (!registration) {
    return;
  }

  if (!conversationMessageUnsubs.has(conversationId)) {
    conversationMessageUnsubs.set(
      conversationId,
      connection.messages.onConversation(conversationId, (message, context) => {
        const active = conversationRegistrations.get(conversationId);
        if (!active) {
          return;
        }
        for (const handler of active.messageHandlers) {
          try {
            handler(message, context);
          } catch {
            // ignore handler errors
          }
        }
      }),
    );
  }

  if (!conversationEventUnsubs.has(conversationId)) {
    conversationEventUnsubs.set(
      conversationId,
      connection.events.onConversation(conversationId, (event, context) => {
        const active = conversationRegistrations.get(conversationId);
        if (!active) {
          return;
        }
        for (const handler of active.eventHandlers) {
          try {
            handler(event, context);
          } catch {
            // ignore handler errors
          }
        }
      }),
    );
  }
}

function attachScopeListeners(
  connection: ImLiveConnection,
  scopeKey: string,
  scopeType: string,
  scopeId: string,
): void {
  const registration = scopeRegistrations.get(scopeKey);
  if (!registration || scopeUnsubs.has(scopeKey)) {
    return;
  }

  scopeUnsubs.set(
    scopeKey,
    connection.events.onScope(scopeType, scopeId, (event, context) => {
      const active = scopeRegistrations.get(scopeKey);
      if (!active) {
        return;
      }
      for (const handler of active.handlers) {
        try {
          handler(event, context);
        } catch {
          // ignore handler errors
        }
      }
    }),
  );
}

function syncLiveSubscriptions(
  connection: ImLiveConnection,
  refreshInboxHandlers = true,
): void {
  if (!connection) {
    return;
  }

  connection.subscriptions.syncConversations(Array.from(conversationIds));
  connection.subscriptions.syncScopes(scopeSubscriptions);

  for (const conversationId of conversationRegistrations.keys()) {
    attachConversationListeners(connection, conversationId);
  }

  for (const [scopeKey, registration] of scopeRegistrations.entries()) {
    attachScopeListeners(connection, scopeKey, registration.scopeType, registration.scopeId);
  }

  if (!refreshInboxHandlers) {
    return;
  }

  for (const handler of inboxRefreshHandlers) {
    try {
      handler();
    } catch {
      // ignore handler errors
    }
  }
}

function teardownConnectionIfIdle(): void {
  if (connectionLeases.size > 0) {
    return;
  }

  clearReconnectTimer();
  connectionGeneration += 1;
  clearWireSubscriptions();

  if (sharedConnection) {
    try {
      sharedConnection.disconnect();
    } catch {
      // ignore
    }
  }

  lifecycleUnsub = undefined;
  stateUnsub = undefined;
  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'idle';
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;
}

function releaseConnectionLease(leaseId: string): void {
  connectionLeases.delete(leaseId);
  teardownConnectionIfIdle();
}

function removeConversationRegistrationIfUnused(conversationId: string): void {
  const registration = conversationRegistrations.get(conversationId);
  const leases = conversationLeaseIds.get(conversationId);
  if (
    (registration?.messageHandlers.size ?? 0) > 0
    || (registration?.eventHandlers.size ?? 0) > 0
    || (leases?.size ?? 0) > 0
  ) {
    return;
  }

  try {
    conversationMessageUnsubs.get(conversationId)?.();
  } catch {
    // ignore
  }
  try {
    conversationEventUnsubs.get(conversationId)?.();
  } catch {
    // ignore
  }
  conversationMessageUnsubs.delete(conversationId);
  conversationEventUnsubs.delete(conversationId);
  conversationRegistrations.delete(conversationId);
  conversationLeaseIds.delete(conversationId);
  conversationIds.delete(conversationId);
  if (sharedConnection && connectionStatus === 'open') {
    sharedConnection.subscriptions.syncConversations(Array.from(conversationIds));
  }
}

function removeScopeRegistrationIfUnused(scopeKey: string): void {
  const registration = scopeRegistrations.get(scopeKey);
  const leases = scopeLeaseIds.get(scopeKey);
  if ((registration?.handlers.size ?? 0) > 0 || (leases?.size ?? 0) > 0) {
    return;
  }

  try {
    scopeUnsubs.get(scopeKey)?.();
  } catch {
    // ignore
  }
  scopeUnsubs.delete(scopeKey);
  scopeRegistrations.delete(scopeKey);
  scopeLeaseIds.delete(scopeKey);
  const subscriptionIndex = scopeSubscriptions.findIndex(
    (scope) => `${scope.scopeType}:${scope.scopeId}` === scopeKey,
  );
  if (subscriptionIndex >= 0) {
    scopeSubscriptions.splice(subscriptionIndex, 1);
  }
  if (sharedConnection && connectionStatus === 'open') {
    sharedConnection.subscriptions.syncScopes(scopeSubscriptions);
  }
}

// ------------------------------------------------------------ public surface

export function disposeImLiveConnection(): void {
  clearReconnectTimer();
  connectionGeneration += 1;
  connectionLeases.clear();
  conversationLeaseIds.clear();
  scopeLeaseIds.clear();
  conversationRegistrations.clear();
  scopeRegistrations.clear();
  inboxRefreshHandlers.clear();
  openListeners.clear();
  conversationIds.clear();
  scopeSubscriptions.length = 0;
  teardownConnectionIfIdle();
}

export interface ImLiveConnectionRecoveryOptions {
  /**
   * Refresh registered inbox handlers when the freshly opened connection
   * reaches the open state. Recovery-driven connections (backoff retries,
   * session invalidations, browser recovery) pass true; connections opened
   * directly for a subscribing page pass false (the page loaded its data).
   */
  refreshInboxOnOpen?: boolean;
}

export async function ensureImLiveConnection(
  options: ImLiveConnectionRecoveryOptions = {},
): Promise<ImLiveConnection> {
  installBrowserRecoveryHooks();
  if (sharedConnection) {
    return sharedConnection;
  }

  if (!sharedConnectionPromise) {
    const pendingConnection = openSharedConnection(options.refreshInboxOnOpen === true);
    const guardedConnection = pendingConnection.catch((error) => {
      if (sharedConnectionPromise === guardedConnection) {
        sharedConnectionPromise = null;
        connectionStatus = connectionLeases.size > 0 ? 'error' : 'idle';
      }
      if (hasConnectionDemand() && isSessionActive() && !isAuthenticationFailure(error)) {
        scheduleReconnect();
      }
      throw error;
    });
    sharedConnectionPromise = guardedConnection;
  }

  return sharedConnectionPromise;
}

/**
 * Tears down the current connection while preserving registrations and
 * leases, then immediately rebuilds it when the session is still active
 * (login success, token refresh, account switch). On logout the provider
 * reports an inactive session and everything is cleared instead.
 */
export function invalidateImLiveConnection(reason = 'auth session changed'): void {
  installBrowserRecoveryHooks();
  clearReconnectTimer();
  connectionGeneration += 1;
  clearWireSubscriptions();
  const staleConnection = sharedConnection;
  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'idle';
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;
  if (staleConnection) {
    try {
      staleConnection.disconnect(1000, reason);
    } catch {
      // ignore
    }
  }

  if (!isSessionActive()) {
    // Logout: drop every registration and lease; the next login rebuilds
    // everything from scratch.
    conversationRegistrations.clear();
    scopeRegistrations.clear();
    inboxRefreshHandlers.clear();
    openListeners.clear();
    conversationIds.clear();
    scopeSubscriptions.length = 0;
    connectionLeases.clear();
    conversationLeaseIds.clear();
    scopeLeaseIds.clear();
    return;
  }

  // Session switch while still authenticated: keep registrations and leases,
  // and reconnect right away (backoff retries cover transient token states).
  if (hasConnectionDemand()) {
    void ensureImLiveConnection({ refreshInboxOnOpen: true }).catch(() => undefined);
  }
}

/**
 * Forces a fresh connection attempt after an environment recovery signal
 * (browser online / visibility change) without waiting for the backoff timer.
 */
export function recoverImLiveConnection(reason = 'realtime recovery requested'): void {
  installBrowserRecoveryHooks();
  if (!hasConnectionDemand() || !isSessionActive()) {
    return;
  }
  if (sharedConnectionPromise) {
    return;
  }
  if (sharedConnection && (connectionStatus === 'open' || connectionStatus === 'connecting')) {
    return;
  }
  clearReconnectTimer();
  connectionGeneration += 1;
  if (sharedConnection) {
    const staleConnection = sharedConnection;
    sharedConnection = null;
    sharedConnectionPromise = null;
    try {
      staleConnection.disconnect(1000, reason);
    } catch {
      // ignore
    }
  } else {
    sharedConnectionPromise = null;
  }
  void ensureImLiveConnection({ refreshInboxOnOpen: true }).catch(() => undefined);
}

/**
 * Injects the authenticated-session check used to gate reconnects and
 * invalidations. The app root binds it to the shared TokenManager; without
 * it the manager treats the session as active and gates on leases alone.
 */
export function setImLiveSessionActiveProvider(provider: (() => boolean) | null): void {
  sessionActiveProvider = provider;
}

export function getImLiveConnectionDiagnostics(): {
  status: ChatLiveConnectionStatus;
  totalConnectionsCreated: number;
  hasSharedConnection: boolean;
  isConnecting: boolean;
  reconnectAttempt: number;
  consecutiveFailures: number;
  circuitOpen: boolean;
} {
  return {
    status: connectionStatus,
    totalConnectionsCreated,
    hasSharedConnection: sharedConnection !== null,
    isConnecting: sharedConnectionPromise !== null,
    reconnectAttempt,
    consecutiveFailures,
    circuitOpen: Date.now() < circuitOpenUntil,
  };
}

export function acquireConversationLiveConnection(conversationId: string, leaseId: string): void {
  conversationIds.add(conversationId);
  connectionLeases.add(leaseId);
  const leases = conversationLeaseIds.get(conversationId) ?? new Set<string>();
  leases.add(leaseId);
  conversationLeaseIds.set(conversationId, leases);
  ensureConversationRegistration(conversationId);
  connectForLease(leaseId, (connection) => {
    attachConversationListeners(connection, conversationId);
    connection.subscriptions.syncConversations(Array.from(conversationIds));
  });
}

export function releaseConversationLiveConnection(conversationId: string, leaseId: string): void {
  const leases = conversationLeaseIds.get(conversationId);
  leases?.delete(leaseId);
  removeConversationRegistrationIfUnused(conversationId);
  releaseConnectionLease(leaseId);
}

export function subscribeConversationMessages(
  conversationId: string,
  handler: ConversationMessageHandler,
): () => void {
  const registration = ensureConversationRegistration(conversationId);
  registration.messageHandlers.add(handler);
  const leaseId = createConnectionLeaseId(`conversation-message:${conversationId}`);
  acquireConversationLiveConnection(conversationId, leaseId);
  return () => {
    registration.messageHandlers.delete(handler);
    releaseConversationLiveConnection(conversationId, leaseId);
  };
}

export function subscribeConversationLiveMessages(
  conversationId: string,
  handler: ConversationMessageHandler,
): () => void {
  return subscribeConversationMessages(conversationId, handler);
}

export function subscribeConversationEvents(
  conversationId: string,
  handler: ConversationEventHandler,
): () => void {
  const registration = ensureConversationRegistration(conversationId);
  registration.eventHandlers.add(handler);
  const leaseId = createConnectionLeaseId(`conversation-event:${conversationId}`);
  acquireConversationLiveConnection(conversationId, leaseId);
  return () => {
    registration.eventHandlers.delete(handler);
    releaseConversationLiveConnection(conversationId, leaseId);
  };
}

export function subscribeScopeEvents(
  scopeType: string,
  scopeId: string,
  handler: ScopeEventHandler,
  eventTypes?: string[],
): () => void {
  const scopeKey = `${scopeType}:${scopeId}`;
  const registration = ensureScopeRegistration(scopeType, scopeId);
  registration.handlers.add(handler);
  const existing = scopeSubscriptions.find((scope) => scope.scopeType === scopeType && scope.scopeId === scopeId);
  if (!existing) {
    scopeSubscriptions.push({ scopeType, scopeId, ...(eventTypes ? { eventTypes } : {}) });
  } else if (eventTypes) {
    // Merge event types so later subscribers never override an earlier one's
    // server-side filter (union semantics).
    existing.eventTypes = Array.from(new Set([...(existing.eventTypes ?? []), ...eventTypes]));
  }
  const leaseId = createConnectionLeaseId(`scope:${scopeKey}`);
  connectionLeases.add(leaseId);
  const leases = scopeLeaseIds.get(scopeKey) ?? new Set<string>();
  leases.add(leaseId);
  scopeLeaseIds.set(scopeKey, leases);
  connectForLease(leaseId, (connection) => {
    attachScopeListeners(connection, scopeKey, scopeType, scopeId);
    connection.subscriptions.syncScopes(scopeSubscriptions);
  });
  return () => {
    registration.handlers.delete(handler);
    leases.delete(leaseId);
    removeScopeRegistrationIfUnused(scopeKey);
    releaseConnectionLease(leaseId);
  };
}

export function subscribeInboxLiveRefresh(handler: () => void): () => void {
  inboxRefreshHandlers.add(handler);
  const leaseId = createConnectionLeaseId('inbox');
  connectionLeases.add(leaseId);
  connectForLease(leaseId, (connection) => {
    // The subscribing page has just loaded its own data and this subscribe
    // may itself have opened the connection, so do not fire the refresh
    // handlers for the initial open. They only matter for reconnects of an
    // already-established connection, which arrive through onStateChange.
    syncLiveSubscriptions(connection, false);
  });
  return () => {
    inboxRefreshHandlers.delete(handler);
    releaseConnectionLease(leaseId);
  };
}

export function onImLiveConnectionOpen(listener: ConnectionOpenListener): () => void {
  openListeners.add(listener);
  return () => {
    openListeners.delete(listener);
  };
}

export function getImLiveConnectionStatus(): ChatLiveConnectionStatus {
  return connectionStatus;
}

export const imLiveService = {
  acquireConversationLiveConnection,
  releaseConversationLiveConnection,
  subscribeConversationMessages,
  subscribeConversationLiveMessages,
  subscribeConversationEvents,
  subscribeScopeEvents,
  subscribeInboxLiveRefresh,
  onImLiveConnectionOpen,
  ensureImLiveConnection,
  invalidateImLiveConnection,
  recoverImLiveConnection,
  setImLiveSessionActiveProvider,
  getImLiveConnectionDiagnostics,
  disposeImLiveConnection,
  getImLiveConnectionStatus,
};
