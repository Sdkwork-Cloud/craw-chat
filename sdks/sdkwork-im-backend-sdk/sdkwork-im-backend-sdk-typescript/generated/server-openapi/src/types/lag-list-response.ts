import type { LagPageData } from './lag-page-data';

export interface LagListResponse {
  code: 0;
  data: unknown & LagPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
