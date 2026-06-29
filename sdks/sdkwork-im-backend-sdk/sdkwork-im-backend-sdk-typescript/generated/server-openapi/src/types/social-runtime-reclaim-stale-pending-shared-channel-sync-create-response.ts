import type { SocialSharedChannelSyncPendingStaleReclaimResponse } from './social-shared-channel-sync-pending-stale-reclaim-response';

export interface SocialRuntimeReclaimStalePendingSharedChannelSyncCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
