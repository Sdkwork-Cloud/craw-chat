import type { PortalConversationProjectionMetrics } from './portal-conversation-projection-metrics';
import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalConversationSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
  projection?: PortalConversationProjectionMetrics;
}
