import type { PortalInt64Count } from './portal-int64-count';

export interface PortalConversationOperationalMetrics {
  laggingScopeCount: PortalInt64Count;
  maxOperationalLag: PortalInt64Count;
  pendingOutboxEventCount: PortalInt64Count;
  failedOutboxAttemptCount: PortalInt64Count;
}
