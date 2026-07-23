import type { PageInfo } from './page-info';
import type { SpaceGroupView } from './space-group-view';

export interface SpacesGroupsListResponse {
  code: 0;
  data: unknown & { items: SpaceGroupView[]; pageInfo: PageInfo; };
  /** Server-owned request correlation id. */
  traceId: string;
}
