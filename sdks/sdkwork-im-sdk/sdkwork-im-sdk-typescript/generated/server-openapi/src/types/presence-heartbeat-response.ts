import type { PresenceView } from './presence-view';

export interface PresenceHeartbeatResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
