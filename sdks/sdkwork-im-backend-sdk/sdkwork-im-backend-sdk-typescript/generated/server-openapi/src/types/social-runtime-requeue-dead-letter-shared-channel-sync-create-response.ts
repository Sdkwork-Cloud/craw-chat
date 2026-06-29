import type { SocialSharedChannelSyncDeadLetterRequeueResponse } from './social-shared-channel-sync-dead-letter-requeue-response';

export interface SocialRuntimeRequeueDeadLetterSharedChannelSyncCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
