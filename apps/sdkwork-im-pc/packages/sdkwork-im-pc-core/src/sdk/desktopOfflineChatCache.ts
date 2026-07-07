import {
  initDesktopOfflineStore,
  isDesktopOfflineStoreEnabled,
  listDesktopOfflineConversations,
  listDesktopOfflineMessages,
  upsertDesktopOfflineConversations,
  upsertDesktopOfflineMessages,
} from './desktopOfflineStore';
import type { DesktopOfflineChat, DesktopOfflineMessage } from './desktopOfflineChatTypes';
import { resolveAppSdkTenantId, readAppSdkSessionTokens } from './session';

export type OfflinePersistableMessage = DesktopOfflineMessage & {
  messageSeq?: number;
};

let cacheInitialized = false;

export async function ensureDesktopOfflineChatCache(): Promise<boolean> {
  if (!isDesktopOfflineStoreEnabled()) {
    return false;
  }
  if (!cacheInitialized) {
    await initDesktopOfflineStore();
    cacheInitialized = true;
  }
  return true;
}

function resolveTenantId(): string | undefined {
  return resolveAppSdkTenantId(readAppSdkSessionTokens());
}

function resolveMessageSeq(message: OfflinePersistableMessage, index: number): number {
  if (typeof message.messageSeq === 'number' && Number.isFinite(message.messageSeq) && message.messageSeq > 0) {
    return message.messageSeq;
  }
  return index + 1;
}

export async function persistDesktopOfflineMessages(messages: OfflinePersistableMessage[]): Promise<void> {
  if (messages.length === 0 || !(await ensureDesktopOfflineChatCache())) {
    return;
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  const updatedAt = new Date().toISOString();
  await upsertDesktopOfflineMessages(
    messages.map((message, index) => ({
      tenantId,
      conversationId: message.chatId,
      messageSeq: resolveMessageSeq(message, index),
      messageId: message.id,
      payloadJson: JSON.stringify(message),
      updatedAt,
    })),
  );
}

export async function loadDesktopOfflineMessages(
  chatId: string,
  afterSeq = 0,
  limit?: number,
): Promise<DesktopOfflineMessage[]> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return [];
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return [];
  }
  const rows = await listDesktopOfflineMessages({
    tenantId,
    conversationId: chatId,
    afterSeq,
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
  if (chats.length === 0 || !(await ensureDesktopOfflineChatCache())) {
    return;
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  const updatedAt = new Date().toISOString();
  await upsertDesktopOfflineConversations(
    chats.map((chat) => ({
      tenantId,
      conversationId: chat.id,
      payloadJson: JSON.stringify(chat),
      updatedAt,
    })),
  );
}

export async function loadDesktopOfflineChats(limit?: number): Promise<DesktopOfflineChat[]> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return [];
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return [];
  }
  const rows = await listDesktopOfflineConversations({ tenantId, limit });
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
  cacheInitialized = false;
}
