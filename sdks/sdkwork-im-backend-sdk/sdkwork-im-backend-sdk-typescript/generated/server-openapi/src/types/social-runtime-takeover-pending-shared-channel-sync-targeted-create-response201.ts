import type { SocialSharedChannelSyncPendingTakeoverResponse } from './social-shared-channel-sync-pending-takeover-response';

export interface SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncPendingTakeoverResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
