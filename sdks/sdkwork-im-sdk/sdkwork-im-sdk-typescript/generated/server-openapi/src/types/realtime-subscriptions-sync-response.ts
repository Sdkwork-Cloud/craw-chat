import type { RealtimeSubscriptionSyncResponse } from './realtime-subscription-sync-response';

export interface RealtimeSubscriptionsSyncResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
