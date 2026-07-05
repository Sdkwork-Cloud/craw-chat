export type SdkworkAppApiResult = Record<string, unknown>;

export function asRecord(value: unknown): Record<string, unknown> | null {
  if (value == null || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }
  return value as Record<string, unknown>;
}

export function readOptionalString(
  record: Record<string, unknown>,
  ...keys: string[]
): string | undefined {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

export function readString(
  record: Record<string, unknown>,
  ...keys: string[]
): string {
  return readOptionalString(record, ...keys) ?? '';
}

export function readNumber(
  record: Record<string, unknown>,
  ...keys: string[]
): number {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const parsed = Number(value);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return 0;
}

export function extractAppSdkPayload(result: SdkworkAppApiResult | unknown): unknown {
  const envelope = asRecord(result);
  if (!envelope) {
    return null;
  }
  if ('data' in envelope) {
    return envelope.data ?? null;
  }
  return envelope;
}

/** Read numeric SdkWorkApiResponse.code; non-numeric legacy codes return NaN. */
export function readSdkWorkApiCode(result: SdkworkAppApiResult | unknown): number {
  const envelope = asRecord(result);
  if (!envelope) {
    return Number.NaN;
  }
  const code = envelope.code;
  if (typeof code === 'number' && Number.isFinite(code)) {
    return code;
  }
  if (typeof code === 'string' && code.trim().length > 0) {
    const parsed = Number(code.trim());
    return Number.isFinite(parsed) ? parsed : Number.NaN;
  }
  return Number.NaN;
}

/**
 * Unwrap SdkWorkApiResponse per API_SPEC.md §4.5 — success requires numeric code === 0.
 */
export function unwrapSdkWorkApiEnvelope(result: SdkworkAppApiResult | unknown): unknown {
  const envelope = asRecord(result);
  if (!envelope) {
    return result;
  }
  if (!('code' in envelope) && !('data' in envelope)) {
    return result;
  }
  const code = readSdkWorkApiCode(envelope);
  if (code !== 0) {
    const detail = asRecord(envelope.detail);
    const message = readOptionalString(detail ?? envelope, 'title', 'detail', 'message', 'msg')
      ?? 'SDK request failed';
    throw new Error(message);
  }
  return envelope.data ?? null;
}

export function extractAppSdkRecords(payload: unknown): Record<string, unknown>[] {
  if (Array.isArray(payload)) {
    return payload.map((entry) => asRecord(entry)).filter((entry): entry is Record<string, unknown> => entry != null);
  }
  const record = asRecord(payload);
  if (!record) {
    return [];
  }
  for (const key of ['items', 'content', 'records', 'list']) {
    const nested = record[key];
    if (Array.isArray(nested)) {
      return nested.map((entry) => asRecord(entry)).filter((entry): entry is Record<string, unknown> => entry != null);
    }
  }
  return Object.keys(record).length > 0 ? [record] : [];
}

export function extractAppSdkRecordsFromResult(
  result: SdkworkAppApiResult | unknown,
): Record<string, unknown>[] {
  return extractAppSdkRecords(extractAppSdkPayload(result));
}

export function parseMoneyAmount(value: unknown): number {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === 'string' && value.trim().length > 0) {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : 0;
  }
  return 0;
}

export interface CursorListPage<T> {
  items: T[];
  hasMore?: boolean;
  nextCursor?: string | null;
}

export interface CollectCursorPagesOptions {
  maxItems?: number;
}

/** Default interactive list page size per PAGINATION_SPEC.md. */
export const SDKWORK_DEFAULT_PAGE_SIZE = 20;

/** Maximum page size unless a documented exception exists. */
export const SDKWORK_MAX_PAGE_SIZE = 200;

export interface ForEachCursorPageOptions {
  maxItems?: number;
}

export interface ForEachOffsetPageOptions {
  maxItems?: number;
  pageSize?: number;
}

/**
 * Process SDK offset pages one at a time without materializing the full collection first.
 */
export async function forEachOffsetPage<T>(
  fetchPage: (page: number, pageSize: number) => Promise<OffsetListPage<T>>,
  onPage: (items: T[], pageIndex: number) => void | Promise<void>,
  options: ForEachOffsetPageOptions = {},
): Promise<number> {
  const pageSize = options.pageSize ?? SDKWORK_DEFAULT_PAGE_SIZE;
  const maxItems = options.maxItems ?? SDKWORK_MAX_PAGE_SIZE;
  let total = 0;
  let page = 1;

  while (total < maxItems) {
    const result = await fetchPage(page, pageSize);
    const remaining = maxItems - total;
    const slice = result.items.slice(0, Math.max(0, remaining));
    if (slice.length > 0) {
      await onPage(slice, page - 1);
      total += slice.length;
    }
    if (!result.hasMore || slice.length === 0) {
      break;
    }
    page += 1;
  }

  return total;
}

/**
 * Process SDK cursor pages one at a time without materializing the full collection first.
 * Intended for bounded startup sync and cache hydration paths.
 */
export async function forEachCursorPage<T>(
  fetchPage: (cursor?: string) => Promise<CursorListPage<T>>,
  onPage: (items: T[], pageIndex: number) => void | Promise<void>,
  options: ForEachCursorPageOptions = {},
): Promise<number> {
  const maxItems = options.maxItems ?? SDKWORK_MAX_PAGE_SIZE;
  let total = 0;
  let cursor: string | undefined;
  let pageIndex = 0;

  do {
    if (total >= maxItems) {
      break;
    }
    const page = await fetchPage(cursor);
    const remaining = maxItems - total;
    const slice = page.items.slice(0, Math.max(0, remaining));
    if (slice.length > 0) {
      await onPage(slice, pageIndex);
      total += slice.length;
    }
    if (!page.hasMore || !page.nextCursor || page.nextCursor === cursor || slice.length === 0) {
      break;
    }
    cursor = page.nextCursor ?? undefined;
    pageIndex += 1;
  } while (cursor);

  return total;
}

/**
 * @deprecated Prefer `forEachCursorPage` for sync paths or explicit page APIs for interactive UI.
 * Retained for bounded operator-only aggregation with an explicit maxItems cap.
 */
export async function collectCursorPages<T>(
  fetchPage: (cursor?: string) => Promise<CursorListPage<T>>,
  options: CollectCursorPagesOptions = {},
): Promise<T[]> {
  const maxItems = options.maxItems ?? SDKWORK_MAX_PAGE_SIZE;
  const items: T[] = [];
  await forEachCursorPage(fetchPage, (pageItems) => {
    items.push(...pageItems);
  }, { maxItems });
  return items;
}

export interface OffsetListPage<T> {
  items: T[];
  page: number;
  pageSize: number;
  hasMore: boolean;
  totalItems?: number;
}

function readOffsetPageInfo(
  payload: Record<string, unknown> | null,
  page: number,
  pageSize: number,
  itemCount: number,
): Pick<OffsetListPage<unknown>, 'hasMore' | 'totalItems'> {
  const pageInfo = asRecord(payload?.pageInfo);
  const totalItems = readNumber(pageInfo ?? {}, 'totalItems', 'total_items');
  const hasMore = pageInfo?.hasMore === true
    || (totalItems > 0 ? page * pageSize < totalItems : itemCount === pageSize);
  return { hasMore, totalItems: totalItems > 0 ? totalItems : undefined };
}

export function mapAppSdkOffsetPage<T>(
  result: unknown,
  mapItem: (record: Record<string, unknown>) => T,
  page: number,
  pageSize: number,
): OffsetListPage<T> {
  const payload = asRecord(extractAppSdkPayload(result));
  const items = extractAppSdkRecordsFromResult(result).map(mapItem);
  const paging = readOffsetPageInfo(payload, page, pageSize, items.length);
  return {
    items,
    page,
    pageSize,
    hasMore: paging.hasMore ?? false,
    totalItems: paging.totalItems,
  };
}

function readCursorPageInfo(payload: Record<string, unknown> | null): Pick<CursorListPage<unknown>, 'hasMore' | 'nextCursor'> {
  const pageInfo = asRecord(payload?.pageInfo);
  const nextCursor = readOptionalString(pageInfo ?? payload ?? {}, 'nextCursor', 'cursor');
  const hasMore = pageInfo?.hasMore === true
    || payload?.hasMore === true
    || Boolean(nextCursor);
  return { hasMore, nextCursor };
}

export function mapAppSdkCursorPage<T>(
  result: unknown,
  mapItem: (record: Record<string, unknown>) => T,
): CursorListPage<T> {
  const payload = asRecord(extractAppSdkPayload(result));
  return {
    items: extractAppSdkRecordsFromResult(result).map(mapItem),
    ...readCursorPageInfo(payload),
  };
}
