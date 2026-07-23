import type { PresenceView } from './presence-view';

export interface PresenceHeartbeatResponse {
  code: 0;
  data: unknown & { item: PresenceView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
