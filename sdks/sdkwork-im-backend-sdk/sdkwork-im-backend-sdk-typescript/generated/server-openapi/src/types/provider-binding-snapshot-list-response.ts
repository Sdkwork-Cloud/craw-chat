import type { ProviderBindingSnapshotPageData } from './provider-binding-snapshot-page-data';

export interface ProviderBindingSnapshotListResponse {
  code: 0;
  data: unknown & ProviderBindingSnapshotPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
