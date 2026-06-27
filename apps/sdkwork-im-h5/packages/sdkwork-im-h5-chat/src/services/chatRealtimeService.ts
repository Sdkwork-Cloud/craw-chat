import type { ImLiveConnection, ImRealtimeScopeSubscription, ImSubscription } from "@sdkwork/im-sdk";
import {
  getImSdkClient,
  isImH5IamSessionAuthenticated,
  readImH5IamSessionTokens,
} from "@sdkwork/im-h5-core";

const INBOX_REALTIME_EVENT_TYPES = [
  "message.posted",
  "conversation.updated",
  "conversation.created",
  "conversation.member_joined",
  "conversation.member_role_changed",
  "conversation.member_removed",
  "conversation.member_left",
  "conversation.owner_transferred",
] as const;

const RECONNECT_BASE_DELAY_MS = 1000;
const RECONNECT_MAX_DELAY_MS = 30000;
const RECONNECT_JITTER_RATIO = 0.2;
const CIRCUIT_BREAKER_FAILURE_THRESHOLD = 5;
const CIRCUIT_BREAKER_COOLDOWN_MS = 60_000;
const RECONNECT_MAX_ATTEMPTS = 20;

type ConversationMessageHandler = (message: unknown) => void;
type InboxRefreshHandler = () => void;

let sharedConnection: ImLiveConnection | null = null;
let sharedConnectionPromise: Promise<ImLiveConnection> | null = null;
let sharedConnectionIsOpen = false;
let connectionGeneration = 0;
let reconnectAttempt = 0;
let reconnectTimer: ReturnType<typeof setTimeout> | undefined;
let consecutiveFailures = 0;
let circuitOpenUntil = 0;
let lifecycleUnsub: ImSubscription | undefined;
let errorUnsub: ImSubscription | undefined;
const conversationHandlers = new Map<string, Set<ConversationMessageHandler>>();
const conversationUnsubs = new Map<string, ImSubscription>();
const inboxHandlers = new Map<string, Set<InboxRefreshHandler>>();
const inboxUnsubs = new Map<string, ImSubscription>();

function realtimeScopeKey(scopeType: string, scopeId: string): string {
  return `${scopeType}:${scopeId}`;
}

function toRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === "string" && value.trim().length > 0) {
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
    || Boolean(
      message
        && /(?:auth|session|token).*(?:failed|expired|invalid|required)|unauthori(?:s|z)ed/iu.test(
          message,
        ),
    );
}

function isFatalLiveConnectionError(error: unknown): boolean {
  const code = pickString(toRecord(error).code);
  return Boolean(
    code
      && (/^websocket_(?:auth|upstream|connect|heartbeat)/u.test(code)
        || /(?:auth|session|token).*(?:failed|expired|invalid|required)/iu.test(code)),
  );
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

function hasSubscriptionDemand(): boolean {
  return conversationHandlers.size > 0 || inboxHandlers.size > 0;
}

function isSessionAuthenticated(): boolean {
  return isImH5IamSessionAuthenticated(readImH5IamSessionTokens());
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

function clearWireSubscriptions(): void {
  for (const unsubscribe of conversationUnsubs.values()) {
    unsubscribe();
  }
  conversationUnsubs.clear();
  for (const unsubscribe of inboxUnsubs.values()) {
    unsubscribe();
  }
  inboxUnsubs.clear();
}

function resetReconnectStats(): void {
  reconnectAttempt = 0;
  consecutiveFailures = 0;
  circuitOpenUntil = 0;
}

function buildScopeSubscriptions(): ImRealtimeScopeSubscription[] {
  const scopes: ImRealtimeScopeSubscription[] = [];
  for (const scopeKey of inboxHandlers.keys()) {
    const [scopeType, scopeId] = scopeKey.split(":");
    if (!scopeType || !scopeId) {
      continue;
    }
    scopes.push({
      scopeType,
      scopeId,
      eventTypes: [...INBOX_REALTIME_EVENT_TYPES],
    });
  }
  return scopes;
}

function syncLiveSubscriptions(): void {
  const connection = sharedConnection;
  if (!connection || !sharedConnectionIsOpen) {
    return;
  }
  for (const conversationId of conversationHandlers.keys()) {
    if (conversationUnsubs.has(conversationId)) {
      continue;
    }
    const handlers = conversationHandlers.get(conversationId);
    const unsubscribe = connection.messages.onConversation(conversationId, (message) => {
      for (const activeHandler of handlers ?? []) {
        activeHandler(message);
      }
    });
    conversationUnsubs.set(conversationId, unsubscribe);
  }
  for (const scopeKey of inboxHandlers.keys()) {
    if (inboxUnsubs.has(scopeKey)) {
      continue;
    }
    const [scopeType, scopeId] = scopeKey.split(":");
    if (!scopeType || !scopeId) {
      continue;
    }
    const handlers = inboxHandlers.get(scopeKey);
    const unsubscribe = connection.events.onScope(scopeType, scopeId, () => {
      for (const activeHandler of handlers ?? []) {
        activeHandler();
      }
    });
    inboxUnsubs.set(scopeKey, unsubscribe);
  }
  connection.subscriptions.syncConversations(Array.from(conversationHandlers.keys()));
  connection.subscriptions.syncScopes(buildScopeSubscriptions());
}

function scheduleReconnect(): void {
  if (
    reconnectTimer
    || !hasSubscriptionDemand()
    || !isSessionAuthenticated()
  ) {
    return;
  }
  if (Date.now() < circuitOpenUntil) {
    return;
  }
  if (reconnectAttempt >= RECONNECT_MAX_ATTEMPTS) {
    return;
  }
  reconnectAttempt += 1;
  const attempt = reconnectAttempt;
  reconnectTimer = setTimeout(() => {
    reconnectTimer = undefined;
    void ensureLiveConnection().catch(() => undefined);
  }, computeReconnectDelay(attempt));
}

function handleConnectionLost(triggerReconnect: boolean): void {
  detachConnectionListeners();
  clearWireSubscriptions();
  sharedConnection = null;
  sharedConnectionPromise = null;
  sharedConnectionIsOpen = false;

  if (!triggerReconnect || !hasSubscriptionDemand()) {
    teardownConnectionIfIdle();
    return;
  }
  if (!isSessionAuthenticated()) {
    return;
  }
  scheduleReconnect();
}

function bindConnection(connection: ImLiveConnection, generation: number): void {
  detachConnectionListeners();
  sharedConnection = connection;
  resetReconnectStats();

  lifecycleUnsub = connection.lifecycle.onStateChange((state) => {
    if (generation !== connectionGeneration) {
      return;
    }
    if (state.status === "open") {
      sharedConnectionIsOpen = true;
      syncLiveSubscriptions();
      return;
    }
    if (state.status === "connecting") {
      return;
    }
    if (state.status === "error") {
      if (state.reason && isAuthenticationFailure({ message: state.reason })) {
        disposeChatLiveConnection("websocket authentication failed");
        return;
      }
      handleConnectionLost(true);
      return;
    }
    if (state.status === "closed") {
      handleConnectionLost(true);
    }
  });

  errorUnsub = connection.lifecycle.onError((error) => {
    if (generation !== connectionGeneration) {
      return;
    }
    if (isAuthenticationFailure(error)) {
      disposeChatLiveConnection("websocket authentication failed");
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

async function ensureLiveConnection(): Promise<ImLiveConnection> {
  if (sharedConnection) {
    return sharedConnection;
  }
  if (sharedConnectionPromise) {
    return sharedConnectionPromise;
  }

  const generation = connectionGeneration + 1;
  connectionGeneration = generation;

  sharedConnectionPromise = getImSdkClient()
    .connect({
      subscriptions: {
        conversations: [],
        scopes: [],
      },
    })
    .then((connection) => {
      if (generation !== connectionGeneration) {
        connection.disconnect(1000, "stale live connection attempt");
        throw new Error("live connection attempt superseded");
      }
      bindConnection(connection, generation);
      sharedConnectionPromise = null;
      return connection;
    })
    .catch((error: unknown) => {
      if (generation === connectionGeneration) {
        sharedConnectionPromise = null;
        if (
          hasSubscriptionDemand()
          && isSessionAuthenticated()
          && !isAuthenticationFailure(error)
          && !isFatalLiveConnectionError(error)
        ) {
          consecutiveFailures += 1;
          if (consecutiveFailures >= CIRCUIT_BREAKER_FAILURE_THRESHOLD) {
            circuitOpenUntil = Date.now() + CIRCUIT_BREAKER_COOLDOWN_MS;
          }
          scheduleReconnect();
        }
      }
      throw error;
    });

  return sharedConnectionPromise;
}

function teardownConnectionIfIdle(): void {
  if (hasSubscriptionDemand()) {
    return;
  }
  clearReconnectTimer();
  clearWireSubscriptions();
  sharedConnection?.disconnect(1000, "no live subscriptions");
  detachConnectionListeners();
  sharedConnection = null;
  sharedConnectionPromise = null;
  sharedConnectionIsOpen = false;
  resetReconnectStats();
}

export function disposeChatLiveConnection(reason = "session ended"): void {
  connectionGeneration += 1;
  clearReconnectTimer();
  detachConnectionListeners();
  for (const unsubscribe of conversationUnsubs.values()) {
    unsubscribe();
  }
  for (const unsubscribe of inboxUnsubs.values()) {
    unsubscribe();
  }
  conversationUnsubs.clear();
  inboxUnsubs.clear();
  conversationHandlers.clear();
  inboxHandlers.clear();
  sharedConnection?.disconnect(1000, reason);
  sharedConnection = null;
  sharedConnectionPromise = null;
  sharedConnectionIsOpen = false;
  resetReconnectStats();
}

export function resolveInboxRealtimeUserId(): string | null {
  const session = readImH5IamSessionTokens();
  const userId =
    session?.context?.userId?.trim()
    ?? session?.user?.userId?.trim()
    ?? session?.user?.id?.trim()
    ?? null;
  return userId || null;
}

export async function subscribeConversationLiveMessages(
  conversationId: string,
  handler: ConversationMessageHandler,
): Promise<() => void> {
  await ensureLiveConnection();
  let handlers = conversationHandlers.get(conversationId);
  if (!handlers) {
    handlers = new Set();
    conversationHandlers.set(conversationId, handlers);
    syncLiveSubscriptions();
  }

  handlers.add(handler);
  return () => {
    const activeHandlers = conversationHandlers.get(conversationId);
    if (!activeHandlers) {
      return;
    }
    activeHandlers.delete(handler);
    if (activeHandlers.size > 0) {
      return;
    }

    conversationUnsubs.get(conversationId)?.();
    conversationUnsubs.delete(conversationId);
    conversationHandlers.delete(conversationId);
    syncLiveSubscriptions();
    teardownConnectionIfIdle();
  };
}

export async function subscribeInboxLiveRefresh(
  handler: InboxRefreshHandler,
  userId = resolveInboxRealtimeUserId(),
): Promise<() => void> {
  if (!userId) {
    return () => undefined;
  }

  const scopeKey = realtimeScopeKey("user", userId);
  await ensureLiveConnection();
  let handlers = inboxHandlers.get(scopeKey);
  if (!handlers) {
    handlers = new Set();
    inboxHandlers.set(scopeKey, handlers);
    syncLiveSubscriptions();
  }

  handlers.add(handler);
  return () => {
    const activeHandlers = inboxHandlers.get(scopeKey);
    if (!activeHandlers) {
      return;
    }
    activeHandlers.delete(handler);
    if (activeHandlers.size > 0) {
      return;
    }

    inboxUnsubs.get(scopeKey)?.();
    inboxUnsubs.delete(scopeKey);
    inboxHandlers.delete(scopeKey);
    syncLiveSubscriptions();
    teardownConnectionIfIdle();
  };
}
