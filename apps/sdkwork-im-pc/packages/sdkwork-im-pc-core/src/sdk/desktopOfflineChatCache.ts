import {
  initDesktopOfflineStore,
  isDesktopOfflineStoreEnabled,
  listDesktopOfflineConversations,
  listDesktopOfflineMessages,
  purgeDesktopOfflinePrincipalCache,
  upsertDesktopOfflineConversations,
  upsertDesktopOfflineMessages,
} from './desktopOfflineStore';
import type { DesktopOfflineChat, DesktopOfflineMessage } from './desktopOfflineChatTypes';
import {
  desktopOfflineScopesEqual,
  resolveDesktopOfflinePrincipalScope,
  type DesktopOfflinePrincipalScope,
} from './desktopOfflineScope';
import { SDKWORK_IM_SESSION_CHANGED_EVENT } from './session';

export type OfflinePersistableMessage = DesktopOfflineMessage & {
  messageSeq?: number;
};

let initializedPrincipalScope: DesktopOfflinePrincipalScope | undefined;
let cacheInitialization: {
  promise: Promise<boolean>;
  scope: DesktopOfflinePrincipalScope;
} | undefined;
let lifecycleListenerInstalled = false;
let activePrincipalScope: DesktopOfflinePrincipalScope | undefined;
const OFFLINE_WRITE_BATCH_SIZE = 200;

function installDesktopOfflineCacheLifecycle(): void {
  if (lifecycleListenerInstalled || typeof window === 'undefined') {
    return;
  }
  lifecycleListenerInstalled = true;
  activePrincipalScope = resolveDesktopOfflinePrincipalScope();
  window.addEventListener(SDKWORK_IM_SESSION_CHANGED_EVENT, () => {
    const previousScope = activePrincipalScope;
    const nextScope = resolveDesktopOfflinePrincipalScope();
    activePrincipalScope = nextScope;
    if (previousScope && !desktopOfflineScopesEqual(previousScope, nextScope)) {
      initializedPrincipalScope = undefined;
      void purgeDesktopOfflinePrincipalCache(previousScope).catch(() => undefined);
    }
  });
}

export async function ensureDesktopOfflineChatCache(): Promise<boolean> {
  if (!isDesktopOfflineStoreEnabled()) {
    return false;
  }
  const scope = resolveDesktopOfflinePrincipalScope();
  if (!scope) {
    return false;
  }
  if (!desktopOfflineScopesEqual(initializedPrincipalScope, scope)) {
    if (!cacheInitialization || !desktopOfflineScopesEqual(cacheInitialization.scope, scope)) {
      const request = {
        scope,
        promise: Promise.resolve(false),
      };
      request.promise = initDesktopOfflineStore(scope)
        .then((initialized) => {
          if (initialized && cacheInitialization === request) {
            initializedPrincipalScope = scope;
          }
          return initialized;
        })
        .finally(() => {
          if (cacheInitialization === request) {
            cacheInitialization = undefined;
          }
        });
      cacheInitialization = request;
    }
    const initialized = await cacheInitialization.promise;
    if (
      !initialized
      || !desktopOfflineScopesEqual(initializedPrincipalScope, scope)
      || !desktopOfflineScopesEqual(scope, resolveDesktopOfflinePrincipalScope())
    ) {
      return false;
    }
  }
  installDesktopOfflineCacheLifecycle();
  return true;
}

function resolveScope(): DesktopOfflinePrincipalScope | undefined {
  return resolveDesktopOfflinePrincipalScope();
}

export async function persistDesktopOfflineMessages(messages: OfflinePersistableMessage[]): Promise<void> {
  const scope = resolveScope();
  if (
    messages.length === 0
    || !scope
    || !(await ensureDesktopOfflineChatCache())
    || !desktopOfflineScopesEqual(scope, resolveScope())
  ) {
    return;
  }
  const updatedAt = new Date().toISOString();
  const records = messages.flatMap((message) => {
    if (
      typeof message.messageSeq !== 'number'
      || !Number.isSafeInteger(message.messageSeq)
      || message.messageSeq <= 0
    ) {
      return [];
    }
    return [{
      scope,
      conversationId: message.chatId,
      messageSeq: message.messageSeq,
      messageId: message.id,
      payloadJson: JSON.stringify(message),
      updatedAt,
    }];
  });
  for (let index = 0; index < records.length; index += OFFLINE_WRITE_BATCH_SIZE) {
    await upsertDesktopOfflineMessages(records.slice(index, index + OFFLINE_WRITE_BATCH_SIZE));
  }
}

export async function loadDesktopOfflineMessages(
  chatId: string,
  beforeSeq?: number,
  limit?: number,
): Promise<DesktopOfflineMessage[]> {
  const scope = resolveScope();
  if (
    !scope
    || !(await ensureDesktopOfflineChatCache())
    || !desktopOfflineScopesEqual(scope, resolveScope())
  ) {
    return [];
  }
  const rows = await listDesktopOfflineMessages({
    scope,
    conversationId: chatId,
    beforeSeq,
    limit,
  });
  const messages: DesktopOfflineMessage[] = [];
  for (const row of rows) {
    try {
      const parsed: unknown = JSON.parse(row.payloadJson);
      if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
        messages.push(parsed as DesktopOfflineMessage);
      }
    } catch {
      // Skip corrupt cache rows.
    }
  }
  return messages.sort((left, right) => left.timestamp - right.timestamp);
}

export async function persistDesktopOfflineChats(chats: DesktopOfflineChat[]): Promise<void> {
  const scope = resolveScope();
  if (
    chats.length === 0
    || !scope
    || !(await ensureDesktopOfflineChatCache())
    || !desktopOfflineScopesEqual(scope, resolveScope())
  ) {
    return;
  }
  const updatedAt = new Date().toISOString();
  const records = chats.map((chat) => ({
      scope,
      conversationId: chat.id,
      payloadJson: JSON.stringify(chat),
      updatedAt,
    }));
  for (let index = 0; index < records.length; index += OFFLINE_WRITE_BATCH_SIZE) {
    await upsertDesktopOfflineConversations(records.slice(index, index + OFFLINE_WRITE_BATCH_SIZE));
  }
}

export async function loadDesktopOfflineChats(limit?: number): Promise<DesktopOfflineChat[]> {
  const scope = resolveScope();
  if (
    !scope
    || !(await ensureDesktopOfflineChatCache())
    || !desktopOfflineScopesEqual(scope, resolveScope())
  ) {
    return [];
  }
  const rows = await listDesktopOfflineConversations({ scope, limit });
  const chats: DesktopOfflineChat[] = [];
  for (const row of rows) {
    try {
      const parsed: unknown = JSON.parse(row.payloadJson);
      if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
        chats.push(parsed as DesktopOfflineChat);
      }
    } catch {
      // Skip corrupt cache rows.
    }
  }
  return chats.sort((left, right) => right.updatedAt - left.updatedAt);
}

export function resetDesktopOfflineChatCacheForTests(): void {
  initializedPrincipalScope = undefined;
  cacheInitialization = undefined;
  activePrincipalScope = undefined;
}
