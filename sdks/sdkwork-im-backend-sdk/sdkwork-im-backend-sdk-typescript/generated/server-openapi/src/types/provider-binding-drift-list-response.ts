import type { ProviderBindingDriftPageData } from './provider-binding-drift-page-data';

export interface ProviderBindingDriftListResponse {
  code: 0;
  data: unknown & ProviderBindingDriftPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
