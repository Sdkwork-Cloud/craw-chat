import type { SpaceBanView } from './space-ban-view';

export interface SpacesBansCreateResponse201 {
  code: 0;
  data: unknown & { item: SpaceBanView; };
  /** Server-owned request correlation id. */
  traceId: string;
}
