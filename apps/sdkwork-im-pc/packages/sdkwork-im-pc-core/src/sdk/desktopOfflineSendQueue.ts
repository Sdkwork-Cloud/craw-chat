import {
  claimDesktopOfflinePendingSends,
  deleteDesktopOfflinePendingSend,
  enqueueDesktopOfflinePendingSend,
  listDesktopOfflinePendingSends,
  quarantineDesktopOfflinePendingSend,
  releaseDesktopOfflinePendingSendClaim,
  type DesktopOfflinePendingSendRecord,
} from './desktopOfflineStore';
import { ensureDesktopOfflineChatCache } from './desktopOfflineChatCache';
import type { DesktopOfflineMessage } from './desktopOfflineChatTypes';
import {
  desktopOfflineScopesEqual,
  resolveDesktopOfflinePrincipalScope,
  type DesktopOfflinePrincipalScope,
} from './desktopOfflineScope';

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

export type DesktopPendingSendClaim = DesktopPendingSendPayload & {
  clientMsgId: string;
  claimId: string;
  scope: DesktopOfflinePrincipalScope;
};

export type DesktopPendingSendFlushResult = {
  retryableFailure: boolean;
};

type DrainOptions<T = never> = {
  signal?: AbortSignal;
  maxBatches?: number;
  backoff?: (delayMs: number, signal?: AbortSignal) => Promise<void>;
  isCurrent?: () => boolean;
  abandon?: (batch: T[]) => Promise<void>;
};

type PendingSendFlushOptions = Pick<DrainOptions<DesktopPendingSendClaim>, 'signal'> & {
  generation?: number;
};

const DEFAULT_PENDING_SEND_FLUSH_LIMIT = 50;
const MAX_PENDING_SEND_BATCHES_PER_RUN = 200;
const MAX_QUARANTINE_BATCHES_PER_CLAIM = 20;
const MAX_PENDING_SEND_BACKOFF_MS = 1_000;
const MAX_CONCURRENT_PENDING_SEND_FLUSH_SCOPES = 32;
const pendingSendFlushesInFlight = new Map<string, Promise<void>>();

function createPendingSendClaimId(): string {
  if (!globalThis.crypto?.randomUUID) {
    throw new Error('Secure random UUID support is required for offline send claims.');
  }
  return `pc-flush-${globalThis.crypto.randomUUID()}`;
}

function resolveScope(): DesktopOfflinePrincipalScope | undefined {
  return resolveDesktopOfflinePrincipalScope();
}

function pendingSendFlushScopeKey(
  scope: DesktopOfflinePrincipalScope,
  generation: number | undefined,
): string {
  return JSON.stringify([
    scope.tenantId,
    scope.organizationId,
    scope.principalKind,
    scope.principalId,
    generation ?? null,
  ]);
}

export function waitForDesktopPendingSendBackoff(
  delayMs: number,
  signal?: AbortSignal,
): Promise<void> {
  if (delayMs <= 0 || signal?.aborted) {
    return Promise.resolve();
  }
  return new Promise((resolve) => {
    let settled = false;
    let timeout: ReturnType<typeof globalThis.setTimeout> | undefined;
    const finish = () => {
      if (settled) {
        return;
      }
      settled = true;
      if (timeout !== undefined) {
        globalThis.clearTimeout(timeout);
      }
      signal?.removeEventListener('abort', finish);
      resolve();
    };
    timeout = globalThis.setTimeout(finish, delayMs);
    signal?.addEventListener('abort', finish, { once: true });
  });
}

export async function drainDesktopPendingSendBatches<T>(
  claim: () => Promise<T[]>,
  flush: (batch: T[]) => Promise<DesktopPendingSendFlushResult>,
  options: DrainOptions<T> = {},
): Promise<void> {
  const maxBatches = options.maxBatches ?? MAX_PENDING_SEND_BATCHES_PER_RUN;
  const backoff = options.backoff ?? waitForDesktopPendingSendBackoff;
  for (let batchIndex = 0; batchIndex < maxBatches; batchIndex += 1) {
    if (options.signal?.aborted || options.isCurrent?.() === false) {
      return;
    }
    const pending = await claim();
    if (pending.length === 0) {
      return;
    }
    if (options.signal?.aborted || options.isCurrent?.() === false) {
      await options.abandon?.(pending);
      return;
    }
    const result = await flush(pending);
    if (result.retryableFailure || options.signal?.aborted) {
      return;
    }
    const delayMs = Math.min(25 * (2 ** Math.min(batchIndex, 5)), MAX_PENDING_SEND_BACKOFF_MS);
    await backoff(delayMs, options.signal);
  }
  throw new Error('Offline pending send drain exceeded its bounded batch budget.');
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
  const scope = resolveScope();
  if (!scope || !(await ensureDesktopOfflineChatCache())) {
    return;
  }
  await enqueueDesktopOfflinePendingSend({
    scope,
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
  const scope = resolveScope();
  if (!scope || !(await ensureDesktopOfflineChatCache())) {
    return [];
  }
  const rows = await listDesktopOfflinePendingSends({ scope, limit });
  return partitionDesktopPendingSendRows(rows).payloads;
}

async function claimDesktopPendingSendsForScope(
  scope: DesktopOfflinePrincipalScope,
  limit = DEFAULT_PENDING_SEND_FLUSH_LIMIT,
): Promise<DesktopPendingSendClaim[]> {
  for (let batchIndex = 0; batchIndex < MAX_QUARANTINE_BATCHES_PER_CLAIM; batchIndex += 1) {
    if (!desktopOfflineScopesEqual(scope, resolveScope())) {
      return [];
    }
    const claimId = createPendingSendClaimId();
    const rows = await claimDesktopOfflinePendingSends({ scope, claimId, limit });
    const partitioned = partitionDesktopPendingSendRows(rows);
    const quarantineResults = await Promise.all(partitioned.quarantined.map((item) => (
      quarantineDesktopOfflinePendingSend({
        scope,
        clientMsgId: item.clientMsgId,
        claimId,
        reason: item.reason,
      })
    )));
    if (quarantineResults.some((accepted) => !accepted)) {
      throw new Error('Offline pending send quarantine claim is stale.');
    }
    if (partitioned.payloads.length > 0 || rows.length === 0) {
      return partitioned.payloads.map((payload) => ({
        ...payload,
        claimId,
        scope,
      }));
    }
  }
  throw new Error('Offline pending send quarantine exceeded its bounded batch budget.');
}

export async function claimDesktopPendingSends(
  limit = DEFAULT_PENDING_SEND_FLUSH_LIMIT,
): Promise<DesktopPendingSendClaim[]> {
  const scope = resolveScope();
  if (!scope || !(await ensureDesktopOfflineChatCache())) {
    return [];
  }
  return claimDesktopPendingSendsForScope(scope, limit);
}

export function isDesktopPendingSendClaimCurrent(claim: DesktopPendingSendClaim): boolean {
  return desktopOfflineScopesEqual(claim.scope, resolveScope());
}

export async function releaseDesktopPendingSendClaim(
  claim: Pick<DesktopPendingSendClaim, 'scope' | 'clientMsgId' | 'claimId'>,
): Promise<void> {
  await releaseDesktopOfflinePendingSendClaim(claim);
}

export async function removeDesktopPendingSend(
  claim: Pick<DesktopPendingSendClaim, 'scope' | 'clientMsgId' | 'claimId'>,
): Promise<void> {
  await deleteDesktopOfflinePendingSend(claim);
}

export async function runDesktopPendingSendFlush(
  flush: (pending: DesktopPendingSendClaim[]) => Promise<DesktopPendingSendFlushResult>,
  options: PendingSendFlushOptions = {},
): Promise<void> {
  const scope = resolveScope();
  if (!scope) {
    return;
  }
  const scopeKey = pendingSendFlushScopeKey(scope, options.generation);
  const existing = pendingSendFlushesInFlight.get(scopeKey);
  if (existing) {
    await existing;
    return;
  }
  if (pendingSendFlushesInFlight.size >= MAX_CONCURRENT_PENDING_SEND_FLUSH_SCOPES) {
    throw new RangeError(
      `Concurrent pending-send flush scope limit (${MAX_CONCURRENT_PENDING_SEND_FLUSH_SCOPES}) reached.`,
    );
  }
  let flushPromise: Promise<void>;
  flushPromise = (async () => {
    if (!(await ensureDesktopOfflineChatCache())) {
      return;
    }
    if (!desktopOfflineScopesEqual(scope, resolveScope())) {
      return;
    }
    await drainDesktopPendingSendBatches(
      () => claimDesktopPendingSendsForScope(scope),
      async (pending) => {
        if (!desktopOfflineScopesEqual(scope, resolveScope())) {
          await Promise.all(pending.map((claim) => releaseDesktopPendingSendClaim(claim)));
          return { retryableFailure: true };
        }
        return flush(pending);
      },
      {
        ...options,
        isCurrent: () => desktopOfflineScopesEqual(scope, resolveScope()),
        abandon: async (pending) => {
          await Promise.all(pending.map((claim) => releaseDesktopPendingSendClaim(claim)));
        },
      },
    );
  })().finally(() => {
    if (pendingSendFlushesInFlight.get(scopeKey) === flushPromise) {
      pendingSendFlushesInFlight.delete(scopeKey);
    }
  });
  pendingSendFlushesInFlight.set(scopeKey, flushPromise);
  await flushPromise;
}

export function partitionDesktopPendingSendRows(
  rows: DesktopOfflinePendingSendRecord[],
): {
  payloads: Array<DesktopPendingSendPayload & { clientMsgId: string }>;
  quarantined: Array<{ clientMsgId: string; reason: string }>;
} {
  const payloads: Array<DesktopPendingSendPayload & { clientMsgId: string }> = [];
  const quarantined: Array<{ clientMsgId: string; reason: string }> = [];
  for (const row of rows) {
    try {
      const parsed: unknown = JSON.parse(row.payloadJson);
      if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
        quarantined.push({
          clientMsgId: row.clientMsgId,
          reason: 'invalid pending send payload',
        });
        continue;
      }
      const record = parsed as DesktopPendingSendPayload;
      if (!isValidPendingSendPayload(record)) {
        quarantined.push({
          clientMsgId: row.clientMsgId,
          reason: 'invalid pending send payload',
        });
        continue;
      }
      payloads.push({
        ...record,
        clientMsgId: row.clientMsgId,
      });
    } catch {
      quarantined.push({
        clientMsgId: row.clientMsgId,
        reason: 'invalid pending send payload',
      });
    }
  }
  return { payloads, quarantined };
}
