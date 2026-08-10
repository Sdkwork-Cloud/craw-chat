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
  };
}

async function openSharedConnection(): Promise<ImLiveConnection> {
  const client: ImSdkClient = getImSdkClient();
  const options = resolveConnectOptions();
  connectionGeneration += 1;
  const generation = connectionGeneration;
  connectionStatus = 'connecting';

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
  connectionStatus = 'open';

  lifecycleUnsub = connection.lifecycle.onError((error: unknown) => {
    connectionStatus = 'error';
    void error;
  });

  stateUnsub = connection.lifecycle.onStateChange((state: ImLiveConnectionState) => {
    if (state.status === "open") {
      syncLiveSubscriptions(connection);
    } else if (state.status === 'closed' || state.status === 'error') {
      connectionStatus = state.status;
    }
  });

  for (const conversationId of conversationRegistrations.keys()) {
    attachConversationListeners(connection, conversationId);
  }

  for (const [scopeKey, registration] of scopeRegistrations.entries()) {
    attachScopeListeners(connection, scopeKey, registration.scopeType, registration.scopeId);
  }

  for (const listener of openListeners) {
    try {
      listener(connection);
    } catch {
      // ignore listener errors
    }
  }

  return connection;
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

function syncLiveSubscriptions(connection: ImLiveConnection): void {
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

  connectionGeneration += 1;

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

export function disposeImLiveConnection(): void {
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

export async function ensureImLiveConnection(): Promise<ImLiveConnection> {
  if (sharedConnection && connectionStatus === 'open') {
    return sharedConnection;
  }

  if (!sharedConnectionPromise) {
    const pendingConnection = openSharedConnection();
    const guardedConnection = pendingConnection.catch((error) => {
      if (sharedConnectionPromise === guardedConnection) {
        sharedConnectionPromise = null;
        connectionStatus = connectionLeases.size > 0 ? 'error' : 'idle';
      }
      throw error;
    });
    sharedConnectionPromise = guardedConnection;
  }

  return sharedConnectionPromise;
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
  } else if (eventTypes && !existing.eventTypes) {
    existing.eventTypes = eventTypes;
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
    syncLiveSubscriptions(connection);
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
  disposeImLiveConnection,
  getImLiveConnectionStatus,
};
