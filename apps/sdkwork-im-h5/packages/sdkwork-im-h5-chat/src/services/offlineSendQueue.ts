import { readImH5IamSessionTokens } from "@sdkwork/im-h5-core";

const LEGACY_SESSION_STORAGE_KEY = "sdkwork-im-h5:pending-sends:v1";
const IDB_NAME = "sdkwork-im-h5-offline";
const IDB_VERSION = 1;
const IDB_STORE = "pending_sends";
const DEFAULT_FLUSH_LIMIT = 50;
const MAX_PENDING_SENDS = 100;

export interface PendingTextSendPayload {
  conversationId: string;
  text: string;
  clientMsgId: string;
}

interface PendingSendRecord {
  tenantId: string;
  clientMsgId: string;
  conversationId: string;
  payloadJson: string;
  createdAt: string;
  attemptCount: number;
  flushClaimId?: string | null;
}

type PendingSendPayloadWithClaim = PendingTextSendPayload & {
  clientMsgId: string;
  claimId: string;
};

let idbReady: Promise<IDBDatabase | null> | null = null;
let pendingSendFlushInFlight: Promise<void> | null = null;

function resolveTenantId(): string | undefined {
  const context = readImH5IamSessionTokens()?.context;
  const tenantId = context?.tenantId?.trim();
  return tenantId || undefined;
}

function createPendingSendClaimId(): string {
  return `h5-flush-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function openOfflineDatabase(): Promise<IDBDatabase | null> {
  if (typeof indexedDB === "undefined") {
    return Promise.resolve(null);
  }
  if (!idbReady) {
    idbReady = new Promise((resolve) => {
      const request = indexedDB.open(IDB_NAME, IDB_VERSION);
      request.onupgradeneeded = () => {
        const db = request.result;
        if (!db.objectStoreNames.contains(IDB_STORE)) {
          const store = db.createObjectStore(IDB_STORE, {
            keyPath: ["tenantId", "clientMsgId"],
          });
          store.createIndex("tenant_created", ["tenantId", "createdAt"], { unique: false });
          store.createIndex("tenant_claim", ["tenantId", "flushClaimId"], { unique: false });
        }
      };
      request.onsuccess = () => {
        void migrateLegacySessionStorage(request.result).finally(() => {
          resolve(request.result);
        });
      };
      request.onerror = () => resolve(null);
    });
  }
  return idbReady;
}

async function migrateLegacySessionStorage(db: IDBDatabase): Promise<void> {
  if (typeof window === "undefined") {
    return;
  }
  const raw = window.sessionStorage.getItem(LEGACY_SESSION_STORAGE_KEY);
  if (!raw) {
    return;
  }
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed)) {
      return;
    }
    for (const item of parsed) {
      if (!item || typeof item !== "object" || Array.isArray(item)) {
        continue;
      }
      const record = item as PendingSendRecord;
      if (
        typeof record.tenantId !== "string"
        || typeof record.clientMsgId !== "string"
        || typeof record.conversationId !== "string"
        || typeof record.payloadJson !== "string"
      ) {
        continue;
      }
      await putRecord(db, {
        tenantId: record.tenantId,
        clientMsgId: record.clientMsgId,
        conversationId: record.conversationId,
        payloadJson: record.payloadJson,
        createdAt: typeof record.createdAt === "string" ? record.createdAt : new Date().toISOString(),
        attemptCount: typeof record.attemptCount === "number" ? record.attemptCount : 0,
        flushClaimId: null,
      });
    }
  } catch {
    // Drop corrupt legacy queue.
  } finally {
    window.sessionStorage.removeItem(LEGACY_SESSION_STORAGE_KEY);
  }
}

function runReadWrite<T>(db: IDBDatabase, operation: (store: IDBObjectStore) => void): Promise<T> {
  return new Promise((resolve, reject) => {
    const transaction = db.transaction(IDB_STORE, "readwrite");
    const store = transaction.objectStore(IDB_STORE);
    let result: T;
    try {
      result = operation(store) as T;
    } catch (error) {
      transaction.abort();
      reject(error);
      return;
    }
    transaction.oncomplete = () => resolve(result);
    transaction.onerror = () => reject(transaction.error ?? new Error("idb transaction failed"));
    transaction.onabort = () => reject(transaction.error ?? new Error("idb transaction aborted"));
  });
}

function runReadonly<T>(db: IDBDatabase, operation: (store: IDBObjectStore) => IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    const transaction = db.transaction(IDB_STORE, "readonly");
    const store = transaction.objectStore(IDB_STORE);
    const request = operation(store);
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("idb read failed"));
  });
}

function collectIndexRecords(db: IDBDatabase, indexName: string, key: IDBValidKey): Promise<PendingSendRecord[]> {
  return new Promise((resolve, reject) => {
    const transaction = db.transaction(IDB_STORE, "readonly");
    const index = transaction.objectStore(IDB_STORE).index(indexName);
    const records: PendingSendRecord[] = [];
    const request = index.openCursor(IDBKeyRange.only(key));
    request.onsuccess = () => {
      const cursor = request.result;
      if (!cursor) {
        resolve(records);
        return;
      }
      records.push(cursor.value as PendingSendRecord);
      cursor.continue();
    };
    request.onerror = () => reject(request.error ?? new Error("idb cursor failed"));
  });
}

function putRecord(db: IDBDatabase, record: PendingSendRecord): Promise<void> {
  return runReadWrite(db, (store) => {
    store.put(record);
  });
}

function deleteRecord(db: IDBDatabase, tenantId: string, clientMsgId: string): Promise<void> {
  return runReadWrite(db, (store) => {
    store.delete([tenantId, clientMsgId]);
  });
}

async function withDatabase<T>(operation: (db: IDBDatabase) => Promise<T>): Promise<T | undefined> {
  const db = await openOfflineDatabase();
  if (!db) {
    return undefined;
  }
  return operation(db);
}

function parsePayload(record: PendingSendRecord): PendingTextSendPayload | undefined {
  try {
    const parsed: unknown = JSON.parse(record.payloadJson);
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
      return undefined;
    }
    const payload = parsed as PendingTextSendPayload;
    if (
      typeof payload.conversationId !== "string"
      || typeof payload.text !== "string"
      || typeof payload.clientMsgId !== "string"
    ) {
      return undefined;
    }
    return payload;
  } catch {
    return undefined;
  }
}

export function isRetryableH5SendError(error: unknown): boolean {
  if (error instanceof TypeError) {
    return true;
  }
  const message = error instanceof Error ? error.message : String(error);
  const normalized = message.toLowerCase();
  return (
    normalized.includes("failed to fetch")
    || normalized.includes("network")
    || normalized.includes("timeout")
    || normalized.includes("service unavailable")
    || normalized.includes("503")
    || normalized.includes("502")
    || normalized.includes("504")
  );
}

export async function enqueuePendingTextSend(payload: PendingTextSendPayload): Promise<void> {
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  await withDatabase(async (db) => {
    const tenantRecords = (await collectIndexRecords(db, "tenant_created", tenantId))
      .filter((record) => !record.flushClaimId);
    if (tenantRecords.length >= MAX_PENDING_SENDS) {
      const sorted = [...tenantRecords].sort((left, right) => left.createdAt.localeCompare(right.createdAt));
      const dropCount = tenantRecords.length - MAX_PENDING_SENDS + 1;
      for (const record of sorted.slice(0, dropCount)) {
        await deleteRecord(db, record.tenantId, record.clientMsgId);
      }
    }
    await putRecord(db, {
      tenantId,
      clientMsgId: payload.clientMsgId,
      conversationId: payload.conversationId,
      payloadJson: JSON.stringify(payload),
      createdAt: new Date().toISOString(),
      attemptCount: 0,
      flushClaimId: null,
    });
  });
}

export async function listPendingTextSends(
  limit = DEFAULT_FLUSH_LIMIT,
): Promise<PendingTextSendPayload[]> {
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return [];
  }
  const rows = await withDatabase(async (db) => {
    const records = (await collectIndexRecords(db, "tenant_created", tenantId))
      .filter((record) => !record.flushClaimId)
      .sort((left, right) => left.createdAt.localeCompare(right.createdAt))
      .slice(0, limit);
    return records;
  });
  if (!rows) {
    return [];
  }
  const payloads: PendingTextSendPayload[] = [];
  for (const record of rows) {
    const payload = parsePayload(record);
    if (payload) {
      payloads.push(payload);
    }
  }
  return payloads;
}

export async function claimPendingTextSends(
  limit = DEFAULT_FLUSH_LIMIT,
): Promise<PendingSendPayloadWithClaim[]> {
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return [];
  }
  const claimId = createPendingSendClaimId();
  const rows = await withDatabase(async (db) => {
    const candidates = (await collectIndexRecords(db, "tenant_created", tenantId))
      .filter((record) => !record.flushClaimId)
      .sort((left, right) => left.createdAt.localeCompare(right.createdAt))
      .slice(0, limit);
    for (const record of candidates) {
      await putRecord(db, {
        ...record,
        flushClaimId: claimId,
        attemptCount: record.attemptCount + 1,
      });
    }
    return collectIndexRecords(db, "tenant_claim", [tenantId, claimId]);
  });
  if (!rows) {
    return [];
  }
  const payloads: PendingSendPayloadWithClaim[] = [];
  for (const record of rows.sort((left, right) => left.createdAt.localeCompare(right.createdAt))) {
    const payload = parsePayload(record);
    if (payload) {
      payloads.push({ ...payload, claimId });
    }
  }
  return payloads;
}

export async function releasePendingTextSendClaim(
  clientMsgId: string,
  claimId: string,
): Promise<void> {
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  await withDatabase(async (db) => {
    const record = await runReadonly<PendingSendRecord | undefined>(db, (store) => store.get([tenantId, clientMsgId]));
    if (!record || record.flushClaimId !== claimId) {
      return;
    }
    await putRecord(db, { ...record, flushClaimId: null });
  });
}

export async function removePendingTextSend(clientMsgId: string): Promise<void> {
  const tenantId = resolveTenantId();
  if (!tenantId) {
    return;
  }
  await withDatabase(async (db) => {
    await deleteRecord(db, tenantId, clientMsgId);
  });
}

export async function runPendingTextSendFlush(
  flush: (pending: PendingSendPayloadWithClaim[]) => Promise<void>,
): Promise<void> {
  if (pendingSendFlushInFlight) {
    await pendingSendFlushInFlight;
    return;
  }
  pendingSendFlushInFlight = (async () => {
    const pending = await claimPendingTextSends();
    if (pending.length === 0) {
      return;
    }
    await flush(pending);
  })().finally(() => {
    pendingSendFlushInFlight = null;
  });
  await pendingSendFlushInFlight;
}
