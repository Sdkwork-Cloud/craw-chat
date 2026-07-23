import type { PortalModuleSnapshot } from './portal-module-snapshot';

export interface HomeRetrieveResponse {
  code: 0;
  data: unknown & { item: PortalModuleSnapshot; };
  /** Server-owned request correlation id. */
  traceId: string;
}
