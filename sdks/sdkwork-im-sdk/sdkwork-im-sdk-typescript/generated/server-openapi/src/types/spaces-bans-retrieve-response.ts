import type { SpaceBanView } from './space-ban-view';

export interface SpacesBansRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
