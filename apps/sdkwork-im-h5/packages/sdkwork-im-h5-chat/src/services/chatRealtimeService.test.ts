import assert from "node:assert/strict";
import test, { afterEach } from "node:test";

import type { ImLiveConnection, ImLiveConnectionState, ImSubscription } from "@sdkwork/im-h5-core/sdk";
import { initImSdkClient, resetImSdkClient } from "@sdkwork/im-h5-core/sdk";

import {
  disposeChatLiveConnection,
  getChatLiveConnectionStatus,
  invalidateChatLiveConnection,
  notifyImInboxRefresh,
  setImLiveSessionActiveProvider,
  subscribeInboxLiveRefresh,
} from "./chatRealtimeService";

afterEach(() => {
  setImLiveSessionActiveProvider(null);
  disposeChatLiveConnection();
  resetImSdkClient();
  restoreBrowserMocks();
});

test("disconnects only after the final inbox lease is released", async () => {
  let connectCount = 0;
  let disconnectCount = 0;
  const connection = createConnection(() => {
    disconnectCount += 1;
  });
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    connectCount += 1;
    return connection;
  };

  const unsubscribeFirst = subscribeInboxLiveRefresh(() => undefined);
  const unsubscribeSecond = subscribeInboxLiveRefresh(() => undefined);
  await settlePromises();

  assert.equal(connectCount, 1);
  connection.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");
  unsubscribeFirst();
  assert.equal(disconnectCount, 0);
  unsubscribeSecond();
  assert.equal(disconnectCount, 1);
  assert.equal(getChatLiveConnectionStatus(), "idle");
});

test("does not fire refresh handlers for the connection open caused by the subscription itself", async () => {
  let refreshCount = 0;
  const connection = createConnection();
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => connection;

  const unsubscribe = subscribeInboxLiveRefresh(() => {
    refreshCount += 1;
  });
  await settlePromises();

  // The subscribing page loaded its data on mount; the connection it opened
  // must not trigger an immediate second load.
  assert.equal(refreshCount, 0);
  unsubscribe();
});

test("notifyImInboxRefresh fires every registered inbox refresh handler without touching the connection", async () => {
  let firstRefresh = 0;
  let secondRefresh = 0;
  const connection = createConnection();
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => connection;

  const unsubscribeFirst = subscribeInboxLiveRefresh(() => {
    firstRefresh += 1;
  });
  const unsubscribeSecond = subscribeInboxLiveRefresh(() => {
    secondRefresh += 1;
  });
  await settlePromises();

  // The welcome conversation is ready: subscribers reload their first page
  // even though the realtime stream itself has no new event to deliver.
  notifyImInboxRefresh();
  assert.equal(firstRefresh, 1);
  assert.equal(secondRefresh, 1);

  unsubscribeFirst();
  notifyImInboxRefresh();
  assert.equal(firstRefresh, 1);
  assert.equal(secondRefresh, 2);
  unsubscribeSecond();

  // No subscribers: must be a no-op, not a throw.
  notifyImInboxRefresh();
});

test("fires refresh handlers when a reconnected connection transitions back to open", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  let refreshCount = 0;
  const connections: MockedLiveConnection[] = [];
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    const connection = createConnection();
    connections.push(connection);
    return connection;
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => {
    refreshCount += 1;
  });
  await settlePromises();

  // A reconnect of an already-established connection (observed after the
  // subscription registered) should refresh subscribed inbox lists.
  connections[0]!.emitState({ status: "open" });
  assert.equal(refreshCount, 0);
  connections[0]!.emitState({ status: "error", reason: "connection reset" });
  t.mock.timers.tick(2_000);
  await settlePromises();
  connections[1]!.emitState({ status: "open" });
  assert.equal(refreshCount, 1);
  unsubscribe();
});

test("contains connection rejection and returns to idle after release", async () => {
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => Promise.reject(new Error("offline"));

  const unsubscribe = subscribeInboxLiveRefresh(() => undefined);
  await settlePromises();

  assert.equal(getChatLiveConnectionStatus(), "error");
  unsubscribe();
  assert.equal(getChatLiveConnectionStatus(), "idle");
});

test("retries the connection with backoff after a transient failure", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  let connectCount = 0;
  let retriedConnection: MockedLiveConnection | undefined;
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    connectCount += 1;
    if (connectCount === 1) {
      throw new Error("offline");
    }
    retriedConnection = createConnection();
    return retriedConnection;
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => undefined);
  await settlePromises();

  assert.equal(connectCount, 1);
  assert.equal(getChatLiveConnectionStatus(), "error");

  // First backoff delay is 800-1200ms; tick well past it.
  t.mock.timers.tick(2_000);
  await settlePromises();

  assert.equal(connectCount, 2);
  retriedConnection?.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");

  unsubscribe();
});

test("reconnects after a handshake error and re-attaches the inbox refresh", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  let connectCount = 0;
  let refreshed = 0;
  const connections: MockedLiveConnection[] = [];
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    connectCount += 1;
    const connection = createConnection();
    connections.push(connection);
    return connection;
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => {
    refreshed += 1;
  });
  await settlePromises();
  connections[0]!.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");
  // The subscribing page loaded its own data, so the initial open must not
  // fire the refresh handlers (they only matter for reconnects).
  assert.equal(refreshed, 0);

  // The wire drops after the handshake completed.
  connections[0]!.emitState({ status: "error", reason: "connection reset" });
  assert.equal(getChatLiveConnectionStatus(), "closed");

  t.mock.timers.tick(2_000);
  await settlePromises();
  assert.equal(connectCount, 2);

  connections[1]!.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");
  assert.equal(refreshed, 1);

  unsubscribe();
});

test("does not reconnect after a definitive authentication failure", async (t) => {
  t.mock.timers.enable({ apis: ["setTimeout"] });
  let connectCount = 0;
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    connectCount += 1;
    throw new Error("session token expired");
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => undefined);
  await settlePromises();

  assert.equal(connectCount, 1);
  assert.equal(getChatLiveConnectionStatus(), "error");

  t.mock.timers.tick(120_000);
  await settlePromises();
  assert.equal(connectCount, 1);

  unsubscribe();
});

test("invalidates and rebuilds the connection on a session change while authenticated", async () => {
  let connectCount = 0;
  let disconnectCount = 0;
  let refreshed = 0;
  const connections: MockedLiveConnection[] = [];
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    connectCount += 1;
    const connection = createConnection(() => {
      disconnectCount += 1;
    });
    connections.push(connection);
    return connection;
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => {
    refreshed += 1;
  });
  await settlePromises();
  assert.equal(connectCount, 1);
  connections[0]!.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");

  // Session switch (login refresh / account change) while still authenticated.
  invalidateChatLiveConnection();
  await settlePromises();

  assert.equal(disconnectCount, 1);
  assert.equal(connectCount, 2);
  assert.equal(getChatLiveConnectionStatus(), "connecting");

  connections[1]!.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");
  assert.equal(refreshed, 1);

  unsubscribe();
});

test("invalidating on logout clears registrations and disconnects without reconnecting", async () => {
  let connectCount = 0;
  let disconnectCount = 0;
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  const connection = createConnection(() => {
    disconnectCount += 1;
  });
  client.connect = async () => {
    connectCount += 1;
    return connection;
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => undefined);
  await settlePromises();
  connection.emitState({ status: "open" });
  assert.equal(getChatLiveConnectionStatus(), "open");

  setImLiveSessionActiveProvider(() => false);
  invalidateChatLiveConnection();
  await settlePromises();

  assert.equal(disconnectCount, 1);
  assert.equal(connectCount, 1);
  assert.equal(getChatLiveConnectionStatus(), "idle");

  // Registrations were cleared: releasing the (already cleared) lease is a
  // no-op and must not open a new connection.
  unsubscribe();
  assert.equal(connectCount, 1);
});

test("recovers the connection on browser online and visibility events", async () => {
  const listeners = installBrowserMocks();

  let connectCount = 0;
  let latestConnection: MockedLiveConnection | undefined;
  const client = initImSdkClient({ apiBaseUrl: "https://im.example.test" });
  client.connect = async () => {
    connectCount += 1;
    if (connectCount === 1) {
      throw new Error("offline");
    }
    latestConnection = createConnection();
    return latestConnection;
  };

  const unsubscribe = subscribeInboxLiveRefresh(() => undefined);
  await settlePromises();
  assert.equal(connectCount, 1);
  assert.equal(getChatLiveConnectionStatus(), "error");

  // Browser goes back online: recovery runs immediately without waiting for
  // the backoff timer.
  listeners.listeners.online?.();
  await settlePromises();
  assert.equal(connectCount, 2);
  assert.equal(getChatLiveConnectionStatus(), "connecting");

  // The connection drops again while the tab is hidden; making the tab
  // visible recovers it.
  latestConnection?.emitState({ status: "closed" });
  assert.equal(getChatLiveConnectionStatus(), "closed");
  (listeners.document as unknown as { visibilityState: string }).visibilityState = "visible";
  listeners.listeners.visibilitychange?.();
  await settlePromises();
  assert.equal(connectCount, 3);
  assert.equal(getChatLiveConnectionStatus(), "connecting");

  unsubscribe();
});

interface BrowserMockListeners {
  document: { addEventListener: (type: string, listener: () => void) => void };
  listeners: Record<string, (() => void) | undefined>;
}

let installedBrowserMocks: BrowserMockListeners | null = null;

function installBrowserMocks(): BrowserMockListeners {
  const listeners: Record<string, (() => void) | undefined> = {};
  const windowStub = {
    addEventListener: (type: string, listener: () => void) => {
      listeners[type] = listener;
    },
  };
  const documentStub = {
    addEventListener: (type: string, listener: () => void) => {
      listeners[type] = listener;
    },
    visibilityState: "hidden",
  };
  (globalThis as Record<string, unknown>).window = windowStub;
  (globalThis as Record<string, unknown>).document = documentStub;
  installedBrowserMocks = { document: documentStub, listeners };
  return installedBrowserMocks;
}

function restoreBrowserMocks(): void {
  if (!installedBrowserMocks) {
    return;
  }
  delete (globalThis as Record<string, unknown>).window;
  delete (globalThis as Record<string, unknown>).document;
  installedBrowserMocks = null;
}

type MockedLiveConnection = ImLiveConnection & {
  emitState(state: ImLiveConnectionState): void;
};

function createConnection(onDisconnect?: () => void): MockedLiveConnection {
  const stateListeners = new Set<(state: ImLiveConnectionState) => void>();
  const subscription: ImSubscription = () => undefined;
  return {
    disconnect: () => onDisconnect?.(),
    events: {
      onConversation: () => subscription,
      onScope: () => subscription,
    },
    lifecycle: {
      onError: () => subscription,
      onStateChange: (handler) => {
        stateListeners.add(handler);
        return () => {
          stateListeners.delete(handler);
        };
      },
    },
    messages: {
      onConversation: () => subscription,
    },
    subscriptions: {
      syncConversations: () => undefined,
      syncScopes: () => undefined,
    },
    emitState: (state) => {
      for (const handler of [...stateListeners]) {
        handler(state);
      }
    },
  };
}

async function settlePromises(): Promise<void> {
  await Promise.resolve();
  await new Promise<void>((resolve) => setImmediate(resolve));
}
