import type { SocialSharedChannelSyncPendingClaimResponse } from './social-shared-channel-sync-pending-claim-response';

export interface SocialRuntimeClaimPendingSharedChannelSyncTargetedCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
