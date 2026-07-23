import type { SocialSharedChannelSyncPendingStaleReclaimResponse } from './social-shared-channel-sync-pending-stale-reclaim-response';

export interface SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncPendingStaleReclaimResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
