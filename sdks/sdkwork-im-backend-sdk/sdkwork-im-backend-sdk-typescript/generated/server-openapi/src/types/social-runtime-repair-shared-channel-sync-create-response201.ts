import type { SocialSharedChannelSyncRepairResponse } from './social-shared-channel-sync-repair-response';

export interface SocialRuntimeRepairSharedChannelSyncCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncRepairResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
