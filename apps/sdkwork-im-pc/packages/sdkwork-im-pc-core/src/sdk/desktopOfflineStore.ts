import { isSdkworkChatDesktopRuntime } from '../runtime/desktopEnvironment';

type TauriInvoke = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

export type DesktopOfflineMessageRecord = {
  tenantId: string;
  conversationId: string;
  messageSeq: number;
  messageId: string;
  payloadJson: string;
  updatedAt: string;
};

export type DesktopOfflineConversationRecord = {
  tenantId: string;
  conversationId: string;
  payloadJson: string;
  updatedAt: string;
};

export type DesktopOfflinePendingSendRecord = {
  tenantId: string;
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

export async function initDesktopOfflineStore(): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  await invoke('sdkwork_im_pc_offline_init');
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
  tenantId: string;
  conversationId: string;
  afterSeq?: number;
  limit?: number;
}): Promise<DesktopOfflineMessageRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_list_messages', {
    tenantId: input.tenantId,
    conversationId: input.conversationId,
    afterSeq: input.afterSeq ?? 0,
    limit: input.limit,
  });
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
  tenantId: string;
  limit?: number;
}): Promise<DesktopOfflineConversationRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_list_conversations', {
    tenantId: input.tenantId,
    limit: input.limit,
  });
  return Array.isArray(rows) ? (rows as DesktopOfflineConversationRecord[]) : [];
}

export async function readDesktopOfflineSyncCursor(
  tenantId: string,
  scope: string,
): Promise<string | null> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return null;
  }
  const value = await invoke('sdkwork_im_pc_offline_get_sync_cursor', { tenantId, scope });
  return typeof value === 'string' && value.trim().length > 0 ? value : null;
}

export async function writeDesktopOfflineSyncCursor(input: {
  tenantId: string;
  scope: string;
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
  tenantId: string;
  limit?: number;
}): Promise<DesktopOfflinePendingSendRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_list_pending_sends', {
    tenantId: input.tenantId,
    limit: input.limit,
  });
  return Array.isArray(rows) ? (rows as DesktopOfflinePendingSendRecord[]) : [];
}

export async function claimDesktopOfflinePendingSends(input: {
  tenantId: string;
  claimId: string;
  limit?: number;
}): Promise<DesktopOfflinePendingSendRecord[]> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return [];
  }
  const rows = await invoke('sdkwork_im_pc_offline_claim_pending_sends', {
    tenantId: input.tenantId,
    claimId: input.claimId,
    limit: input.limit,
  });
  return Array.isArray(rows) ? (rows as DesktopOfflinePendingSendRecord[]) : [];
}

export async function releaseDesktopOfflinePendingSendClaim(input: {
  tenantId: string;
  clientMsgId: string;
  claimId: string;
}): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  const released = await invoke('sdkwork_im_pc_offline_release_pending_send_claim', input);
  return released === true;
}

export async function deleteDesktopOfflinePendingSend(input: {
  tenantId: string;
  clientMsgId: string;
}): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }
  const deleted = await invoke('sdkwork_im_pc_offline_delete_pending_send', input);
  return deleted === true;
}
