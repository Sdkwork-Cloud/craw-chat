import type { PresenceView } from './presence-view';

export interface PresenceMeRetrieveResponse {
  code: 0;
  data: unknown & { item: PresenceView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
