import type { SocialSharedChannelSyncPendingTakeoverResponse } from './social-shared-channel-sync-pending-takeover-response';

export interface SocialRuntimeTakeoverPendingSharedChannelSyncTargetedCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
