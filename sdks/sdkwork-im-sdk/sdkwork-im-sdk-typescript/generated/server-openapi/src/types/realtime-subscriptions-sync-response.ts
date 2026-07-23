import type { RealtimeSubscriptionSyncResponse } from './realtime-subscription-sync-response';

export interface RealtimeSubscriptionsSyncResponse {
  code: 0;
  data: unknown & { item: RealtimeSubscriptionSyncResponse; };
  /** Server-owned request correlation id. */
  traceId: string;
}
