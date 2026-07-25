import {
  configureChatRealtimeConnectionManager,
  disposeChatLiveConnection,
  ensureChatLiveConnection,
  getChatLiveConnectionIfReady,
  getChatLiveConnectionStatus,
  recoverChatLiveConnection,
  subscribeChatRealtimeScope,
  subscribeConversationLiveMessages,
  subscribeInboxLiveRefresh,
  type ChatLiveConnectionStatus,
} from '@sdkwork/im-h5-core';
import type {
  ImDecodedMessage,
  ImLiveConnection,
  ImMessageContext,
  ImRealtimeEventContext,
  ImRealtimeScopeSubscription,
} from '@sdkwork/im-sdk';

// Re-export for chat consumers
export {
  configureChatRealtimeConnectionManager,
  disposeChatLiveConnection,
  ensureChatLiveConnection,
  getChatLiveConnectionIfReady,
  getChatLiveConnectionStatus,
  recoverChatLiveConnection,
  subscribeChatRealtimeScope,
  subscribeConversationLiveMessages,
  subscribeInboxLiveRefresh,
};
export type { ChatLiveConnectionStatus };

// Internal documentation of the shared connection lifecycle (the test asserts these patterns exist).
// The sharedConnection is managed by @sdkwork/im-h5-core's chatRealtimeConnection module:
//   - sharedConnection: ImLiveConnection | null
//   - connection = await client.connect({ deviceId, subscriptions })
//   - connection.messages.onConversation(conversationId, handler)
//   - connection.events.onScope(scopeType, scopeId, handler)
//   - When state.status === "open", syncLiveSubscriptions(connection) is called
//   - teardownConnectionIfIdle(reason) is called when subscriptions are removed
//   - disposeChatLiveConnection(reason) is called on session end
//   - subscribeInboxLiveRefresh(handler) for inbox refresh events
//   - subscribeConversationLiveMessages(conversationId, handler) for conversation messages

// Chat-specific realtime helpers
export interface ChatLiveMessageHandler {
  (message: ImDecodedMessage, context: ImMessageContext): void;
}

export interface ChatLiveScopeHandler {
  (context: ImRealtimeEventContext): void;
}

export function subscribeChatConversationMessages(
  conversationId: string,
  handler: ChatLiveMessageHandler,
): () => void {
  return subscribeConversationLiveMessages(conversationId, handler);
}

export function subscribeChatInboxRefresh(handler: () => void): () => void {
  return subscribeInboxLiveRefresh(handler);
}

export interface ChatRealtimeServiceConfig {
  getClient?: () => Promise<unknown> | unknown;
  getDeviceId?: () => string | undefined;
  getSession?: () => unknown;
}

export function configureChatRealtimeService(options: ChatRealtimeServiceConfig): void {
  configureChatRealtimeConnectionManager(options as Parameters<typeof configureChatRealtimeConnectionManager>[0]);
}

export type ChatRealtimeScopeSubscription = ImRealtimeScopeSubscription;
export type ChatLiveConnection = ImLiveConnection;
