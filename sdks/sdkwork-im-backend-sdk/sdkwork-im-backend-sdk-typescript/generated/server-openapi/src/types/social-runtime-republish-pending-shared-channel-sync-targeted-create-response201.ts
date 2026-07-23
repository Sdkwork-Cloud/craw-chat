import type { SocialSharedChannelSyncTargetedRepublishResponse } from './social-shared-channel-sync-targeted-republish-response';

export interface SocialRuntimeRepublishPendingSharedChannelSyncTargetedCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncTargetedRepublishResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
