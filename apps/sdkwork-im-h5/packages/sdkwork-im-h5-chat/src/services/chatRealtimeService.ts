/**
 * Compatibility re-export of the shared IM live connection service.
 *
 * The lease-based realtime manager lives in `@sdkwork/im-h5-core/realtime`
 * so that non-chat packages (contacts, user, ...) can subscribe to scoped
 * events over the same shared connection. This module keeps the previous
 * `chatRealtimeService` surface stable for existing chat package callers.
 */
import { imLiveService } from "@sdkwork/im-h5-core/realtime";

export {
  acquireConversationLiveConnection,
  releaseConversationLiveConnection,
  subscribeConversationMessages,
  subscribeConversationLiveMessages,
  subscribeConversationEvents,
  subscribeScopeEvents,
  subscribeInboxLiveRefresh,
  ensureImLiveConnection as ensureChatLiveConnection,
  disposeImLiveConnection as disposeChatLiveConnection,
  onImLiveConnectionOpen as onChatLiveConnectionOpen,
  getImLiveConnectionStatus as getChatLiveConnectionStatus,
  imLiveService as chatRealtimeService,
} from "@sdkwork/im-h5-core/realtime";

export type {
  ChatLiveConnectionStatus,
  ConversationMessageHandler,
  ConversationEventHandler,
  ScopeEventHandler,
  ConnectionOpenListener,
} from "@sdkwork/im-h5-core/realtime";

export default imLiveService;
