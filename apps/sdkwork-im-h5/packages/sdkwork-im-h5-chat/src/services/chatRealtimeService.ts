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
} from '@sdkwork/im-sdk';
import { getImSdkClient } from './chatConversationService';

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
    throw new Error('chat live connection superseded');
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

  if (!sharedConnection) {
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
  conversationEventUnsubs = new Map()
  scopeUnsubs = new Map();

  try {
    sharedConnection.disconnect();
  } catch {
    // ignore
  }

  sharedConnection = null;
  sharedConnectionPromise = null;
  connectionStatus = 'idle';
}

export function disposeChatLiveConnection(): void {
  conversationLeases.clear();
  conversationRegistrations.clear();
  scopeRegistrations.clear();
  inboxRefreshHandlers.clear();
  openListeners.clear();
  conversationIds.clear();
  scopeSubscriptions.length = 0;
  teardownConnectionIfIdle();
}

export async function ensureChatLiveConnection(): Promise<ImLiveConnection> {
  if (sharedConnection && connectionStatus === 'open') {
    return sharedConnection;
  }

  if (!sharedConnectionPromise) {
    sharedConnectionPromise = openSharedConnection().catch((error) => {
      sharedConnectionPromise = null;
      connectionStatus = 'error';
      throw error;
    });
  }

  return sharedConnectionPromise;
}

export function acquireConversationLiveConnection(conversationId: string, leaseId: string): void {
  conversationIds.add(conversationId);
  conversationLeases.add(leaseId);
  ensureConversationRegistration(conversationId);
  void ensureChatLiveConnection().then((connection) => {
    attachConversationListeners(connection, conversationId);
    connection.subscriptions.syncConversations(Array.from(conversationIds));
  });
}

export function releaseConversationLiveConnection(conversationId: string, leaseId: string): void {
  connectionLeases.delete(leaseId);
  teardownConnectionIfIdle();
}

export function subscribeConversationMessages(
  conversationId: string,
  handler: ConversationMessageHandler,
): () => void {
  const registration = ensureConversationRegistration(conversationId);
  registration.messageHandlers.add(handler);
  conversationIds.add(conversationId);
  void ensureChatLiveConnection().then((connection) => {
    attachConversationListeners(connection, conversationId);
    connection.subscriptions.syncConversations(Array.from(conversationIds));
  });
  return () => {
    registration.messageHandlers.delete(handler);
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
  return () => {
    registration.eventHandlers.delete(handler);
  };
}

export function subscribeScopeEvents(
  scopeType: string,
  scopeId: string,
  handler: ScopeEventHandler,
): () => void {
  const scopeKey = `${scopeType}:${scopeId}`;
  const registration = ensureScopeRegistration(scopeType, scopeId);
  registration.handlers.add(handler);
  const existing = scopeSubscriptions.find((scope) => scope.scopeType === scopeType && scope.scopeId === scopeId);
  if (!existing) {
    scopeSubscriptions.push({ scopeType, scopeId });
  }
  void ensureChatLiveConnection().then((connection) => {
    attachScopeListeners(connection, scopeKey, scopeType, scopeId);
    connection.subscriptions.syncScopes(scopeSubscriptions);
  });
  return () => {
    registration.handlers.delete(handler);
  };
}

export function subscribeInboxLiveRefresh(handler: () => void): () => void {
  inboxRefreshHandlers.add(handler);
  void ensureChatLiveConnection().then((connection) => {
    syncLiveSubscriptions(connection);
  });
  return () => {
    inboxRefreshHandlers.delete(handler);
  };
}

export function onChatLiveConnectionOpen(listener: ConnectionOpenListener): () => void {
  openListeners.add(listener);
  return () => {
    openListeners.delete(listener);
  };
}

export function getChatLiveConnectionStatus(): ChatLiveConnectionStatus {
  return connectionStatus;
}

export const chatRealtimeService = {
  acquireConversationLiveConnection,
  releaseConversationLiveConnection,
  subscribeConversationMessages,
  subscribeConversationLiveMessages,
  subscribeConversationEvents,
  subscribeScopeEvents,
  subscribeInboxLiveRefresh,
  onChatLiveConnectionOpen,
  ensureChatLiveConnection,
  disposeChatLiveConnection,
  getChatLiveConnectionStatus,
};

export default chatRealtimeService;
