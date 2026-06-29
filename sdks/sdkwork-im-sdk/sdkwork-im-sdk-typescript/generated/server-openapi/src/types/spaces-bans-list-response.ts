import type { PageInfo } from './page-info';
import type { SpaceBanView } from './space-ban-view';

export interface SpacesBansListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
