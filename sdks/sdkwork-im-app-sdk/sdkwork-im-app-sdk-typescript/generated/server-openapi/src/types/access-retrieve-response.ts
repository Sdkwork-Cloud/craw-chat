import type { PortalSnapshot } from './portal-snapshot';

export interface AccessRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
