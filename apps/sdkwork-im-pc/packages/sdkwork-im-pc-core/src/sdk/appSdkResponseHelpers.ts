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
