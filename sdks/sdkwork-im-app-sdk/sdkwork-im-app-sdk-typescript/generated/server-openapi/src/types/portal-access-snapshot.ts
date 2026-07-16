import type { PortalAuditRecordView } from './portal-audit-record-view';
import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalAccessSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
  tenantId?: string;
  principalId?: string;
  recentItems: PortalAuditRecordView[];
  hasMore: boolean;
}
