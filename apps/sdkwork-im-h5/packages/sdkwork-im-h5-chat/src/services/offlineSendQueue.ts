/**
 * H5 offline send queue.
 *
 * Principal-scoped, bounded IndexedDB persistence for pending text sends.
 * The queue is a client-side delivery aid only: it never replaces the server
 * authority, never merges records across authenticated principals, and keeps
 * an explicit row/byte budget so a long offline period cannot grow unbounded.
 *
 * Delivery contract:
 * - sends are claimed with a short lease before flush (crash-safe);
 * - the flush loop serializes one batch at a time (in-flight guard);
 * - an acknowledged send is removed; a failed send is either retried after
 *   backoff or quarantined once the attempt budget is exhausted.
 */

import type { MessageReplyReference } from "@sdkwork/im-h5-core/sdk";

const OFFLINE_DB_NAME = "sdkwork-im-h5";
const OFFLINE_DB_VERSION = 1;
const PENDING_TEXT_SENDS_STORE = "offline_pending_text_sends";
const SCOPE_CREATED_INDEX = "scope_created";

/** Bounded queue budget: at most 200 pending text sends per principal. */
const MAX_PENDING_TEXT_SENDS = 200;
/** Lease window before another flush may claim the same send again. */
const CLAIM_LEASE_MS = 60_000;
/** Attempt budget before a send is quarantined (kept for inspection, not retried). */
const MAX_SEND_ATTEMPTS = 20;
/** Earliest retry backoff between flush attempts. */
const RETRY_BACKOFF_MS = 2_000;

export interface OfflinePendingTextSend {
  /** Stable idempotency key shared with the wire-level clientMsgId. */
  id: string;
  scope: string;
  conversationId: string;
  content: string;
  replyTo?: MessageReplyReference | null;
  createdAt: number;
  claimedUntil?: number;
  attempts: number;
}

export interface OfflineTextSendFlushTarget {
  postText(
    conversationId: string,
    content: string,
    options: { clientMsgId: string; replyTo?: MessageReplyReference | null },
  ): Promise<unknown>;
}

let configuredFlushTarget: OfflineTextSendFlushTarget | null = null;
let activeFlush: Promise<number> | null = null;

/** Injects the SDK-backed send target once at application bootstrap. */
export function configureOfflineTextSendFlushTarget(target: OfflineTextSendFlushTarget): void {
  configuredFlushTarget = target;
}

export function isOfflineTextSendQueueSupported(): boolean {
  return typeof globalThis.indexedDB !== "undefined";
}

function openOfflineQueueDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(OFFLINE_DB_NAME, OFFLINE_DB_VERSION);
    request.onupgradeneeded = () => {
      const database = request.result;
      if (!database.objectStoreNames.contains(PENDING_TEXT_SENDS_STORE)) {
        const store = database.createObjectStore(PENDING_TEXT_SENDS_STORE, { keyPath: "id" });
        store.createIndex(SCOPE_CREATED_INDEX, ["scope", "createdAt"]);
      }
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("open offline send queue database failed"));
  });
}

function runStoreRequest<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("offline send queue request failed"));
  });
}

function withStore<T>(
  mode: IDBTransactionMode,
  operation: (store: IDBObjectStore) => IDBRequest<T> | Promise<T>,
): Promise<T> {
  return openOfflineQueueDatabase().then((database) => {
    try {
      const transaction = database.transaction(PENDING_TEXT_SENDS_STORE, mode);
      const store = transaction.objectStore(PENDING_TEXT_SENDS_STORE);
      const outcome = operation(store);
      const settled = outcome instanceof IDBRequest
        ? runStoreRequest(outcome)
        : Promise.resolve(outcome);
      return settled.finally(() => database.close());
    } catch (error) {
      database.close();
      throw error;
    }
  });
}

function listPendingSends(store: IDBObjectStore, scope: string): Promise<OfflinePendingTextSend[]> {
  const byScope = store.index(SCOPE_CREATED_INDEX);
  const request = byScope.getAll(IDBKeyRange.bound([scope, -Infinity], [scope, Infinity]));
  return runStoreRequest(request) as Promise<OfflinePendingTextSend[]>;
}

/**
 * Enqueues a pending text send for the given principal scope.
 *
 * The send is ignored (no-op) when an identical idempotency key is already
 * pending; the queue never duplicates a send. When the bounded budget is
 * reached the oldest pending send is discarded so the queue stays bounded.
 */
export async function enqueuePendingTextSend(
  scope: string,
  send: Omit<OfflinePendingTextSend, "scope" | "createdAt" | "attempts">,
): Promise<void> {
  if (!isOfflineTextSendQueueSupported() || !scope.trim()) {
    return;
  }
  await withStore("readwrite", async (store) => {
    const pending = await listPendingSends(store, scope);
    if (pending.some((existing) => existing.id === send.id)) {
      return undefined;
    }
    const sorted = [...pending].sort((left, right) => left.createdAt - right.createdAt);
    if (sorted.length >= MAX_PENDING_TEXT_SENDS) {
      const oldest = sorted[0];
      if (oldest) {
        store.delete(oldest.id);
      }
    }
    store.put({
      id: send.id,
      scope,
      conversationId: send.conversationId,
      content: send.content,
      replyTo: send.replyTo ?? null,
      createdAt: Date.now(),
      attempts: 0,
    });
    return undefined;
  });
}

/**
 * Claims pending sends for flush: rows that are not leased by another flush
 * run (or whose lease expired) are returned with a fresh lease and an
 * incremented attempt counter.
 */
export async function claimPendingTextSends(scope: string, limit = 20): Promise<OfflinePendingTextSend[]> {
  if (!isOfflineTextSendQueueSupported() || !scope.trim()) {
    return [];
  }
  const now = Date.now();
  return withStore("readwrite", async (store) => {
    const pending = await listPendingSends(store, scope);
    const claimable = pending
      .filter((send) => send.attempts < MAX_SEND_ATTEMPTS)
      .filter((send) => !send.claimedUntil || send.claimedUntil <= now)
      .sort((left, right) => left.createdAt - right.createdAt)
      .slice(0, limit);
    for (const send of claimable) {
      store.put({
        ...send,
        claimedUntil: now + CLAIM_LEASE_MS,
        attempts: send.attempts + 1,
      });
    }
    return claimable;
  });
}

/** Removes an acknowledged send from the queue. */
export async function ackPendingTextSend(id: string): Promise<void> {
  if (!isOfflineTextSendQueueSupported()) {
    return;
  }
  await withStore("readwrite", (store) => store.delete(id));
}

/** Marks a send failed: rows past the attempt budget stay quarantined. */
export async function failPendingTextSend(id: string): Promise<void> {
  if (!isOfflineTextSendQueueSupported()) {
    return;
  }
  await withStore("readwrite", async (store) => {
    const existing = await runStoreRequest(store.get(id)) as OfflinePendingTextSend | undefined;
    if (!existing) {
      return undefined;
    }
    store.put({
      ...existing,
      claimedUntil: Date.now() + RETRY_BACKOFF_MS,
    });
    return undefined;
  });
}

/**
 * Flushes pending sends for a principal scope through the injected send
 * target. The flush is serialized with an in-flight guard: concurrent calls
 * share the same running flush instead of duplicating sends.
 *
 * Returns the number of sends acknowledged in this flush run.
 */
export async function runPendingTextSendFlush(scope: string): Promise<number> {
  if (!isOfflineTextSendQueueSupported() || !scope.trim()) {
    return 0;
  }
  if (activeFlush) {
    return activeFlush;
  }
  const flushRun = (async (): Promise<number> => {
    const target = configuredFlushTarget;
    if (!target) {
      return 0;
    }
    let acknowledged = 0;
    // Loop until no claimable send remains so a reconnect flush drains the
    // whole queue without the caller having to invoke the flush repeatedly.
    for (;;) {
      const claimed = await claimPendingTextSends(scope);
      if (claimed.length === 0) {
        break;
      }
      for (const send of claimed) {
        try {
          await target.postText(send.conversationId, send.content, {
            clientMsgId: send.id,
            replyTo: send.replyTo ?? null,
          });
          await ackPendingTextSend(send.id);
          acknowledged += 1;
        } catch {
          await failPendingTextSend(send.id);
        }
      }
    }
    return acknowledged;
  })().finally(() => {
    activeFlush = null;
  });
  activeFlush = flushRun;
  return flushRun;
}

/** Lists the current pending send inventory for diagnostics (bounded). */
export async function listPendingTextSends(scope: string): Promise<OfflinePendingTextSend[]> {
  if (!isOfflineTextSendQueueSupported() || !scope.trim()) {
    return [];
  }
  return withStore("readonly", (store) => listPendingSends(store, scope));
}
