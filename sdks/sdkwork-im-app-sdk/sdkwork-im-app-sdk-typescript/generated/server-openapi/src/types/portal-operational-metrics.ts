import type { PortalInt64Count } from './portal-int64-count';

export interface PortalOperationalMetrics {
  clientRouteWindowCount: PortalInt64Count;
  pendingRealtimeEventCount: PortalInt64Count;
}
