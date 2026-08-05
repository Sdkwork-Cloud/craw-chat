export {
  asRecord,
  extractAppSdkRecords,
  mapAppSdkOffsetPage,
  readNumber,
  readOptionalString,
  readRecordNumber,
  readRecordString,
  readString,
  SDKWORK_DEFAULT_PAGE_SIZE,
  SDKWORK_MAX_PAGE_SIZE,
  unwrapSdkWorkApiEnvelope,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';

import {
  asRecord,
  extractAppSdkRecords,
  readNumber,
  unwrapSdkWorkApiEnvelope,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';

export function extractBackendSdkRecords(
  value: unknown,
  domainCollectionKeys: string[] = [],
): Record<string, unknown>[] {
  const payload = unwrapSdkWorkApiEnvelope(value);
  const standard = extractAppSdkRecords(payload);
  if (standard.length > 0) {
    return standard;
  }

  const record = asRecord(payload);
  if (!record) {
    return [];
  }
  for (const key of domainCollectionKeys) {
    const nested = record[key];
    if (Array.isArray(nested)) {
      return nested
        .map((entry) => asRecord(entry))
        .filter(
          (entry): entry is Record<string, unknown> =>
            entry !== null && Object.keys(entry).length > 0,
        );
    }
  }
  return [];
}

export function readBackendPageTotal(value: unknown, fallback: number): number {
  const data = asRecord(unwrapSdkWorkApiEnvelope(value));
  if (!data) {
    return fallback;
  }
  const pageInfo = asRecord(data.pageInfo);
  const total = readNumber(
    pageInfo ?? {},
    'totalItems',
    'total_items',
    'total',
    'totalElements',
    'totalCount',
    'count',
  );
  return total > 0 ? total : fallback;
}
