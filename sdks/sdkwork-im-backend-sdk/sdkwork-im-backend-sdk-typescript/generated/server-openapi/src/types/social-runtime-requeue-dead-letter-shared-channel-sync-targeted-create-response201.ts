import type { SocialSharedChannelSyncDeadLetterTargetedRequeueResponse } from './social-shared-channel-sync-dead-letter-targeted-requeue-response';

export interface SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse201 {
  code: 0;
  data: unknown & { item: SocialSharedChannelSyncDeadLetterTargetedRequeueResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
