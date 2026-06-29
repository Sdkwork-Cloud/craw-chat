import type { SocialSharedChannelSyncDeadLetterTargetedRequeueResponse } from './social-shared-channel-sync-dead-letter-targeted-requeue-response';

export interface SocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
