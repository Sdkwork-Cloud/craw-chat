import type { PresenceView } from './presence-view';

export interface PresenceHeartbeatCreateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
