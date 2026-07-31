import assert from "node:assert/strict";
import test, { afterEach } from "node:test";

import type { ImLiveConnection, ImSubscription } from "@sdkwork/im-h5-core/sdk";
import { initImSdkClient, resetImSdkClient } from "@sdkwork/im-h5-core/sdk";

import {
  disposeChatLiveConnection,
  getChatLiveConnectionStatus,
  subscribeInboxLiveRefresh,
} from "./chatRealtimeService";

afterEach(() => {
  disposeChatLiveConnection();
  resetImSdkClient();
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
  assert.equal(getChatLiveConnectionStatus(), "open");
  unsubscribeFirst();
  assert.equal(disconnectCount, 0);
  unsubscribeSecond();
  assert.equal(disconnectCount, 1);
  assert.equal(getChatLiveConnectionStatus(), "idle");
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

function createConnection(onDisconnect: () => void): ImLiveConnection {
  const subscription: ImSubscription = () => undefined;
  return {
    disconnect: onDisconnect,
    events: {
      onConversation: () => subscription,
      onScope: () => subscription,
    },
    lifecycle: {
      onError: () => subscription,
      onStateChange: () => subscription,
    },
    messages: {
      onConversation: () => subscription,
    },
    subscriptions: {
      syncConversations: () => undefined,
      syncScopes: () => undefined,
    },
  };
}

async function settlePromises(): Promise<void> {
  await Promise.resolve();
  await new Promise<void>((resolve) => setImmediate(resolve));
}
