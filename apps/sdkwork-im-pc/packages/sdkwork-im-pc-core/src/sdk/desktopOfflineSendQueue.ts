import {
  claimDesktopOfflinePendingSends,
  deleteDesktopOfflinePendingSend,
  enqueueDesktopOfflinePendingSend,
  listDesktopOfflinePendingSends,
  releaseDesktopOfflinePendingSendClaim,
} from './desktopOfflineStore';
import { ensureDesktopOfflineChatCache } from './desktopOfflineChatCache';
import type { DesktopOfflineMessage } from './desktopOfflineChatTypes';
import { resolveAppSdkTenantId, readAppSdkSessionTokens } from './session';

export type DesktopPendingMediaPart = {
  kind?: string;
  text?: string;
  media?: Record<string, unknown>;
  payloadJson?: string;
};

export type DesktopPendingSendPayload = {
  chatId: string;
  content: string;
  type: DesktopOfflineMessage['type'];
  clientMsgId: string;
  replyTo?: DesktopOfflineMessage['replyTo'];
  extraInfo?: Record<string, unknown>;
  summary?: string;
  parts?: DesktopPendingMediaPart[];
  renderHints?: Record<string, unknown>;
};

const DEFAULT_PENDING_SEND_FLUSH_LIMIT = 50;
let pendingSendFlushInFlight: Promise<void> | null = null;

function createPendingSendClaimId(): string {
  return `pc-flush-${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
}

function resolveTenantId(): string | undefined {
  return resolveAppSdkTenantId(readAppSdkSessionTokens());
}

export function isRetryableDesktopSendError(error: unknown): boolean {
  if (error instanceof TypeError) {
    return true;
  }
  const message = error instanceof Error ? error.message : String(error);
  const normalized = message.toLowerCase();
  return (
    normalized.includes('failed to fetch')
    || normalized.includes('network')
    || normalized.includes('timeout')
    || normalized.includes('econnrefused')
    || normalized.includes('enotfound')
    || normalized.includes('service unavailable')
    || normalized.includes('503')
    || normalized.includes('502')
    || normalized.includes('504')
  );
}

function isValidPendingSendPayload(record: DesktopPendingSendPayload): boolean {
  if (
    typeof record.chatId !== 'string'
    || typeof record.content !== 'string'
    || typeof record.clientMsgId !== 'string'
    || typeof record.type !== 'string'
  ) {
    return false;
  }
  if (record.type === 'text') {
    return true;
  }
  return Array.isArray(record.parts) && record.parts.length > 0 && typeof record.summary === 'string';
}

export async function enqueueDesktopPendingSend(
  payload: DesktopPendingSendPayload,
): Promise<void> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return;
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  await enqueueDesktopOfflinePendingSend({
    tenantId,
    clientMsgId: payload.clientMsgId,
    conversationId: payload.chatId,
    payloadJson: JSON.stringify(payload),
    createdAt: new Date().toISOString(),
    attemptCount: 0,
  });
}

export async function listDesktopPendingSends(
  limit = DEFAULT_PENDING_SEND_FLUSH_LIMIT,
): Promise<Array<DesktopPendingSendPayload & { clientMsgId: string }>> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return [];
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return [];
  }
  const rows = await listDesktopOfflinePendingSends({ tenantId, limit });
  return parseDesktopPendingSendRows(rows);
}

export async function claimDesktopPendingSends(
  limit = DEFAULT_PENDING_SEND_FLUSH_LIMIT,
): Promise<Array<DesktopPendingSendPayload & { clientMsgId: string; claimId: string }>> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return [];
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return [];
  }
  const claimId = createPendingSendClaimId();
  const rows = await claimDesktopOfflinePendingSends({ tenantId, claimId, limit });
  return parseDesktopPendingSendRows(rows).map((payload) => ({
    ...payload,
    claimId,
  }));
}

export async function releaseDesktopPendingSendClaim(
  clientMsgId: string,
  claimId: string,
): Promise<void> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return;
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  await releaseDesktopOfflinePendingSendClaim({ tenantId, clientMsgId, claimId });
}

export async function runDesktopPendingSendFlush(
  flush: (
    pending: Array<DesktopPendingSendPayload & { clientMsgId: string; claimId: string }>,
  ) => Promise<void>,
): Promise<void> {
  if (pendingSendFlushInFlight) {
    await pendingSendFlushInFlight;
    return;
  }
  pendingSendFlushInFlight = (async () => {
    const pending = await claimDesktopPendingSends();
    if (pending.length === 0) {
      return;
    }
    await flush(pending);
  })().finally(() => {
    pendingSendFlushInFlight = null;
  });
  await pendingSendFlushInFlight;
}

function parseDesktopPendingSendRows(
  rows: Awaited<ReturnType<typeof listDesktopOfflinePendingSends>>,
): Array<DesktopPendingSendPayload & { clientMsgId: string }> {
  const payloads: Array<DesktopPendingSendPayload & { clientMsgId: string }> = [];
  for (const row of rows) {
    try {
      const parsed: unknown = JSON.parse(row.payloadJson);
      if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
        continue;
      }
      const record = parsed as DesktopPendingSendPayload;
      if (!isValidPendingSendPayload(record)) {
        continue;
      }
      payloads.push({
        ...record,
        clientMsgId: row.clientMsgId,
      });
    } catch {
      // Skip corrupt queue rows.
    }
  }
  return payloads;
}

export async function removeDesktopPendingSend(clientMsgId: string): Promise<void> {
  if (!(await ensureDesktopOfflineChatCache())) {
    return;
  }
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  await deleteDesktopOfflinePendingSend({ tenantId, clientMsgId });
}
