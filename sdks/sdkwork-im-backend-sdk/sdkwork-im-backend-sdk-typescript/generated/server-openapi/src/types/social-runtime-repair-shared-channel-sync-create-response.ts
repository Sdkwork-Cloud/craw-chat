import type { SocialSharedChannelSyncRepairResponse } from './social-shared-channel-sync-repair-response';

export interface SocialRuntimeRepairSharedChannelSyncCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
