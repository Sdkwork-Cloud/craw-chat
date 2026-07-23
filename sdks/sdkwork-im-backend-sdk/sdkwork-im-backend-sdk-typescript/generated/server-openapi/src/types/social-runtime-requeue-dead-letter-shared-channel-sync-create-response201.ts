import type { SocialSharedChannelSyncDeadLetterRequeueResponse } from './social-shared-channel-sync-dead-letter-requeue-response';

export interface SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncDeadLetterRequeueResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
