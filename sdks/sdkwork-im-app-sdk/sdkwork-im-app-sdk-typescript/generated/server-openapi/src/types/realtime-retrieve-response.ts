import type { PortalSnapshot } from './portal-snapshot';

export interface RealtimeRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
