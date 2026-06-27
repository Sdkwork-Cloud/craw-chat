type UnknownRecord = Record<string, unknown>;

export function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? (value as UnknownRecord)
    : {};
}

export function unwrapCourseBackendEnvelope(value: unknown): unknown {
  const record = asRecord(value);
  if (!('code' in record) && !('data' in record)) {
    return value;
  }

  const code = record.code;
  const normalizedCode = code === undefined || code === null ? '2000' : String(code).trim();
  if (!['0', '200', '2000'].includes(normalizedCode)) {
    throw new Error(String(record.msg || record.message || 'Course backend request failed'));
  }
  return record.data;
}

export function readString(record: UnknownRecord, keys: string[], fallback = ''): string {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'string' && value.trim()) {
      return value.trim();
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      return String(value);
    }
  }
  return fallback;
}

export function readNumber(record: UnknownRecord, keys: string[], fallback = 0): number {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim()) {
      const parsed = Number(value.replace(/[,%\s]/gu, ''));
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return fallback;
}

export function readRecords(value: unknown, collectionKeys: string[]): UnknownRecord[] {
  const unwrapped = unwrapCourseBackendEnvelope(value);
  if (Array.isArray(unwrapped)) {
    return unwrapped.map(asRecord).filter((record) => Object.keys(record).length > 0);
  }

  const record = asRecord(unwrapped);
  for (const key of collectionKeys) {
    const nested = record[key];
    if (Array.isArray(nested)) {
      return nested.map(asRecord).filter((item) => Object.keys(item).length > 0);
    }
  }
  return [];
}

export function readSingleRecord(value: unknown): UnknownRecord {
  const unwrapped = unwrapCourseBackendEnvelope(value);
  if (Array.isArray(unwrapped)) {
    return asRecord(unwrapped[0]);
  }
  return asRecord(unwrapped);
}
