import {
  asRecord,
  extractAppSdkRecords,
  readNumber,
  readString,
  unwrapSdkWorkApiEnvelope,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';

export { asRecord, readNumber, readString };

/** @deprecated Use unwrapSdkWorkApiEnvelope directly. */
export const unwrapCourseBackendEnvelope = unwrapSdkWorkApiEnvelope;

export function readRecords(value: unknown, collectionKeys: string[]): Record<string, unknown>[] {
  const payload = unwrapSdkWorkApiEnvelope(value);
  const standard = extractAppSdkRecords(payload);
  if (standard.length > 0) {
    return standard;
  }

  const record = asRecord(payload);
  for (const key of collectionKeys) {
    const nested = record[key];
    if (Array.isArray(nested)) {
      return nested
        .map((entry) => asRecord(entry))
        .filter((item) => Object.keys(item).length > 0);
    }
  }
  return [];
}

export function readSingleRecord(value: unknown): Record<string, unknown> {
  const unwrapped = unwrapSdkWorkApiEnvelope(value);
  if (Array.isArray(unwrapped)) {
    return asRecord(unwrapped[0]);
  }
  return asRecord(unwrapped);
}
