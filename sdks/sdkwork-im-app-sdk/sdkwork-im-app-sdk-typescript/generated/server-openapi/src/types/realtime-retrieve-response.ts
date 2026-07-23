import type { PortalRealtimeSnapshot } from './portal-realtime-snapshot';

export interface RealtimeRetrieveResponse {
  code: 0;
  data: unknown & { item: PortalRealtimeSnapshot; };
  /** Server-owned request correlation id. */
  traceId: string;
}
