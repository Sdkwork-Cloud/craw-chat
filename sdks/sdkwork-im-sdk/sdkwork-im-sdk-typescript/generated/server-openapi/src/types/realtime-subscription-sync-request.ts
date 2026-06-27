import type { RealtimeSubscriptionItemInput } from './realtime-subscription-item-input';

export interface RealtimeSubscriptionSyncRequest {
  deviceId?: string | null;
  conversations?: string[];
  items?: RealtimeSubscriptionItemInput[];
}
