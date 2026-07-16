import type { PortalInt64Count } from './portal-int64-count';

export interface PortalRealtimeMetrics {
  clientRouteWindowCount: PortalInt64Count;
  pendingEventCount: PortalInt64Count;
  maxClientRouteWindowEventCount: PortalInt64Count;
  clientRouteWindowCapacity: PortalInt64Count;
  maxClientRouteWindowUsagePermille: number;
  capacityTrimmedEventCount: PortalInt64Count;
  oldestPendingOccurredAt?: string;
}
