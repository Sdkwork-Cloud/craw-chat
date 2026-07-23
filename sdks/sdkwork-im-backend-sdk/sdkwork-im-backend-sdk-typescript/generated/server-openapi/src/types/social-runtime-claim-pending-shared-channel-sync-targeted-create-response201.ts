import type { SocialSharedChannelSyncPendingClaimResponse } from './social-shared-channel-sync-pending-claim-response';

export interface SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncPendingClaimResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
