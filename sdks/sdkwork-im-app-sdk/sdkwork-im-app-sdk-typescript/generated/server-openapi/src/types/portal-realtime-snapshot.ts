import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalRealtimeMetrics } from './portal-realtime-metrics';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalRealtimeSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
  metrics?: PortalRealtimeMetrics;
}
