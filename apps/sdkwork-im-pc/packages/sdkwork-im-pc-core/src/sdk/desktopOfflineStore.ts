import { isSdkworkChatDesktopRuntime } from '../runtime/desktopEnvironment';
import type { DesktopOfflinePrincipalScope } from './desktopOfflineScope';

type TauriInvoke = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

export type DesktopOfflineMessageRecord = {
  scope: DesktopOfflinePrincipalScope;
  conversationId: string;
  messageSeq: number;
  messageId: string;
  payloadJson: string;
  updatedAt: string;
};

export type DesktopOfflineConversationRecord = {
  scope: DesktopOfflinePrincipalScope;
  conversationId: string;
  payloadJson: string;
  updatedAt: string;
};

export type DesktopOfflinePendingSendRecord = {
  scope: DesktopOfflinePrincipalScope;
  clientMsgId: string;
  conversationId: string;
  payloadJson: string;
  createdAt: string;
  attemptCount: number;
};

function resolveTauriInvoke(): TauriInvoke | undefined {
  const invoke = (globalThis as {
    __TAURI__?: {
      core?: {
        invoke?: TauriInvoke;
      };
    };
  }).__TAURI__?.core?.invoke;

  return typeof invoke === 'function' ? invoke : undefined;
}

export function isDesktopOfflineStoreEnabled(): boolean {
  return isSdkworkChatDesktopRuntime() && Boolean(resolveTauriInvoke());
}

export async function initDesktopOfflineStore(
  scope: DesktopOfflinePrincipalScope,
): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  await invoke('sdkwork_im_pc_offline_init', { scope });
  return true;
}

export async function upsertDesktopOfflineMessages(
  records: DesktopOfflineMessageRecord[],
): Promise<number> {
  const invoke = resolveTauriInvoke();
  if (!invoke || records.length === 0) {
    return 0;
  }
  const count = await invoke('sdkwork_im_pc_offline_upsert_messages', { records });
  return typeof count === 'number' ? count : 0;
}

export async function listDesktopOfflineMessages(input: {
  scope: DesktopOfflinePrincipalScope;
  conversationId: string;
  beforeSeq?: number;
  limit?: number;
}): Promise<DesktopOfflineMessageRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_list_messages', input);
  return Array.isArray(rows) ? (rows as DesktopOfflineMessageRecord[]) : [];
}

export async function upsertDesktopOfflineConversations(
  records: DesktopOfflineConversationRecord[],
): Promise<number> {
  const invoke = resolveTauriInvoke();
  if (!invoke || records.length === 0) {
    return 0;
  }
  const count = await invoke('sdkwork_im_pc_offline_upsert_conversations', { records });
  return typeof count === 'number' ? count : 0;
}

export async function listDesktopOfflineConversations(input: {
  scope: DesktopOfflinePrincipalScope;
  limit?: number;
}): Promise<DesktopOfflineConversationRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_list_conversations', input);
  return Array.isArray(rows) ? (rows as DesktopOfflineConversationRecord[]) : [];
}

export async function readDesktopOfflineSyncCursor(input: {
  scope: DesktopOfflinePrincipalScope;
  cursorScope: string;
}): Promise<string | null> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return null;
  }
  const value = await invoke('sdkwork_im_pc_offline_get_sync_cursor', input);
  return typeof value === 'string' && value.trim().length > 0 ? value : null;
}

export async function writeDesktopOfflineSyncCursor(input: {
  scope: DesktopOfflinePrincipalScope;
  cursorScope: string;
  cursorJson: string;
  updatedAt: string;
}): Promise<void> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return;
  }
  await invoke('sdkwork_im_pc_offline_set_sync_cursor', input);
}

export async function enqueueDesktopOfflinePendingSend(
  record: DesktopOfflinePendingSendRecord,
): Promise<void> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return;
  }
  await invoke('sdkwork_im_pc_offline_enqueue_pending_send', { record });
}

export async function listDesktopOfflinePendingSends(input: {
  scope: DesktopOfflinePrincipalScope;
  limit?: number;
}): Promise<DesktopOfflinePendingSendRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_list_pending_sends', input);
  return Array.isArray(rows) ? (rows as DesktopOfflinePendingSendRecord[]) : [];
}

export async function claimDesktopOfflinePendingSends(input: {
  scope: DesktopOfflinePrincipalScope;
  claimId: string;
  limit?: number;
}): Promise<DesktopOfflinePendingSendRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_claim_pending_sends', input);
  return Array.isArray(rows) ? (rows as DesktopOfflinePendingSendRecord[]) : [];
}

export async function releaseDesktopOfflinePendingSendClaim(input: {
  scope: DesktopOfflinePrincipalScope;
  clientMsgId: string;
  claimId: string;
}): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  return await invoke('sdkwork_im_pc_offline_release_pending_send_claim', input) === true;
}

export async function deleteDesktopOfflinePendingSend(input: {
  scope: DesktopOfflinePrincipalScope;
  clientMsgId: string;
  claimId: string;
}): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  return await invoke('sdkwork_im_pc_offline_delete_pending_send', input) === true;
}

export async function quarantineDesktopOfflinePendingSend(input: {
  scope: DesktopOfflinePrincipalScope;
  clientMsgId: string;
  claimId: string;
  reason: string;
}): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  return await invoke('sdkwork_im_pc_offline_quarantine_pending_send', input) === true;
}

export async function purgeDesktopOfflinePrincipalCache(
  scope: DesktopOfflinePrincipalScope,
): Promise<number> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return 0;
  }
  const deleted = await invoke('sdkwork_im_pc_offline_purge_principal_cache', { scope });
  return typeof deleted === 'number' ? deleted : 0;
}
