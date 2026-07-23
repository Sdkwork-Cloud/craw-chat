import type { PortalConversationOperationalMetrics } from './portal-conversation-operational-metrics';
import type { PortalDataAvailability } from './portal-data-availability';
import type { PortalSnapshotMeta } from './portal-snapshot-meta';

export interface PortalConversationSnapshot {
  meta: PortalSnapshotMeta;
  availability: PortalDataAvailability;
  metrics?: PortalConversationOperationalMetrics;
}
