import type { PortalModuleSnapshot } from './portal-module-snapshot';

export interface MediaRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
