import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalOperationalMetrics } from './portal-operational-metrics';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalDashboardSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
  metrics?: PortalOperationalMetrics;
}
