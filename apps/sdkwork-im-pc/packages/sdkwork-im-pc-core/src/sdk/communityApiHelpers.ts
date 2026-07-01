import type { CommunityCategory, CommunityEntry } from 'sdkwork-community-app-sdk-generated-typescript';

export type { CommunityCategory, CommunityEntry };

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
  }
  return 0;
}

export function extractCommunityItems<T>(payload: unknown): T[] {
  if (Array.isArray(payload)) {
    return payload as T[];
  }
  const record = asRecord(payload);
  if (!record) {
    return [];
  }
  if (Array.isArray(record.items)) {
    return record.items as T[];
  }
  if ('data' in record) {
    return extractCommunityItems<T>(record.data);
  }
  return [];
}

export function extractCommunityEntity<T>(payload: unknown): T | null {
  const record = asRecord(payload);
  if (!record) {
    return null;
  }
  if ('data' in record && record.data != null) {
    return record.data as T;
  }
  return record as T;
}

export function communityCategoryRecord(category: CommunityCategory): Record<string, unknown> {
  return category as unknown as Record<string, unknown>;
}

export function communityEntryRecord(entry: CommunityEntry): Record<string, unknown> {
  return entry as unknown as Record<string, unknown>;
}
