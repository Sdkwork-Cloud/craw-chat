import type { SpaceBanView } from './space-ban-view';

export interface SpacesBansRetrieveResponse {
  code: 0;
  data: unknown & { item: SpaceBanView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
