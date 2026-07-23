import type { PageInfo } from './page-info';
import type { SpaceBanView } from './space-ban-view';

export interface SpacesBansListResponse {
  code: 0;
  data: unknown & { items: SpaceBanView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
