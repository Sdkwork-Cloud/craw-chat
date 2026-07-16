import type { PortalDashboardSnapshot } from './portal-dashboard-snapshot';

export interface DashboardRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
