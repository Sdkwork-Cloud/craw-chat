import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalModuleSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
}
