import type { PortalInt64Count } from './portal-int64-count';

export interface PortalConversationProjectionMetrics {
  persistSuccessCount: PortalInt64Count;
  persistFailureCount: PortalInt64Count;
  restoreSuccessCount: PortalInt64Count;
  replayBacklogSize: PortalInt64Count;
  replayedEventCount: PortalInt64Count;
}
